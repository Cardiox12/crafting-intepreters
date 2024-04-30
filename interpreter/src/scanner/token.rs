use std::any::Any;

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    // Single character tokens
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[allow(unused)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    // Don't know for now what to use
    literal: Box<dyn Any>,
    line: usize,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Box<dyn Any>, line: usize) -> Self {
        Self {
            kind,
            lexeme,
            literal,
            line
        }
    }

    pub fn to_str(&self) -> String {
        format!("{:?} {}", self.kind, self.lexeme)
    }
}
