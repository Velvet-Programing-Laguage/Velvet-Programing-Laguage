#[derive(Debug, Clone)]
pub enum Statement {
    Say(Expr),
    Val(String, Option<Expr>, Option<String>),
    Const(String, Expr, Option<String>),
    Fun(String, Vec<(String, String)>, Option<String>, Vec<Statement>),
    If(Expr, Vec<Statement>, Option<Vec<Statement>>),
    For(String, Expr, Vec<Statement>),
    While(Expr, Vec<Statement>),
    Break,
    Continue,
    Try(String, Vec<Statement>, Vec<Statement>),
    Match(Expr, Vec<(String, Vec<Statement>)>),
    Expr(Expr),
    Return(Expr),
    Import(String, String),
    Test(String, Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    Number(f64),
    Bool(bool),
    Ident(String),
    Binary(Box<Expr>, String, Box<Expr>),
    Unary(String, Box<Expr>),
    Call(String, Vec<Expr>),
    List(Vec<Expr>),
    Index(String, Box<Expr>),
}
