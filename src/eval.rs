use crate::ast::*;
use std::error::Error;
use simple_error::SimpleError;

fn eval(expr: &Expr) -> Result<i64, Box<dyn Error>> {
    match expr {
        Number(i, _) => Ok(*i),
        BinOp(Add, a, b) => Ok(eval(&a)? + eval(&b)?),
        BinOp(Sub, a, b) => Ok(eval(&a)? - eval(&b)?),
        BinOp(Mul, a, b) => Ok(eval(&a)? * eval(&b)?),
        _ => Err(simple_error!("Unknown expr {:?}", expr))?,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        assert_eq!(eval(&Number(5, Digits)).unwrap(), 5);
        assert_eq!(eval(&BinOp(Add,
                               Box::new(Number(1, Digits)),
                               Box::new(Number(2, Words))))
                   .unwrap(),
                   3);
    }
}
