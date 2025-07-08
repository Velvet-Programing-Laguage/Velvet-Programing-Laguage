pub mod parser;
pub mod interpreter;
pub mod stdlib;

pub use parser::{Node, WindowProp, Expr, Type};
pub use interpreter::Interpreter;
pub use stdlib::register_stdlib;
