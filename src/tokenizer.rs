use anyhow::{bail,Result};
use std::fs;
use crate::token::{Token, TokenType};
use crate::error::{Error};

pub fn tokenize(filename: &String) -> Result<()> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => bail!("Failed to read the file."),
    };
    let mut char_cont = file_contents.chars().peekable();
    let mut line = 1;
    let mut done = false;
    let mut literalStr = String::new();
    let mut literalNum = String::new();
    let mut has_error = false;
    let mut token = vec![];
    while let Some(c) = char_cont.next() {

        match c {
            '(' => token.push(Token::new(TokenType::LEFT_PAREN, c.to_string(), "".to_string())),

            ')' =>
                token.push(Token::new(TokenType::RIGHT_PAREN, c.to_string(), "".to_string())),

            '{' => token.push(Token::new(TokenType::LEFT_BRACE, c.to_string(), "".to_string())),

            '}' => token.push(Token::new(TokenType::RIGHT_BRACE, c.to_string(), "".to_string())),
            ',' => token.push(Token::new(TokenType::COMMA, c.to_string(), "".to_string())),
            '.' => token.push(Token::new(TokenType::DOT, c.to_string(), "".to_string())),
            '-' => token.push(Token::new(TokenType::MINUS, c.to_string(), "".to_string())),
            '+' => {
                token.push(Token::new(TokenType::PLUS, c.to_string(), "".to_string()));
            }
            ';' => token.push(Token::new(TokenType::SEMICOLON, c.to_string(), "".to_string())),
            '*' => token.push(Token::new(TokenType::STAR, c.to_string(), "".to_string())),
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
                        token.push(Token::new(TokenType::SLASH, "/".to_string(), "".to_string()));
                    }
                } else {
                    token.push(Token::new(TokenType::SLASH, "/".to_string(), "".to_string()));
                }
            },
            '=' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::EQUAL, c.to_string(), "".to_string()));
                }
            },
            '!' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::BANG_EQUAL, "!=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::BANG, c.to_string(), "".to_string()));
                }
            },
            '>' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::GREATER_EQUAL, ">=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::GREATER, c.to_string(), "".to_string()));
                }
            },
            '<' => {
                if char_cont.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::LESS_EQUAL, "<=".to_string(), "".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::LESS, c.to_string(), "".to_string()));
                }
            }
            '\n' => {
                line += 1;
            },
            '"' => {
                while let Some(stringL) = char_cont.next() {
                    if stringL == '"' {
                        token.push(Token::new(TokenType::STRING, literalStr.to_string(), literalNum.to_string()));
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
                    token.push(Token::new(TokenType::NUMBER, cont.to_string(), cont.to_string()));
                    token.push(Token::new(TokenType::DOT,".".to_string(),"".to_string()))
                }
                else {
                    token.push(Token::new(TokenType::NUMBER, cont.to_string(), cont.to_string()));
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

                token.push(Token::new(TokenType::IDENTIFIER,cont.to_string(),"".to_string()));
            },
            ' '|'\t'| '\r' =>{continue;},
            


            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                has_error = true;

            }

        }
    }

    token.push(Token::new(TokenType::EOF, "".to_string(),"".to_string()));

    for token in &token {
        println!("{}", token);
    }

    return if has_error {
        bail!(Error::new(65))
    } else {
        Ok(())
    };
}
pub fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
pub fn is_number(c:char) -> bool{
    c.is_digit(10)
}