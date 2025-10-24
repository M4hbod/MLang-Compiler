use crate::error::ParseError;
use crate::token::Token;
use std::collections::HashMap;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    identifier_map: HashMap<String, usize>,
    next_id: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
            identifier_map: HashMap::new(),
            next_id: 1,
        }
    }

    fn get_identifier_index(&mut self, name: &str) -> usize {
        *self
            .identifier_map
            .entry(name.to_string())
            .or_insert_with(|| {
                let idx = self.next_id;
                self.next_id += 1;
                idx
            })
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<f64, ParseError> {
        let mut num_str = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        num_str
            .parse()
            .map_err(|_| ParseError::InvalidNumber(num_str))
    }

    fn read_identifier(&mut self) -> String {
        let mut id = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                id.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        id
    }

    fn next_token(&mut self) -> Result<Option<Token>, ParseError> {
        self.skip_whitespace();

        let ch = match self.peek() {
            Some(c) => c,
            None => return Ok(None),
        };

        let token = match ch {
            '+' => {
                self.advance();
                Token::Plus
            }
            '-' => {
                self.advance();
                Token::Minus
            }
            '*' => {
                self.advance();
                Token::Multiply
            }
            '/' => {
                self.advance();
                Token::Divide
            }
            '^' => {
                self.advance();
                Token::Power
            }
            '(' => {
                self.advance();
                Token::LParen
            }
            ')' => {
                self.advance();
                Token::RParen
            }
            '=' => {
                self.advance();
                Token::Assign
            }
            ch if ch.is_numeric() => Token::Number(self.read_number()?),
            ch if ch.is_alphabetic() || ch == '_' => {
                let name = self.read_identifier();

                // Check if it's the sqrt keyword
                if name.to_lowercase() == "sqrt" {
                    return Ok(Some(Token::Sqrt));
                }

                let idx = self.get_identifier_index(&name);
                Token::Identifier(name, idx)
            }
            _ => return Err(ParseError::InvalidToken(ch.to_string())),
        };

        Ok(Some(token))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    pub fn into_identifier_table(self) -> Vec<(String, usize)> {
        let mut ids: Vec<_> = self.identifier_map.into_iter().collect();
        ids.sort_by_key(|(_, idx)| *idx);
        ids
    }
}
