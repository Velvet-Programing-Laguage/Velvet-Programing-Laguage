use pest::Parser;
use pest_derive::Parser;
use std::error::Error;
use std::fmt;

#[derive(Parser)]
#[grammar = "velvet.pest"]
pub struct VelvetParser;

#[derive(Debug, Clone)]
pub enum Node {
    Say(Expr),
    Let(String, Expr),
    Const(String, Expr),
    If(Expr, Vec<Node>, Option<Vec<Node>>),
    For(String, i32, i32, Vec<Node>),
    Fn(String, Vec<String>, Vec<Node>),
    Import(String),
    Try(Vec<Node>, String, Vec<Node>),
    Struct(String, Vec<(String, Type)>),
    Match(Expr, Vec<(Pattern, Vec<Node>)>),
    Return(Expr),
    Window(Vec<WindowProp>),
}

#[derive(Debug, Clone)]
pub enum WindowProp {
    Title(String),
    Size(u32, u32),
    Button(String, Vec<Node>),
    TextInput(String, String),
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(f64),
    Call(String, Vec<Expr>),
    List(Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Type {
    String,
    Number,
    List,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    String(String),
    Number(f64),
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
                nodes.push(Node::Say(expr));
            }
            Rule::let => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let expr = parse_expr(inner.next().unwrap())?;
                nodes.push(Node::Let(ident, expr));
            }
            Rule::const => {
                let mut inner = pair.into_inner();
                let ident = inner.next().unwrap().as_str().to_string();
                let expr = parse_expr(inner.next().unwrap())?;
                nodes.push(Node::Const(ident, expr));
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
                nodes.push(Node::If(condition, then_branch, else_branch));
            }
            Rule::for_stmt => {
                let mut inner = pair.into_inner();
                let var = inner.next().unwrap().as_str().to_string();
                let start = inner.next().unwrap().as_str().parse()?;
                let end = inner.next().unwrap().as_str().parse()?;
                let body = inner
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::For(var, start, end, body));
            }
            Rule::fn => {
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
                nodes.push(Node::Fn(name, params, body));
            }
            Rule::import => {
                let module = pair.into_inner().next().unwrap().as_str().trim_matches('"').to_string();
                nodes.push(Node::Import(module));
            }
            Rule::try_stmt => {
                let mut inner = pair.into_inner();
                let try_body = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                let catch_var = inner.next().unwrap().as_str().to_string();
                let catch_body = inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(parse_statement)
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::Try(try_body, catch_var, catch_body));
            }
            Rule::struct => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let fields = inner
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(|p| {
                        let mut field = p.into_inner();
                        let field_name = field.next().unwrap().as_str().to_string();
                        let field_type = match field.next().unwrap().as_str() {
                            "String" => Type::String,
                            "Number" => Type::Number,
                            "List" => Type::List,
                            _ => Type::String,
                        };
                        Ok((field_name, field_type))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::Struct(name, fields));
            }
            Rule::match_stmt => {
                let mut inner = pair.into_inner();
                let expr = parse_expr(inner.next().unwrap())?;
                let branches = inner
                    .filter(|p| p.as_rule() != Rule::DEDENT)
                    .map(|p| {
                        let mut branch = p.into_inner();
                        let pattern = parse_pattern(branch.next().unwrap())?;
                        let body = branch
                            .map(parse_statement)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok((pattern, body))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                nodes.push(Node::Match(expr, branches));
            }
            Rule::return_stmt => {
                let expr = parse_expr(pair.into_inner().next().unwrap())?;
                nodes.push(Node::Return(expr));
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
        Rule::call => {
            let mut inner = pair.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = inner
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Expr::Call(name, args))
        }
        Rule::list => {
            let items = pair
                .into_inner()
                .filter(|p| p.as_rule() != Rule::EOI)
                .map(parse_expr)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Expr::List(items))
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
            Ok(Node::Say(expr))
        }
        Rule::let => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Node::Let(ident, expr))
        }
        Rule::const => {
            let mut inner = pair.into_inner();
            let ident = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Node::Const(ident, expr))
        }
        Rule::for_stmt => {
            let mut inner = pair.into_inner();
            let var = inner.next().unwrap().as_str().to_string();
            let start = inner.next().unwrap().as_str().parse()?;
            let end = inner.next().unwrap().as_str().parse()?;
            let body = inner
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(parse_statement)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Node::For(var, start, end, body))
        }
        Rule::fn => {
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
            Ok(Node::Fn(name, params, body))
        }
        Rule::try_stmt => {
            let mut inner = pair.into_inner();
            let try_body = inner
                .next()
                .unwrap()
                .into_inner()
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(parse_statement)
                .collect::<Result<Vec<_>, _>>()?;
            let catch_var = inner.next().unwrap().as_str().to_string();
            let catch_body = inner
                .next()
                .unwrap()
                .into_inner()
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(parse_statement)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Node::Try(try_body, catch_var, catch_body))
        }
        Rule::match_stmt => {
            let mut inner = pair.into_inner();
            let expr = parse_expr(inner.next().unwrap())?;
            let branches = inner
                .filter(|p| p.as_rule() != Rule::DEDENT)
                .map(|p| {
                    let mut branch = p.into_inner();
                    let pattern = parse_pattern(branch.next().unwrap())?;
                    let body = branch
                        .map(parse_statement)
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok((pattern, body))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Node::Match(expr, branches))
        }
        Rule::return_stmt => {
            let expr = parse_expr(pair.into_inner().next().unwrap())?;
            Ok(Node::Return(expr))
        }
        _ => Err(Box::new(ParseError {
            message: format!("Invalid statement: {}", pair.as_str()),
        })),
    }
}

fn parse_pattern(pair: pest::iterators::Pair<Rule>) -> Result<Pattern, Box<dyn Error>> {
    match pair.as_rule() {
        Rule::STRING => Ok(Pattern::String(pair.as_str().trim_matches('"').to_string())),
        Rule::NUMBER => Ok(Pattern::Number(pair.as_str().parse()?)),
        _ => Err(Box::new(ParseError {
            message: format!("Invalid pattern: {}", pair.as_str()),
        })),
    }
                }
