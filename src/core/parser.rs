use pest::Parser;
use pest_derive::Parser;
use super::ast::{Expr, Statement};

#[derive(Parser)]
#[grammar = "src/parser/velvet.pest"]
pub struct VelvetParser;

pub fn parse_velvet(input: &str) -> Result<Vec<Statement>, pest::error::Error<Rule>> {
    let pairs = VelvetParser::parse(Rule::program, input)?;
    let mut ast = Vec::new();
    for pair in pairs {
        if pair.as_rule() != Rule::EOI {
            ast.push(parse_statement(pair));
        }
    }
    Ok(ast)
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::say => {
            let expr = pair.into_inner().next().unwrap();
            Statement::Say(parse_expr(expr))
        }
        Rule::let => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let ty = inner.next().map(|t| t.as_str().to_string());
            let expr = inner.next().map(parse_expr);
            Statement::Let(ident, ty, expr)
        }
        Rule::const => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = inner.next().map(parse_expr);
            Statement::Const(ident, expr)
        }
        Rule::expr_stmt => {
            let expr = pair.into_inner().next().unwrap();
            Statement::Expr(parse_expr(expr))
        }
        _ => unimplemented!("Statement type not supported: {:?}", pair.as_rule()),
    }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => parse_expr(pair.into_inner().next().unwrap()),
        Rule::pipe => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());
            for pipe in inner {
                expr = Expr::Binary("|>".to_string(), Box::new(expr), Box::new(parse_expr(pipe)));
            }
            expr
        }
        Rule::logic => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());
            for op_pair in inner {
                let op = op_pair.as_str().to_string();
                let right = parse_expr(op_pair.into_inner().next().unwrap());
                expr = Expr::Binary(op, Box::new(expr), Box::new(right));
            }
            expr
        }
        Rule::equality | Rule::comparison | Rule::term | Rule::factor => {
            let mut inner = pair.into_inner();
            let mut expr = parse_expr(inner.next().unwrap());
            for op_pair in inner {
                let op = op_pair.as_str().to_string();
                let right = parse_expr(op_pair.into_inner().next().unwrap());
                expr = Expr::Binary(op, Box::new(expr), Box::new(right));
            }
            expr
        }
        Rule::unary => {
            let mut inner = pair.into_inner();
            let op = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap());
            Expr::Unary(op, Box::new(expr))
        }
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::STRING => Expr::String(inner.as_str().trim_matches('"').to_string()),
                Rule::NUMBER => Expr::Number(inner.as_str().parse().unwrap()),
                Rule::IDENT => Expr::Ident(inner.as_str().to_string()),
                Rule::call => {
                    let mut inner = inner.into_inner();
                    let ident = inner.next().unwrap().as_str().to_string();
                    let args = inner.map(parse_expr).collect();
                    Expr::Call(ident, args)
                }
                Rule::list => {
                    let args = inner.into_inner().map(parse_expr).collect();
                    Expr::List(args)
                }
                Rule::expr => parse_expr(inner),
                _ => unimplemented!("Primary type not supported: {:?}", inner.as_rule()),
            }
        }
        _ => unimplemented!("Expression type not supported: {:?}", pair.as_rule()),
    }
                            }
