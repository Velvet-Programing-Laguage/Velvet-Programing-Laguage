use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Default, serde::Deserialize)]
pub struct Config {
    pub debug: bool,
    pub dependencies: std::collections::HashMap<String, String>,
    pub gui: GuiConfig,
    pub plugins: Vec<String>,
}

#[derive(Default, serde::Deserialize)]
pub struct GuiConfig {
    pub theme: String,
    pub wayland_enabled: bool,
}

impl Config {
    pub fn load(path: &str) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    }
}
