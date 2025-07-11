#[cfg(test)]
mod tests {
    use velvet::core::{parser::parse_velvet, interpreter::VelvetInterpreter};

    #[test]
    fn test_parser() {
        let input = r#"
            say "Hello, Velvet!"
            let x: Number = 42
            fun greet(name: String) -> {
                say "Hello, " + name
            }
        "#;
        let ast = parse_velvet(input).unwrap();
        assert_eq!(ast.len(), 3);
    }

    #[test]
    fn test_interpreter() {
        let input = r#"
            say "Hello, Velvet!"
            let x: Number = 42
            say x
            fun greet(name: String) -> {
                say "Hello, " + name
            }
            greet("Alice")
        "#;
        let ast = parse_velvet(input).unwrap();
        let mut interpreter = VelvetInterpreter::new();
        assert!(interpreter.interpret(ast).is_ok());
    }
}
