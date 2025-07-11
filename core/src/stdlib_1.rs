use crate::types::VelvetValue;
use regex::Regex;
use std::fs;
use std::path::Path;
use tokio::fs as async_fs;

/// Czyta plik synchronicznie i zwraca zawartość lub błąd.
pub fn fs_read(path: &str) -> VelvetValue {
    match fs::read_to_string(path) {
        Ok(content) => VelvetValue::new("string", &content),
        Err(e) => VelvetValue::new("error", &format!("Failed to read file: {}", e)),
    }
}

/// Czyta plik asynchronicznie i zwraca zawartość lub błąd.
pub async fn fs_read_async(path: &str) -> VelvetValue {
    match async_fs::read_to_string(path).await {
        Ok(content) => VelvetValue::new("string", &content),
        Err(e) => VelvetValue::new("error", &format!("Async read failed: {}", e)),
    }
}

/// Zapisuje plik synchronicznie.
pub fn fs_write(path: &str, content: &str) -> VelvetValue {
    match fs::write(path, content) {
        Ok(_) => VelvetValue::new("string", "File written successfully"),
        Err(e) => VelvetValue::new("error", &format!("Failed to write file: {}", e)),
    }
}

/// Zapisuje plik asynchronicznie.
pub async fn fs_write_async(path: &str, content: &str) -> VelvetValue {
    match async_fs::write(path, content).await {
        Ok(_) => VelvetValue::new("string", "File written successfully"),
        Err(e) => VelvetValue::new("error", &format!("Async write failed: {}", e)),
    }
}

/// Placeholder – sprawdza, czy plik istnieje, nie nasłuchuje zmian!
pub fn fs_watch(path: &str) -> VelvetValue {
    if Path::new(path).exists() {
        VelvetValue::new("string", &format!("Watching file: {}", path))
    } else {
        VelvetValue::new("error", &format!("File not found: {}", path))
    }
}

/// Dopasowanie wyrażenia regularnego do tekstu.
pub fn regex_match(pattern: &str, text: &str) -> VelvetValue {
    match Regex::new(pattern) {
        Ok(re) => {
            if let Some(mat) = re.find(text) {
                VelvetValue::new("string", &format!("Match found: {}", mat.as_str()))
            } else {
                VelvetValue::new("string", "No match")
            }
        }
        Err(e) => VelvetValue::new("error", &format!("Regex error: {}", e)),
    }
}

/// Placeholder dla żądania HTTP GET.
pub fn http_get(url: &str) -> VelvetValue {
    VelvetValue::new("string", &format!("HTTP GET: {}", url))
}
