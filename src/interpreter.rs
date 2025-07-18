use crate::ast::*;
use crate::cli;
use crate::runtime::Value;
use std::collections::HashMap;
use std::path::Path;

pub fn run(statements: Vec<Statement>, debug: bool) -> Result<(), String> {
    let mut env = HashMap::new();
    execute_block(&statements, &mut env, debug)?;
    Ok(())
}

fn execute_stmt(stmt: &Statement, env: &mut HashMap<String, Value>, debug: bool) -> Result<(), String> {
    if debug {
        cli::debug(&format!("Executing: {:?}", stmt));
    }
    match stmt {
        Statement::Say(expr) => {
            println!("{}", eval_expr(expr, env, debug)?);
            Ok(())
        }
        Statement::Val(ident, expr, type_anno) => {
            let value = if let Some(e) = expr {
                let val = eval_expr(e, env, debug)?;
                check_type(&val, type_anno)?;
                val
            } else {
                Value::None
            };
            env.insert(ident.clone(), value);
            Ok(())
        }
        Statement::Const(ident, expr, type_anno) => {
            if env.contains_key(ident) {
                return Err(format!("Constant '{}' already defined", ident));
            }
            let val = eval_expr(expr, env, debug)?;
            check_type(&val, type_anno)?;
            env.insert(ident.clone(), val);
            Ok(())
        }
        Statement::Fun(name, params, body) => {
            env.insert(name.clone(), Value::Function(params.clone(), body.clone()));
            Ok(())
        }
        Statement::If(condition, then_block, else_block) => {
            let cond = eval_expr(condition, env, debug)?;
            if cond.as_bool()? {
                execute_block(then_block, env, debug)?;
            } else if let Some(else_block) = else_block {
                execute_block(else_block, env, debug)?;
            }
            Ok(())
        }
        Statement::For(ident, expr, body) => {
            let iterable = eval_expr(expr, env, debug)?;
            for value in iterable.as_list()? {
                env.insert(ident.clone(), value);
                match execute_block(body, env, debug) {
                    Err(e) if e == "Break" => break,
                    Err(e) if e == "Continue" => continue,
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
            Ok(())
        }
        Statement::While(condition, body) => {
            while eval_expr(condition, env, debug)?.as_bool()? {
                match execute_block(body, env, debug) {
                    Err(e) if e == "Break" => break,
                    Err(e) if e == "Continue" => continue,
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
            Ok(())
        }
        Statement::Break => Err("Break".to_string()),
        Statement::Continue => Err("Continue".to_string()),
        Statement::Try(error_ident, try_block, catch_block) => {
            match execute_block(try_block, env, debug) {
                Err(e) => {
                    env.insert(error_ident.clone(), Value::String(e));
                    execute_block(catch_block, env, debug)?;
                }
                Ok(_) => {}
            }
            Ok(())
        }
        Statement::Match(expr, branches) => {
            let value = eval_expr(expr, env, debug)?;
            for (pattern, statements) in branches {
                if pattern == "_" || pattern == &value.to_string() {
                    execute_block(statements, env, debug)?;
                    break;
                }
            }
            Ok(())
        }
        Statement::Expr(expr) => {
            eval_expr(expr, env, debug)?;
            Ok(())
        }
        Statement::Return(expr) => {
            let value = eval_expr(expr, env, debug)?;
            Err(format!("Return: {}", value))
        }
        Statement::Import(module) => {
            let module_path = format!("{}.velvet", module);
            if !Path::new(&module_path).exists() {
                return Err(format!("Module '{}' not found", module));
            }
            let source = crate::utils::read_file(&module_path)
                .map_err(|_| format!("Failed to load module '{}'", module))?;
            let ast = crate::parser::parse(&source)?;
            execute_block(&ast, env, debug)?;
            Ok(())
        }
        Statement::Test(name, body) => {
            if debug {
                cli::info(&format!("Running test: {}", name));
            }
            execute_block(body, &mut env.clone(), debug)?;
            Ok(())
        }
    }
}

fn execute_block(stmts: &[Statement], env: &mut HashMap<String, Value>, debug: bool) -> Result<(), String> {
    for stmt in stmts {
        execute_stmt(stmt, env, debug)?;
    }
    Ok(())
}

fn eval_expr(expr: &Expr, env: &HashMap<String, Value>, debug: bool) -> Result<Value, String> {
    if debug {
        cli::debug(&format!("Evaluating: {:?}", expr));
    }
    match expr {
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Ident(id) => env.get(id).cloned().ok_or(format!("Undefined variable '{}'", id)),
        Expr::Binary(left, op, right) => {
            let left_val = eval_expr(left, env, debug)?;
            let right_val = eval_expr(right, env, debug)?;
            match op.as_str() {
                "+" => Ok(Value::Number(left_val.as_number()? + right_val.as_number()?)),
                "-" => Ok(Value::Number(left_val.as_number()? - right_val.as_number()?)),
                "*" => Ok(Value::Number(left_val.as_number()? * right_val.as_number()?)),
                "/" => {
                    let r = right_val.as_number()?;
                    if r == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    Ok(Value::Number(left_val.as_number()? / r))
                }
                "==" => Ok(Value::Bool(left_val == right_val)),
                "!=" => Ok(Value::Bool(left_val != right_val)),
                ">" => Ok(Value::Bool(left_val.as_number()? > right_val.as_number()?)),
                ">=" => Ok(Value::Bool(left_val.as_number()? >= right_val.as_number()?)),
                "<" => Ok(Value::Bool(left_val.as_number()? < right_val.as_number()?)),
                "<=" => Ok(Value::Bool(left_val.as_number()? <= right_val.as_number()?)),
                "and" => Ok(Value::Bool(left_val.as_bool()? && right_val.as_bool()?)),
                "or" => Ok(Value::Bool(left_val.as_bool()? || right_val.as_bool()?)),
                _ => Err(format!("Unsupported operator '{}'", op)),
            }
        }
        Expr::Unary(op, expr) => {
            let value = eval_expr(expr, env, debug)?;
            match op.as_str() {
                "-" => Ok(Value::Number(-value.as_number()?)),
                "!" => Ok(Value::Bool(!value.as_bool()?)),
                _ => Err(format!("Unsupported unary operator '{}'", op)),
            }
        }
        Expr::Call(name, args) => {
            let func = env.get(name).ok_or(format!("Undefined function '{}'", name))?;
            if let Value::Function(params, body) = func {
                let mut local_env = env.clone();
                if params.len() != args.len() {
                    return Err(format!("Expected {} arguments, got {}", params.len(), args.len()));
                }
                for ((param, _), arg) in params.iter().zip(args.iter()) {
                    local_env.insert(param.clone(), eval_expr(arg, env, debug)?);
                }
                execute_block(body, &mut local_env, debug)?;
                Ok(Value::None)
            } else {
                Err(format!("'{}' is not a function", name))
            }
        }
        Expr::List(elements) => {
            let mut values = Vec::new();
            for elem in elements {
                values.push(eval_expr(elem, env, debug)?);
            }
            Ok(Value::List(values))
        }
    }
}

fn check_type(value: &Value, type_anno: &Option<String>) -> Result<(), String> {
    if let Some(type_anno) = type_anno {
        match (type_anno.as_str(), value) {
            ("str", Value::String(_)) => Ok(()),
            ("f64", Value::Number(_)) => Ok(()),
            ("bool", Value::Bool(_)) => Ok(()),
            ("list", Value::List(_)) => Ok(()),
            _ => Err(format!("Type mismatch: expected {}, got {}", type_anno, value)),
        }
    } else {
        Ok(())
    }
}
