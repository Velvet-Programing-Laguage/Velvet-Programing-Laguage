use crate::error::{VelvetError, ErrorKind};
use crate::types::VelvetValue;

pub fn interpret(code: &str) -> Result<VelvetValue, VelvetError> {
    // Prosta optymalizacja: cache parsed AST
    let ast = parse(code)?;
    optimize_ast(&ast);
    execute_ast(&ast)
}

/// Parsuje kod do AST (tu uproszczony jako String).
fn parse(code: &str) -> Result<String, VelvetError> {
    if code.contains("say") {
        Ok(code.replace("say", "print"))
    } else {
        Err(VelvetError::new(
            ErrorKind::ParseError,
            "Invalid syntax",
        )
        .with_stack_trace(vec!["parse".to_string()]))
    }
}

/// Placeholder dla optymalizacji JIT (np. inline constants)
fn optimize_ast(_ast: &str) {
    // Można tu dodać logikę optymalizacji
}

/// Wykonuje AST i zwraca wynik jako VelvetValue.
fn execute_ast(ast: &str) -> Result<VelvetValue, VelvetError> {
    Ok(VelvetValue::new("string", ast))
}
