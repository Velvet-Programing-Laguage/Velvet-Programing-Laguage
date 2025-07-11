use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub gui: GuiConfig,
    #[serde(default)]
    pub plugins: Vec<String>,
    #[serde(default)]
    pub runtime: RuntimeConfig,
}

#[derive(Default, Deserialize)]
pub struct GuiConfig {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub wayland_enabled: bool,
}

#[derive(Default, Deserialize)]
pub struct RuntimeConfig {
    #[serde(default)]
    pub max_threads: usize,
    #[serde(default)]
    pub async_enabled: bool,
}

impl Config {
    /// Loads configuration from a JSON file. Returns None if file is missing or invalid.
    pub fn load(path: &str) -> Option<Self> {
        let mut file = File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    }
}
