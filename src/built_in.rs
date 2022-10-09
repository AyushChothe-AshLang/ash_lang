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

pub fn ash_min(args: Vec<Value>) -> Value {
    let mut min = args.first().unwrap();
    for arg in args.iter() {
        if arg < min {
            min = arg
        }
    }
    min.to_owned()
}
pub fn ash_max(args: Vec<Value>) -> Value {
    let mut max = args.first().unwrap();
    for arg in args.iter() {
        if arg > max {
            max = arg
        }
    }
    max.to_owned()
}
