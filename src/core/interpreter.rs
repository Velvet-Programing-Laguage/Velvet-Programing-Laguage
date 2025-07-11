use super::ast::{Expr, Statement};
use std::collections::HashMap;

pub struct VelvetInterpreter {
    variables: HashMap<String, Expr>,
    functions: HashMap<String, (Vec<(String, Option<String>)>, Option<String>, Vec<Statement>)>,
}

impl VelvetInterpreter {
    pub fn new() -> Self {
        VelvetInterpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, ast: Vec<Statement>) -> Result<(), String> {
        for stmt in ast {
            self.execute_stmt(&stmt)?;
        }
        Ok(())
    }

    fn execute_stmt(&mut self, stmt: &Statement) -> Result<(), String> {
        match stmt {
            Statement::Say(expr) => {
                let value = self.eval_expr(expr)?;
                println!("Output: {}", self.expr_to_string(&value));
            }
            Statement::Let(ident, _ty, expr) => {
                if let Some(e) = expr {
                    let value = self.eval_expr(e)?;
                    self.variables.insert(ident.clone(), value);
                }
            }
            Statement::Const(ident, expr) => {
                if let Some(e) = expr {
                    let value = self.eval_expr(e)?;
                    self.variables.insert(ident.clone(), value);
                }
            }
            Statement::If(condition, then_block, else_block) => {
                let cond_val = self.eval_expr(condition)?;
                if self.is_truthy(&cond_val) {
                    for stmt in then_block {
                        self.execute_stmt(stmt)?;
                    }
                } else if let Some(else_block) = else_block {
                    for stmt in else_block {
                        self.execute_stmt(stmt)?;
                    }
                }
            }
            Statement::For(ident, expr, block) => {
                let list = self.eval_expr(expr)?;
                if let Expr::List(items) = list {
                    for item in items {
                        self.variables.insert(ident.clone(), item);
                        for stmt in block {
                            self.execute_stmt(stmt)?;
                        }
                    }
                } else {
                    return Err("For loop expects a list".to_string());
                }
            }
            Statement::Fun(ident, params, ret_type, block) => {
                self.functions.insert(ident.clone(), (params.clone(), ret_type.clone(), block.clone()));
            }
            Statement::Expr(expr) => {
                self.eval_expr(expr)?;
            }
        }
        Ok(())
    }

    fn eval_expr(&self, expr: &Expr) -> Result<Expr, String> {
        match expr {
            Expr::String(s) => Ok(Expr::String(s.clone())),
            Expr::Number(n) => Ok(Expr::Number(*n)),
            Expr::Ident(id) => self.variables.get(id).cloned().ok_or_else(|| format!("Undefined variable: {}", id)),
            Expr::Binary(op, left, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                match op.as_str() {
                    "+" => match (left_val, right_val) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l + r)),
                        (Expr::String(l), Expr::String(r)) => Ok(Expr::String(l + &r)),
                        _ => Err("Invalid types for +".to_string()),
                    },
                    "-" => match (left_val, right_val) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l - r)),
                        _ => Err("Invalid types for -".to_string()),
                    },
                    "*" => match (left_val, right_val) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l * r)),
                        _ => Err("Invalid types for *".to_string()),
                    },
                    "/" => match (left_val, right_val) {
                        (Expr::Number(l), Expr::Number(r)) => Ok(Expr::Number(l / r)),
                        _ => Err("Invalid types for /".to_string()),
                    },
                    "==" => Ok(Expr::Number(if left_val == right_val { 1.0 } else { 0.0 })),
                    "|>" => Ok(right_val), // Simplified pipe: pass right value
                    "and" => Ok(Expr::Number(if self.is_truthy(&left_val) && self.is_truthy(&right_val) { 1.0 } else { 0.0 })),
                    "or" => Ok(Expr::Number(if self.is_truthy(&left_val) || self.is_truthy(&right_val) { 1.0 } else { 0.0 })),
                    _ => Err(format!("Unsupported operator: {}", op)),
                }
            }
            Expr::Unary(op, expr) => {
                let val = self.eval_expr(expr)?;
                match op.as_str() {
                    "-" => match val {
                        Expr::Number(n) => Ok(Expr::Number(-n)),
                        _ => Err("Invalid type for unary -".to_string()),
                    },
                    "!" => match val {
                        Expr::Number(n) => Ok(Expr::Number(if n == 0.0 { 1.0 } else { 0.0 })),
                        _ => Err("Invalid type for !".to_string()),
                    },
                    _ => Err(format!("Unsupported unary operator: {}", op)),
                }
            }
            Expr::Call(ident, args) => {
                if let Some((params, _ret_type, block)) = self.functions.get(ident) {
                    let mut local_vars = HashMap::new();
                    for (i, (param_name, _)) in params.iter().enumerate() {
                        if i < args.len() {
                            let arg_val = self.eval_expr(&args[i])?;
                            local_vars.insert(param_name.clone(), arg_val);
                        }
                    }
                    let mut local_interpreter = VelvetInterpreter {
                        variables: local_vars,
                        functions: self.functions.clone(),
                    };
                    for stmt in block {
                        local_interpreter.execute_stmt(stmt)?;
                    }
                    Ok(Expr::Number(0.0)) // Placeholder return
                } else {
                    Err(format!("Undefined function: {}", ident))
                }
            }
            Expr::List(items) => {
                let evaluated = items.iter().map(|i| self.eval_expr(i)).collect::<Result<Vec<_>, _>>()?;
                Ok(Expr::List(evaluated))
            }
        }
    }

    fn is_truthy(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Number(n) => *n != 0.0,
            Expr::String(s) => !s.is_empty(),
            _ => true,
        }
    }

    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::String(s) => s.clone(),
            Expr::Number(n) => n.to_string(),
            Expr::Ident(id) => id.clone(),
            Expr::Call(id, args) => format!("{}({})", id, args.iter().map(|a| self.expr_to_string(a)).collect::<Vec<_>>().join(", ")),
            Expr::List(items) => format!("[{}]", items.iter().map(|i| self.expr_to_string(i)).collect::<Vec<_>>().join(", ")),
            Expr::Binary(op, l, r) => format!("({} {} {})", self.expr_to_string(l), op, self.expr_to_string(r)),
            Expr::Unary(op, e) => format!("{}{}", op, self.expr_to_string(e)),
        }
    }
}
