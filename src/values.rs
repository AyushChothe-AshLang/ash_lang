// Value
pub trait Value<T>: std::fmt::Debug {
    fn get_literal(&self) -> T;
}

// NumberValue
#[derive(Debug)]
pub struct NumberValue {
    pub value: f64,
}
impl NumberValue {
    pub fn new(value: f64) -> Self {
        NumberValue { value }
    }
}
impl Value<f64> for NumberValue {
    fn get_literal(&self) -> f64 {
        self.value
    }
}
