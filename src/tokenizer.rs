use anyhow::{bail,Result};
use std::fs;
use crate::token::{KeywordTokenType, LiteralTokenType, SymbolTokenType, Token, TokenType};
use crate::error::{Error};
use std::collections::{HashMap};
use crate::token::KeywordTokenType::AND;

pub fn tokenize(filename: &String, parsingKey:bool) -> Result<Vec<Token>,Error> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(e) => { eprintln!("Error reading file {}: {}", filename, e);
        return Err(Error::new(65));

        }
    };


    let mut char_cont = file_contents.chars().peekable();
    let mut line = 1;
    let mut literalStr = String::new();

    let mut has_error = false;
    let mut token = vec![];
    while let Some(c) = char_cont.next() {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::Keyword(KeywordTokenType::AND));
        keywords.insert("class", TokenType::Keyword(KeywordTokenType::CLASS));
        keywords.insert("else", TokenType::Keyword(KeywordTokenType::ELSE));
        keywords.insert("false", TokenType::Keyword(KeywordTokenType::FALSE));
        keywords.insert("for", TokenType::Keyword(KeywordTokenType::FOR));
        keywords.insert("fun", TokenType::Keyword(KeywordTokenType::FUN));
        keywords.insert("if", TokenType::Keyword(KeywordTokenType::IF));
        keywords.insert("nil", TokenType::Keyword(KeywordTokenType::NIL));
        keywords.insert("or", TokenType::Keyword(KeywordTokenType::OR));
        keywords.insert("print", TokenType::Keyword(KeywordTokenType::PRINT));
        keywords.insert("return", TokenType::Keyword(KeywordTokenType::RETURN));
        keywords.insert("super", TokenType::Keyword(KeywordTokenType::SUPER));
        keywords.insert("this", TokenType::Keyword(KeywordTokenType::THIS));
        keywords.insert("true", TokenType::Keyword(KeywordTokenType::TRUE));
        keywords.insert("var", TokenType::Keyword(KeywordTokenType::VAR));
        keywords.insert("while", TokenType::Keyword(KeywordTokenType::WHILE));

        fn push_token(token_vec: &mut Vec<Token>, token_type: TokenType, c: char) {
            token_vec.push(Token::new(token_type, c.to_string(), "".to_string()));
        }
        match c {

            '(' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::LEFT_PAREN), c),
            ')' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::RIGHT_PAREN), c),
            '{' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::LEFT_BRACE), c),
            '}' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::RIGHT_BRACE), c),
            ',' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::COMMA), c),
            '.' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::DOT), c),
            '-' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::MINUS), c),
            '+' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::PLUS), c),
            ';' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::SEMICOLON), c),
            '*' => push_token(&mut token, TokenType::Symbol(SymbolTokenType::STAR), c),
            '/' => {
                if let Some(&next_char) = char_cont.peek() {
                    if next_char == '/' {
                        while let Some(comment_char) = char_cont.next() {
                            if comment_char == '\n' {
                                line += 1;
                                break;
                            }
                        } // Continue to the next character
                    } else {
                        push_token(&mut token,TokenType::Symbol(SymbolTokenType::SLASH),c);
                    }
                } else {

                    push_token(&mut token,TokenType::Symbol(SymbolTokenType::SLASH),c);
;
                }
            },
            '=' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::EQUAL_EQUAL), "==".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::EQUAL), c.to_string(), "".to_string()));
                }
            },
            '!' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::BANG_EQUAL), "!=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::EQUAL), c.to_string(), "".to_string()));
                }
            },
            '>' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::GREATER_EQUAL), ">=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::GREATER), c.to_string(), "".to_string()));
                }
            },
            '<' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::LESS_EQUAL), "<=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::LESS), c.to_string(), "".to_string()));
                }
            }
            '\n' => {
                line += 1;
            },
            '"' => {
                while let Some(stringL) = char_cont.next() {
                    if stringL == '"' {
                        token.push(Token::new(TokenType::Literal(LiteralTokenType::STRING), literalStr.to_string(), "".to_string()));
                        literalStr.clear();
                        break;
                    } else {
                        literalStr.push(stringL);
                    }
                }
                if !literalStr.is_empty() {
                    eprintln!("[line {}] Error: Unterminated string.", line);
                    has_error = true;
                    literalStr.clear();
                    break;
                }
            },
            c if c.is_digit(10) => {
                let mut cont = String::from(c);
                let mut comma_detected = false;
                while let Some(t) = char_cont.peek() {
                    if t.is_digit(10) {
                        cont.push(*t);
                        char_cont.next();
                    } else if *t == '.' && !comma_detected {
                        comma_detected = true;
                        cont.push(*t);
                        char_cont.next();
                        }
                    else {
                        break;
                    }

                }
                if cont.ends_with('.') {
                    cont.push('0');
                    token.push(Token::new(TokenType::Literal(LiteralTokenType::NUMBER), cont.to_string(), cont.to_string()));
                    token.push(Token::new(TokenType::Symbol(SymbolTokenType::DOT),".".to_string(),"".to_string()))
                }
                else {
                    token.push(Token::new(TokenType::Literal(LiteralTokenType::NUMBER), cont.to_string(), cont.to_string()));
                }
            },
            c if  is_alpha(c) => {
                let mut cont = String::from(c);
                while let Some(&L) = char_cont.peek(){
                    if !is_alpha(L) && !is_number(L){
                        break;
                    }else {
                        cont.push(L);
                        //println!("{}",cont);
                        char_cont.next();
                    }
                  // token.push(Token::new(TokenType::IDENTIFIER,cont.to_string(),"".to_string()));
                }
                if let Some(keyword_type) = keywords.get(cont.as_str()){
                    let token_type = keyword_type.clone();
                    if parsingKey == false {
                        token.push(Token::new(token_type, cont.to_string(), "".to_string()));
                    }else {
                        token.push(Token::new(token_type, cont.to_string(), "".to_string()));
                    }

                }else {
                    token.push(Token::new(TokenType::Literal(LiteralTokenType::IDENTIFIER), cont.to_string(), "".to_string()));
                }},
            ' '|'\t'| '\r' =>{continue;},
            


            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                has_error = true;

            }


        }
    }


    if parsingKey == false {
        token.push(Token::new(TokenType::EOF, "".to_string(), "".to_string()));

        for token in &token {
            println!("{}", token);
        }
    }
    return if has_error {
        Err(Error::new(65))
    }else {
        let tokens: Vec<Token> = token; // Replace with actual tokenization result
        Ok(tokens)
    }
}
pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
pub fn is_number(c:char) -> bool{
    c.is_digit(10)
}
fn string_to_bool(s: &String) -> Option<bool> {
    match s.to_lowercase().as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None, // Return None if the string doesn't match "true" or "false"
    }
}
