// Scanning parse

// pub struct ScanningParse<'input> {
//     tokens: Vec<TokenLexer::Item>;
//     parser: grammar::TopLevelParser;
// }

// impl ScanningParse {
//     pub fn new(input: &'input str) {
//         tokens = parer
//     }
// }

use crate::grammar;
use crate::util;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_once() {
        let string = "one + 2 and some junk";

        //let parser = grammar::TopLevelParser::new();
        let parser = grammar::TopLevelParser::new();
        let mut lexer = util::TokenLexer::new(string);

        let parse = parser.parse(string, &mut lexer);

        println!("{:?} -> {:?} (current token: {:?}", string, parse, lexer.lexer.slice());
    }
}
