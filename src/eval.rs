use crate::ast::{self, *};
use crate::types::Result;
use std::collections::HashMap;
use rand;
use num::traits::Zero;
use num::{FromPrimitive, BigRational};

type Env = HashMap<String, ast::Num>;

pub fn eval(expr: &Node, env: &Env) -> Result<ast::Num> {
    match expr {
        Number(i, _) => Ok(i.clone()),
        Roll(n, sides) => Ok(roll(*n, *sides)),
        Var(name) => Ok(env.get(name).map(|v| v.clone()).ok_or_else(
            || Box::new(simple_error!("Unbound variable {:?}", name)))?),
        BinOp(Add, a, b) => Ok(eval(&a, env)? + eval(&b, env)?),
        BinOp(Sub, a, b) => Ok(eval(&a, env)? - eval(&b, env)?),
        BinOp(Mul, a, b) => Ok(eval(&a, env)? * eval(&b, env)?),
        BinOp(Div, a, b) => Ok(eval(&a, env)? / eval(&b, env)?),
        Funcall(f, _args) => Err(simple_error!("Unknown function '{}'", f))?,
        BadParse(e) => Err(simple_error!(
            "Bad parse encountered in execution! near {:?}", e))?,
    }
}

fn roll(n: i64, sides: i64) -> ast::Num {
    let mut res = Zero::zero();
    for _ in 0..n {
        res += BigRational::from_integer(
            (rand::random::<u64>() % (sides as u64) + 1).into());
    }
    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let mut env = Env::new();
        env.insert("i".to_string(), 2);

        assert_eq!(eval(&Number(5, Digits), &env).unwrap(), 5);
        assert_eq!(eval(&BinOp(Add,
                               Box::new(Number(1, Digits)),
                               Box::new(Number(2, Words))),
                        &env) .unwrap(),
                   3);
        assert_eq!(eval(&BinOp(Add,
                               Box::new(Var("i".to_string())),
                               Box::new(Number(2, Words))),
                        &env) .unwrap(),
                   4);
        assert!(eval(&BinOp(Add,
                            Box::new(Var("j".to_string())),
                            Box::new(Number(2, Words))),
                     &env) .is_err());
    }

    #[test]
    fn test_roll() {
        println!("1d2 -> {}", roll(5, 2));
    }
}
