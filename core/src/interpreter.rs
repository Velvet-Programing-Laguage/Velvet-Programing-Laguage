use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use crate::parser::{Node, WindowProp, Expr, Type, Pattern};
use crate::stdlib::{register_stdlib, stdlib_call};

#[derive(Clone)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    Bool(bool),
    List(Vec<RuntimeValue>),
    Struct(String, HashMap<String, RuntimeValue>),
    Function(String, Vec<String>, Vec<Node>),
}

pub struct Interpreter {
    env: HashMap<String, RuntimeValue>,
    modules: HashMap<String, Vec<Node>>,
    last_return: Option<RuntimeValue>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            env: HashMap::new(),
            modules: HashMap::new(),
            last_return: None,
        };
        register_stdlib(&mut interpreter.env);
        interpreter
    }

    pub fn load_module(&mut self, module: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.modules.contains_key(module) {
            let path = format!("vel_modules/{}.vel", module);
            let code = fs::read_to_string(&path).map_err(|e| {
                Box::new(std::io::Error::new(
                    e.kind(),
                    format!("Failed to load module {}: {}", module, e),
                ))
            })?;
            let ast = crate::parser::parse_velvet(&code)?;
            self.modules.insert(module.to_string(), ast);
        }
        Ok(())
    }

    pub fn execute(&mut self, nodes: Vec<Node>) -> Value {
        let mut gui_data = Vec::new();
        for node in nodes {
            match node {
                Node::Say(expr) => {
                    let value = self.eval_expr(expr);
                    if let RuntimeValue::String(s) = value {
                        println!("{}", s);
                    }
                }
                Node::Let(ident, expr) => {
                    let value = self.eval_expr(expr);
                    self.env.insert(ident, value);
                }
                Node::Const(ident, expr) => {
                    if self.env.contains_key(&ident) {
                        println!("Error: Cannot redefine constant {}", ident);
                        continue;
                    }
                    let value = self.eval_expr(expr);
                    self.env.insert(ident, value);
                }
                Node::If(condition, then_branch, else_branch) => {
                    let cond_val = self.eval_expr(condition);
                    if let RuntimeValue::Bool(true) = cond_val {
                        gui_data.extend(self.execute(then_branch));
                    } else if let Some(else_nodes) = else_branch {
                        gui_data.extend(self.execute(else_nodes));
                    }
                }
                Node::For(var, start, end, body) => {
                    for i in start..end {
                        self.env.insert(var.clone(), RuntimeValue::Number(i as f64));
                        gui_data.extend(self.execute(body.clone()));
                    }
                }
                Node::Fn(name, params, body) => {
                    self.env.insert(name, RuntimeValue::Function(name, params, body));
                }
                Node::Import(module) => {
                    if let Err(e) = self.load_module(&module) {
                        println!("Error loading module {}: {}", module, e);
                    } else {
                        if let Some(module_nodes) = self.modules.get(&module).cloned() {
                            gui_data.extend(self.execute(module_nodes));
                        }
                    }
                }
                Node::Try(try_body, catch_var, catch_body) => {
                    match self.execute(try_body.clone()) {
                        _ if self.last_return.is_some() => {
                            gui_data.extend(self.last_return.take().map(|v| json!({"type": "return", "value": v.to_string()})).into_iter());
                        }
                        _ => {
                            self.env.insert(catch_var.clone(), RuntimeValue::String("error".to_string()));
                            gui_data.extend(self.execute(catch_body));
                        }
                    }
                }
                Node::Struct(name, fields) => {
                    let mut struct_fields = HashMap::new();
                    for (field_name, _field_type) in fields {
                        struct_fields.insert(field_name, RuntimeValue::String("".to_string()));
                    }
                    self.env.insert(name, RuntimeValue::Struct(name, struct_fields));
                }
                Node::Match(expr, branches) => {
                    let value = self.eval_expr(expr);
                    for (pattern, body) in branches {
                        match (&value, &pattern) {
                            (RuntimeValue::String(v), Pattern::String(p)) if v == p => {
                                gui_data.extend(self.execute(body));
                                break;
                            }
                            (RuntimeValue::Number(v), Pattern::Number(p)) if (v - p).abs() < f64::EPSILON => {
                                gui_data.extend(self.execute(body));
                                break;
                            }
                            _ => continue,
                        }
                    }
                }
                Node::Return(expr) => {
                    self.last_return = Some(self.eval_expr(expr));
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
                                            let val = self.eval_expr(expr);
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
                            WindowProp::TextInput(id, placeholder) => {
                                window["props"]["inputs"] = json!([{
                                    "id": id,
                                    "placeholder": placeholder
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
            Expr::Call(name, args) => {
                if let Some(func) = self.env.get(&name) {
                    match func {
                        RuntimeValue::Function(_, params, body) => {
                            let mut local_env = self.env.clone();
                            for (param, arg) in params.iter().zip(args.iter()) {
                                local_env.insert(param.clone(), self.eval_expr(arg.clone()));
                            }
                            let mut local_interpreter = Interpreter {
                                env: local_env,
                                modules: self.modules.clone(),
                                last_return: None,
                            };
                            local_interpreter.execute(body.clone());
                            local_interpreter.last_return.unwrap_or(RuntimeValue::String("".to_string()))
                        }
                        _ => stdlib_call(&name, args, self),
                    }
                } else {
                    RuntimeValue::String(format!("Function {} not found", name))
                }
            }
            Expr::List(items) => {
                let values = items.into_iter().map(|item| self.eval_expr(item)).collect();
                RuntimeValue::List(values)
            }
        }
    }
}

impl RuntimeValue {
    pub fn to_string(&self) -> String {
        match self {
            RuntimeValue::String(s) => s.clone(),
            RuntimeValue::Number(n) => n.to_string(),
            RuntimeValue::Bool(b) => b.to_string(),
            RuntimeValue::List(items) => format!("[{}]", items.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(", ")),
            RuntimeValue::Struct(name, fields) => {
                let fields_str = fields.iter().map(|(k, v)| format!("{}: {}", k, v.to_string())).collect::<Vec<_>>().join(", ");
                format!("{} {{ {} }}", name, fields_str)
            }
            RuntimeValue::Function(name, _, _) => format!("Function({})", name),
        }
    }
                        }
