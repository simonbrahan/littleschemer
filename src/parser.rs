#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Symbol(String),
    String(String),
    List(Vec<Expr>),
}
