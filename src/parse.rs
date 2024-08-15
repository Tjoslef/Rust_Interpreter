use std::process::exit;
use log::error;
use crate::evaluator::ast::Stmt::Expr as OtherExpr;
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
        let mut minus_mod = false;
        loop {
            let token = self.advance(); // Get the current token
            if bang_active {
                if token._type == TokenType::Keyword(KeywordTokenType::FALSE) {
                    bang_active = false;
                    return Some(Expr::BoolLite(true));  // Negation of `false` yields `true`
                } else if token._type == TokenType::Keyword(KeywordTokenType::NIL) {
                    bang_active = false;
                    return Some(Expr::BoolLite(true));  // Negation of `nil` yields `true`
                } else if token._type == TokenType::Symbol(SymbolTokenType::BANG) {
                    // Encountering another `!` means double negation, which cancels out
                    bang_active = false;
                    return Some(Expr::BoolLite(true)); // Double negation yields `false`
                } else {
                    // If `bang_active` is true, but none of the specific tokens match
                    bang_active = false;
                    return Some(Expr::BoolLite(false)); // Default to `false`
                }
            }

            if minus_mod {
                minus_mod = false;
                if token._type == TokenType::Literal(LiteralTokenType::NUMBER){
                    if let Ok(int_val) = token._string.parse::<i64>(){
                        return Some(Expr::IntLit(-int_val))
                    } else if let Ok(float_val) = token._string.parse::<f64>() {
                        return Some(Expr::FloatLit(-float_val))
                    } else {
                        eprintln!("Failed to parse number: {}", token._string);
                        return None
                    }
                }else { eprintln!("Unexpected token after '-': {:?}", token._type);
                    return None; }
            }
            match &token._type {


                TokenType::Literal(LiteralTokenType::NUMBER) => {
                        if let Ok(int_val) = token._string.parse::<i64>() {
                            return Some(Expr::IntLit(int_val))
                        } else if let Ok(float_val) = token._string.parse::<f64>() {
                            return Some(Expr::FloatLit(float_val))
                        } else {
                            eprintln!("Failed to parse number: {}", token._string);
                            return None
                        }
                }
                TokenType::Keyword(_) =>{
                    return Some(Expr::Literal(token._string.clone()))
                }
                TokenType::Symbol(SymbolTokenType::MINUS) =>{
                    minus_mod = true;
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