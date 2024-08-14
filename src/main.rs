use std::env;
use std::process::exit;

mod evaluator;
mod error;
mod token;
mod tokenizer;
mod parse;

use crate::evaluator::visit;
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
        "tokenize" => match tokenize(filename,false) {
            Ok(_) => {}, // Tokenization was successful, nothing to do here.
            Err(e) => {
                eprintln!("Error during tokenization: {:?}", e);
                exit(65);
            }
        },
        "parse" => match tokenize(filename,false) {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);

                match parser.parse() {
                    Some(_) => {}, // Successfully parsed, the result is printed if `print_expr` is true.
                    None => {
                        eprintln!("Error during parsing");
                        exit(65);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error during tokenization: {:?}", e);
                exit(65);
            }
        },
        "evaluate" => match tokenize(filename,true) {
            Ok(tokens) => {
                let mut parser = Parser::new(&tokens);
                match parser.parse() {
                    Some(expression) => {
                        let result = visit::Evaluator::evaluate(Some(expression));
                        println!
                        ("{}",result);
                    }
                    None => {
                        eprintln!("Error during parsing");
                        exit(0);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error during tokenization: {:?}", e);
                exit(65);
            }
        },
        _ => eprintln!("Unknown command: {}", command),
    }
}
