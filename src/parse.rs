
use crate::token::{KeywordTokenType, LiteralTokenType, SymbolTokenType, Token, TokenType};


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
        // Start by parsing a primary expression
        let mut left_expr = self.process_token()?;

        // While the current token is a symbol (operator), parse the right-hand side
        while let TokenType::Symbol(symbol) = &self.tokens.get(self.current)?._type {
            match symbol {
                SymbolTokenType::STAR | SymbolTokenType::SLASH => {

                    let op = symbol.clone();
                    self.current += 1; // Move to the next token

                    let right_expr = self.process_token()?;
                    println!("{:?}",right_expr);
                    left_expr = Expr::Binary(Box::new(BinaryExpr {
                        left: left_expr,
                        op,
                        right: right_expr,
                    }));
                    println!("{:?}",left_expr);
                }
                _ => break, // Break the loop if it's not an operator we are handling
            }
        }

        Some(left_expr);
        self.process_token()
    }
    pub fn process_token(&mut self) -> Option<Expr> {
        let mut bang_active = false;
        let mut minus_mod = false;
        let mut result: Option<Expr> = None;

        loop {
            let token = self.advance();
            // Handle bang operations
            if bang_active {
                result = match token._type {
                    TokenType::Keyword(KeywordTokenType::FALSE) | TokenType::Keyword(KeywordTokenType::NIL) => {
                        bang_active = false;
                        Some(Expr::BoolLite(true))
                    },
                    TokenType::Symbol(SymbolTokenType::BANG) => {
                        bang_active = false;
                        Some(Expr::BoolLite(false)) // Double negation
                    },
                    _ => {
                        bang_active = false;
                        Some(Expr::BoolLite(false)) // Default to `false`
                    }
                };
                continue;
            }

            // Handle minus modifier
            if minus_mod {
                minus_mod = false;
                if token._type == TokenType::Literal(LiteralTokenType::NUMBER) {
                    result = if let Ok(int_val) = token._value.parse::<i64>() {
                        Some(Expr::IntLit(Box::new(-int_val)))
                    } else if let Ok(float_val) = token._value.parse::<f64>() {
                        Some(Expr::FloatLit(Box::new(-float_val)))
                    } else {
                        eprintln!("Failed to parse number: {}", token._string);
                        return None;
                    };
                } else {
                    eprintln!("Unexpected token after '-': {:?}", token._type);
                    return None;
                }
                continue;
            }

            // Parse the current token
            result = match &token._type {

                TokenType::Keyword(_) => Some(Expr::Literal(token._string.clone())),
                TokenType::Symbol(SymbolTokenType::MINUS) => {
                    minus_mod = true;
                    continue;
                },
                TokenType::Symbol(SymbolTokenType::BANG) => {
                    bang_active = true;
                    continue;
                },
                TokenType::Literal(LiteralTokenType::STRING) => Some(Expr::Literal(token._string.clone())),
                /*TokenType::Symbol(SymbolTokenType::STAR) | TokenType::Symbol(SymbolTokenType::SLASH) => {

                    let op = token._type.clone();
                    let left_expr = result.take();
                    self.current += 1;
                    println!("{:?} herre",left_expr);
                    let right_expr = self.expression()?;
                   println!("{:?}",right_expr) ;
                    Some(Expr::Binary(Box::new(BinaryExpr {
                        left: left_expr?,
                        op,
                        right: right_expr
                    })))
                },*/
                TokenType::Symbol(SymbolTokenType::LEFT_PAREN) => {
                    continue;
                },
                TokenType::Literal(LiteralTokenType::NUMBER) => {
                    if let Ok(int_val) = token._value.parse::<i64>() {
                       return  Some(Expr::IntLit(Box::new(int_val)))
                    } else if let Ok(float_val) = token._value.parse::<f64>() {
                        return Some(Expr::FloatLit(Box::new(float_val)))
                    } else {
                        eprintln!("Failed to parse number: {}", token._string);
                        return None;
                    }
                },
                _ => {
                    eprintln!("Unexpected token {:?}", token._type);
                    return None;
                }
            };
        return result;
        }

    }



    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn advance(&mut self) -> &Token {

        let token = &self.tokens[self.current];

        self.current += 1;

        token

    }
}
#[derive(Clone,Debug)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: SymbolTokenType,
    pub right: Expr,
}
// Define Expr as required
#[derive(Clone, Debug)]
pub enum Expr {
    IntLit(Box<i64>),
    FloatLit(Box<f64>),
    Literal(String),
    BoolLite(bool),
    Mult(f64),
    Binary(Box<BinaryExpr>),
}