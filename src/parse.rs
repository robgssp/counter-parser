use crate::grammar;
use crate::util;
use crate::ast::*;

type ParseResult<'a> =
    Result<Expr, lalrpop_util::ParseError<usize, util::Token<'a>, String>>;

fn good_parse<'a>(r: &ParseResult<'a>) -> bool {
    match r {
        Ok(expr) => match **expr {
            BadParse(_) => false,
            _ => true,
        }
        Err(_) => false,
    }
}

// An expression's size in number of nodes. This is one possibility
// for choosing the best parse of a string. Another would be its
// length in characters.
//
// I think I can capture the span of an expression just at the
// top-level parse in lalrpop, so I'd wrap the top-level expression in
// a "spanned" type.
fn expr_size(e: &Node) -> i32 {
    match e {
        Number(_, _) => 1,
        Roll(_, _) => 2,
        Var(_) => 1,
        UnaOp(_, e) => 1 + expr_size(&e),
        BinOp(_, l, r) => 1 + expr_size(&l) + expr_size(&r),
        Funcall(_, exprs) => 1 + exprs.iter().map(|e| { expr_size(&e) }).sum::<i32>(),
        BadParse(e) => expr_size(&e),
    }
}

pub fn best_parse(line: &str) -> Option<Expr> {
    let lexer = util::TokenLexer::new(line);
    let tokens: Vec<_> = lexer.collect();
    let parser = grammar::TopLevelParser::new();

    let parse1 = parser.parse(line, tokens.iter().cloned());

    if good_parse(&parse1) {
        return Some(parse1.unwrap());
    }

    // Iterator<Item=Expr>
    let parses = std::iter::once(parse1).chain(
        (1..tokens.len()).map(|i| {
            parser.parse(line, tokens[i..].iter().cloned())
        })
    )
        .filter_map(|p| p.ok())
        .map(|p| match *p {
            BadParse(e) => e,
            _ => p,
        });

    parses
        .map(|p| {
            let size = expr_size(&p);
            (p, size)
        })
        .filter(|(_parse, size)| *size > 1)
        .reduce(|(parse1, size1), (parse2, size2)| {
            if size1 >= size2 {
                (parse1, size1)
            } else {
                (parse2, size2)
            }
        })
        .map(|(parse, _size)| parse)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_once() {
        let string = "one + 2 and some junk";

        let parser = grammar::TopLevelParser::new();
        let mut lexer = util::TokenLexer::new(string);

        let parse = parser.parse(string, &mut lexer);

        // println!("{:?} -> {:?} (current token: {:?})", string, parse, lexer.lexer.slice());
        println!("{:?} -> {:?}", string, parse);
    }

    #[test]
    fn test_best_parse() {
        let string = "one + 2";

        println!("Best parse for {:?} is {:?}", string, best_parse(string));
    }
}
