use std::fs;
use std::process::Command;

pub fn read_file(path: &str) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read {}: {}", path, e))
}

pub fn write_file(path: &str, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Cannot write {}: {}", path, e))
}

pub fn run_python_script(script: &str, args: &[&str]) -> Result<std::process::Output, String> {
    Command::new("python3")
        .arg(script)
        .args(args)
        .output()
        .map_err(|e| format!("Python script error: {}", e))
}
