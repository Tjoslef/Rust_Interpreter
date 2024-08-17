use std::env;
use std::process::exit;
use log::__private_api::Value;
use crate::evaluator::Evaluator;

mod evaluator;
mod error;
mod token;
mod tokenizer;
mod parse;

use crate::tokenizer::tokenize;
use crate::parse::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => match tokenize(filename, false) {
            Ok(_) => {}, // Tokenization was successful, nothing to do here.
            Err(e) => {
                eprintln!("Error during tokenization: {:?}", e);
                exit(65);
            }
        },
        "parse" => match tokenize(filename, false) {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);
            }
            Err(e) => {
                eprintln!("Error during tokenization: {:?}", e);
                exit(65);
            }
        },
        "evaluate" => match tokenize(filename, true) {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);
                let expressions = parser.parse();
                for expr in expressions {
                    match Evaluator::eval(expr) {
                        Ok(value) => println!("{}", value),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Err(token_err) => {
                eprintln!("Tokenization error: {:?}", token_err);
            }
        }
        _=> {}
    }

}