use std::fmt;

#[derive(Debug)]
pub enum VelvetError {
    ParseError(String),
    RuntimeError(String),
    ModuleError(String),
}

impl fmt::Display for VelvetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VelvetError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            VelvetError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            VelvetError::ModuleError(msg) => write!(f, "Module error: {}", msg),
        }
    }
}

impl std::error::Error for VelvetError {}
