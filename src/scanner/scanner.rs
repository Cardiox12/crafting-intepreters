use std::any::Any;
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::LOX;

use super::token::Token;
use super::token::TokenKind;


lazy_static! {
    static ref IDENTIFIERS: HashMap<String, TokenKind> = {
        let mut ids = HashMap::new();
        ids.insert("and".to_string(),       TokenKind::And);
        ids.insert("class".to_string(),     TokenKind::Class);
        ids.insert("else".to_string(),      TokenKind::Else);
        ids.insert("false".to_string(),     TokenKind::False);
        ids.insert("for".to_string(),       TokenKind::For);
        ids.insert("fun".to_string(),       TokenKind::Fun);
        ids.insert("if".to_string(),        TokenKind::If);
        ids.insert("nil".to_string(),       TokenKind::Nil);
        ids.insert("or".to_string(),        TokenKind::Or);
        ids.insert("print".to_string(),     TokenKind::Print);
        ids.insert("return".to_string(),    TokenKind::Return);
        ids.insert("super".to_string(),     TokenKind::Super);
        ids.insert("this".to_string(),      TokenKind::This);
        ids.insert("true".to_string(),      TokenKind::True);
        ids.insert("var".to_string(),       TokenKind::Var);
        ids.insert("while".to_string(),     TokenKind::While);

        ids
    };
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(
            Token::new(TokenKind::Eof, String::new(), Box::new(""), self.line)
        );
        
        vec![]
    }
    
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => {
                let kind = if self.match_char('=') { 
                    TokenKind::BangEqual 
                } else { 
                    TokenKind::Equal 
                };
                self.add_token(kind);
            },
            '=' => {
                let kind = if self.match_char('=') { 
                    TokenKind::EqualEqual 
                } else { 
                    TokenKind::Equal
                };
                self.add_token(kind);
            },
            '<' => {
                let kind = if self.match_char('=') { 
                    TokenKind::LessEqual
                } else { 
                    TokenKind::Less
                };
                self.add_token(kind);
            },
            '>' => {
                let kind = if self.match_char('=') { 
                    TokenKind::GreaterEqual
                } else { 
                    TokenKind::Greater
                };
                self.add_token(kind);
            },
            '/' => {
                if self.match_char('/') {
                    // This is a comment it just skeep it
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
            },
            // Ignore whitespaces
            ' ' | '\r' | '\t' => {},
            '\n' => self.line += 1, 
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    LOX.lock().unwrap().error(self.line, String::from("Unterminated string."));
                }

                self.advance();

                let value = self.source[self.start + 1..self.current - 1].to_string();
                self.add_token_literal(TokenKind::String, Box::new(value));

            },
            _ => {
                if c.is_digit(10) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                }
                else {
                    LOX
                        .lock()
                        .unwrap()
                        .error(self.line, "Unexpected character.".to_string())
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current < self.source.len()
    }
    
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        } else if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }
    
    fn add_token(&mut self, kind: TokenKind) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
            kind, 
            text.clone(), 
            Box::new(text), 
            self.line,
        );

        self.tokens.push(token);
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn add_token_literal(&mut self, kind: TokenKind, literal: Box<dyn Any>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(
            kind, 
            text, 
            literal, 
            self.line,
        );

        self.tokens.push(token);
    }
    
    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&mut self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }
    
    fn number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_some() && self.peek_next().unwrap().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value: f64 = self.source[self.start..self.current]
            .to_string()
            .parse()
            .unwrap();
        self.add_token_literal(TokenKind::Number, Box::new(value));
    }
    
    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }
        let lexeme = self.source[self.start..self.current].to_string();
        let kind = IDENTIFIERS
            .get(&lexeme)
            .unwrap_or(&TokenKind::Nil);
        self.add_token(*kind);
    }
}
