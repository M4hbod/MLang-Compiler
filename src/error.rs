use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidToken(String),
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected token: {}", msg),
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
        }
    }
}
