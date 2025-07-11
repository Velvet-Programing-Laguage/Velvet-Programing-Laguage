use crate::interpreter::interpret;
use crate::logger::Logger;
use std::io::{self, Write};

pub fn start_repl(logger: &Logger) {
    logger.info("Starting Velvet REPL. Type 'exit' to quit.");
    loop {
        print!("velvet> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        match interpret(input) {
            Ok(result) => logger.info(&format!("Result: {}", result.value)),
            Err(e) => logger.error(&format!("Error: {}", e)),
        }
    }
}
