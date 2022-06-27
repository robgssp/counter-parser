use crate::ast::{self, *};
use crate::types::Result;
use std::collections::HashMap;
use rand;
use num::traits::Zero;
use num::pow::Pow;
use num::{BigRational};

type Env = HashMap<String, ast::Num>;

pub fn eval(expr: &Node, env: &Env) -> Result<ast::Num> {
    match expr {
        Number(i, _) => Ok(i.clone()),
        Roll(n, sides) => Ok(roll(*n, *sides)),
        Var(name) => Ok(env.get(name).map(|v| v.clone()).ok_or_else(
            || Box::new(simple_error!("Unbound variable {:?}", name)))?),
        UnaOp(Factorial, a) => Ok(factorial(eval(&a, env)?)?),
        BinOp(Add, a, b) => Ok(eval(&a, env)? + eval(&b, env)?),
        BinOp(Sub, a, b) => Ok(eval(&a, env)? - eval(&b, env)?),
        BinOp(Mul, a, b) => Ok(eval(&a, env)? * eval(&b, env)?),
        BinOp(Div, a, b) => Ok(eval(&a, env)? / eval(&b, env)?),
        BinOp(Exp, a, b) => Ok(exp(eval(&a, env)?, eval(&b, env)?)?),
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

fn factorial(n: ast::Num) -> Result<ast::Num> {
    if n.is_integer() {
        let top = n.to_integer();
        let mut res = 1.into();
        let mut iter: num::BigInt = 2.into();
        while iter <= top {
            res *= &iter;
            iter += 1;
        }

        return Ok(Num::from_integer(res))
    } else {
        return Err(simple_error!("Factorial of non-integer {:?}", n))?;
    }
}

fn exp(n: ast::Num, e: ast::Num) -> Result<ast::Num> {
    if e.is_integer() {
        let ie = e.to_integer();
        return Ok(n.pow(ie));
    } else {
        return Err(simple_error!("Exponent to non-integer {:?}", e))?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eval() {
        let mut env = Env::new();
        env.insert("i".to_string(), to_num(2));

        assert_eq!(eval(&Number(to_num(5), Digits), &env).unwrap(), to_num(5));
        assert_eq!(eval(&BinOp(Add,
                               Box::new(Number(to_num(1), Digits)),
                               Box::new(Number(to_num(2), Words))),
                        &env) .unwrap(),
                   to_num(3));
        assert_eq!(eval(&BinOp(Add,
                               Box::new(Var("i".to_string())),
                               Box::new(Number(to_num(2), Words))),
                        &env) .unwrap(),
                   to_num(4));
        assert!(eval(&BinOp(Add,
                            Box::new(Var("j".to_string())),
                            Box::new(Number(to_num(2), Words))),
                     &env) .is_err());
    }

    #[test]
    fn test_roll() {
        println!("1d2 -> {}", roll(5, 2));
    }
}
