use ordered_float::OrderedFloat;
use std::fmt::Display;
// Value
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Value {
    IntValue(i64),
    DoubleValue(OrderedFloat<f64>),
    StringValue(String),
    ListValue(Vec<Value>),
    BooleanValue(bool),
    ReturnValue(Box<Value>),
    None,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::IntValue(i) => i.fmt(f),
            Value::DoubleValue(d) => d.fmt(f),
            Value::StringValue(s) => s.fmt(f),
            Value::ListValue(l) => {
                write!(f, "[",)?;

                if l.len() >= 1 {
                    write!(f, "{}", l[0])?;
                }
                if l.len() > 1 {
                    l.iter()
                        .skip(1)
                        .map(|e| write!(f, ", {}", e))
                        .collect::<std::fmt::Result>()?;
                }
                write!(f, "]",)
            }
            Value::BooleanValue(b) => b.fmt(f),
            Value::None => write!(f, "None"),
            _ => write!(f, "Return"),
        }
    }
}
