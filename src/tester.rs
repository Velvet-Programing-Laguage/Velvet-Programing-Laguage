use crate::cli;
use crate::parser;
use crate::interpreter;
use std::fs;

pub fn run_tests() -> Result<(), String> {
    let test_dir = "tests";
    if !fs::metadata(test_dir).is_ok() {
        return Err("No tests directory".to_string());
    }

    let mut test_count = 0;
    let mut passed = 0;
    for entry in fs::read_dir(test_dir)? {
        let path = entry?.path();
        if path.extension().map_or(false, |ext| ext == "velvet") {
            test_count += 1;
            cli::info(&format!("Running {}", path.display()));
            let source = fs::read_to_string(&path)?;
            let ast = parser::parse(&source)?;
            if interpreter::run(ast, false).is_ok() {
                passed += 1;
                cli::success(&format!("{} passed", path.display()));
            } else {
                cli::error(&format!("{} failed", path.display()));
            }
        }
    }
    cli::success(&format!("{}/{} tests passed", passed, test_count));
    Ok(())
}
