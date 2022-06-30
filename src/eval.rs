use crate::ast::{self, *};
use crate::types::Result;
use std::collections::HashMap;
use rand;
use num::traits::Zero;
use num::pow::Pow;
use num::{BigRational, BigInt, ToPrimitive};

type Env = HashMap<String, ast::Num>;

pub fn eval(expr: &Node, env: &Env) -> Result<ast::Num> {
    match expr {
        Number(i, _) => Ok(i.clone()),
        Roll(n, sides) => Ok(roll(*n, *sides)),
        Var(name) => Ok(env.get(name).map(|v| v.clone()).ok_or_else(
            || Box::new(simple_error!("Unbound variable {:?}", name)))?),
        UnaOp(Factorial, a) => Ok(factorial(eval(&a, env)?)?),
        BinOp(op, l, r) => {
            let a = eval(&l, env)?;
            let b = eval(&r, env)?;

            let res = match op {
                Add => a + b,
                Sub => a - b,
                Mul => a * b,
                Div => a / b,
                Exp => exp(a, b)?,
                And => bitand(a, b)?,
                Or  => bitor(a, b)?,
                Xor => bitxor(a, b)?,
                LShift => bitshift(a, b)?,
                RShift => bitshift(a, -b)?,
            };
            Ok(res)
        }
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
    if let Some(top) = to_int(&n) {
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
    match to_int(&e) {
        Some(ie) => return Ok(n.pow(ie)),
        None => Err(simple_error!("Exponent to non-integer {:?}", e))?,
    }
}

fn bitand(a: ast::Num, b: ast::Num) -> Result<ast::Num> {
    match (to_int(&a), to_int(&b)) {
        (Some(ia), Some(ib)) => Ok((ia & ib).into()),
        _ => Err(simple_error!("Bitand of non-integers"))?,
    }
}

fn bitor(a: ast::Num, b: ast::Num) -> Result<ast::Num> {
    match (to_int(&a), to_int(&b)) {
        (Some(ia), Some(ib)) => Ok((ia | ib).into()),
        _ => Err(simple_error!("Bitor of non-integers"))?,
    }
}

fn bitxor(a: ast::Num, b: ast::Num) -> Result<ast::Num> {
    match (to_int(&a), to_int(&b)) {
        (Some(ia), Some(ib)) => Ok((ia ^ ib).into()),
        _ => Err(simple_error!("Bitxor of non-integers"))?,
    }
}

fn bitshift(a: ast::Num, b: ast::Num) -> Result<ast::Num> {
    match (to_int(&a), to_int(&b).and_then(|i| i.to_isize())) {
        (Some(ia), Some(ib)) => {
            if ib > 0 {
                Ok((ia << ib).into())
            } else {
                Ok((ia >> -ib).into())
            }
        }
        _ => Err(simple_error!("Bit shift of non-integers"))?,
    }
}

fn to_int(n: &ast::Num) -> Option<BigInt> {
    if n.is_integer() {
        Some(n.to_integer())
    } else {
        None
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
