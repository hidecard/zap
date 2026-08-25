use std::{
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    ops::Deref,
    path::{Path, PathBuf},
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
    pub(crate) type_params: Vec<String>,
    pub(crate) return_annotation: Option<String>,
}

/// Maximum UTF-8 payload held by one runtime text value.
pub(crate) const MAX_RUNTIME_TEXT_BYTES: usize = 8 * 1024 * 1024;
/// Maximum number of entries in one runtime list or map value.
pub(crate) const MAX_RUNTIME_COLLECTION_ITEMS: usize = 100_000;
/// Maximum number of reachable runtime values visited during one boundary check.
pub(crate) const MAX_RUNTIME_VALUE_NODES: usize = 100_000;

/// Collect runtime values without materializing more than the collection budget.
pub(crate) fn collect_bounded_values(
    values: impl IntoIterator<Item = Value>,
    operation: &str,
) -> Result<Vec<Value>, String> {
    let mut output = Vec::new();
    for value in values {
        if output.len() >= MAX_RUNTIME_COLLECTION_ITEMS {
            return Err(format!(
                "memory limit exceeded: {operation} produced more than {MAX_RUNTIME_COLLECTION_ITEMS} items"
            ));
        }
        output.push(value);
    }
    Ok(output)
}

#[derive(Debug)]
pub(crate) struct TrackedObjectFields {
    fields: RefCell<HashMap<String, Value>>,
    store: Option<Rc<RefCell<ObjectStore>>>,
}

impl PartialEq for TrackedObjectFields {
    fn eq(&self, other: &Self) -> bool {
        match (self.fields.try_borrow(), other.fields.try_borrow()) {
            (Ok(left), Ok(right)) => left.eq(&right),
            _ => false,
        }
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
    base_path: Option<PathBuf>,
}

impl EnvFrame {
    pub(crate) fn from_map(values: &HashMap<String, Value>) -> Rc<Self> {
        Self::from_map_with_base_option(values, None)
    }

    pub(crate) fn from_map_with_base(values: &HashMap<String, Value>, base: &Path) -> Rc<Self> {
        Self::from_map_with_base_option(values, Some(base.to_path_buf()))
    }

