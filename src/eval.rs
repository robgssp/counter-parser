use crate::ast::*;
use std::error::Error;
use simple_error::SimpleError;
use std::collections::HashMap;

type Env = HashMap<String, i64>;

fn eval(expr: &Expr, env: &Env) -> Result<i64, Box<dyn Error>> {
    match expr {
        Number(i, _) => Ok(*i),
        Var(name) => Ok(env.get(name).map(|v| *v).ok_or_else(
            || Box::new(simple_error!("Unbound variable {:?}", name)))?),
        BinOp(Add, a, b) => Ok(eval(&a, env)? + eval(&b, env)?),
        BinOp(Sub, a, b) => Ok(eval(&a, env)? - eval(&b, env)?),
        BinOp(Mul, a, b) => Ok(eval(&a, env)? * eval(&b, env)?),
        _ => Err(simple_error!("Unknown expr {:?}", expr))?,
    }
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
}
