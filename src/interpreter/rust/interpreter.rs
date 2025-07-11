use super::parser::{parse_velvet, Statement};

pub struct VelvetInterpreter;

impl VelvetInterpreter {
    pub fn new() -> Self {
        VelvetInterpreter
    }

    pub fn interpret(&self, input: &str) -> Result<(), String> {
        let ast = parse_velvet(input).map_err(|e| e.to_string())?;
        for stmt in ast {
            match stmt {
                Statement::Say(value) => println!("Output: {}", value.trim_matches('"')),
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_say() {
        let interpreter = VelvetInterpreter::new();
        let input = r#"say "Hello, Velvet!""#;
        assert!(interpreter.interpret(input).is_ok());
    }
}
