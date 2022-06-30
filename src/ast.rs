use num::rational::BigRational;

pub type Num = BigRational;

#[derive(Debug, PartialEq)]
pub enum Node {
    Number(Num, NumSource),
    Roll(i64, i64),
    Var(String),
    BinOp(BinOpcode, Expr, Expr),
    UnaOp(UnaOpcode, Expr),
    Funcall(String, Vec<Expr>),
    BadParse(Expr),
}
pub use Node::*;

pub type Expr = Box<Node>;

#[derive(Debug, PartialEq)]
pub enum UnaOpcode {
    Factorial,
}
pub use UnaOpcode::*;

#[derive(Debug, PartialEq)]
pub enum BinOpcode {
    Add, Sub, Mul, Div, Exp, And, Or, Xor, LShift, RShift
}
pub use BinOpcode::*;

#[derive(Debug, PartialEq)]
pub enum NumSource {
    Digits, Words
}
pub use NumSource::*;

pub fn to_num(i: i64) -> Num {
    Num::from_integer(i.into())
}
