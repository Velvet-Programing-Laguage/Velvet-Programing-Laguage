pub mod parser;
pub mod interpreter;
pub mod stdlib_1;
pub mod stdlib_2;

pub use parser::parse_velvet;
pub use interpreter::{Interpreter, RuntimeValue};
