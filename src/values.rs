// Value
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    IntValue(i64),
    DoubleValue(f64),
    BooleanValue(bool),
}
