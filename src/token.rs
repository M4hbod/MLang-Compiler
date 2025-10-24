use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Number(f64),
    Identifier(String, usize),
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    LParen,
    RParen,
    Sqrt,
    Assign,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "NUMBER({})", n),
            Token::Identifier(_, idx) => write!(f, "id{}", idx),
            Token::Plus => write!(f, "PLUS"),
            Token::Minus => write!(f, "MINUS"),
            Token::Multiply => write!(f, "MUL"),
            Token::Divide => write!(f, "DIV"),
            Token::Power => write!(f, "POW"),
            Token::LParen => write!(f, "LPAREN"),
            Token::RParen => write!(f, "RPAREN"),
            Token::Sqrt => write!(f, "SQRT"),
            Token::Assign => write!(f, "ASSIGN"),
        }
    }
}
