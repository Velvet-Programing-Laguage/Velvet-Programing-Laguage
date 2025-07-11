use crate::types::VelvetValue;

pub fn fs_read(path: &str) -> VelvetValue {
    VelvetValue::new("string", format!("Read file: {}", path))
}

pub fn fs_write(path: &str, content: &str) -> VelvetValue {
    VelvetValue::new("string", format!("Wrote to {}: {}", path, content))
}

pub fn http_get(url: &str) -> VelvetValue {
    VelvetValue::new("string", format!("HTTP GET: {}", url))
}

pub fn math_sqrt(value: f64) -> VelvetValue {
    VelvetValue::new("float", value.sqrt().to_string())
}

pub fn string_upper(s: &str) -> VelvetValue {
    VelvetValue::new("string", s.to_uppercase())
}
