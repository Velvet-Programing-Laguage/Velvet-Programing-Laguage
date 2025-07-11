use crate::types::VelvetValue;
use std::fs;
use std::path::Path;

pub fn fs_read(path: &str) -> VelvetValue {
    match fs::read_to_string(path) {
        Ok(content) => VelvetValue::new("string", &content),
        Err(e) => VelvetValue::new("error", &format!("Failed to read file: {}", e)),
    }
}

pub fn fs_write(path: &str, content: &str) -> VelvetValue {
    match fs::write(path, content) {
        Ok(_) => VelvetValue::new("string", "File written successfully"),
        Err(e) => VelvetValue::new("error", &format!("Failed to write file: {}", e)),
    }
}

pub fn fs_watch(path: &str) -> VelvetValue {
    if Path::new(path).exists() {
        VelvetValue::new("string", &format!("Watching file: {}", path))
    } else {
        VelvetValue::new("error", &format!("File not found: {}", path))
    }
}

pub fn http_get(url: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("HTTP GET: {}", url))
}

pub fn math_sqrt(value: f64) -> VelvetValue {
    VelvetValue::new("float", &value.sqrt().to_string())
}

pub fn string_upper(s: &str) -> VelvetValue {
    VelvetValue::new("string", &s.to_uppercase())
}
