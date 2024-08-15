use std::process::exit;
use log::error;
use crate::parse::Expr::FloatLit;
use crate::token;
use crate::token::{KeywordTokenType, LiteralTokenType, SymbolTokenType, Token, TokenType};
use crate::token::KeywordTokenType::FALSE;
// Ensure this path is correct

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Option<Expr> {
        self.process_token()
    }

    pub fn process_token(&mut self) -> Option<Expr> {
        let mut bang_active = false;
        loop {
            let token = self.advance(); // Get the current token
            if bang_active && token._type != TokenType::Keyword(KeywordTokenType::FALSE){
                bang_active = false;
                return Some(Expr::BoolLite(false));
            }
            else if token._type == TokenType::Keyword(KeywordTokenType::FALSE)  {
                return Some(Expr::BoolLite(true));
            }
            else {

            match &token._type {


                TokenType::Literal(LiteralTokenType::NUMBER) => {
                    if bang_active == false {
                        if let Ok(int_val) = token._string.parse::<i64>() {
                            return Some(Expr::IntLit(int_val))
                        } else if let Ok(float_val) = token._string.parse::<f64>() {
                            return Some(Expr::FloatLit(float_val))
                        } else {
                            eprintln!("Failed to parse number: {}", token._string);
                            return None
                        }
                    }
                }
                TokenType::Keyword(_) =>{
                    return Some(Expr::Literal(token._string.clone()))
                }
                TokenType::Symbol(SymbolTokenType::BANG) => {
                   bang_active = true;
                    continue;
                }
                TokenType::Literal(LiteralTokenType::STRING) => {
                   return Some(Expr::Literal(token._string.clone()))
                }
                TokenType::Symbol(SymbolTokenType::LEFT_PAREN)

                 => {
                    continue // Token was ignored
                }

                _ => {
                    eprintln!("Unexpected token {:?}", token._type);
                    return None
                }
            }
        }
    }}
    pub fn advance(&mut self) -> &Token
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
    BoolLite(bool),
}