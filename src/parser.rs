use crate::ast::ASTNode;
use crate::error::ParseError;
use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        self.tokens
            .get(self.pos)
            .cloned()
            .ok_or(ParseError::UnexpectedEndOfInput)
            .inspect(|_| self.pos += 1)
    }

    pub fn parse(&mut self) -> Result<ASTNode, ParseError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, ParseError> {
        let left = self.parse_expr()?;

        if matches!(self.peek(), Some(Token::Assign)) {
            self.advance()?;
            let right = self.parse_assignment()?;
            return Ok(ASTNode::BinaryOp {
                op: '=',
                left: Box::new(left),
                right: Box::new(right),
            });
        }

        Ok(left)
    }

    fn parse_expr(&mut self) -> Result<ASTNode, ParseError> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_mul_div()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Plus => '+',
                Token::Minus => '-',
                _ => break,
            };
            self.advance()?;
            let right = self.parse_mul_div()?;
            left = ASTNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_power()?;

        while let Some(token) = self.peek() {
            let op = match token {
                Token::Multiply => '*',
                Token::Divide => '/',
                _ => break,
            };
            self.advance()?;
            let right = self.parse_power()?;
            left = ASTNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<ASTNode, ParseError> {
        let mut left = self.parse_unary()?;

        if matches!(self.peek(), Some(Token::Power)) {
            self.advance()?;
            let right = self.parse_power()?;
            left = ASTNode::BinaryOp {
                op: '^',
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<ASTNode, ParseError> {
        if matches!(self.peek(), Some(Token::Sqrt)) {
            self.advance()?;
            let operand = self.parse_primary()?;
            return Ok(ASTNode::UnaryOp {
                op: "√".to_string(),
                operand: Box::new(operand),
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<ASTNode, ParseError> {
        match self.advance()? {
            Token::Number(n) => Ok(ASTNode::Number(n)),
            Token::Identifier(name, idx) => Ok(ASTNode::Identifier(name, idx)),
            Token::LParen => {
                let expr = self.parse_expr()?;
                if matches!(self.peek(), Some(Token::RParen)) {
                    self.advance()?;
                }
                Ok(expr)
            }
            token => Err(ParseError::UnexpectedToken(format!("{}", token))),
        }
    }
}

pub struct ParseResult {
    pub tokens: Vec<Token>,
    pub ast: ASTNode,
    pub identifier_table: Vec<(String, usize)>,
}

impl ParseResult {
    pub fn from_input(input: &str) -> Result<Self, ParseError> {
        let mut lexer = crate::lexer::Lexer::new(input);
        let tokens = lexer.tokenize()?;

        if tokens.is_empty() {
            return Err(ParseError::UnexpectedEndOfInput);
        }

        let identifier_table = lexer.into_identifier_table();
        let mut parser = Parser::new(tokens.clone());
        let ast = parser.parse()?;

        Ok(Self {
            tokens,
            ast,
            identifier_table,
        })
    }
}
