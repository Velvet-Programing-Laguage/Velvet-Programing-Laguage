use super::ast::{Expr, Statement};
use std::collections::HashMap;

pub struct VelvetInterpreter {
    variables: HashMap<String, Expr>,
}

impl VelvetInterpreter {
    pub fn new() -> Self {
        VelvetInterpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, ast: Vec<Statement>) -> Result<(), String> {
        for stmt in ast {
            match stmt {
                Statement::Say(expr) => {
                    let value = self.eval_expr(&expr)?;
                    println!("Output: {}", self.expr_to_string(&value));
                }
                Statement::Let(ident, _ty, expr) => {
                    if let Some(e) = expr {
                        let value = self.eval_expr(&e)?;
                        self.variables.insert(ident, value);
                    }
                }
                Statement::Const(ident, expr) => {
                    if let Some(e) = expr {
                        let value = self.eval_expr(&e)?;
                        self.variables.insert(ident, value);
                    }
                }
                Statement::Expr(expr) => {
                    self.eval_expr(&expr)?;
                }
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
                    "==" => Ok(Expr::Number(if left_val == right_val { 1.0 } else { 0.0 })),
                    "|>" => Ok(right_val), // Simplified pipe: pass right value
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
            Expr::Call(_ident, _args) => Err("Function calls not implemented".to_string()),
            Expr::List(_items) => Err("Lists not implemented".to_string()),
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
