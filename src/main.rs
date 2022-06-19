#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate simple_error;
#[macro_use] extern crate lazy_static;

lalrpop_mod!(pub grammar);
mod ast;
mod eval;
mod util;
mod parse;
mod query;

use std::io::{self, BufRead, Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::default::Default;
use clap::Parser;
use serde_json::{Value};
use query::{Result, Request, Response};

/// A normal number parser
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Whether to speak JSON. Defaults to true on the network, false on stdin.
    #[clap(long)]
    json: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    respond(io::stdin(), io::stdout(), args.json)?;

    Ok(())
}

fn respond<R, W>(reader: R, writer: W, json: bool) -> Result<()>
    where R: Read,
          W: Write
{
    if json {
        return respond_json(reader, writer);
    } else {
        return respond_lines(reader, writer);
    }
}

fn respond_lines<R, W>(reader: R, mut writer: W) -> Result<()>
    where R: Read,
          W: Write,
{
    let mut breader = io::BufReader::new(reader);
    let mut linebuf = Vec::new();

    println!("Reading lines...");

    while {
        linebuf.clear();
        let n = breader.read_until(b'\n', &mut linebuf)?;
        n > 0
    } {
        let line = str::from_utf8(&linebuf)?;
        let msg = match eval_line(&line) {
            Ok(res) => res,
            Err(e) => format!("{}\n", e),
        };
        println!("Responding with {:?}", msg);
        writer.write_all(msg.as_bytes())?;
        writer.flush()?;
    }

    println!("Session closed");
    Ok(())
}

fn respond_json<R, W>(reader: R, mut writer: W) -> Result<()>
    where R: Read,
          W: Write,
{
    let mut breader = io::BufReader::new(reader);

    println!("Reading JSON...");

    while let Ok(req) = serde_json::from_reader::<_, Request>(&mut breader)
        .map_err(|e| { println!("JSON read failed: {:?}", e); e })
    {
        println!("Got JSON!");
        let res = match eval_line(&req.message) {
            Ok(str) => Response::Good { val: Some(str) },
            Err(e) => Response::Bad { message: format!("{}", e) }
        };
        println!("Responding with {:?}", res);
        serde_json::to_writer(&mut writer, &res)?;
    }

    println!("Session closed");
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
            assert!(parse.is_err(),
                    "Parsing {:?} incorrectly succeeded, with {:?}",
                    string,
                    parse.unwrap());
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
