#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate simple_error;

lalrpop_mod!(pub grammar);
mod ast;
mod eval;
mod util;
use ast::*;


fn main() {
    // let teststr = "(42)+5";
    let teststr = "1 + 2 3 + 4";

    let lexer = util::TokenLexer::new(teststr);
    println!("{:?} -> {:?}",
             teststr,
             grammar::TopLevelParser::new().parse(teststr, lexer).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_terms() {
        let cases = &[("(42)", Box::new(Number(42, Digits))),
                      ("one hundred fifty + 3",
                       Box::new(BinOp(Add,
                                      Box::new(Number(150, Words)),
                                      Box::new(Number(3, Digits)))))
        ];

        let parser = grammar::TermParser::new();

        for (string, num) in cases.iter() {
            let lexer = util::TokenLexer::new(string);
            let parse = parser.parse(string, lexer);
            assert!(parse.is_ok(), "Failed to parse {:?}: {:?}", string, parse.err());
            assert_eq!(parse.unwrap(), *num, "String parsed to the wrong expr: {:?}", string);
        }
    }

    #[test]
    fn test_numword_parser() {
        let cases =
            &[("one hundred five", 105),
              ("one hundred twenty two", 122),
              ("one hundred fifteen", 115),
              ("fifteen", 15),
              ("zero hundred fifteen", 15),
              ("two", 2),
              ("one thousand", 1000),
              ("twenty", 20),
              ("twenty thousand", 20000),
              ("twenty thousand five hundred fifteen", 20515),
              ("one hundred twenty three thousand five hundred fifteen", 123515),
              ("one million two thousand three", 1_002_003)
            ];

        let parser = grammar::NumWordsParser::new();

        for (string, num) in cases.iter() {
            let lexer = util::TokenLexer::new(string);
            let parse = parser.parse(string, lexer);
            assert!(parse.is_ok(), "Failed to parse {:?}: {:?}", string, parse.err());
            assert_eq!(parse.unwrap(), *num, "String parsed to the wrong number: {:?}", string);
        }
    }

    #[test]
    fn test_numword_parser_fails() {
        let cases = &["ten one",
                      "eleven hundred",
                      "one thousand million",
                      "one thousand two million",
                      "one thousand two thousand",
        ];

        let parser = grammar::NumWordsParser::new();

        for string in cases.iter() {
            let lexer = util::TokenLexer::new(string);
            let parse = parser.parse(string, lexer);
            assert!(parse.is_err(), "Parsing {:?} incorrectly succeeded, with {:?}", string, parse.unwrap());
        }
    }
}
