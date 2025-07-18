use crate::ast::*;
use crate::cli;
use crate::runtime::Value;
use std::collections::HashMap;
use std::path::Path;

pub fn run(statements: Vec<Statement>, debug: bool) -> Result<(), String> {
    let mut env = HashMap::with_capacity(statements.len());
    execute_block(&statements, &mut env, debug)?;
    Ok(())
}

fn execute_stmt(stmt: &Statement, env: &mut HashMap<String, Value>, debug: bool) -> Result<(), String> {
    if debug {
        cli::debug(&format!("Stmt: {:?}", stmt));
    }
    match stmt {
        Statement::Say(expr) => {
            println!("{}", eval_expr(expr, env, debug)?);
            Ok(())
        }
        Statement::Val(ident, expr, type_anno) => {
            let value = expr.as_ref().map_or(Value::None, |e| eval_expr(e, env, debug).unwrap());
            check_type(&value, type_anno)?;
            env.insert(ident.clone(), value);
            Ok(())
        }
        Statement::Const(ident, expr, type_anno) => {
            if env.contains_key(ident) {
                return Err(format!("Const '{}' redefinition", ident));
            }
            let value = eval_expr(expr, env, debug)?;
            check_type(&value, type_anno)?;
            env.insert(ident.clone(), value);
            Ok(())
        }
        Statement::Fun(name, params, ret_type, body) => {
            env.insert(name.clone(), Value::Function(params.clone(), ret_type.clone(), body.clone()));
            Ok(())
        }
        Statement::If(condition, then_block, else_block) => {
            if eval_expr(condition, env, debug)?.as_bool()? {
                execute_block(then_block, env, debug)?;
            } else if let Some(else_block) = else_block {
                execute_block(else_block, env, debug)?;
            }
            Ok(())
        }
        Statement::For(ident, expr, body) => {
            for value in eval_expr(expr, env, debug)?.as_list()? {
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
        Statement::Return(expr) => Err(format!("Return: {}", eval_expr(expr, env, debug)?)),
        Statement::Import(module, source) => {
            let module_path = format!("{}.velvet", module);
            if !Path::new(&module_path).exists() {
                return Err(format!("Module '{}' not found", module));
            }
            let module_source = crate::utils::read_file(&module_path)?;
            let ast = crate::parser::parse(&module_source)?;
            execute_block(&ast, env, debug)?;
            Ok(())
        }
        Statement::Test(name, body) => {
            if debug {
                cli::info(&format!("Test: {}", name));
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
        cli::debug(&format!("Expr: {:?}", expr));
    }
    match expr {
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Ident(id) => env.get(id).cloned().ok_or(format!("Var '{}' not found", id)),
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
                _ => Err(format!("Unknown operator '{}'", op)),
            }
        }
        Expr::Unary(op, expr) => {
            let value = eval_expr(expr, env, debug)?;
            match op.as_str() {
                "-" => Ok(Value::Number(-value.as_number()?)),
                "!" => Ok(Value::Bool(!value.as_bool()?)),
                _ => Err(format!("Unknown unary op '{}'", op)),
            }
        }
        Expr::Call(name, args) => {
            let func = env.get(name).ok_or(format!("Function '{}' not found", name))?;
            if let Value::Function(params, ret_type, body) = func {
                if params.len() != args.len() {
                    return Err(format!("Expected {} args, got {}", params.len(), args.len()));
                }
                let mut local_env = env.clone();
                for ((param, _), arg) in params.iter().zip(args.iter()) {
                    local_env.insert(param.clone(), eval_expr(arg, env, debug)?);
                }
                let result = execute_block(body, &mut local_env, debug)?;
                if let Some(ret_type) = ret_type {
                    if ret_type != "void" {
                        return Err("Missing return value".to_string());
                    }
                }
                Ok(Value::None)
            } else {
                Err(format!("'{}' is not a function", name))
            }
        }
        Expr::List(elements) => Ok(Value::List(elements.iter().map(|e| eval_expr(e, env, debug)).collect::<Result<_, _>>()?)),
        Expr::Index(ident, index) => {
            let list = env.get(ident).ok_or(format!("Var '{}' not found", ident))?.as_list()?;
            let idx = eval_expr(index, env, debug)?.as_number()? as usize;
            list.get(idx).cloned().ok_or(format!("Index {} out of bounds", idx))
        }
    }
}

fn check_type(value: &Value, type_anno: &Option<String>) -> Result<(), String> {
    if let Some(type_anno) = type_anno {
        match (type_anno.as_str(), value) {
            ("str", Value::String(_)) | ("f64", Value::Number(_)) | ("bool", Value::Bool(_)) | ("list", Value::List(_)) | ("fn", Value::Function(_, _, _)) => Ok(()),
            _ => Err(format!("Expected {}, got {}", type_anno, value)),
        }
    } else {
        Ok(())
    }
}
