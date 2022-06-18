extern crate logos;
use logos::Logos;
use core::ops::Range;
use regex::Regex;
// use std::str::FromStr;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token<'input> {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("/")]
    Slash,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex(r"\d*d\d+", |lex| parse_roll(lex.slice()))]
    Roll((i64, i64)),
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Digits(i64),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice())]
    Var(&'input str),

    #[token("zero")] Zero,
    #[token("one")] One,
    #[token("two")] Two,
    #[token("three")] Three,
    #[token("four")] Four,
    #[token("five")] Five,
    #[token("six")] Six,
    #[token("seven")] Seven,
    #[token("eight")] Eight,
    #[token("nine")] Nine,
    #[token("ten")] Ten,
    #[token("eleven")] Eleven,
    #[token("twelve")] Twelve,
    #[token("thirteen")] Thirteen,
    #[token("fourteen")] Fourteen,
    #[token("fifteen")] Fifteen,
    #[token("sixteen")] Sixteen,
    #[token("seventeen")] Seventeen,
    #[token("eighteen")] Eighteen,
    #[token("nineteen")] Nineteen,
    #[token("twenty")] Twenty,
    #[token("thirty")] Thirty,
    #[token("forty")] Forty,
    #[token("fifty")] Fifty,
    #[token("sixty")] Sixty,
    #[token("seventy")] Seventy,
    #[token("eighty")] Eighty,
    #[token("ninety")] Ninety,
    #[token("hundred")] Hundred,
    #[token("thousand")] Thousand,
    #[token("million")] Million,
    #[token("billion")] Billion,
    #[token("trillion")] Trillion,

    #[error]
    #[regex(r"\s+", logos::skip)]
    Unknown,
}

fn parse_roll(roll: &str) -> (i64, i64) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\d*)d(\d+)$").unwrap();
    }

    let captures = REGEX.captures(roll).unwrap();

    return (captures[1].parse().unwrap_or(1), captures[2].parse().unwrap());
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
        let tok = self.lexer.next()?;
        let Range { start: s, end: e } = self.lexer.span();
        Some(Ok((s, tok, e)))
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
