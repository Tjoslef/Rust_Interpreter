use std::fmt;
use std::fmt::Display;
use std::iter::Peekable;
use std::process::exit;
use std::slice::Iter;

use crate::{error, parse};
use crate::token::{KeywordTokenType, LiteralTokenType, SymbolTokenType, Token, TokenType};

#[derive(Clone,Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    exprs: Vec<Expr>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            exprs: vec![],
        }
    }
    pub fn parse(&mut self) -> &Vec<Expr> {
        let mut tokens = self.tokens.iter().peekable();
        while let Some(token) = tokens.next() {
            let expr = self.get_expr(token, &mut tokens);
            self.exprs.push(expr);
        }
        &self.exprs
    }

    fn get_expr(&mut self, token: &Token, tokens: &mut Peekable<Iter<Token>>) -> Expr {
        let expr = match token._type {

            TokenType::Keyword(KeywordTokenType::TRUE) => Expr::BoolLite(true),
            TokenType::Keyword(KeywordTokenType::FALSE) => Expr::BoolLite(false),
            TokenType::Literal(LiteralTokenType::STRING) => Expr::Literal(token._string.clone()),
            TokenType::Literal(LiteralTokenType::NUMBER) => {
                let new_num = token._value.clone();
                Expr::FloatLit(new_num.parse().unwrap())},
            TokenType::Symbol(SymbolTokenType::LEFT_PAREN) => {
                while let Some(token) = tokens.next() {
                    if token._type == TokenType::Symbol(SymbolTokenType::RIGHT_PAREN) {
                        break;
                    }
                    if tokens.peek().is_none() {
                        eprintln!("Error: Unmatched parentheses.");
                        exit(65);
                    }
                    let expr = self.get_expr(token, tokens);
                    self.exprs.push(expr);
                }
                // no expression was found after the left parenthesis
                if self.exprs.is_empty() {
                    parse::expr_error(token)
                }
                Expr::Group(Box::new(self.exprs.pop().unwrap()))
            }
            TokenType::Symbol(SymbolTokenType::RIGHT_PAREN) => {
                // right parenthesis was reached before the end of the expression
                expr_error(token)
            }
            TokenType::Symbol(SymbolTokenType::BANG) => Expr::Unary(
                token.clone(),
                Box::new(self.get_expr(tokens.next().unwrap(), tokens)),
            ),
            TokenType::Symbol(SymbolTokenType::STAR)
           | TokenType::Symbol(SymbolTokenType::SLASH)
           | TokenType::Symbol(SymbolTokenType::PLUS)
            | TokenType::Symbol(SymbolTokenType::LESS)
            | TokenType::Symbol(SymbolTokenType::GREATER)
             | TokenType::Symbol(SymbolTokenType::GREATER_EQUAL)
           | TokenType::Symbol(SymbolTokenType::LESS_EQUAL)
           | TokenType::Symbol(SymbolTokenType::EQUAL_EQUAL)
           | TokenType::Symbol(SymbolTokenType::BANG_EQUAL) => {
                if self.exprs.is_empty() {
                    expr_error(token)
                }
                let left = self.exprs.pop().unwrap();
                let next_token = tokens.next();
                if next_token.is_none() {
                    expr_error(token)
                }
                let right = self.get_expr(next_token.unwrap(), tokens);
                Expr::Binary(token.clone(), (Box::new(left)), Box::new(right))
            }
            TokenType::Symbol(SymbolTokenType::MINUS) => {
                let next_token = tokens.next();
                if next_token.is_none() {
                    expr_error(token)
                }
                if self.exprs.is_empty() {
                    Expr::Unary(
                        token.clone(),
                        Box::new(self.get_expr(next_token.unwrap(), tokens)),
                    )
                } else {
                    let left = self.exprs.pop().unwrap();
                    let right = self.get_expr(next_token.unwrap(), tokens);
                    Expr::Binary(token.clone(), Box::new(left), Box::new(right))
                }
            }
            TokenType::Keyword(KeywordTokenType::NIL) => Expr::Nil,
            _ => todo!(),
        };
        expr
    }
}
#[derive(Clone,Debug)]

pub enum Expr{
    FloatLit(f64),
    Literal(String),
    BoolLite(bool),
    Binary(Token, Box<Expr>, Box<Expr>),
    Group(Box<Expr>),
    Unary(Token, Box<Expr>),
    Nil,
}
impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::BoolLite(b) => write!(f, "{b}"),
            Expr::Literal(n) => write!(f, "{n}"),
            Expr::Literal(s) => write!(f, "{s}"),
            Expr::Group(g) => {
                write!(f, "(group {g})")
            }
            Expr::Unary(op, expr) => {
                write!(f, "({} {})", op._value, expr)
            }
            Expr::Binary(op, left, right) => {
                write!(f, "({} {} {})", op._value, left, right)
            }
            Expr::Nil => write!(f, "nil"),
            _=> Ok(())
        }
    }
}
pub fn expr_error(token: &Token) -> ! {
    eprintln!(
        "Error at line'{}': Expect expression.{}",
        token._num_line,token._string
    );
    exit(65);
}

