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


    fn get_literal(&self) -> String {
        match self.kind {
            TokenKind::LeftParen => String::from("("),
            TokenKind::RightParen => String::from(")"),
            TokenKind::LeftBrace => String::from("{"),
            TokenKind::RightBrace => String::from("}"),
            TokenKind::Comma => String::from(","),
            TokenKind::Dot => String::from("."),
            TokenKind::Minus => String::from("-"),
            TokenKind::Plus => String::from("+"),
            TokenKind::Semicolon => String::from(";"),
            TokenKind::Slash => String::from("/"),
            TokenKind::Star => String::from("*"),
            TokenKind::Bang => String::from("!"),
            TokenKind::BangEqual => String::from("!="),
            TokenKind::Equal => String::from("="),
            TokenKind::EqualEqual => String::from("=="),
            TokenKind::Greater => String::from(">"),
            TokenKind::GreaterEqual => String::from(">="),
            TokenKind::Less => String::from("<"),
            TokenKind::LessEqual => String::from("<="),
            TokenKind::Identifier | TokenKind::String =>
                self.literal.downcast_ref::<String>().unwrap().to_string(),
            TokenKind::Number => {
                let value = self.literal.downcast_ref::<f64>().unwrap();
                format!("{value}")
            }
            TokenKind::And => String::from("and"),
            TokenKind::Class => String::from("class"),
            TokenKind::Else => String::from("else"),
            TokenKind::False => String::from("false"),
            TokenKind::Fun => String::from("fun"),
            TokenKind::For => String::from("for"),
            TokenKind::If => String::from("if"),
            TokenKind::Nil => String::from("Nil"),
            TokenKind::Or => String::from("or"),
            TokenKind::Print => String::from("print"),
            TokenKind::Return => String::from("return"),
            TokenKind::Super => String::from("super"),
            TokenKind::This => String::from("this"),
            TokenKind::True => String::from("true"),
            TokenKind::Var => String::from("var"),
            TokenKind::While => String::from("while"),
            TokenKind::Eof => String::from("EOF"),
        }
    }
}
