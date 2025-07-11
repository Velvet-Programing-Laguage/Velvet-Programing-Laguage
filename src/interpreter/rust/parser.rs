use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/velvet.pest"]
pub struct VelvetParser;

pub fn parse_velvet(input: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let pairs = VelvetParser::parse(Rule::program, input)?;
    let mut ast = Vec::new();
    for pair in pairs {
        ast.push(parse_statement(pair));
    }
    Ok(ast)
}

#[derive(Debug)]
pub enum Statement {
    Say(String),
    // Add other statement types as needed
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::say => {
            let expr = pair.into_inner().next().unwrap();
            Statement::Say(expr.as_str().to_string())
        }
        _ => unimplemented!(),
    }
}
