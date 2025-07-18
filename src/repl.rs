use crate::cli;
use crate::interpreter;
use crate::parser;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::fs;

pub fn start() {
    let mut env = std::collections::HashMap::new();
    let mut history = VecDeque::new();
    let history_file = ".velvet_history";
    if let Ok(content) = fs::read_to_string(history_file) {
        history.extend(content.lines().map(String::from));
    }

    println!("\x1b[1;34mVelvet REPL v1.4 (exit, clear)\x1b[0m");
    loop {
        print!("\x1b[1;36m>> \x1b[0m");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "exit" {
            fs::write(history_file, history.iter().collect::<Vec<_>>().join("\n"))?;
            break;
        }
        if input == "clear" {
            history.clear();
            cli::success("History cleared.");
            continue;
        }
        if input.is_empty() {
            continue;
        }

        history.push_back(input.to_string());
        match parser::parse(input) {
            Ok(statements) => match interpreter::run(statements, false) {
                Ok(_) => {}
                Err(e) => cli::error(&e),
            },
            Err(e) => cli::error(&e),
        }
    }
}
