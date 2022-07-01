extern crate logos;
use logos::Logos;
use core::ops::Range;
use regex::Regex;
use crate::ast::Num;
use num::BigInt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token<'input> {
    #[token("plus", ignore(case))]
    #[token("+")]
    Plus,
    #[token("minus", ignore(case))]
    #[token("-")]
    Minus,
    #[token("times", ignore(case))]
    #[token("*")]
    Times,
    #[token("/")]
    Slash,
    #[token("^")]
    Hat,
    #[token("!")]
    Excl,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("&")]
    #[token("and", ignore(case))]
    And,
    #[token("|")]
    #[token("or", ignore(case))]
    Or,
    #[token("xor", ignore(case))]
    Xor,
    #[token("shift", ignore(case))]
    #[token("<<")]
    LShift,
    #[token(">>")]
    RShift,
    #[regex(r"\d*(d|D)\d+", |lex| parse_roll(lex.slice()))]
    Roll((i64, i64)),
    #[regex(r"-?[0-9]+", |lex| Num::from_integer(lex.slice().parse().unwrap()))]
    #[regex(r"-?0x[0-9a-fA-F]+", |lex| parse_radix(lex.slice(), 16))]
    #[regex(r"-?0o[0-7]+", |lex| parse_radix(lex.slice(), 8))]
    #[regex(r"-?0b[01]+", |lex| parse_radix(lex.slice(), 2))]
    Digits(Num),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice())]
    Var(&'input str),

    #[token("zero", ignore(case))] Zero,
    #[token("one", ignore(case))] One,
    #[token("two", ignore(case))] Two,
    #[token("three", ignore(case))] Three,
    #[token("four", ignore(case))] Four,
    #[token("five", ignore(case))] Five,
    #[token("six", ignore(case))] Six,
    #[token("seven", ignore(case))] Seven,
    #[token("eight", ignore(case))] Eight,
    #[token("nine", ignore(case))] Nine,
    #[token("ten", ignore(case))] Ten,
    #[token("eleven", ignore(case))] Eleven,
    #[token("twelve", ignore(case))] Twelve,
    #[token("thirteen", ignore(case))] Thirteen,
    #[token("fourteen", ignore(case))] Fourteen,
    #[token("fifteen", ignore(case))] Fifteen,
    #[token("sixteen", ignore(case))] Sixteen,
    #[token("seventeen", ignore(case))] Seventeen,
    #[token("eighteen", ignore(case))] Eighteen,
    #[token("nineteen", ignore(case))] Nineteen,
    #[token("twenty", ignore(case))] Twenty,
    #[token("thirty", ignore(case))] Thirty,
    #[token("forty", ignore(case))] Forty,
    #[token("fifty", ignore(case))] Fifty,
    #[token("sixty", ignore(case))] Sixty,
    #[token("seventy", ignore(case))] Seventy,
    #[token("eighty", ignore(case))] Eighty,
    #[token("ninety", ignore(case))] Ninety,
    #[token("hundred", ignore(case))] Hundred,
    #[token("thousand", ignore(case))] Thousand,
    #[token("million", ignore(case))] Million,
    #[token("billion", ignore(case))] Billion,
    #[token("trillion", ignore(case))] Trillion,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Unknown,
}

fn parse_roll(roll: &str) -> (i64, i64) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\d*)(d|D)(\d+)$").unwrap();
    }

    let captures = REGEX.captures(roll).unwrap();

    return (captures[1].parse().unwrap_or(1), captures[3].parse().unwrap());
}

fn parse_radix(istring: &str, radix: u32) -> Num {
    let slice;
    let sign: BigInt;
    if istring.as_bytes()[0] == b'-' {
        slice = &istring.as_bytes()[3..];
        sign = (-1).into();
    } else {
        slice = &istring.as_bytes()[2..];
        sign = 1.into();
    }
    (sign * BigInt::parse_bytes(slice, radix).unwrap()).into()
}

// lalrpop takes an Iterator with item = Result<(Loc, Tok, Loc), LexError>

pub struct TokenLexer<'input> {
    pub lexer: logos::Lexer<'input, Token<'input>>,
}

impl<'input> TokenLexer<'input> {
    pub fn new(input: &'input str) -> TokenLexer<'input> {
        TokenLexer { lexer: Token::lexer(input) }
    }
}

impl<'input> Iterator for TokenLexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(tok) => {
                let Range { start: s, end: e } = self.lexer.span();
                Some(Ok((s, tok, e)))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lexer() {
        let input = "12 + + asdf asdf  zero";
        let lex = Token::lexer(input);

        for (tok, span) in lex.spanned() {
            println!("Token {:?} from {:?}", tok, &input[span]);
        }
    }

    #[test]
    fn test_parse_roll() {
        assert_eq!(parse_roll("3d7"), (3, 7));
        assert_eq!(parse_roll("d8"), (1, 8));
    }

    #[test]
    #[should_panic]
    fn test_parse_roll_fails() {
        parse_roll("3d");
    }
}
