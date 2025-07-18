use std::fs;
use std::path::Path;

pub fn check_project() -> Result<(), String> {
    if !Path::new("main.velvet").exists() || !Path::new("velvet.json").exists() {
        return Err("Missing main.velvet or velvet.json".to_string());
    }
    Ok(())
}

pub fn init_project() -> Result<(), String> {
    fs::create_dir_all("lib/.velvet_library")?;
    fs::create_dir_all("tests")?;
    fs::write("main.velvet", "@ Simple Velvet program\nsay \"Hello, Velvet!\"\n")?;
    fs::write("velvet.json", r#"{"name":"new-project","version":"0.1.0","dependencies":{},"modules":[]}"#)?;
    fs::write("tests/test_basic.velvet", r#"@ Basic test
test "basic test":
    say "Test passed"
"#)?;
    Ok(())
}

pub fn clean_project() -> Result<(), String> {
    for file in &["velvet_out", "velvet_out.rs"] {
        if Path::new(file).exists() {
            fs::remove_file(file)?;
        }
    }
    Ok(())
}
