#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(f64),
    Ident(String),
    Call(String, Vec<Expr>),
    List(Vec<Expr>),
    Binary(String, Box<Expr>, Box<Expr>),
    Unary(String, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Say(Expr),
    Let(String, Option<String>, Option<Expr>),
    Const(String, Option<Expr>),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    For(String, Expr, Vec<Statement>),
    Fun(String, Vec<(String, Option<String>)>, Option<String>, Vec<Statement>),
    Expr(Expr),
    // Add other statement types as needed
}
