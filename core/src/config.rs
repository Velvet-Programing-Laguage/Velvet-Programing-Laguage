use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct Config {
    pub debug: bool,
    pub dependencies: std::collections::HashMap<String, String>,
}

impl Config {
    pub fn load(path: &str) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    }
}
