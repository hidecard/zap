use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

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

#[derive(Debug, PartialEq)]
pub(crate) struct TrackedObjectFields {
    fields: RefCell<HashMap<String, Value>>,
}

impl TrackedObjectFields {
    fn new() -> Rc<Self> {
        record_object_allocation();
        Rc::new(Self {
            fields: RefCell::new(HashMap::new()),
        })
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
        record_object_deallocation();
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
    pub(crate) closure: Rc<RefCell<HashMap<String, Value>>>,
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
    ResultOk(Box<Value>),
    ResultErr(Box<Value>),
    OptionSome(Box<Value>),
    OptionNone,
    Future(Box<Value>),
    None,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemoryStats {
    pub(crate) live_objects: usize,
    pub(crate) object_allocations: u64,
    pub(crate) object_deallocations: u64,
}

thread_local! {
    static MEMORY_STATS: RefCell<MemoryStats> = const { RefCell::new(MemoryStats {
        live_objects: 0,
        object_allocations: 0,
        object_deallocations: 0,
    }) };
}

fn record_object_allocation() {
    MEMORY_STATS.with(|stats| {
        let mut stats = stats.borrow_mut();
        stats.live_objects = stats.live_objects.saturating_add(1);
        stats.object_allocations = stats.object_allocations.saturating_add(1);
    });
}

fn record_object_deallocation() {
    let _ = MEMORY_STATS.try_with(|stats| {
        let mut stats = stats.borrow_mut();
        stats.live_objects = stats.live_objects.saturating_sub(1);
        stats.object_deallocations = stats.object_deallocations.saturating_add(1);
    });
}

pub(crate) fn memory_stats() -> MemoryStats {
    MEMORY_STATS.with(|stats| *stats.borrow())
}

impl Value {
    /// Construct an object whose fields are reference-counted independently from
    /// the value handle. This keeps object ownership explicit at runtime.
    pub(crate) fn object(class_name: impl Into<String>) -> Self {
        Self::Object {
            class_name: class_name.into(),
            fields: TrackedObjectFields::new(),
        }
    }

    /// Return the stable, bounded memory-statistics record exposed by the runtime.
    pub(crate) fn memory_stats_value() -> Self {
        let stats = memory_stats();
        let mut values = HashMap::new();
        values.insert(
            "live_objects".into(),
            Self::Number(stats.live_objects as i64),
        );
        values.insert(
            "object_allocations".into(),
            Self::Number(stats.object_allocations.min(i64::MAX as u64) as i64),
        );
        values.insert(
            "object_deallocations".into(),
            Self::Number(stats.object_deallocations.min(i64::MAX as u64) as i64),
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
        Self::Map(values)
    }

    /// Validate a value at a public runtime boundary without recursing forever
    /// through cyclic object graphs.
    pub(crate) fn validate_memory_limits(&self) -> Result<(), String> {
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
    pub(crate) fn clear_object_fields(&self) -> bool {
        if let Self::Object { fields, .. } = self {
            fields.borrow_mut().clear();
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn object_field_count(&self) -> Option<usize> {
        match self {
            Self::Object { fields, .. } => Some(fields.borrow().len()),
            _ => None,
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
            Self::ResultOk(x) => format!("Ok({})", x.show()),
            Self::ResultErr(x) => format!("Err({})", x.show()),
            Self::OptionSome(x) => format!("Some({})", x.show()),
            Self::OptionNone => "Option.none".into(),
            Self::Future(value) => format!("Future({})", value.show()),
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
            Self::Object { .. } => true,
            Self::ResultOk(_) => true,
            Self::ResultErr(_) => false,
            Self::OptionSome(_) => true,
            Self::Future(_) => true,
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
                for nested in fields.borrow().values() {
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
        Value::Bool(_) | Value::Number(_) | Value::OptionNone | Value::None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::{memory_stats, Value, MAX_RUNTIME_COLLECTION_ITEMS, MAX_RUNTIME_TEXT_BYTES};
    use std::rc::Rc;

    #[test]
    fn cyclic_object_graph_can_be_explicitly_broken() {
        let baseline = memory_stats();
        let object = Value::object("Node");
        let Value::Object { fields, .. } = &object else {
            panic!("object constructor must create an object value");
        };
        let weak_fields = Rc::downgrade(fields);
        fields.borrow_mut().insert("self".into(), object.clone());
        assert_eq!(object.object_field_count(), Some(1));
        assert_eq!(memory_stats().live_objects, baseline.live_objects + 1);
        assert!(weak_fields.upgrade().is_some());
        assert!(object.validate_memory_limits().is_ok());

        assert!(object.clear_object_fields());
        assert_eq!(object.object_field_count(), Some(0));
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
        fields.borrow_mut().insert("self".into(), object.clone());
        assert!(object.validate_memory_limits().is_ok());
        object.clear_object_fields();
    }
}
