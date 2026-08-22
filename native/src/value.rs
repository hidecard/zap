use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use crate::runtime_state::{MemoryBudget, MemoryBudgetStats, ObjectStore};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Param {
    pub(crate) name: String,
    pub(crate) annotation: Option<String>,
    pub(crate) default: Option<String>,
}
#[derive(Clone, Debug)]
pub(crate) struct StaticSignature {
    pub(crate) params: Vec<Param>,
    pub(crate) return_annotation: Option<String>,
}

/// Maximum UTF-8 payload held by one runtime text value.
pub(crate) const MAX_RUNTIME_TEXT_BYTES: usize = 8 * 1024 * 1024;
/// Maximum number of entries in one runtime list or map value.
pub(crate) const MAX_RUNTIME_COLLECTION_ITEMS: usize = 100_000;
/// Maximum number of reachable runtime values visited during one boundary check.
pub(crate) const MAX_RUNTIME_VALUE_NODES: usize = 100_000;

#[derive(Debug)]
pub(crate) struct TrackedObjectFields {
    fields: RefCell<HashMap<String, Value>>,
    store: Option<Rc<RefCell<ObjectStore>>>,
}

impl PartialEq for TrackedObjectFields {
    fn eq(&self, other: &Self) -> bool {
        self.fields.borrow().eq(&other.fields.borrow())
    }
}

impl TrackedObjectFields {
    fn new(store: Option<Rc<RefCell<ObjectStore>>>) -> Rc<Self> {
        if let Some(store_ref) = &store {
            store_ref.borrow_mut().record_allocation();
        } else {
            #[cfg(test)]
            record_object_allocation();
        }
        Rc::new(Self {
            fields: RefCell::new(HashMap::new()),
            store,
        })
    }

    pub(crate) fn try_borrow(&self) -> Result<Ref<'_, HashMap<String, Value>>, String> {
        self.fields
            .try_borrow()
            .map_err(|_| "BorrowError: object fields are already borrowed".into())
    }

    pub(crate) fn try_borrow_mut(&self) -> Result<RefMut<'_, HashMap<String, Value>>, String> {
        self.fields
            .try_borrow_mut()
            .map_err(|_| "BorrowError: object fields are already borrowed".into())
    }
}

impl Deref for TrackedObjectFields {
    type Target = RefCell<HashMap<String, Value>>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}

impl Drop for TrackedObjectFields {
    fn drop(&mut self) {
        if let Some(store) = &self.store {
            store.borrow_mut().record_deallocation();
        } else {
            #[cfg(test)]
            record_object_deallocation();
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct EnvFrame {
    values: RefCell<HashMap<String, Rc<RefCell<Value>>>>,
    parent: Option<Rc<EnvFrame>>,
}

impl EnvFrame {
    pub(crate) fn from_map(values: &HashMap<String, Value>) -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(
                values
                    .iter()
                    .map(|(name, value)| (name.clone(), Rc::new(RefCell::new(value.clone()))))
                    .collect(),
            ),
            parent: None,
        })
    }

