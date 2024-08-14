
use std::fmt::Display;
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    // Grouped as symbols
    Symbol(SymbolTokenType),

    // Grouped as literals
    Literal(LiteralTokenType),

    // Grouped as keywords
    Keyword(KeywordTokenType),

    // Miscellaneous
    EOF,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum SymbolTokenType {
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
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum LiteralTokenType {
    STRING,
    NUMBER,
    ROUNDED_NUMBER,
    IDENTIFIER,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum KeywordTokenType {

    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}
pub struct Token {
   pub _type: TokenType,
   pub _string: String,
   pub _value: String,

}
impl Copy for TokenType{}

impl Clone for TokenType {
    fn clone(&self) -> TokenType {
        *self
    }
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
            TokenType::Literal(LiteralTokenType::STRING) => write!(f,"{:?} \"{}\" {}",self._type,self._string,self._string),
            TokenType::Literal(LiteralTokenType::NUMBER) =>{ if self._string.ends_with(".0") {
                write!(f, "NUMBER {} {}", self._string.replace(".0", ""), self._string)
            } else if !self._string.contains(".") {
                write!(f, "NUMBER {} {}.0", self._string, self._string)}
                else if self._string.ends_with(".00") {
                    write!(f, "NUMBER {} {}.0", self._string, self._string.replace(".00",""))
                }
            else {
                write!(f, "NUMBER {} {}", self._string, self._string)
            } }
            TokenType::Literal(LiteralTokenType::IDENTIFIER) => write!(f, "{:?} {} null", self._type, self._string),
            TokenType::Keyword(KeywordTokenType::TRUE) => write!(f,"{}",self._string),
            TokenType::Keyword(KeywordTokenType::FALSE) => write!(f,"{}",self._string),
            TokenType::Keyword(KeywordTokenType::NIL) => write!(f,"{}",self._string),
            _=> {
                write!(
                    f,
                    "{} {} {}null",
                    self._type.to_string(),
                    self._string,
                    self._value,
                )
            }
        }
    }
}
