
use std::fmt::Display;
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,
    SLASH,
    EQUAL,
    EQUAL_EQUAL,
    BANG_EQUAL,
    BANG,
    GREATER_EQUAL,
    GREATER,
    LESS_EQUAL,
    LESS,
    STRING,
    EOF,
    NUMBER,
    ROUNDED_NUMBER,
}
pub struct Token {
    _type: TokenType,
    _string: String,
    _value: String,

}
impl Token {
    pub fn new(_type: TokenType, _string: String, _value:String ) -> Self {
        Token {
            _type,
            _string,
            _value,
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self._type {
            TokenType::STRING => write!(f,"{:?} \"{}\" {}",self._type,self._string,self._string),
            TokenType::NUMBER =>{ if self._string.ends_with(".0") {
                write!(f, "NUMBER {} {}", self._string.replace(".0", ""), self._string)
            } else if !self._string.contains(".") {
                write!(f, "NUMBER {} {}.0", self._string, self._string)
            } else {
                write!(f, "NUMBER {} {}", self._string, self._string)
            } }
           // TokenType::ROUNDED_NUMBER =>  write!(f,"{:?} {} {}.0","NUMBER",self._value.clone().unwrap_or(self._string.to_string()),self._string),
            _ => {
                write!(
                    f,
                    "{:?} {} {}null",
                    self._type,
                    self._string,
                    self._value,
                )
            }
        }
    }
}
