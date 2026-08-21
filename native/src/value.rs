use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        fields: Rc<RefCell<HashMap<String, Value>>>,
    },
    ResultOk(Box<Value>),
    ResultErr(Box<Value>),
    OptionSome(Box<Value>),
    OptionNone,
    Future(Box<Value>),
    None,
}
impl Value {
    /// Construct an object whose fields are reference-counted independently from
    /// the value handle. This keeps object ownership explicit at runtime.
    #[allow(dead_code)]
    pub(crate) fn object(class_name: impl Into<String>) -> Self {
        Self::Object {
            class_name: class_name.into(),
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
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

#[cfg(test)]
mod tests {
    use super::Value;
    use std::rc::Rc;

    #[test]
    fn cyclic_object_graph_can_be_explicitly_broken() {
        let object = Value::object("Node");
        let Value::Object { fields, .. } = &object else {
            panic!("object constructor must create an object value");
        };
        let weak_fields = Rc::downgrade(fields);
        fields.borrow_mut().insert("self".into(), object.clone());
        assert_eq!(object.object_field_count(), Some(1));
        assert!(weak_fields.upgrade().is_some());

        assert!(object.clear_object_fields());
        assert_eq!(object.object_field_count(), Some(0));
        drop(object);
        assert!(weak_fields.upgrade().is_none());
    }
}
