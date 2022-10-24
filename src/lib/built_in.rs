use std::io::{self, Write};

use ordered_float::OrderedFloat;

use super::values::Value;

// use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

pub fn ash_print(args: Vec<Value>) -> Value {
    let res = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    // console_log!("{}", res);
    print!("{}", res);

    Value::None
}
pub fn ash_println(args: Vec<Value>) -> Value {
    let res = args
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    // console_log!("{}", res);
    println!("{}", res);

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

pub fn ash_str(args: Vec<Value>) -> Value {
    let val = args.first().expect("Expected 1 argument 0 found");
    Value::StringValue(match val {
        Value::IntValue(_i) => _i.to_string(),
        Value::DoubleValue(_d) => _d.0.to_string(),
        Value::StringValue(_s) => _s.to_string(),
        Value::BooleanValue(_b) => _b.to_string(),
        Value::ListValue(_l) => _l
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(""),
        _ => panic!("Invalid Type Conversion"),
    })
}

pub fn ash_list(args: Vec<Value>) -> Value {
    let val = args.first().expect("Expected 1 argument 0 found");
    Value::ListValue(match val {
        Value::StringValue(_s) => _s
            .chars()
            .map(|x| Value::StringValue(x.to_string()))
            .collect::<Vec<Value>>(),
        Value::ListValue(_l) => _l.clone(),
        _ => panic!("Invalid Type Conversion"),
    })
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
    let mut min = args.first().expect("Expected at least 1 argument found 0");
    for arg in args.iter() {
        if arg < min {
            min = arg
        }
    }
    min.to_owned()
}
pub fn ash_max(args: Vec<Value>) -> Value {
    let mut max = args.first().expect("Expected at least 1 argument found 0");
    for arg in args.iter() {
        if arg > max {
            max = arg
        }
    }
    max.to_owned()
}

pub fn ash_get(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("Invalid arguments")
    }

    let this = args.first().expect("Expected at least 1 argument found 0");
    let idx_val = args.get(1).expect("Expeced index as second argument");

    match this {
        Value::ListValue(_l) => {
            if let Value::IntValue(idx) = idx_val {
                _l.get(*idx as usize).expect("Index out of bounds").clone()
            } else {
                panic!("Invalid Index")
            }
        }
        Value::StringValue(_s) => {
            if let Value::IntValue(idx) = idx_val {
                Value::StringValue(
                    _s.chars()
                        .nth(*idx as usize)
                        .expect("Index out of bounds")
                        .to_string(),
                )
            } else {
                panic!("Invalid Index")
            }
        }
        _ => panic!("Invalid argument"),
    }
}
pub fn ash_set(args: Vec<Value>) -> Value {
    if args.len() != 3 {
        panic!("Invalid arguments")
    }

    let this = args.first().expect("Expected at least 1 argument found 0");
    let mut this_mut = this.clone();
    let idx_val = args.get(1).expect("Expected index as second argument");
    let val = args.get(2).expect("Expected value as third argument");

    match &mut this_mut {
        Value::ListValue(_l) => {
            if let Value::IntValue(idx) = idx_val {
                _l[(*idx as usize)] = val.clone();
            } else {
                panic!("Invalid Index")
            }
        }
        Value::StringValue(_s) => {
            if let Value::IntValue(idx) = idx_val {
                if let Value::StringValue(_val) = val {
                    *_s = (&_s.as_str()[..(*idx as usize)]).to_string()
                        + _val
                        + (&_s.as_str()[(*idx as usize + 1)..]);
                } else {
                    panic!("Invalid Value")
                }
            } else {
                panic!("Invalid Index")
            }
        }
        _ => panic!("Invalid argument"),
    }
    this_mut
}
pub fn ash_len(args: Vec<Value>) -> Value {
    if args.len() != 1 {
        panic!("Invalid arguments")
    }

    let this = args.first().expect("Expected at least 1 argument found 0");

    match this {
        Value::ListValue(_l) => Value::IntValue(_l.len() as i64),
        Value::StringValue(_s) => Value::IntValue(_s.len() as i64),
        _ => panic!("Invalid argument"),
    }
}

pub fn ash_pop(args: Vec<Value>) -> Value {
    if args.len() != 2 {
        panic!("Invalid arguments")
    }

    let mut this = args
        .first()
        .expect("Expected at least 1 argument found 0")
        .clone();
    let idx_val = args.get(1).expect("Expected index as second argument");

    match &mut this {
        Value::ListValue(_l) => {
            if let Value::IntValue(idx) = idx_val {
                *_l = [&_l[..(*idx as usize)], &_l[(*idx as usize + 1)..]].concat();
            } else {
                panic!("Invalid Index")
            }
        }
        Value::StringValue(_s) => {
            if let Value::IntValue(idx) = idx_val {
                *_s = (&_s.as_str()[..(*idx as usize)]).to_string()
                    + (&_s.as_str()[(*idx as usize + 1)..]);
            } else {
                panic!("Invalid Index")
            }
        }
        _ => panic!("Invalid argument"),
    }
    this.clone()
}
