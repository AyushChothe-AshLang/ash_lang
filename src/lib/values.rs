use ordered_float::OrderedFloat;
use std::{
    collections::HashMap,
    fmt::Display,
    hash::{Hash, Hasher},
};
// Value
// #[derive(Debug, Clone)]
#[derive(Debug, Clone)]
pub enum Value {
    IntValue(i64),
    DoubleValue(OrderedFloat<f64>),
    StringValue(String),
    ListValue(Vec<Value>),
    MapValue(HashMap<Value, Value>),
    BooleanValue(bool),
    ReturnValue(Box<Value>),
    Break,
    Continue,
    None,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IntValue(l0), Self::IntValue(r0)) => l0 == r0,
            (Self::DoubleValue(l0), Self::DoubleValue(r0)) => l0 == r0,
            (Self::StringValue(l0), Self::StringValue(r0)) => l0 == r0,
            (Self::ListValue(l0), Self::ListValue(r0)) => l0 == r0,
            (Self::MapValue(l0), Self::MapValue(r0)) => {
                let mut eq = true;
                if l0.len() != r0.len() {
                    return false;
                }
                for (k, v) in l0 {
                    if !(r0.contains_key(k) && r0.get(k).unwrap() == v) {
                        eq = false;
                        break;
                    }
                }
                eq
            }
            (Self::BooleanValue(l0), Self::BooleanValue(r0)) => l0 == r0,
            (Self::ReturnValue(l0), Self::ReturnValue(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}
impl Eq for Value {}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::IntValue(l0), Value::IntValue(r0)) => l0.cmp(r0),
            (Value::IntValue(l0), Value::DoubleValue(r0)) => {
                OrderedFloat::cmp(&OrderedFloat(*l0 as f64), &r0)
            }
            (Value::DoubleValue(l0), Value::IntValue(r0)) => {
                OrderedFloat::cmp(&l0, &OrderedFloat(*r0 as f64))
            }
            (Value::DoubleValue(l0), Value::DoubleValue(r0)) => l0.cmp(r0),
            (Value::StringValue(l0), Value::StringValue(r0)) => l0.len().cmp(&r0.len()),
            (Value::ListValue(l0), Value::ListValue(r0)) => l0.len().cmp(&r0.len()),
            _ => panic!("Invalid Comparison"),
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::IntValue(l0) => l0.hash(state),
            Value::DoubleValue(l0) => l0.hash(state),
            Value::StringValue(l0) => l0.hash(state),
            Value::ListValue(l0) => l0.hash(state),
            Value::BooleanValue(l0) => l0.hash(state),
            Value::MapValue(l0) => {
                for (k, v) in l0.iter() {
                    (k, v).hash(state);
                }
            }
            _ => core::mem::discriminant(self).hash(state),
        }
    }
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
            Value::MapValue(m) => {
                write!(f, "{{",)?;
                let len = m.len();
                for (i, (k, v)) in m.iter().enumerate() {
                    write!(f, "{}:{}", k, v)?;
                    if i != len - 1 {
                        write!(f, ", ",)?;
                    }
                }
                write!(f, "}}",)
            }
            Value::BooleanValue(b) => b.fmt(f),
            Value::ReturnValue(r) => write!(f, "{}", *r),
            Value::Break => write!(f, "Break"),
            Value::Continue => write!(f, "Continue"),
            Value::None => write!(f, "None"),
        }
    }
}
