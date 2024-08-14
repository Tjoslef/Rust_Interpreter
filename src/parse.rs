use std::process::exit;
use log::error;
use crate::token::{LiteralTokenType, Token, TokenType}; // Ensure this path is correct

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.literal()
    }

    fn literal(&mut self) -> Result<Expr, String> {
        let token = self.advance();
        match &token._type {
            TokenType::Literal(LiteralTokenType::NUMBER) => {
                if token._string.contains('.') {
                    // If the string contains a dot, parse it as a floating-point number
                    match token._string.parse::<f64>() {
                        Ok(value) => Ok(Expr::FloatLit(value)) ,
                        Err(e) => {
                            eprintln!("Error during parsing a float number: {:?}", e);
                            exit(65); // Exit with status code 65
                        }
                    }

                } else {
                    match token._string.parse::<i64>() {
                        Ok(value) => Ok(Expr::IntLit(value)),
                        Err(e) => {
                            eprintln!("Error during parsing an integer number: {:?}", e);
                            exit(65); // Exit with status code 65
                        }
                    }

                }
            }
            TokenType::Keyword(_) => Ok(Expr::Literal(token._string.clone())),
            TokenType::Literal(LiteralTokenType::STRING) => Ok(Expr::Literal(token._string.clone())),
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn advance(&mut self) -> &Token
    {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}

// Define Expr as required
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    Literal(String),
}