    pub(crate) fn child(parent: Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(HashMap::new()),
            parent: Some(parent),
        })
    }

    pub(crate) fn get_local(&self, name: &str) -> Option<Value> {
        self.values
            .borrow()
            .get(name)
            .map(|cell| cell.borrow().clone())
    }

    pub(crate) fn contains_local(&self, name: &str) -> bool {
        self.values.borrow().contains_key(name)
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.contains_local(name)
            || self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.contains(name))
    }

    pub(crate) fn get(&self, name: &str) -> Option<Value> {
        self.get_local(name)
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(name)))
    }

    #[allow(dead_code)]
    pub(crate) fn insert_local(&self, name: String, value: Value) {
        self.values
            .borrow_mut()
            .insert(name, Rc::new(RefCell::new(value)));
    }

    pub(crate) fn remove_local(&self, name: &str) -> Option<Value> {
        self.values
            .borrow_mut()
            .remove(name)
            .map(|cell| match Rc::try_unwrap(cell) {
                Ok(value) => value.into_inner(),
                Err(cell) => cell.borrow().clone(),
            })
    }

    pub(crate) fn assign(&self, name: &str, value: Value) {
        let local_cell = self.values.borrow().get(name).cloned();
        if let Some(cell) = local_cell {
            *cell.borrow_mut() = value;
        } else if let Some(parent) = &self.parent {
            if parent.contains(name) {
                parent.assign(name, value);
                return;
            }
            self.insert_local(name.to_string(), value);
        } else {
            self.insert_local(name.to_string(), value);
        }
    }

    pub(crate) fn snapshot(&self) -> HashMap<String, Value> {
        let mut values = self
            .parent
            .as_ref()
            .map(|parent| parent.snapshot())
            .unwrap_or_default();
        values.extend(
            self.values
                .borrow()
                .iter()
                .map(|(key, cell)| (key.clone(), cell.borrow().clone())),
        );
        values
    }

    pub(crate) fn capture_keys(&self) -> Vec<String> {
        let mut keys = self.snapshot().into_keys().collect::<Vec<_>>();
        keys.sort();
        keys
    }

    pub(crate) fn sync_captured(&self, keys: &[String], values: &HashMap<String, Value>) {
        for key in keys {
            if let Some(value) = values.get(key) {
                self.assign(key, value.clone());
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn sync_from_snapshot(&self, values: &HashMap<String, Value>) {
        let local_keys = self.values.borrow().keys().cloned().collect::<Vec<_>>();
        for key in local_keys {
            if let Some(value) = values.get(&key) {
                self.assign(&key, value.clone());
            }
        }
        for (key, value) in values {
            if self.contains_local(key) {
                continue;
            }
            if self
                .parent
                .as_ref()
                .is_some_and(|parent| parent.contains(key))
            {
                self.parent
                    .as_ref()
                    .expect("parent checked")
                    .assign(key, value.clone());
            } else {
                self.insert_local(key.clone(), value.clone());
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Function {
    pub(crate) visibility: String,
    pub(crate) params: Vec<Param>,
    pub(crate) return_annotation: Option<String>,
    pub(crate) is_async: bool,
    /// Legacy source lines retained for compatibility with older declarations.
    pub(crate) body: Vec<String>,
    /// Native AST body used by the migration path when available.
    pub(crate) ast_body: Option<crate::ast::Program>,
    pub(crate) closure: Rc<EnvFrame>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Text(String),
    Number(i64),
    Bool(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
    Object {
        class_name: String,
        fields: Rc<TrackedObjectFields>,
    },
    Callable(Rc<Function>),
    ResultOk(Box<Value>),
    ResultErr(Box<Value>),
    OptionSome(Box<Value>),
    OptionNone,
    Future(Box<Value>),
    ScheduledFuture(u64),
    None,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemoryStats {
    pub(crate) live_objects: usize,
    pub(crate) object_allocations: u64,
    pub(crate) object_deallocations: u64,
}

#[cfg(test)]
thread_local! {
    static TEST_MEMORY_STATS: RefCell<MemoryStats> = const { RefCell::new(MemoryStats {
        live_objects: 0,
        object_allocations: 0,
        object_deallocations: 0,
    }) };
}

#[cfg(test)]
fn record_object_allocation() {
    TEST_MEMORY_STATS.with(|stats| {
        let mut stats = stats.borrow_mut();
        stats.live_objects = stats.live_objects.saturating_add(1);
        stats.object_allocations = stats.object_allocations.saturating_add(1);
    });
}

#[cfg(test)]
fn record_object_deallocation() {
    TEST_MEMORY_STATS.with(|stats| {
        let mut stats = stats.borrow_mut();
        stats.live_objects = stats.live_objects.saturating_sub(1);
        stats.object_deallocations = stats.object_deallocations.saturating_add(1);
    });
}

#[cfg(test)]
pub(crate) fn memory_stats() -> MemoryStats {
    TEST_MEMORY_STATS.with(|stats| *stats.borrow())
}

impl Value {
    /// Construct an object whose fields are reference-counted independently from
    /// the value handle. This keeps object ownership explicit at runtime.
    #[cfg(test)]
    pub(crate) fn object(class_name: impl Into<String>) -> Self {
        Self::object_with_store(class_name, None)
    }

    pub(crate) fn object_with_store(
        class_name: impl Into<String>,
        store: Option<Rc<RefCell<ObjectStore>>>,
    ) -> Self {
        Self::Object {
            class_name: class_name.into(),
            fields: TrackedObjectFields::new(store),
        }
    }

    /// Return the stable, bounded memory-statistics record exposed by the runtime.
    #[cfg(test)]
    pub(crate) fn memory_stats_value() -> Self {
        Self::memory_stats_value_for_store(None, None)
    }

    pub(crate) fn memory_stats_value_for_store(
        store: Option<&Rc<RefCell<ObjectStore>>>,
        budget: Option<&MemoryBudgetStats>,
    ) -> Self {
        let (live_objects, object_allocations, object_deallocations) = store
            .map(|store| store.borrow().stats())
            .unwrap_or_default();
        let (cleanup_attempts, cleanup_successes, cleanup_failures, validation_runs) = store
            .map(|store| store.borrow().lifecycle_stats())
            .unwrap_or_default();
        let budget = budget
            .copied()
            .unwrap_or_else(|| MemoryBudget::new().stats());
        let mut values = HashMap::new();
        values.insert("live_objects".into(), Self::Number(live_objects as i64));
        values.insert(
            "object_allocations".into(),
            Self::Number(object_allocations.min(i64::MAX as u64) as i64),
        );
        values.insert(
            "object_deallocations".into(),
            Self::Number(object_deallocations.min(i64::MAX as u64) as i64),
        );
        values.insert(
            "max_text_bytes".into(),
            Self::Number(MAX_RUNTIME_TEXT_BYTES as i64),
        );
        values.insert(
            "max_collection_items".into(),
            Self::Number(MAX_RUNTIME_COLLECTION_ITEMS as i64),
        );
        values.insert(
            "max_value_nodes".into(),
            Self::Number(MAX_RUNTIME_VALUE_NODES as i64),
        );
        values.insert(
            "weak_references".into(),
            Self::Text("unsupported_public_api".into()),
        );
        values.insert(
            "tracing_collector".into(),
            Self::Text("not_implemented".into()),
        );
        values.insert(
            "cleanup_attempts".into(),
            Self::Number(cleanup_attempts as i64),
        );
        values.insert(
            "cleanup_successes".into(),
            Self::Number(cleanup_successes as i64),
        );
        values.insert(
            "cleanup_failures".into(),
            Self::Number(cleanup_failures as i64),
        );
        values.insert(
            "validation_runs".into(),
            Self::Number(validation_runs as i64),
        );
        values.insert("max_bytes".into(), Self::Number(budget.max_bytes as i64));
        values.insert("used_bytes".into(), Self::Number(budget.used_bytes as i64));
        values.insert("max_tasks".into(), Self::Number(budget.max_tasks as i64));
        values.insert(
            "admitted_tasks".into(),
            Self::Number(budget.admitted_tasks as i64),
        );
        values.insert(
            "max_output_bytes".into(),
            Self::Number(budget.max_output_bytes as i64),
        );
        values.insert(
            "used_output_bytes".into(),
            Self::Number(budget.used_output_bytes as i64),
        );
        Self::Map(values)
    }

    /// Validate a value at a public runtime boundary without recursing forever
    /// through cyclic object graphs.
    pub(crate) fn validate_memory_limits(&self) -> Result<(), String> {
        if let Self::Object { fields, .. } = self {
            if let Some(store) = &fields.store {
                store.borrow_mut().record_validation();
            }
        }
        let mut visited_objects = HashSet::new();
        let mut nodes = 0usize;
        validate_value(
            self,
            &mut visited_objects,
            &mut nodes,
            0,
            MAX_RUNTIME_VALUE_NODES,
        )
    }

    /// Remove all fields from an object, which is the explicit cycle-breaking
    /// operation used by embedders before releasing cyclic object graphs.
    #[allow(dead_code)]
    pub(crate) fn clear_object_fields(&self) -> Result<bool, String> {
        if let Self::Object { fields, .. } = self {
            let store = fields.store.clone();
            if let Some(store) = &store {
                store.borrow_mut().record_cleanup_attempt();
            }
            let result = fields.try_borrow_mut();
            match result {
                Ok(mut fields) => {
                    fields.clear();
                    if let Some(store) = store {
                        store.borrow_mut().record_cleanup_success();
                    }
                    Ok(true)
                }
                Err(error) => {
                    if let Some(store) = store {
                        store.borrow_mut().record_cleanup_failure();
                    }
                    Err(error)
                }
            }
        } else {
            Ok(false)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn object_field_count(&self) -> Result<Option<usize>, String> {
        match self {
            Self::Object { fields, .. } => Ok(Some(fields.try_borrow()?.len())),
            _ => Ok(None),
        }
    }

    pub(crate) fn show(&self) -> String {
        match self {
            Self::Text(x) => x.clone(),
            Self::Number(x) => x.to_string(),
            Self::Bool(x) => x.to_string(),
            Self::List(x) => format!(
                "[{}]",
                x.iter().map(Self::show).collect::<Vec<_>>().join(", ")
            ),
            Self::Map(x) => format!("{{{} keys}}", x.len()),
            Self::Object { class_name, .. } => format!("<object {class_name}>"),
            Self::Callable(_) => "<callable>".into(),
            Self::ResultOk(x) => format!("Ok({})", x.show()),
            Self::ResultErr(x) => format!("Err({})", x.show()),
            Self::OptionSome(x) => format!("Some({})", x.show()),
            Self::OptionNone => "Option.none".into(),
            Self::Future(value) => format!("Future({})", value.show()),
            Self::ScheduledFuture(id) => format!("Future(task#{id})"),
            Self::None => "none".into(),
        }
    }
    pub(crate) fn truthy(&self) -> bool {
        match self {
            Self::Bool(x) => *x,
            Self::Number(x) => *x != 0,
            Self::Text(x) => !x.is_empty(),
            Self::List(x) => !x.is_empty(),
            Self::Map(x) => !x.is_empty(),
            Self::Object { .. } | Self::Callable(_) => true,
            Self::ResultOk(_) => true,
            Self::ResultErr(_) => false,
            Self::OptionSome(_) => true,
            Self::Future(_) | Self::ScheduledFuture(_) => true,
            Self::OptionNone | Self::None => false,
        }
    }
}

fn validate_value(
    value: &Value,
    visited_objects: &mut HashSet<usize>,
    nodes: &mut usize,
    depth: usize,
    node_limit: usize,
) -> Result<(), String> {
    *nodes = nodes.saturating_add(1);
    if *nodes > node_limit {
        return Err(format!(
            "memory limit exceeded: value graph contains more than {node_limit} nodes"
        ));
    }
    if depth > node_limit {
        return Err("memory limit exceeded: value nesting is too deep".into());
    }
    match value {
        Value::Text(text) if text.len() > MAX_RUNTIME_TEXT_BYTES => Err(format!(
            "memory limit exceeded: text value exceeds {MAX_RUNTIME_TEXT_BYTES} bytes"
        )),
        Value::Text(_) => Ok(()),
        Value::List(values) => {
            if values.len() > MAX_RUNTIME_COLLECTION_ITEMS {
                return Err(format!(
                    "memory limit exceeded: list contains more than {MAX_RUNTIME_COLLECTION_ITEMS} items"
                ));
            }
            for nested in values {
                validate_value(nested, visited_objects, nodes, depth + 1, node_limit)?;
            }
            Ok(())
        }
        Value::Map(values) => {
            if values.len() > MAX_RUNTIME_COLLECTION_ITEMS {
                return Err(format!(
                    "memory limit exceeded: map contains more than {MAX_RUNTIME_COLLECTION_ITEMS} entries"
                ));
            }
            for (key, nested) in values {
                if key.len() > MAX_RUNTIME_TEXT_BYTES {
                    return Err(format!(
                        "memory limit exceeded: map key exceeds {MAX_RUNTIME_TEXT_BYTES} bytes"
                    ));
                }
                validate_value(nested, visited_objects, nodes, depth + 1, node_limit)?;
            }
            Ok(())
        }
        Value::Object { fields, .. } => {
            let identity = Rc::as_ptr(fields) as usize;
            if visited_objects.insert(identity) {
                for nested in fields.try_borrow()?.values() {
                    validate_value(nested, visited_objects, nodes, depth + 1, node_limit)?;
                }
            }
            Ok(())
        }
        Value::ResultOk(nested)
        | Value::ResultErr(nested)
        | Value::OptionSome(nested)
        | Value::Future(nested) => {
            validate_value(nested, visited_objects, nodes, depth + 1, node_limit)
        }
        Value::Bool(_)
        | Value::Number(_)
        | Value::Callable(_)
        | Value::ScheduledFuture(_)
        | Value::OptionNone
        | Value::None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{memory_stats, Value, MAX_RUNTIME_COLLECTION_ITEMS, MAX_RUNTIME_TEXT_BYTES};
    use crate::runtime_state::ExecutionContext;
    use std::rc::Rc;

    #[test]
    fn cyclic_object_graph_can_be_explicitly_broken() {
        let baseline = memory_stats();
        let object = Value::object("Node");
        let Value::Object { fields, .. } = &object else {
            panic!("object constructor must create an object value");
        };
        let weak_fields = Rc::downgrade(fields);
        fields
            .try_borrow_mut()
            .unwrap()
            .insert("self".into(), object.clone());
        assert_eq!(object.object_field_count().unwrap(), Some(1));
        assert_eq!(memory_stats().live_objects, baseline.live_objects + 1);
        assert!(weak_fields.upgrade().is_some());
        assert!(object.validate_memory_limits().is_ok());

        assert!(object.clear_object_fields().unwrap());
        assert_eq!(object.object_field_count().unwrap(), Some(0));
        drop(object);
        assert!(weak_fields.upgrade().is_none());
        assert_eq!(memory_stats().live_objects, baseline.live_objects);
        assert!(memory_stats().object_deallocations > baseline.object_deallocations);
    }

    #[test]
    fn memory_stats_expose_stable_limits_and_deferred_features() {
        let Value::Map(stats) = Value::memory_stats_value() else {
            panic!("memory stats must be a map");
        };
        assert_eq!(
            stats["max_text_bytes"],
            Value::Number(MAX_RUNTIME_TEXT_BYTES as i64)
        );
        assert_eq!(
            stats["max_collection_items"],
            Value::Number(MAX_RUNTIME_COLLECTION_ITEMS as i64)
        );
        assert_eq!(
            stats["weak_references"],
            Value::Text("unsupported_public_api".into())
        );
        assert_eq!(
            stats["tracing_collector"],
            Value::Text("not_implemented".into())
        );
    }

    #[test]
    fn conflicting_object_borrows_return_typed_failures() {
        let object = Value::object("Borrowed");
        let Value::Object { fields, .. } = &object else {
            panic!("object constructor must create an object value");
        };
        let _active_borrow = fields.try_borrow_mut().unwrap();
        assert_eq!(
            object.object_field_count().unwrap_err(),
            "BorrowError: object fields are already borrowed"
        );
        assert_eq!(
            object.clear_object_fields().unwrap_err(),
            "BorrowError: object fields are already borrowed"
        );
    }

    #[test]
    fn context_object_store_reports_validation_and_cleanup_lifecycle() {
        let context = ExecutionContext::new();
        let store = context.state().object_store().clone();
        let object = Value::object_with_store("Tracked", Some(store.clone()));

        object
            .validate_memory_limits()
            .expect("a small object should validate");
        assert_eq!(store.borrow().lifecycle_stats(), (0, 0, 0, 1));
        assert!(object
            .clear_object_fields()
            .expect("cleanup should succeed"));
        assert_eq!(store.borrow().lifecycle_stats(), (1, 1, 0, 1));

        let _active_borrow = match &object {
            Value::Object { fields, .. } => fields.try_borrow_mut().unwrap(),
            _ => panic!("object constructor must create an object value"),
        };
        assert!(object.clear_object_fields().is_err());
        assert_eq!(store.borrow().lifecycle_stats(), (2, 1, 1, 1));
    }

    #[test]
    fn memory_limit_validation_is_cycle_safe_and_bounded() {
        let oversized = Value::Text("x".repeat(MAX_RUNTIME_TEXT_BYTES + 1));
        let error = oversized
            .validate_memory_limits()
            .expect_err("oversized text must be rejected");
        assert!(error.contains("text value exceeds"));

        let oversized_list = Value::List(vec![Value::None; MAX_RUNTIME_COLLECTION_ITEMS + 1]);
        let error = oversized_list
            .validate_memory_limits()
            .expect_err("oversized lists must be rejected");
        assert!(error.contains("list contains more than"));

        let object = Value::object("Cycle");
        let Value::Object { fields, .. } = &object else {
            panic!("object constructor must create an object value");
        };
        fields
            .try_borrow_mut()
            .unwrap()
            .insert("self".into(), object.clone());
        assert!(object.validate_memory_limits().is_ok());
        object.clear_object_fields().unwrap();
    }
}
