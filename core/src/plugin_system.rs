use crate::types::VelvetValue;
use std::collections::HashMap;

pub struct PluginSystem {
    plugins: HashMap<String, Box<dyn Fn(&str) -> VelvetValue>>,
}

impl PluginSystem {
    pub fn new() -> Self {
        PluginSystem {
            plugins: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, func: Box<dyn Fn(&str) -> VelvetValue>) {
        self.plugins.insert(name.to_string(), func);
    }

    pub fn execute(&self, name: &str, args: &str) -> Option<VelvetValue> {
        self.plugins.get(name).map(|f| f(args))
    }
}
