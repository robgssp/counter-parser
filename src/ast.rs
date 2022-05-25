#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64, NumSource),
    Var(String),
    BinOp(BinOpcode, Box<Expr>, Box<Expr>),
    Funcall(String, Vec<Expr>),
    BadParse(Box<Expr>),
}
pub use Expr::*;

#[derive(Debug, PartialEq)]
pub enum BinOpcode {
    Add, Sub, Mul
}
pub use BinOpcode::*;

#[derive(Debug, PartialEq)]
pub enum NumSource {
    Digits, Words
}
pub use NumSource::*;
