use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use crate::parser::{Node, WindowProp, Expr};
use crate::stdlib::{register_stdlib, stdlib_call};

#[derive(Clone)]
pub enum RuntimeValue {
    String(String),
    Number(f64),
    Bool(bool),
    Function(String, Vec<String>, Vec<Node>),
}

pub struct Interpreter {
    env: HashMap<String, RuntimeValue>,
    modules: HashMap<String, Vec<Node>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            env: HashMap::new(),
            modules: HashMap::new(),
        };
        register_stdlib(&mut interpreter.env);
        interpreter
    }

    pub fn load_module(&mut self, module: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !self.modules.contains_key(module) {
            let path = format!("vel_modules/{}.vel", module);
            let code = fs::read_to_string(&path)?;
            let ast = crate::parser::parse_velvet(&code)?;
            self.modules.insert(module.to_string(), ast);
        }
        Ok(())
    }

    pub fn execute(&mut self, nodes: Vec<Node>) -> Value {
        let mut gui_data = Vec::new();
        for node in nodes {
            match node {
                Node::Say(str) => {
                    println!("{}", str);
                }
                Node::Set(ident, expr) => {
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
                Node::Def(name, params, body) => {
                    self.env.insert(name, RuntimeValue::Function(name, params, body));
                }
                Node::Import(module) => {
                    if let Err(e) = self.load_module(&module) {
                        println!("Error loading module {}: {}", module, e);
                    } else {
                        if let Some(module_nodes) = self.modules.get(&module) {
                            gui_data.extend(self.execute(module_nodes.clone()));
                        }
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
                                        Node::Say(str) => json!({"type": "say", "value": str}),
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
                            let mut local_interpreter = Interpreter { env: local_env, modules: self.modules.clone() };
                            local_interpreter.execute(body.clone());
                            RuntimeValue::String("".to_string())
                        }
                        _ => stdlib_call(&name, args, self),
                    }
                } else {
                    RuntimeValue::String(format!("Function {} not found", name))
                }
            }
        }
    }
}
