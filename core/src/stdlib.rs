use std::collections::HashMap;
use crate::parser::Expr;
use crate::interpreter::RuntimeValue;
use regex::Regex;

pub fn register_stdlib(env: &mut HashMap<String, RuntimeValue>) {
    env.insert(
        "str_upper".to_string(),
        RuntimeValue::Function("str_upper".to_string(), vec!["s".to_string()], vec![]),
    );
    env.insert(
        "str_lower".to_string(),
        RuntimeValue::Function("str_lower".to_string(), vec!["s".to_string()], vec![]),
    );
    env.insert(
        "math_sqrt".to_string(),
        RuntimeValue::Function("math_sqrt".to_string(), vec!["n".to_string()], vec![]),
    );
}

pub fn stdlib_call(name: &str, args: Vec<Expr>, interpreter: &Interpreter) -> RuntimeValue {
    let evaluated_args: Vec<RuntimeValue> = args.into_iter().map(|arg| interpreter.eval_expr(arg)).collect();
    match name {
        "str_upper" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_uppercase())
            } else {
                RuntimeValue::String("error".to_string())
            }
        }
        "str_lower" => {
            if let Some(RuntimeValue::String(s)) = evaluated_args.get(0) {
                RuntimeValue::String(s.to_lowercase())
            } else {
                RuntimeValue::String("error".to_string())
            }
        }
        "math_sqrt" => {
            if let Some(RuntimeValue::Number(n)) = evaluated_args.get(0) {
                RuntimeValue::Number(n.sqrt())
            } else {
                RuntimeValue::String("error".to_string())
            }
        }
        _ => RuntimeValue::String(format!("Unknown function: {}", name)),
    }
}