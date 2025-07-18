use std::env;
use std::process;

mod parser;
mod ast;
mod interpreter;
mod compiler;
mod utils;
mod velvet_config;
mod cli;
mod repl;
mod tester;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        cli::error("No command provided. Use 'vel help' for usage.");
        process::exit(1);
    }

    match args[1].as_str() {
        "help" => cli::print_help(),
        "start" => run_project(&args),
        "update" => update_libraries(),
        "install" => install_library(&args),
        "build" => build_project(),
        "init" => init_project(),
        "debug" => debug_project(&args),
        "test" => tester::run_tests(),
        "clean" => clean_project(),
        "version" => cli::print_version(),
        "repl" => repl::start(),
        "fmt" => format_project(),
        "list-libs" => list_libraries(),
        "check" => check_project(),
        _ => {
            cli::error(&format!("Unknown command '{}'. Use 'vel help' for usage.", args[1]));
            process::exit(1);
        }
    }
}

fn run_project(args: &[String]) {
    velvet_config::check_project().expect("Not in a valid Velvet project directory");
    let file = args.get(2).cloned().unwrap_or("main.velvet".to_string());
    let source = utils::read_file(&file).expect("Failed to read source file");
    let ast = parser::parse(&source).expect("Parse error");
    interpreter::run(ast, false).expect("Execution error");
}

fn update_libraries() {
    utils::run_python_script("lib_manager.py", &["update"]).expect("Failed to update libraries");
    cli::success("Libraries updated successfully.");
}

fn install_library(args: &[String]) {
    if args.len() < 4 {
        cli::error("Usage: vel install <lang> <command>");
        process::exit(1);
    }
    let lang = &args[2];
    let command = args[3..].join(" ");
    utils::run_python_script("lib_manager.py", &["install", lang, &command])
        .expect("Failed to install library");
    cli::success(&format!("Installed {} for {}", command, lang));
}

fn build_project() {
    velvet_config::check_project().expect("Not in a valid Velvet project directory");
    let source = utils::read_file("main.velvet").expect("Failed to read main.velvet");
    let ast = parser::parse(&source).expect("Parse error");
    compiler::compile(ast).expect("Compilation error");
    cli::success("Project compiled to 'velvet_out' executable.");
}

fn init_project() {
    velvet_config::init_project().expect("Failed to initialize project");
    cli::success("Velvet project initialized successfully.");
}

fn debug_project(args: &[String]) {
    velvet_config::check_project().expect("Not in a valid Velvet project directory");
    let file = args.get(2).cloned().unwrap_or("main.velvet".to_string());
    let source = utils::read_file(&file).expect("Failed to read source file");
    let ast = parser::parse(&source).expect("Parse error");
    interpreter::run(ast, true).expect("Debug execution error");
}

fn clean_project() {
    velvet_config::clean_project().expect("Failed to clean project");
    cli::success("Project cleaned successfully.");
}

fn format_project() {
    velvet_config::check_project().expect("Not in a valid Velvet project directory");
    let source = utils::read_file("main.velvet").expect("Failed to read main.velvet");
    let formatted = format_code(&source);
    utils::write_file("main.velvet", &formatted).expect("Failed to write formatted code");
    cli::success("Project files formatted successfully.");
}

fn list_libraries() {
    let output = utils::run_python_script("lib_manager.py", &["list"]).expect("Failed to list libraries");
    cli::success("Installed libraries:");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn check_project() {
    if velvet_config::check_project().is_ok() {
        cli::success("Project configuration is valid.");
    } else {
        cli::error("Invalid project configuration.");
        process::exit(1);
    }
}

fn format_code(source: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level = 0;
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('@') {
            continue;
        }
        if trimmed.ends_with(':') {
            formatted.push_str(&"    ".repeat(indent_level));
            formatted.push_str(trimmed);
            formatted.push('\n');
            indent_level += 1;
        } else {
            formatted.push_str(&"    ".repeat(indent_level));
            formatted.push_str(trimmed);
            formatted.push('\n');
        }
        if trimmed == "}" {
            indent_level = indent_level.saturating_sub(1);
        }
    }
    formatted
}
