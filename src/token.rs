use std::fmt;
use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
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


#[derive(Debug, PartialEq, Eq, Copy,Clone)]
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


#[derive(Debug, PartialEq, Eq, Copy,Clone)]
pub enum LiteralTokenType {
    STRING,
    NUMBER,
    ROUNDED_NUMBER,
    IDENTIFIER,
}


#[derive(Debug, PartialEq, Eq, Copy,Clone)]
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
  fn clone(&self) -> TokenType { *self }
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



impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self._type {
            TokenType::Symbol(symbol) => {
                let symbol_str = match symbol {
                    SymbolTokenType::LEFT_PAREN => "LEFT_PAREN",
                    SymbolTokenType::RIGHT_PAREN => "RIGHT_PAREN",
                    SymbolTokenType::LEFT_BRACE => "LEFT_BRACE",
                    SymbolTokenType::RIGHT_BRACE => "RIGHT_BRACE",
                    SymbolTokenType::COMMA => "COMMA",
                    SymbolTokenType::DOT => "DOT",
                    SymbolTokenType::MINUS => "MINUS",
                    SymbolTokenType::PLUS => "PLUS",
                    SymbolTokenType::SEMICOLON => "SEMICOLON",
                    SymbolTokenType::STAR => "STAR",
                    SymbolTokenType::SLASH => "SLASH",
                    SymbolTokenType::EQUAL => "EQUAL",
                    SymbolTokenType::EQUAL_EQUAL => "EQUAL_EQUAL",
                    SymbolTokenType::BANG_EQUAL => "BANG_EQUAL",
                    SymbolTokenType::BANG => "BANG",
                    SymbolTokenType::GREATER_EQUAL => "GREATER_EQUAL",
                    SymbolTokenType::GREATER => "GREATER",
                    SymbolTokenType::LESS_EQUAL => "LESS_EQUAL",
                    SymbolTokenType::LESS => "LESS",
                };
                write!(f, "{} {} {}null", symbol_str, self._string, self._value)
            }
            TokenType::Literal(literal) => {
                match literal {
                    LiteralTokenType::STRING => write!(f, "STRING \"{}\" {}",self._string, self._string),
                    LiteralTokenType::NUMBER => {
                        if self._string.ends_with(".0") {
                            write!(f, "NUMBER {} {}", self._string.replace(".0", ""), self._string)
                        } else if !self._string.contains(".") {
                            write!(f, "NUMBER {} {}.0", self._string, self._string)}
                        else if self._string.ends_with(".00") {
                            write!(f, "NUMBER {} {}.0", self._string, self._string.replace(".00",""))
                        }
                        else {
                            write!(f, "NUMBER {} {}", self._string, self._string)
                        } }
                    LiteralTokenType::ROUNDED_NUMBER => write!(f, "ROUNDED_NUMBER {}", self._string),
                    LiteralTokenType::IDENTIFIER => write!(f, "IDENTIFIER {} null", self._string),
                }
            }
            TokenType::Keyword(keyword) => {
                let keyword_str = match keyword {
                    KeywordTokenType::AND => "AND",
                    KeywordTokenType::CLASS => "CLASS",
                    KeywordTokenType::ELSE => "ELSE",
                    KeywordTokenType::FALSE => "FALSE",
                    KeywordTokenType::FOR => "FOR",
                    KeywordTokenType::FUN => "FUN",
                    KeywordTokenType::IF => "IF",
                    KeywordTokenType::NIL => "NIL",
                    KeywordTokenType::OR => "OR",
                    KeywordTokenType::PRINT => "PRINT",
                    KeywordTokenType::RETURN => "RETURN",
                    KeywordTokenType::SUPER => "SUPER",
                    KeywordTokenType::THIS => "THIS",
                    KeywordTokenType::TRUE => "TRUE",
                    KeywordTokenType::VAR => "VAR",
                    KeywordTokenType::WHILE => "WHILE",
                };
                write!(f, "{} {} null", keyword_str, self._string)
            }
            TokenType::EOF => write!(f, "EOF  null"),
        }
    }
}
