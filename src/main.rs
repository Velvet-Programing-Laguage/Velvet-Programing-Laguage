use velvet::core::{interpreter::VelvetInterpreter, parser::parse_velvet};
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: velvet_core <file.vel>");
        std::process::exit(1);
    }

    let input = fs::read_to_string(&args[1]).expect("Failed to read file");
    let ast = parse_velvet(&input).expect("Failed to parse Velvet program");
    let mut interpreter = VelvetInterpreter::new();
    interpreter.interpret(ast).expect("Failed to interpret Velvet program");
}