    fn from_map_with_base_option(
        values: &HashMap<String, Value>,
        base_path: Option<PathBuf>,
    ) -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(
                values
                    .iter()
                    .map(|(name, value)| (name.clone(), Rc::new(RefCell::new(value.clone()))))
                    .collect(),
            ),
            parent: None,
            base_path,
        })
    }

    pub(crate) fn child(parent: Rc<Self>) -> Rc<Self> {
        Rc::new(Self {
            values: RefCell::new(HashMap::new()),
            parent: Some(parent),
            base_path: None,
        })
    }

    pub(crate) fn base_path(&self) -> Option<PathBuf> {
        self.base_path
            .clone()
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.base_path()))
    }

    fn borrow_error() -> String {
        "BorrowError: environment frame is already borrowed".into()
    }

    pub(crate) fn try_get_local(&self, name: &str) -> Result<Option<Value>, String> {
        let values = self.values.try_borrow().map_err(|_| Self::borrow_error())?;
        let Some(cell) = values.get(name) else {
            return Ok(None);
        };
        cell.try_borrow()
            .map(|value| Some(value.clone()))
            .map_err(|_| Self::borrow_error())
    }

    #[allow(dead_code)]
    pub(crate) fn get_local(&self, name: &str) -> Option<Value> {
        self.try_get_local(name).ok().flatten()
    }

    pub(crate) fn try_contains_local(&self, name: &str) -> Result<bool, String> {
        Ok(self
            .values
            .try_borrow()
            .map_err(|_| Self::borrow_error())?
            .contains_key(name))
    }

    #[allow(dead_code)]
    pub(crate) fn contains_local(&self, name: &str) -> bool {
        self.try_contains_local(name).unwrap_or(false)
    }

    pub(crate) fn try_contains(&self, name: &str) -> Result<bool, String> {
        if self.try_contains_local(name)? {
            return Ok(true);
        }
        self.parent
            .as_ref()
            .map_or(Ok(false), |parent| parent.try_contains(name))
    }

    #[allow(dead_code)]
    pub(crate) fn contains(&self, name: &str) -> bool {
        self.try_contains(name).unwrap_or(false)
    }

    pub(crate) fn try_get(&self, name: &str) -> Result<Option<Value>, String> {
        if let Some(value) = self.try_get_local(name)? {
            return Ok(Some(value));
        }
        self.parent
            .as_ref()
            .map_or(Ok(None), |parent| parent.try_get(name))
    }

    #[allow(dead_code)]
    pub(crate) fn get(&self, name: &str) -> Option<Value> {
        self.try_get(name).ok().flatten()
    }

    pub(crate) fn try_insert_local(&self, name: String, value: Value) -> Result<(), String> {
        self.values
            .try_borrow_mut()
            .map_err(|_| Self::borrow_error())?
            .insert(name, Rc::new(RefCell::new(value)));
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn insert_local(&self, name: String, value: Value) {
        let _ = self.try_insert_local(name, value);
    }

    pub(crate) fn try_remove_local(&self, name: &str) -> Result<Option<Value>, String> {
        let cell = self
            .values
            .try_borrow_mut()
            .map_err(|_| Self::borrow_error())?
            .remove(name);
        let Some(cell) = cell else {
            return Ok(None);
        };
        match Rc::try_unwrap(cell) {
            Ok(value) => Ok(Some(value.into_inner())),
            Err(cell) => cell
                .try_borrow()
                .map(|value| Some(value.clone()))
                .map_err(|_| Self::borrow_error()),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn remove_local(&self, name: &str) -> Option<Value> {
        self.try_remove_local(name).ok().flatten()
    }

    pub(crate) fn try_assign(&self, name: &str, value: Value) -> Result<(), String> {
        let local_cell = self
            .values
            .try_borrow()
            .map_err(|_| Self::borrow_error())?
            .get(name)
            .cloned();
        if let Some(cell) = local_cell {
            *cell.try_borrow_mut().map_err(|_| Self::borrow_error())? = value;
        } else if let Some(parent) = &self.parent {
            if parent.try_contains(name)? {
                parent.try_assign(name, value)?;
                return Ok(());
            }
            self.try_insert_local(name.to_string(), value)?;
        } else {
            self.try_insert_local(name.to_string(), value)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn assign(&self, name: &str, value: Value) {
        let _ = self.try_assign(name, value);
    }

    pub(crate) fn try_snapshot(&self) -> Result<HashMap<String, Value>, String> {
        let mut values = self
            .parent
            .as_ref()
            .map(|parent| parent.try_snapshot())
            .transpose()?
            .unwrap_or_default();
        let local_values = self.values.try_borrow().map_err(|_| Self::borrow_error())?;
        for (key, cell) in local_values.iter() {
            let value = cell.try_borrow().map_err(|_| Self::borrow_error())?;
            values.insert(key.clone(), value.clone());
        }
        Ok(values)
    }

    #[allow(dead_code)]
    pub(crate) fn snapshot(&self) -> HashMap<String, Value> {
        self.try_snapshot().unwrap_or_default()
    }

    pub(crate) fn try_capture_keys(&self) -> Result<Vec<String>, String> {
        let mut keys = self.try_snapshot()?.into_keys().collect::<Vec<_>>();
        keys.sort();
        Ok(keys)
    }

    #[allow(dead_code)]
    pub(crate) fn capture_keys(&self) -> Vec<String> {
        self.try_capture_keys().unwrap_or_default()
    }

    pub(crate) fn try_sync_captured(
        &self,
        keys: &[String],
        values: &HashMap<String, Value>,
    ) -> Result<(), String> {
        for key in keys {
            if let Some(value) = values.get(key) {
                self.try_assign(key, value.clone())?;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn sync_captured(&self, keys: &[String], values: &HashMap<String, Value>) {
        let _ = self.try_sync_captured(keys, values);
    }

    pub(crate) fn try_sync_from_snapshot(
        &self,
        values: &HashMap<String, Value>,
    ) -> Result<(), String> {
        let local_keys = self
            .values
            .try_borrow()
            .map_err(|_| Self::borrow_error())?
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        for key in local_keys {
            if let Some(value) = values.get(&key) {
                self.try_assign(&key, value.clone())?;
            }
        }
        for (key, value) in values {
            if self.try_contains_local(key)? {
                continue;
            }
            if let Some(parent) = self.parent.as_ref() {
                if parent.try_contains(key)? {
                    parent.try_assign(key, value.clone())?;
                    continue;
                }
            }
            self.try_insert_local(key.clone(), value.clone())?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub(crate) fn sync_from_snapshot(&self, values: &HashMap<String, Value>) {
        let _ = self.try_sync_from_snapshot(values);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Function {
    pub(crate) visibility: String,
    pub(crate) params: Vec<Param>,
    pub(crate) type_params: Vec<String>,
    pub(crate) return_annotation: Option<String>,
    pub(crate) is_async: bool,
    /// Legacy source lines retained for compatibility with older declarations.
    pub(crate) body: Vec<String>,
    /// Native AST body used by the migration path when available.
    pub(crate) ast_body: Option<crate::ast::Program>,
    pub(crate) closure: Rc<EnvFrame>,
}
#[derive(Clone, Debug)]
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

pub(crate) fn try_values_equal(left: &Value, right: &Value) -> Result<bool, String> {
    let mut seen_objects = HashSet::new();
    let mut nodes = 0usize;
    values_equal_inner(left, right, &mut seen_objects, &mut nodes)
}

fn values_equal_inner(
    left: &Value,
    right: &Value,
    seen_objects: &mut HashSet<(usize, usize)>,
    nodes: &mut usize,
) -> Result<bool, String> {
    *nodes = nodes
        .checked_add(1)
        .ok_or_else(|| "value equality node counter overflow".to_string())?;
    if *nodes > MAX_RUNTIME_VALUE_NODES {
        return Err(format!(
            "value equality exceeded {MAX_RUNTIME_VALUE_NODES} nodes"
        ));
    }
    match (left, right) {
        (Value::Text(left), Value::Text(right)) => Ok(left == right),
        (Value::Number(left), Value::Number(right)) => Ok(left == right),
        (Value::Bool(left), Value::Bool(right)) => Ok(left == right),
        (Value::List(left), Value::List(right)) => {
            if left.len() != right.len() {
                return Ok(false);
            }
            left.iter()
                .zip(right)
                .try_fold(true, |equal, (left, right)| {
                    if !equal {
                        return Ok(false);
                    }
                    values_equal_inner(left, right, seen_objects, nodes)
                })
        }
        (Value::Map(left), Value::Map(right)) => {
            if left.len() != right.len() {
                return Ok(false);
            }
            left.iter().try_fold(true, |equal, (key, left)| {
                if !equal {
                    return Ok(false);
                }
                let Some(right) = right.get(key) else {
                    return Ok(false);
                };
                values_equal_inner(left, right, seen_objects, nodes)
            })
        }
        (
            Value::Object {
                class_name: left_class,
                fields: left_fields,
            },
            Value::Object {
                class_name: right_class,
                fields: right_fields,
            },
        ) => {
            if left_class != right_class {
                return Ok(false);
            }
            let identity = (
                Rc::as_ptr(left_fields) as usize,
                Rc::as_ptr(right_fields) as usize,
            );
            if !seen_objects.insert(identity) {
                return Ok(true);
            }
            let left_fields = left_fields.try_borrow()?;
            let right_fields = right_fields.try_borrow()?;
            if left_fields.len() != right_fields.len() {
                return Ok(false);
            }
            left_fields.iter().try_fold(true, |equal, (key, left)| {
                if !equal {
                    return Ok(false);
                }
                let Some(right) = right_fields.get(key) else {
                    return Ok(false);
                };
                values_equal_inner(left, right, seen_objects, nodes)
            })
        }
        (Value::Callable(left), Value::Callable(right)) => Ok(Rc::ptr_eq(left, right)),
        (Value::ResultOk(left), Value::ResultOk(right))
        | (Value::ResultErr(left), Value::ResultErr(right))
        | (Value::OptionSome(left), Value::OptionSome(right))
        | (Value::Future(left), Value::Future(right)) => {
            values_equal_inner(left, right, seen_objects, nodes)
        }
        (Value::OptionNone, Value::OptionNone)
        | (Value::ScheduledFuture(_), Value::ScheduledFuture(_))
        | (Value::None, Value::None) => match (left, right) {
            (Value::ScheduledFuture(left), Value::ScheduledFuture(right)) => Ok(left == right),
            _ => Ok(true),
        },
        _ => Ok(false),
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        try_values_equal(self, other).unwrap_or(false)
    }
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
            "cycle_policy".into(),
            Self::Text("explicit_clear_object_fields".into()),
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
        let mut visited_frames = HashSet::new();
        let mut visited_functions = HashSet::new();
        let mut nodes = 0usize;
        validate_value(
            self,
            &mut visited_objects,
            &mut visited_frames,
            &mut visited_functions,
            &mut nodes,
            0,
            MAX_RUNTIME_VALUE_NODES,
        )
    }

    /// Return the deterministic logical charge for a fully materialized value.
    /// This is a quota estimate, not an allocator or process-memory measurement.
    pub(crate) fn logical_size(&self) -> Result<u64, String> {
        self.validate_memory_limits()?;
        let mut visited_objects = HashSet::new();
        let mut visited_frames = HashSet::new();
        let mut visited_functions = HashSet::new();
        let mut nodes = 0usize;
        logical_value_size(
            self,
            &mut visited_objects,
            &mut visited_frames,
            &mut visited_functions,
            &mut nodes,
        )
    }

    /// Return the charge for a container's own storage and metadata. Nested
    /// values are charged when they are materialized or by `logical_size` for
    /// values created atomically by a builtin such as JSON decoding.
    pub(crate) fn logical_shallow_size(&self) -> Result<u64, String> {
        self.validate_memory_limits()?;
        let size = match self {
            Self::Text(text) => logical_add(16, text.len()),
            Self::List(values) => logical_add(24, values.len().saturating_mul(8)),
            Self::Map(values) => {
                let key_bytes = values
                    .keys()
                    .try_fold(0usize, |total, key| total.checked_add(key.len()))
                    .ok_or_else(|| "memory budget exceeded: map key charge overflow".to_string())?;
                logical_add(32, values.len().saturating_mul(8).saturating_add(key_bytes))
            }
            Self::Object { class_name, fields } => {
                let field_count = fields.try_borrow()?.len();
                logical_add(
                    64u64.saturating_add(class_name.len() as u64),
                    field_count.saturating_mul(32),
                )
            }
            Self::Callable(function) => logical_function_shallow_size(function),
            Self::ResultOk(_) | Self::ResultErr(_) | Self::OptionSome(_) | Self::Future(_) => {
                Ok(16)
            }
            Self::Number(_)
            | Self::Bool(_)
            | Self::OptionNone
            | Self::ScheduledFuture(_)
            | Self::None => Ok(16),
        }?;
        Ok(size)
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

fn logical_add(base: u64, extra: usize) -> Result<u64, String> {
    let extra = u64::try_from(extra)
        .map_err(|_| "memory budget exceeded: logical size conversion overflow".to_string())?;
    base.checked_add(extra)
        .ok_or_else(|| "memory budget exceeded: logical size overflow".to_string())
}

fn logical_value_size(
    value: &Value,
    visited_objects: &mut HashSet<usize>,
    visited_frames: &mut HashSet<usize>,
    visited_functions: &mut HashSet<usize>,
    nodes: &mut usize,
) -> Result<u64, String> {
    *nodes = nodes
        .checked_add(1)
        .ok_or_else(|| "memory budget exceeded: logical node counter overflow".to_string())?;
    if *nodes > MAX_RUNTIME_VALUE_NODES {
        return Err(format!(
            "memory limit exceeded: value graph contains more than {MAX_RUNTIME_VALUE_NODES} nodes"
        ));
    }
    match value {
        Value::Text(text) => logical_add(16, text.len()),
        Value::Number(_)
        | Value::Bool(_)
        | Value::OptionNone
        | Value::ScheduledFuture(_)
        | Value::None => Ok(16),
        Value::List(values) => {
            let mut total = logical_add(24, values.len().saturating_mul(8))?;
            for nested in values {
                total = total
                    .checked_add(logical_value_size(
                        nested,
                        visited_objects,
                        visited_frames,
                        visited_functions,
                        nodes,
                    )?)
                    .ok_or_else(|| {
                        "memory budget exceeded: logical list size overflow".to_string()
                    })?;
            }
            Ok(total)
        }
        Value::Map(values) => {
            let key_bytes = values
                .keys()
                .try_fold(0usize, |total, key| total.checked_add(key.len()))
                .ok_or_else(|| "memory budget exceeded: map key charge overflow".to_string())?;
            let mut total =
                logical_add(32, values.len().saturating_mul(8).saturating_add(key_bytes))?;
            for (key, nested) in values {
                total = total
                    .checked_add(logical_add(0, key.len())?)
                    .ok_or_else(|| {
                        "memory budget exceeded: logical map size overflow".to_string()
                    })?;
                total = total
                    .checked_add(logical_value_size(
                        nested,
                        visited_objects,
                        visited_frames,
                        visited_functions,
                        nodes,
                    )?)
                    .ok_or_else(|| {
                        "memory budget exceeded: logical map size overflow".to_string()
                    })?;
            }
            Ok(total)
        }
        Value::Object { class_name, fields } => {
            let identity = Rc::as_ptr(fields) as usize;
            if !visited_objects.insert(identity) {
                return Ok(0);
            }
            let field_values = fields.try_borrow()?;
            let mut total = logical_add(
                64,
                class_name
                    .len()
                    .checked_add(field_values.len().saturating_mul(32))
                    .ok_or_else(|| "memory budget exceeded: object charge overflow".to_string())?,
            )?;
            for nested in field_values.values() {
                total = total
                    .checked_add(logical_value_size(
                        nested,
                        visited_objects,
                        visited_frames,
                        visited_functions,
                        nodes,
                    )?)
                    .ok_or_else(|| {
                        "memory budget exceeded: logical object size overflow".to_string()
                    })?;
            }
            Ok(total)
        }
        Value::Callable(function) => {
            let identity = Rc::as_ptr(function) as usize;
            if !visited_functions.insert(identity) {
                return Ok(0);
            }
            let mut total = logical_function_shallow_size(function)?;
            total = total
                .checked_add(logical_frame_size(
                    &function.closure,
                    visited_objects,
                    visited_frames,
                    visited_functions,
                    nodes,
                )?)
                .ok_or_else(|| {
                    "memory budget exceeded: logical closure size overflow".to_string()
                })?;
            Ok(total)
        }
        Value::ResultOk(nested)
        | Value::ResultErr(nested)
        | Value::OptionSome(nested)
        | Value::Future(nested) => Ok(16u64
            .checked_add(logical_value_size(
                nested,
                visited_objects,
                visited_frames,
                visited_functions,
                nodes,
            )?)
            .ok_or_else(|| "memory budget exceeded: logical wrapper size overflow".to_string())?),
    }
}

fn logical_frame_size(
    frame: &EnvFrame,
    visited_objects: &mut HashSet<usize>,
    visited_frames: &mut HashSet<usize>,
    visited_functions: &mut HashSet<usize>,
    nodes: &mut usize,
) -> Result<u64, String> {
    let identity = frame as *const EnvFrame as usize;
    if !visited_frames.insert(identity) {
        return Ok(0);
    }
    let values = frame
        .values
        .try_borrow()
        .map_err(|_| EnvFrame::borrow_error())?;
    let mut total = logical_add(32, values.len().saturating_mul(8))?;
    for (name, cell) in values.iter() {
        total = total
            .checked_add(logical_add(0, name.len())?)
            .ok_or_else(|| "memory budget exceeded: logical frame size overflow".to_string())?;
        total = total
            .checked_add(logical_value_size(
                &*cell.try_borrow().map_err(|_| EnvFrame::borrow_error())?,
                visited_objects,
                visited_frames,
                visited_functions,
                nodes,
            )?)
            .ok_or_else(|| "memory budget exceeded: logical frame size overflow".to_string())?;
    }
    if let Some(parent) = &frame.parent {
        total = total
            .checked_add(logical_frame_size(
                parent,
                visited_objects,
                visited_frames,
                visited_functions,
                nodes,
            )?)
            .ok_or_else(|| {
                "memory budget exceeded: logical parent-frame size overflow".to_string()
            })?;
    }
    Ok(total)
}

fn logical_function_shallow_size(function: &Function) -> Result<u64, String> {
    let mut total = 64u64;
    total = total
        .checked_add(function.visibility.len() as u64)
        .and_then(|value| {
            value.checked_add(function.body.iter().map(String::len).sum::<usize>() as u64)
        })
        .ok_or_else(|| "memory budget exceeded: logical function size overflow".to_string())?;
    for parameter in &function.params {
        total = total
            .checked_add(parameter.name.len() as u64)
            .and_then(|value| {
                value.checked_add(parameter.annotation.as_deref().map_or(0, str::len) as u64)
            })
            .and_then(|value| {
                value.checked_add(parameter.default.as_deref().map_or(0, str::len) as u64)
            })
            .ok_or_else(|| "memory budget exceeded: logical parameter size overflow".to_string())?;
    }
    if let Some(annotation) = &function.return_annotation {
        total = total.checked_add(annotation.len() as u64).ok_or_else(|| {
            "memory budget exceeded: logical return annotation size overflow".to_string()
        })?;
    }
    Ok(total)
}

fn validate_frame(
    frame: &EnvFrame,
    visited_objects: &mut HashSet<usize>,
    visited_frames: &mut HashSet<usize>,
    visited_functions: &mut HashSet<usize>,
    nodes: &mut usize,
    depth: usize,
    node_limit: usize,
) -> Result<(), String> {
    let identity = frame as *const EnvFrame as usize;
    if !visited_frames.insert(identity) {
        return Ok(());
    }
    let values = frame
        .values
        .try_borrow()
        .map_err(|_| EnvFrame::borrow_error())?;
    for cell in values.values() {
        validate_value(
            &*cell.try_borrow().map_err(|_| EnvFrame::borrow_error())?,
            visited_objects,
            visited_frames,
            visited_functions,
            nodes,
            depth + 1,
            node_limit,
        )?;
    }
    if let Some(parent) = &frame.parent {
        validate_frame(
            parent,
            visited_objects,
            visited_frames,
            visited_functions,
            nodes,
            depth + 1,
            node_limit,
        )?;
    }
    Ok(())
}

fn validate_value(
    value: &Value,
    visited_objects: &mut HashSet<usize>,
    visited_frames: &mut HashSet<usize>,
    visited_functions: &mut HashSet<usize>,
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
                validate_value(
                    nested,
                    visited_objects,
                    visited_frames,
                    visited_functions,
                    nodes,
                    depth + 1,
                    node_limit,
                )?;
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
                validate_value(
                    nested,
                    visited_objects,
                    visited_frames,
                    visited_functions,
                    nodes,
                    depth + 1,
                    node_limit,
                )?;
            }
            Ok(())
        }
        Value::Object { fields, .. } => {
            let identity = Rc::as_ptr(fields) as usize;
            if visited_objects.insert(identity) {
                for nested in fields.try_borrow()?.values() {
                    validate_value(
                        nested,
                        visited_objects,
                        visited_frames,
                        visited_functions,
                        nodes,
                        depth + 1,
                        node_limit,
                    )?;
                }
            }
            Ok(())
        }
        Value::ResultOk(nested)
        | Value::ResultErr(nested)
        | Value::OptionSome(nested)
        | Value::Future(nested) => validate_value(
            nested,
            visited_objects,
            visited_frames,
            visited_functions,
            nodes,
            depth + 1,
            node_limit,
        ),
        Value::Callable(function) => {
            let identity = Rc::as_ptr(function) as usize;
            if visited_functions.insert(identity) {
                for parameter in &function.params {
                    if parameter
                        .default
                        .as_deref()
                        .is_some_and(|default| default.len() > MAX_RUNTIME_TEXT_BYTES)
                    {
                        return Err(format!(
                            "memory limit exceeded: function default exceeds {MAX_RUNTIME_TEXT_BYTES} bytes"
                        ));
                    }
                }
                validate_frame(
                    &function.closure,
                    visited_objects,
                    visited_frames,
                    visited_functions,
                    nodes,
                    depth + 1,
                    node_limit,
                )?;
            }
            Ok(())
        }
        Value::Bool(_)
        | Value::Number(_)
        | Value::ScheduledFuture(_)
        | Value::OptionNone
        | Value::None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        memory_stats, try_values_equal, EnvFrame, Function, Param, Value,
        MAX_RUNTIME_COLLECTION_ITEMS, MAX_RUNTIME_TEXT_BYTES,
    };
    use crate::runtime_state::ExecutionContext;
    use std::{collections::HashMap, rc::Rc};

    #[test]
    fn logical_size_counts_nested_values_and_callable_captures() {
        let mut captured = HashMap::new();
        captured.insert(
            "payload".into(),
            Value::List(vec![Value::Text("captured".into()), Value::Number(7)]),
        );
        let callable = Value::Callable(Rc::new(Function {
            visibility: "public".into(),
            params: vec![Param {
                name: "value".into(),
                annotation: Some("text".into()),
                default: Some("\"fallback\"".into()),
            }],
            type_params: Vec::new(),
            return_annotation: Some("text".into()),
            is_async: false,
            body: Vec::new(),
            ast_body: None,
            closure: EnvFrame::from_map(&captured),
        }));
        let size = callable
            .logical_size()
            .expect("callable capture should have a deterministic logical size");
        assert!(size > 16);
        assert!(callable.validate_memory_limits().is_ok());

        captured.insert(
            "oversized".into(),
            Value::Text("x".repeat(MAX_RUNTIME_TEXT_BYTES + 1)),
        );
        let oversized = Value::Callable(Rc::new(Function {
            visibility: "public".into(),
            params: Vec::new(),
            type_params: Vec::new(),
            return_annotation: None,
            is_async: false,
            body: Vec::new(),
            ast_body: None,
            closure: EnvFrame::from_map(&captured),
        }));
        assert!(oversized.validate_memory_limits().is_err());
        assert!(oversized.logical_size().is_err());
    }

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
        assert_eq!(
            stats["cycle_policy"],
            Value::Text("explicit_clear_object_fields".into())
        );
    }

    #[test]
    fn value_equality_is_cycle_safe_and_borrow_checked() {
        let left = Value::object("Cycle");
        let right = Value::object("Cycle");
        let Value::Object {
            fields: left_fields,
            ..
        } = &left
        else {
            panic!("object constructor must create an object value");
        };
        let Value::Object {
            fields: right_fields,
            ..
        } = &right
        else {
            panic!("object constructor must create an object value");
        };
        left_fields
            .try_borrow_mut()
            .unwrap()
            .insert("self".into(), left.clone());
        right_fields
            .try_borrow_mut()
            .unwrap()
            .insert("self".into(), right.clone());
        assert!(try_values_equal(&left, &right).unwrap());
        let _active_borrow = left_fields.try_borrow_mut().unwrap();
        assert_eq!(
            try_values_equal(&left, &left).unwrap_err(),
            "BorrowError: object fields are already borrowed"
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
    fn checked_envframe_borrows_return_typed_failures() {
        let frame = EnvFrame::from_map(&HashMap::from([("value".into(), Value::Number(1))]));
        let _active_borrow = frame.values.try_borrow_mut().unwrap();
        assert_eq!(
            frame.try_snapshot().unwrap_err(),
            "BorrowError: environment frame is already borrowed"
        );
        assert_eq!(
            frame
                .try_insert_local("other".into(), Value::Number(2))
                .unwrap_err(),
            "BorrowError: environment frame is already borrowed"
        );
        assert_eq!(
            frame.try_assign("value", Value::Number(3)).unwrap_err(),
            "BorrowError: environment frame is already borrowed"
        );
    }

    #[test]
    fn frame_borrows_propagate_through_accounting_and_validation() {
        let frame = EnvFrame::from_map(&HashMap::from([("value".into(), Value::Number(1))]));
        let callable = Value::Callable(Rc::new(Function {
            visibility: "public".into(),
            params: Vec::new(),
            type_params: Vec::new(),
            return_annotation: None,
            is_async: false,
            body: Vec::new(),
            ast_body: None,
            closure: frame.clone(),
        }));
        let _active_borrow = frame.values.try_borrow_mut().unwrap();
        assert_eq!(
            callable.logical_size().unwrap_err(),
            "BorrowError: environment frame is already borrowed"
        );
        assert_eq!(
            callable.validate_memory_limits().unwrap_err(),
            "BorrowError: environment frame is already borrowed"
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
