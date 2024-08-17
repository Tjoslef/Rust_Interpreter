use std::fmt::Display;
use crate::parse::Expr;
use crate::token::{SymbolTokenType, TokenType};

pub mod ast {
    use crate::Parser;

    pub enum Stmt{
        Expr(Expr),
        Let(Name,Expr),
    }
    pub struct Name{
       pub value:String
    }
    pub enum Expr {
        Literal(String),
        IntLit(i64),
        FloatLit(f64),

    }

}
#[derive(Debug)]
pub enum Value {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", remove_trailing_zeros(n)),
            Value::String(s) => write!(f, "{}", s),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn remove_trailing_zeros(n: &f64) -> String {
    let y = (n * 100_000_000.0).round() / 100_000_000.0;
    format!("{}", y)
}


    pub trait Visitor<T> {
        fn visit_literal_expr(&mut self, e: Option<(Expr)>) -> T;
    }

    pub struct Evaluator;
    impl Evaluator {
        pub fn eval(expr: &Expr) -> Result<Value, &'static str> {
            let value = match expr {
               Expr::Nil => Value::Nil,
                Expr::Literal(literal) => Value::String(literal.clone()),
                Expr::FloatLit(int_val) => Value::Number(int_val.clone()),
                Expr::BoolLite(bool_val) => Value::Bool(*bool_val),
                Expr::Group(expr) => Evaluator::eval(expr)?,
                Expr::Unary(op, expr) => {
                    let value = Evaluator::eval(expr)?;
                    match op._type {
                        TokenType::Symbol(SymbolTokenType::BANG) => match value {
                            Value::Bool(b) => Value::Bool(!b),
                            Value::Number(n) => Value::Bool(n == 0.0),
                            Value::String(s) => Value::Bool(s.is_empty()),
                            Value::Nil => Value::Bool(true),
                        },
                        TokenType::Symbol(SymbolTokenType::MINUS) => match value {
                            Value::Number(n) => Value::Number(-n),
                            _ => return Err("Operand must be a number."),
                        },
                        _ => unreachable!(),
                    }
                }
                Expr::Binary(op, left, right) => {
                    let left = Evaluator::eval(left)?;
                    let right = Evaluator::eval(right)?;
                    match op._type {
                        TokenType::Symbol(SymbolTokenType::STAR) => {
                            match (left, right) {
                                (Value::Number(l), Value::Number(r)) => Value::Number(l * r),
                                _ =>  return Err("Operands must be numbers."),
                            }
                        },
                        TokenType::Symbol(SymbolTokenType::SLASH) => match (left, right) {
                            (Value::Number(l), Value::Number(r)) => Value::Number(l / r),
                            _ => return Err("Operands must be numbers."),
                        },
                        // Handle other operators as needed
                        _ => todo!(),  // Placeholder for unhandled operations
                    }
                }
                _=>{return Err("mismatch")}
            };
            Ok(value)
        }
    }