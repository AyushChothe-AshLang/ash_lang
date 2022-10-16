use std::io::{self, Write};

use ordered_float::OrderedFloat;

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
pub fn ash_input(args: Vec<Value>) -> Value {
    print!(
        "{}",
        args.first()
            .unwrap_or(&Value::StringValue(String::from("Input: ")))
    );

    io::stdout().flush().expect("Error while flushing stdout");

    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Error occured while taking input");
    Value::StringValue(buf.trim().to_owned())
}

pub fn ash_int(args: Vec<Value>) -> Value {
    let val = args.first().expect("Expected 1 argument 0 found");
    Value::IntValue(match val {
        Value::IntValue(_i) => *_i as i64,
        Value::DoubleValue(_d) => _d.0 as i64,
        Value::StringValue(_s) => _s.parse::<i64>().expect("int Parsing Error"),
        Value::BooleanValue(_b) => {
            if *_b {
                1
            } else {
                0
            }
        }
        _ => panic!("Invalid Type Conversion"),
    })
}

pub fn ash_double(args: Vec<Value>) -> Value {
    let val = args.first().expect("Expected 1 argument 0 found");
    Value::DoubleValue(OrderedFloat(match val {
        Value::IntValue(_i) => *_i as f64,
        Value::DoubleValue(_d) => _d.0 as f64,
        Value::StringValue(_s) => _s.parse::<f64>().expect("double Parsing Error"),
        Value::BooleanValue(_b) => {
            if *_b {
                1.0
            } else {
                0.0
            }
        }
        _ => panic!("Invalid Type Conversion"),
    }))
}

pub fn ash_min(args: Vec<Value>) -> Value {
    let mut min = args.first().expect("Expected at least 1 argument 0 found");
    for arg in args.iter() {
        if arg < min {
            min = arg
        }
    }
    min.to_owned()
}
pub fn ash_max(args: Vec<Value>) -> Value {
    let mut max = args.first().expect("Expected at least 1 argument 0 found");
    for arg in args.iter() {
        if arg > max {
            max = arg
        }
    }
    max.to_owned()
}
