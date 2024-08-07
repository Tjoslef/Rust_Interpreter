use anyhow::{bail};
use std::fs;
use crate::token::{Token, TokenType};
use crate::error::{Error};

pub fn tokenize(filename: &String) -> anyhow::Result<()> {
    let file_contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => bail!("Failed to read the file."),
    };
    let mut char_cont = file_contents.chars().peekable();
    let line = 1usize;
    let mut has_error = false;
    let mut token = vec![];
    while let Some(&c) = char_cont.peek() {
        match c {
            '(' => token.push(Token::new(TokenType::LEFT_PAREN, c.to_string())),
            ')' => token.push(Token::new(TokenType::RIGHT_PAREN, c.to_string())),
            '{' => token.push(Token::new(TokenType::LEFT_BRACE, c.to_string())),
            '}' => token.push(Token::new(TokenType::RIGHT_BRACE, c.to_string())),
            ',' => token.push(Token::new(TokenType::COMMA, c.to_string())),
            '.' => token.push(Token::new(TokenType::DOT, c.to_string())),
            '-' => token.push(Token::new(TokenType::MINUS, c.to_string())),
            '+' => token.push(Token::new(TokenType::PLUS, c.to_string())),
            ';' => token.push(Token::new(TokenType::SEMICOLON, c.to_string())),
            '*' => token.push(Token::new(TokenType::STAR, c.to_string())),
            '/' => {
                char_cont.next(); // Consume the '/'
                if let Some(&next_char) = char_cont.peek() {
                    if next_char == '/' {
                        // This is a single-line comment, skip until end of line
                        while let  Some(&comment_char) = char_cont.peek() {
                            if comment_char == '\n' {
                                break;
                            }
                            char_cont.next();
                        }
                        continue; // Continue to the next character
                    } else {
                        token.push(Token::new(TokenType::FOWARD_SLASH, "/".to_string()));
                    }
                } else {
                    token.push(Token::new(TokenType::FOWARD_SLASH, "/".to_string()));
                }
            },
            '=' => {
                let mut peekable = char_cont.clone().peekable();
                if peekable.peek() == Some(&'=') {
                    token.push(Token::new(TokenType::EQUAL_EQUAL, "==".to_string()));
                    char_cont.next();
                } else {
                    token.push(Token::new(TokenType::EQUAL, c.to_string()));
                }
            },
            '!' => {
                let mut peekable = char_cont.clone().peekable();
                if peekable.peek() == Some(&'='){
                    token.push(Token::new(TokenType::BANG_EQUAL,"!=".to_string()));
                    char_cont.next();
                }
                else {
                    token.push(Token::new(TokenType::BANG,c.to_string()));
                }
            }
            '>' =>{
                let mut peekable = char_cont.clone().peekable();
                if peekable.peek() == Some(&'='){
                    token.push(Token::new(TokenType::GREATER_EQUAL,">=".to_string()));
                    char_cont.next();
                }
                else {
                    token.push(Token::new(TokenType::GREATER,c.to_string()));
                }
            }
            '<' => {
                let mut peekable = char_cont.clone().peekable();
                if peekable.peek() == Some(&'='){
                    token.push(Token::new(TokenType::LESS_EQUAL,"<=".to_string()));
                    char_cont.next();
                }
                else {
                    token.push(Token::new(TokenType::LESS,c.to_string()));
                }
                }



            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", line, c);
                has_error = true
            }

        }
    }
    token.push(Token::new(TokenType::EOF, "".to_string()));
    for token in &token {
        println!("{}", token);
    }
    return if has_error {
        bail!(Error::new(65))
    } else {
        Ok(())
    };
}