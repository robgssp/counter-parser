#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate simple_error;
#[macro_use] extern crate lazy_static;

lalrpop_mod!(pub grammar);
mod ast;
mod eval;
mod util;
mod parse;

use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::str;
use std::default::Default;
use simple_error::SimpleError;

pub type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:2369").await?;
    println!("listening...");

    loop {
        let (socket, addr) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Accepted connection from {:?}", addr);

            respond(socket).await.expect("respond() succeeds");
        });
    }

    // let lexer = util::TokenLexer::new(teststr);
    // println!("{:?} -> {:?}",
    //          teststr,
    //          grammar::TopLevelParser::new().parse(teststr, lexer).unwrap());
}

async fn respond(sock: TcpStream) -> Result<()> {
    let mut stream = io::BufStream::new(sock);
    let mut linebuf = Vec::new();

    while {
        linebuf.clear();
        let n = stream.read_until(b'\n', &mut linebuf).await?;
        let line = str::from_utf8(&linebuf)?;
        let msg = match eval_line(&line) {
            Ok(res) => res,
            Err(e) => format!("{}\n", e),
        };
        stream.write_all(msg.as_bytes()).await?;
        stream.flush().await?;
        n > 0
    } {}

    println!("Connection closed");
    Ok(())
}

fn eval_line(line: &str) -> Result<String> {
    let expr = parse::best_parse(line).ok_or_else(
        || simple_error!("No good parses in '{}'", line))?;

    eval::eval(&expr, &Default::default()).map(|v| format!("{}\n", v.to_string()))
}

#[cfg(test)]
mod tests {
    use ast::*;
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

    #[test]
    fn test_precedence() {
        let cases: &[(&'static str, Expr)] = &[
            ("1 + 2 * 3",
             Box::new(BinOp(Add,
                            Box::new(Number(1, Digits)),
                            Box::new(BinOp(Mul,
                                           Box::new(Number(2, Digits)),
                                           Box::new(Number(3, Digits))))))),
            ("1 * 2 + 3",
             Box::new(BinOp(Add,
                            Box::new(BinOp(Mul,
                                           Box::new(Number(1, Digits)),
                                           Box::new(Number(2, Digits)))),
                            Box::new(Number(3, Digits)))))
        ];

        let parser = grammar::TopLevelParser::new();

        for (string, res) in cases.iter() {
            let lexer = util::TokenLexer::new(string);
            let parse = parser.parse(string, lexer);
            assert_eq!(parse.unwrap(), *res, "Wrong parse");
        }
    }
}
