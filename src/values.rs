use std::fmt::Display;

// Value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    IntValue(i64),
    DoubleValue(f64),
    BooleanValue(bool),
    ReturnValue(Box<Value>),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::IntValue(i) => i.fmt(f),
            Value::DoubleValue(d) => d.fmt(f),
            Value::BooleanValue(b) => b.fmt(f),
            Value::None => write!(f, "None"),
            _ => write!(f, "Return"),
        }
    }
}
