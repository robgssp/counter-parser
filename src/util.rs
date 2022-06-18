extern crate logos;
use logos::Logos;
use core::ops::Range;
// use std::str::FromStr;

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum Token<'input> {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Times,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Digits(i64),
    #[regex(r"[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice())]
    Var(&'input str),

    #[error]
    #[regex(r"\s+", logos::skip)]
    Unknown,

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
}
