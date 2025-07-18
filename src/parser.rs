use pest::Parser;
use pest_derive::Parser;
use crate::ast::*;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "velvet.pest"]
pub struct VelvetParser;

pub fn parse(source: &str) -> Result<Vec<Statement>, String> {
    let pairs = VelvetParser::parse(Rule::program, source)
        .map_err(|e| format!("Parse error at {}: {}", e.location, e))?;
    let mut statements = Vec::new();
    for pair in pairs {
        for inner_pair in pair.into_inner() {
            statements.push(parse_statement(inner_pair)?);
        }
    }
    Ok(statements)
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Statement, String> {
    match pair.as_rule() {
        Rule::say => {
            let expr = parse_expr(pair.into_inner().next().unwrap())?;
            Ok(Statement::Say(expr))
        }
        Rule::val => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let type_anno = inner.next().map(|p| p.as_str().to_string());
            let expr = inner.next().map(|p| parse_expr(p).unwrap());
            Ok(Statement::Val(ident, expr, type_anno))
        }
        Rule::const => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let type_anno = inner.next().map(|p| p.as_str().to_string());
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Statement::Const(ident, expr, type_anno))
        }
        Rule::return_stmt => {
            let expr = parse_expr(pair.into_inner().next().unwrap())?;
            Ok(Statement::Return(expr))
        }
        Rule::fn => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let mut params = Vec::new();
            for param in inner.next().unwrap().into_inner() {
                if param.as_rule() == Rule::IDENT {
                    params.push((param.as_str().to_string(), "f64".to_string()));
                } else if param.as_rule() == Rule::TYPE {
                    params.last_mut().unwrap().1 = param.as_str().to_string();
                }
            }
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::Fun(ident, params, body))
        }
        Rule::if_stmt => {
            let mut inner = pair.into_inner();
            let condition = parse_expr(inner.next().unwrap())?;
            let then_block = parse_block(inner.next().unwrap())?;
            let else_block = inner.next().map(|p| parse_block(p).unwrap());
            Ok(Statement::If(condition, then_block, else_block))
        }
        Rule::for_stmt => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::For(ident, expr, body))
        }
        Rule::while_stmt => {
            let mut inner = pair.into_inner();
            let condition = parse_expr(inner.next().unwrap())?;
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::While(condition, body))
        }
        Rule::break_stmt => Ok(Statement::Break),
        Rule::continue_stmt => Ok(Statement::Continue),
        Rule::try_stmt => {
            let mut inner = pair.into_inner();
            let try_block = parse_block(inner.next().unwrap())?;
            let error_ident = inner.next().unwrap().as_str().to_string();
            let catch_block = parse_block(inner.next().unwrap())?;
            Ok(Statement::Try(error_ident, try_block, catch_block))
        }
        Rule::match_stmt => {
            let mut inner = pair.into_inner();
            let expr = parse_expr(inner.next().unwrap())?;
            let mut branches = Vec::new();
            for branch in inner.next().unwrap().into_inner() {
                let mut branch_inner = branch.into_inner();
                let pattern = branch_inner.next().unwrap().as_str().to_string();
                let statements = parse_block(branch_inner.next().unwrap())?;
                branches.push((pattern, statements));
            }
            Ok(Statement::Match(expr, branches))
        }
        Rule::import => {
            let module = pair.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
            Ok(Statement::Import(module))
        }
        Rule::test_stmt => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().trim_matches('"').to_string();
            let body = parse_block(inner.next().unwrap())?;
            Ok(Statement::Test(name, body))
        }
        Rule::expr_stmt => {
            let expr = parse_expr(pair.into_inner().next().unwrap())?;
            Ok(Statement::Expr(expr))
        }
        _ => Err(format!("Unexpected rule: {:?}", pair.as_rule())),
    }
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, String> {
    match pair.as_rule() {
        Rule::logic | Rule::equality | Rule::comparison | Rule::term | Rule::factor => {
            let mut inner = pair.into_inner();
            let mut left = parse_expr(inner.next().unwrap())?;
            while let Some(op) = inner.next() {
                let right = parse_expr(inner.next().unwrap())?;
                left = Expr::Binary(Box::new(left), op.as_str().to_string(), Box::new(right));
            }
            Ok(left)
        }
        Rule::unary => {
            let mut inner = pair.into_inner();
            let op = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Expr::Unary(op, Box::new(expr)))
        }
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::STRING => Ok(Expr::String(inner.as_str().trim_matches('"').to_string())),
                Rule::NUMBER => Ok(Expr::Number(inner.as_str().parse().unwrap_or(0.0))),
                Rule::BOOL => Ok(Expr::Bool(inner.as_str() == "true")),
                Rule::IDENT => Ok(Expr::Ident(inner.as_str().to_string())),
                Rule::call => {
                    let mut inner = inner.into_inner();
                    let name = inner.next().unwrap().as_str().to_string();
                    let mut args = Vec::new();
                    for arg in inner {
                        args.push(parse_expr(arg)?);
                    }
                    Ok(Expr::Call(name, args))
                }
                Rule::list => {
                    let mut elements = Vec::new();
                    for elem in inner.into_inner() {
                        elements.push(parse_expr(elem)?);
                    }
                    Ok(Expr::List(elements))
                }
                Rule::expr => parse_expr(inner),
                _ => Err(format!("Unexpected primary rule: {:?}", inner.as_rule())),
            }
        }
        _ => Err(format!("Unexpected expr rule: {:?}", pair.as_rule())),
    }
}

fn parse_block(pair: pest::iterators::Pair<Rule>) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() != Rule::NEWLINE && inner.as_rule() != Rule::INDENT && inner.as_rule() != Rule::DEDENT {
            statements.push(parse_statement(inner)?);
        }
    }
    Ok(statements)
}
