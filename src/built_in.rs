use super::values::Value;

pub fn ash_print(args: Vec<Value>) -> Value {
    print!(
        "{}",
        args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", "),
    );
    Value::None
}
pub fn ash_println(args: Vec<Value>) -> Value {
    println!(
        "{}",
        args.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    Value::None
}
