use serde_json::{json, Value};
use std::collections::HashMap;
use crate::parser::{Node, WindowProp, Expr};

#[derive(Clone)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    Bool(bool),
}

pub struct Interpreter {
    env: HashMap<String, RuntimeValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: HashMap::new(),
        }
    }

    pub fn execute(&mut self, nodes: Vec<Node>) -> Value {
        let mut gui_data = Vec::new();
        for node in nodes {
            match node {
                Node::Say(expr) => {
                    let value = self.eval_expr(*expr);
                    if let RuntimeValue::String(s) = value {
                        println!("{}", s);
                    }
                }
                Node::Set(ident, expr) => {
                    let value = self.eval_expr(*expr);
                    self.env.insert(ident, value);
                }
                Node::If(condition, then_branch, else_branch) => {
                    let cond_val = self.eval_expr(*condition);
                    if let RuntimeValue::Bool(true) = cond_val {
                        gui_data.extend(self.execute(then_branch));
                    } else if let Some(else_nodes) = else_branch {
                        gui_data.extend(self.execute(else_nodes));
                    }
                }
                Node::Window(props) => {
                    let mut window = json!({
                        "type": "window",
                        "props": {}
                    });
                    for prop in props {
                        match prop {
                            WindowProp::Title(title) => {
                                window["props"]["title"] = json!(title);
                            }
                            WindowProp::Size(w, h) => {
                                window["props"]["size"] = json!([w, h]);
                            }
                            WindowProp::Button(text, actions) => {
                                let action_data = actions
                                    .into_iter()
                                    .map(|a| match a {
                                        Node::Say(expr) => {
                                            let val = self.eval_expr(*expr);
                                            if let RuntimeValue::String(s) = val {
                                                json!({"type": "say", "value": s})
                                            } else {
                                                json!({})
                                            }
                                        }
                                        _ => json!({}),
                                    })
                                    .collect::<Vec<_>>();
                                window["props"]["buttons"] = json!([{
                                    "text": text,
                                    "action": action_data
                                }]);
                            }
                        }
                    }
                    gui_data.push(window);
                }
            }
        }
        json!(gui_data)
    }

    fn eval_expr(&self, expr: Expr) -> RuntimeValue {
        match expr {
            Expr::String(s) => RuntimeValue::String(s),
            Expr::Number(n) => RuntimeValue::Number(n),
            Expr::Ident(id) => self.env.get(&id).cloned().unwrap_or(RuntimeValue::String("undefined".to_string())),
            Expr::Arith(left, op, right) => {
                let left_val = self.eval_expr(*left);
                let right_val = self.eval_expr(*right);
                match (left_val, op.as_str(), right_val) {
                    (RuntimeValue::Number(l), "+", RuntimeValue::Number(r)) => RuntimeValue::Number(l + r),
                    (RuntimeValue::Number(l), "-", RuntimeValue::Number(r)) => RuntimeValue::Number(l - r),
                    _ => RuntimeValue::String("error".to_string()),
                }
            }
        }
    }
}
