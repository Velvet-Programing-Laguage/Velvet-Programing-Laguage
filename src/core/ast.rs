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
    Expr(Expr),
    // Add other statement types (if, for, fun, etc.) as needed
}
