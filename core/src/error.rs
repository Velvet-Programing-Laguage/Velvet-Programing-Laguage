use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VelvetError {
    pub kind: ErrorKind,
    pub message: String,
    pub stack_trace: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    ParseError,
    RuntimeError,
    ModuleError,
    AsyncError,
}

impl VelvetError {
    /// Tworzy nowy VelvetError z podanym typem błędu i komunikatem.
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
            stack_trace: Vec::new(),
        }
    }

    /// Ustawia cały stack trace i zwraca zmodyfikowany obiekt błędu.
    pub fn with_stack_trace(mut self, trace: Vec<String>) -> Self {
        self.stack_trace = trace;
        self
    }

    /// Dodaje pojedynczy wpis do stack trace.
    pub fn add_trace_line(&mut self, line: String) {
        self.stack_trace.push(line);
    }
}

impl fmt::Display for VelvetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            match self.kind {
                ErrorKind::ParseError => "Parse error",
                ErrorKind::RuntimeError => "Runtime error",
                ErrorKind::ModuleError => "Module error",
                ErrorKind::AsyncError => "Async error",
            },
            self.message
        )?;
        if !self.stack_trace.is_empty() {
            write!(f, "\nStack trace:\n{}", self.stack_trace.join("\n"))?;
        }
        Ok(())
    }
}

impl std::error::Error for VelvetError {}
