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
                let value = token._string.parse::<i64>().unwrap();
                Ok(Expr::IntLit(value))
            }
            TokenType::Keyword(_) => Ok(Expr::Literal(token._string.clone())),
            _ => Err("Unexpected token".to_string()),
        }
    }

    fn advance(&mut self) -> &Token {
        let token = &self.tokens[self.current];
        self.current += 1;
        token
    }
}

// Define Expr as required
pub enum Expr {
    IntLit(i64),
    Literal(String),
}