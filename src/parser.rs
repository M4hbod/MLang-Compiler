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
            // Transform sqrt(x) into x^0.5
            return Ok(ASTNode::BinaryOp {
                op: '^',
                left: Box::new(operand),
                right: Box::new(ASTNode::Number(0.5)),
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
    pub semantic_warnings: Vec<String>,
    pub three_address_code: Vec<String>,
    pub optimized_ast: ASTNode,
    pub optimized_three_address_code: Vec<String>,
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

        // Semantic analysis
        let semantic_warnings = ast.semantic_check();

        // Intermediate code generation
        let mut temp_counter = 1;
        let (three_address_code, _) = ast.to_three_address_code(&mut temp_counter);

        // Code optimization
        let optimized_ast = ast.optimize();

        // Generate optimized three-address code
        let mut temp_counter = 1;
        let (mut optimized_three_address_code, _) =
            optimized_ast.to_three_address_code(&mut temp_counter);

        // Apply peephole optimization to eliminate unnecessary temporaries
        optimized_three_address_code = Self::peephole_optimize(optimized_three_address_code);

        Ok(Self {
            tokens,
            ast,
            identifier_table,
            semantic_warnings,
            three_address_code,
            optimized_ast,
            optimized_three_address_code,
        })
    }

    /// Peephole optimization: eliminate unnecessary temporary variables
    /// Transforms patterns like:
    ///   t5 = t4 - 10
    ///   id1 = t5
    /// Into:
    ///   id1 = t4 - 10
    fn peephole_optimize(code: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;

        let mut temp_definitions: HashMap<String, String> = HashMap::new();
        let mut temp_usage_count: HashMap<String, usize> = HashMap::new();
        let mut skip_indices = std::collections::HashSet::new();

        // First pass: count how many times each temp is used and store definitions
        for line in &code {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "=" {
                let lhs = parts[0];
                let rhs = parts[2..].join(" ");

                // Store temp definitions
                if lhs.starts_with('t') && lhs.chars().skip(1).all(|c| c.is_numeric()) {
                    temp_definitions.insert(lhs.to_string(), rhs.clone());
                }

                // Count usages of temps on the right side
                for part in &parts[2..] {
                    let clean_part = part.trim_end_matches(|c: char| !c.is_alphanumeric());
                    if clean_part.starts_with('t')
                        && clean_part.chars().skip(1).all(|c| c.is_numeric())
                    {
                        *temp_usage_count.entry(clean_part.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }

        // Second pass: check for pattern "temp = expr; var = temp" and mark temp line for removal
        for i in 0..code.len().saturating_sub(1) {
            let curr_parts: Vec<&str> = code[i].split_whitespace().collect();
            let next_parts: Vec<&str> = code[i + 1].split_whitespace().collect();

            if curr_parts.len() >= 3
                && curr_parts[1] == "="
                && next_parts.len() >= 3
                && next_parts[1] == "="
            {
                let curr_lhs = curr_parts[0];
                let next_rhs = next_parts[2..].join(" ");

                // Check if current assigns to a temp, and next line uses only that temp
                if curr_lhs.starts_with('t')
                    && curr_lhs.chars().skip(1).all(|c| c.is_numeric())
                    && next_rhs == curr_lhs
                    && temp_usage_count.get(curr_lhs).copied().unwrap_or(0) == 1
                {
                    // Mark the temp assignment for skipping
                    skip_indices.insert(i);
                }
            }
        }

        // Third pass: build optimized code
        let mut optimized = Vec::new();
        for (i, line) in code.iter().enumerate() {
            if skip_indices.contains(&i) {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "=" {
                let lhs = parts[0];
                let rhs = parts[2..].join(" ");

                // If rhs is a single temp that was used only once, substitute its definition
                if rhs.starts_with('t')
                    && rhs.chars().all(|c| c.is_alphanumeric())
                    && temp_definitions.contains_key(&rhs)
                    && temp_usage_count.get(&rhs).copied().unwrap_or(0) == 1
                {
                    let definition = temp_definitions.get(&rhs).unwrap();
                    optimized.push(format!("{} = {}", lhs, definition));
                    continue;
                }
            }

            optimized.push(line.clone());
        }

        optimized
    }
}
