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
    pub fn parse_multiplication_or_division(&mut self, left_expr: Option<Expr>) -> Option<Expr> {
        if let Some(ref expr) = left_expr {
            let next_token = self.peek()._type.clone(); // Look ahead without consuming
            if next_token == TokenType::Symbol(SymbolTokenType::STAR)
                || next_token == TokenType::Symbol(SymbolTokenType::SLASH) {
                self.advance();
                let right_expr = self.process_token();
                match (expr, right_expr) {
                    (Expr::IntLit(left_val), Some(Expr::IntLit(right_val))) => {
                        if next_token == TokenType::Symbol(SymbolTokenType::STAR) {
                            println!("fgfdgfg");
                            Some(Expr::IntLit(left_val * right_val));
                        } else {
                            println!("fgfdgfg");
                            Some(Expr::IntLit(left_val / right_val));
                        }
                    }
                    (Expr::FloatLit(left_val), Some(Expr::FloatLit(right_val))) => {
                        if next_token == TokenType::Symbol(SymbolTokenType::STAR) {
                            return Some(Expr::FloatLit(left_val * right_val));
                        } else {
                            return Some(Expr::FloatLit(left_val / right_val));
                        }
                    }
                    _ => {
                        Some(expr);
                    }
                }
            } else {
                return Some(expr.clone());
            }
        }

        return None;
    }
    // Return the original expression if no '*' or '/' was found

    pub fn process_token(&mut self) -> Option<Expr> {
        let mut result: Option<Expr> = None;
        let mut bang_active = false;
        let mut minus_mod = false;
        while let Some(token) = self.advance() {
            self.current += 1;
            let token = self.advance(); // Get the current token
            if bang_active {
                if token._type == TokenType::Keyword(KeywordTokenType::FALSE) {
                    bang_active = false;
                    Some(Expr::BoolLite(true));  // Negation of `false` yields `true`
                } else if token._type == TokenType::Keyword(KeywordTokenType::NIL) {
                    bang_active = false;
                    Some(Expr::BoolLite(true));  // Negation of `nil` yields `true`
                } else if token._type == TokenType::Symbol(SymbolTokenType::BANG) {
                    // Encountering another `!` means double negation, which cancels out
                    bang_active = false;
                    Some(Expr::BoolLite(true)); // Double negation yields `false`
                } else {
                    // If `bang_active` is true, but none of the specific tokens match
                    bang_active = false;
                    Some(Expr::BoolLite(false)); // Default to `false`
                }
            }

            if minus_mod {
                minus_mod = false;
                if token._type == TokenType::Literal(LiteralTokenType::NUMBER) {
                    if let Ok(int_val) = token._string.parse::<i64>() {
                        Some(Expr::IntLit(-int_val));
                    } else if let Ok(float_val) = token._string.parse::<f64>() {
                        Some(Expr::FloatLit(-float_val));
                    } else {
                        eprintln!("Failed to parse number: {}", token._string);
                        return None
                    }
                } else {
                    eprintln!("Unexpected token after '-': {:?}", token._type);
                    return None;
                }
                continue;
            }
            let mut result = match &token._type {
                TokenType::Literal(LiteralTokenType::NUMBER) => {
                    if let Ok(int_val) = token._string.parse::<i64>() {
                        Some(Expr::IntLit(int_val))
                    } else if let Ok(float_val) = token._string.parse::<f64>() {
                        Some(Expr::FloatLit(float_val))
                    } else {
                        eprintln!("Failed to parse number: {}", token._string);
                        return None;
                    }
                }
                TokenType::Keyword(_) => {
                    Some(Expr::Literal(token._string.clone()))
                }
                TokenType::Symbol(SymbolTokenType::MINUS) => {
                    minus_mod = true;
                    continue;
                }
                TokenType::Symbol(SymbolTokenType::BANG) => {
                    bang_active = true;
                    continue;
                }
                TokenType::Literal(LiteralTokenType::STRING) => {
                    Some(Expr::Literal(token._string.clone()))
                }
                TokenType::Symbol(SymbolTokenType::STAR) =>
                    {
                        Some(Expr::Literal(token._string.clone()))
                    }
                TokenType::Symbol(SymbolTokenType::SLASH) =>
                    {
                        Some(Expr::Literal(token._string.clone()))
                    }
                TokenType::Symbol(SymbolTokenType::LEFT_PAREN)

                => {
                    continue // Token was ignored
                }

                _ => {
                    eprintln!("Unexpected token {:?}", token._type);
                    return None
                }
            };
            if let Some(res) = result {
                result = self.parse_multiplication_or_division(Some(res));
                if result.is_none() {
                    return None
                }
            }
        }
        return result;
    }


    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn advance(&mut self) -> Option<&Token> {
        if self.current < self.tokens.len() {
            let token = &self.tokens[self.current];
            self.current += 1;
            Some(token)
        } else {
            None
        }
    }
}
// Define Expr as required
#[derive(Clone)]
pub enum Expr {
    IntLit(i64),
    FloatLit(f64),
    Literal(String),
    BoolLite(bool),
    Mult(f64),

}