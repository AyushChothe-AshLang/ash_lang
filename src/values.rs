use std::fmt::Display;

// Value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    IntValue(i64),
    DoubleValue(f64),
    BooleanValue(bool),
    None,
}
impl Value {
    pub fn val(&self) -> String {
        match self {
            Value::IntValue(i) => format!("{i}"),
            Value::DoubleValue(d) => format!("{d}"),
            Value::BooleanValue(b) => format!("{b}"),
            Value::None => format!("None"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::IntValue(i) => write!(f, "{i}"),
            Value::DoubleValue(d) => write!(f, "{d}"),
            Value::BooleanValue(b) => write!(f, "{b}"),
            Value::None => write!(f, "None"),
        }
    }
}
