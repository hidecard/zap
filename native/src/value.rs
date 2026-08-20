use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Param {
    pub(crate) name: String,
    pub(crate) annotation: Option<String>,
}
#[derive(Clone, Debug)]
pub(crate) struct StaticSignature {
    pub(crate) params: Vec<Param>,
    pub(crate) return_annotation: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Function {
    pub(crate) params: Vec<Param>,
    pub(crate) return_annotation: Option<String>,
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
    None,
}
impl Value {
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
            Self::OptionNone | Self::None => false,
        }
    }
}
