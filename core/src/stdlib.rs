use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest::blocking::get;
use sha2::{Sha256, Digest};
use rand::Rng;
use crate::parser::Expr;
use crate::interpreter::RuntimeValue;

pub fn register_stdlib(env: &mut HashMap<String, RuntimeValue>) {
    let functions = vec![
        "fs_read", "fs_write", "http_get", "time_now", "crypto_sha256",
        "math_add", "math_sub", "math_sqrt", "os_env", "random_int", "string_upper", "string_lower"
    ];
    for func in functions {
        env.insert(
            func.to_string(),
            RuntimeValue::Function(func.to_string(), vec!["arg".to_string()], vec![]),
        );
    }
}

pub fn stdlib_call(name: &str, args: Vec<Expr>, interpreter: &Interpreter) -> RuntimeValue {
    let evaluated_args: Vec<RuntimeValue> = args.into_iter().map(|arg| interpreter.eval_expr(arg)).collect();
    match name {
        "fs_read" => {
            if let Some(RuntimeValue::String(path)) = evaluated_args.get(0) {
                match fs::read_to_string(path) {
                    Ok(content) => RuntimeValue::String(content),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid path".to_string())
            }
        }
        "fs_write" => {
            if let (Some(RuntimeValue::String(path)), Some(RuntimeValue::String(content))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                match fs::write(path, content) {
                    Ok(_) => RuntimeValue::String("Success".to_string()),
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid arguments".to_string())
            }
        }
        "http_get" => {
            if let Some(RuntimeValue::String(url)) = evaluated_args.get(0) {
                match get(url) {
                    Ok(resp) => match resp.text() {
                        Ok(text) => RuntimeValue::String(text),
                        Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                    },
                    Err(e) => RuntimeValue::String(format!("Error: {}", e)),
                }
            } else {
                RuntimeValue::String("Invalid URL".to_string())
            }
        }
        "time_now" => {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            RuntimeValue::Number(timestamp as f64)
        }
        "crypto_sha256" => {
            if let Some(RuntimeValue::String(input)) = evaluated_args.get(0) {
                let mut hasher = Sha256::new();
                hasher.update(input);
                let result = hasher.finalize();
                RuntimeValue::String(format!("{:x}", result))
            } else {
                RuntimeValue::String("Invalid input".to_string())
            }
        }
        "math_add" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                RuntimeValue::Number(a + b)
            } else {
                RuntimeValue::String("Invalid numbers".to_string())
            }
        }
        "math_sub" => {
            if let (Some(RuntimeValue::Number(a)), Some(RuntimeValue::Number(b))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                RuntimeValue::Number(a - b)
            } else {
                RuntimeValue::String("Invalid numbers".to_string())
            }
        }
        "math_sqrt" => {
            if let Some(RuntimeValue::Number(n)) = evaluated_args.get(0) {
                RuntimeValue::Number(n.sqrt())
            } else {
                RuntimeValue::String("Invalid number".to_string())
            }
        }
        "os_env" => {
            if let Some(RuntimeValue::String(key)) = evaluated_args.get(0) {
                match std::env::var(key) {
                    Ok(value) => RuntimeValue::String(value),
                    Err(_) => RuntimeValue::String("".to_string()),
                }
            } else {
                RuntimeValue::String("Invalid key".to_string())
            }
        }
        "random_int" => {
            if let (Some(RuntimeValue::Number(min)), Some(RuntimeValue::Number(max))) = (evaluated_args.get(0), evaluated_args.get(1)) {
                let min = min.floor() as i32;
                let max = max.floor() as i32;
                let value = rand::thread_rng().gen_range(min..=max);
                RuntimeValue::Number(value as f64)
            } else {
                RuntimeValue::String("Invalid range".to_string())
            }
        }
        "string_upper" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_uppercase())
            } else {
                RuntimeValue::String("Invalid string".to_string())
            }
        }
        "string_lower" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_lowercase())
            } else {
                RuntimeValue::String("Invalid string".to_string())
            }
        }
        _ => RuntimeValue::String(format!("Unknown function: {}", name)),
    }
}
