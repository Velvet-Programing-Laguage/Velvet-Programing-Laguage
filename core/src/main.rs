use velvet_core::{parse_velvet, Interpreter};
use std::env;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).expect("Provide a .vel file");
    let code = fs::read_to_string(file).expect("Failed to read file");
    let ast = parse_velvet(&code).expect("Parsing failed");
    let mut interpreter = Interpreter::new();
    let gui_data = interpreter.execute(ast);
    println!("{}", gui_data);
}
