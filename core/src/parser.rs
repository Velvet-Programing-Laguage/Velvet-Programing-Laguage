use pest::Parser;
use pest_derive::Parser;
use std::error::Error;
use std::fmt;

#[derive(Parser)]
#[grammar = "velvet.pest"]
pub struct VelvetParser;

#[derive(Debug, Clone)]
pub enum Node {
    Say(Box<Expr>),
    Set(String, Box<Expr>),
    If(Box<Expr>, Vec<Node>, Option<Vec<Node>>),
    For(String, Box<Expr>, Vec<Node>),
    Function(String, Vec<String>, Vec<Node>),
    Import(String),
    Window(Vec<WindowProp>),
}

#[derive(Debug, Clone)]
pub enum WindowProp {
    Title(String),
    Size(u32, u32),
    Button(String, Vec<Node>),
    TextInput(String, String), // Added: Text input with id and placeholder
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(f64),
    Ident(String),
    Arith(Box<Expr>, String, Box<Expr>),
    Call(String, Vec<Expr>), // Added: Function call
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

pub fn parse_velvet(code: &str) -> Result<Vec<Node>, Box<dyn Error>> {
    let pairs = VelvetParser::parse(Rule::program, code)?;
    let mut nodes = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::say => {
                let expr = parse_expr(pair.into_inner().next().unwrap())?;
                nodes.push(Node::Say(Box::new(expr)));
            }
            Rule::set => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let expr = parse_expr(inner.next().unwrap())?;
                nodes.push(Node::Set(ident, Box::new(expr)));
            }
            Rule::if_stmt => {
                let mut inner = pair.into_inner();
                let condition = parse_expr(inner.next().unwrap())?;
                let then_branch = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                let else_branch = inner
                    .next()
                    .map(|else_pair| {
                        else_pair
                            .into_inner()
                            .filter(|p| p.as_rule() != Rule::DEDENT)
                            .map(parse_statement)
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .transpose()?;
                nodes.push(Node::If(Box::new(condition), then_branch, else_branch));
            }
            Rule::for_stmt => {
                let mut inner = pair.into_inner();
                let var = inner.next().unwrap().as_str().to_string();
                let range = parse_expr(inner.next().unwrap())?;
                let body = inner
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::For(var, Box::new(range), body));
            }
            Rule::function => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let params = inner
                    .next()
                    .map(|p| {
                        p.into_inner()
                            .filter(|p| p.as_rule() == Rule::IDENT)
                            .map(|p| p.as_str().to_string())
                            .collect::<Vec<_>>()
                    })
                    .unwrap_or_default();
                let body = inner
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::Function(name, params, body));
            }
            Rule::import => {
                let module = pair.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                nodes.push(Node::Import(module));
            }
            Rule::window => {
                let mut props = Vec::new();
                for prop in pair.into_inner().filter(|p| p.as_rule() != Rule::DEDENT) {
                    match prop.as_rule() {
                        Rule::title => {
                            let title = prop.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                            props.push(WindowProp::Title(title));
                        }
                        Rule::size => {
                            let mut inner = prop.into_inner();
                            let w = inner.next().unwrap().as_str().parse()?;
                            let h = inner.next().unwrap().as_str().parse()?;
                            props.push(WindowProp::Size(w, h));
                        }
                        Rule::button => {
                            let mut inner = prop.into_inner();
                            let text = inner.next().unwrap().as_str().trim_matches('"').to_string();
                            let actions = inner
                                .filter(|p| p.as_rule() != Rule::DEDENT)
                                .map(parse_statement)
                                .collect::<Result<Vec<_>, _>>()?;
                            props.push(WindowProp::Button(text, actions));
                        }
                        Rule::textinput => {
                            let mut inner = prop.into_inner();
                            let id = inner.next().unwrap().as_str().trim_matches('"').to_string();
                            let placeholder = inner.next().unwrap().as_str().trim_matches('"').to_string();
                            props.push(WindowProp::TextInput(id, placeholder));
                        }
                        _ => {}
                    }
                }
                nodes.push(Node::Window(props));
            }
            _ => {}
        }
    }
    Ok(nodes)
}

fn parse_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, Box<dyn Error>> {
    match pair.as_rule() {
        Rule::STRING => Ok(Expr::String(pair.as_str().trim_matches('"').to_string())),
        Rule::NUMBER => Ok(Expr::Number(pair.as_str().parse()?)),
        Rule::IDENT => Ok(Expr::Ident(pair.as_str().to_string())),
        Rule::arith_expr => {
            let mut inner = pair.into_inner();
            let left = parse_expr(inner.next().unwrap())?;
            let op = inner.next().unwrap().as_str().to_string();
            let right = parse_expr(inner.next().unwrap())?;
            Ok(Expr::Arith(Box::new(left), op, Box::new(right)))
        }
        Rule::call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = inner
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Expr::Call(name, args))
        }
        _ => Err(Box::new(ParseError {
            message: format!("Invalid expression: {}", pair.as_str()),
        })),
    }
}

fn parse_statement(pair: pest::iterators::Pair<Rule>) -> Result<Node, Box<dyn Error>> {
    match pair.as_rule() {
        Rule::say => {
            let expr = parse_expr(pair.into_inner().next().unwrap())?;
            Ok(Node::Say(Box::new(expr)))
        }
        Rule::set => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Node::Set(ident, Box::new(expr)))
        }
        Rule::for_stmt => {
            let mut inner = pair.into_inner();
            let var = inner.next().unwrap().as_str().to_string();
            let range = parse_expr(inner.next().unwrap())?;
            let body = inner
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(parse_statement)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Node::For(var, Box::new(range), body))
        }
        Rule::function => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let params = inner
                .next()
                .map(|p| {
                    p.into_inner()
                        .filter(|p| p.as_rule() == Rule::IDENT)
                        .map(|p| p.as_str().to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let body = inner
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(parse_statement)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Node::Function(name, params, body))
        }
        _ => Err(Box::new(ParseError {
            message: format!("Invalid statement: {}", pair.as_str()),
        })),
    }
}