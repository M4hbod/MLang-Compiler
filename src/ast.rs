use std::fmt;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(f64),
    Identifier(String, usize),
    BinaryOp {
        op: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    UnaryOp {
        op: String,
        operand: Box<ASTNode>,
    },
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ASTNode::Number(n) => write!(f, "{}", n),
            ASTNode::Identifier(_name, idx) => write!(f, "id{}", idx),
            ASTNode::BinaryOp { op, left, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            ASTNode::UnaryOp { op, operand } => {
                write!(f, "{}({})", op, operand)
            }
        }
    }
}

impl ASTNode {
    pub fn has_variables(&self) -> bool {
        match self {
            ASTNode::Identifier(_, _) => true,
            ASTNode::Number(_) => false,
            ASTNode::BinaryOp { left, right, .. } => left.has_variables() || right.has_variables(),
            ASTNode::UnaryOp { operand, .. } => operand.has_variables(),
        }
    }

    pub fn evaluate(&self) -> f64 {
        match self {
            ASTNode::Number(n) => *n,
            ASTNode::Identifier(_, _) => 0.0,
            ASTNode::BinaryOp { op, left, right } => {
                let l = left.evaluate();
                let r = right.evaluate();
                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    '^' => l.powf(r),
                    '=' => r, // For assignment, return the right-hand side value
                    _ => 0.0,
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                let val = operand.evaluate();
                match op.as_str() {
                    "sqrt" => val.powf(0.5),
                    _ => val,
                }
            }
        }
    }

    pub fn to_three_address_code(&self, temp_counter: &mut usize) -> (Vec<String>, String) {
        match self {
            ASTNode::Number(n) => (vec![], format!("{}", n)),
            ASTNode::Identifier(_name, idx) => (vec![], format!("id{}", idx)),
            ASTNode::BinaryOp { op, left, right } => {
                let (mut left_code, left_result) = left.to_three_address_code(temp_counter);
                let (mut right_code, right_result) = right.to_three_address_code(temp_counter);

                let temp = format!("t{}", temp_counter);
                *temp_counter += 1;

                let mut code = vec![];
                code.append(&mut left_code);
                code.append(&mut right_code);

                let op_str = match op {
                    '=' => "=",
                    '+' => "+",
                    '-' => "-",
                    '*' => "*",
                    '/' => "/",
                    '^' => "^",
                    _ => "?",
                };

                if *op == '=' {
                    code.push(format!("{} = {}", left_result, right_result));
                    (code, left_result)
                } else {
                    code.push(format!(
                        "{} = {} {} {}",
                        temp, left_result, op_str, right_result
                    ));
                    (code, temp)
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                let (mut operand_code, operand_result) =
                    operand.to_three_address_code(temp_counter);
                let temp = format!("t{}", temp_counter);
                *temp_counter += 1;

                operand_code.push(format!("{} = {}({})", temp, op, operand_result));
                (operand_code, temp)
            }
        }
    }

    pub fn semantic_check(&self) -> Vec<String> {
        let mut warnings = vec![];
        self.semantic_check_recursive(&mut warnings);
        warnings
    }

    fn semantic_check_recursive(&self, warnings: &mut Vec<String>) {
        match self {
            ASTNode::BinaryOp { op, left, right } => {
                // Check division by zero
                if *op == '/' {
                    if let ASTNode::Number(n) = **right {
                        if n == 0.0 {
                            warnings.push("Warning: Division by zero detected".to_string());
                        }
                    }
                }

                // Check power with negative base and fractional exponent
                if *op == '^' {
                    if let (ASTNode::Number(base), ASTNode::Number(exp)) = (&**left, &**right) {
                        if *base < 0.0 && exp.fract() != 0.0 {
                            warnings.push("Warning: Negative base with fractional exponent may produce complex numbers".to_string());
                        }
                    }
                }

                left.semantic_check_recursive(warnings);
                right.semantic_check_recursive(warnings);
            }
            ASTNode::UnaryOp { operand, .. } => {
                operand.semantic_check_recursive(warnings);
            }
            _ => {}
        }
    }

    pub fn optimize(&self) -> ASTNode {
        match self {
            ASTNode::Number(_) | ASTNode::Identifier(_, _) => self.clone(),
            ASTNode::BinaryOp { op, left, right } => {
                let left_opt = left.optimize();
                let right_opt = right.optimize();

                // Constant folding
                if let (ASTNode::Number(l), ASTNode::Number(r)) = (&left_opt, &right_opt) {
                    let result = match op {
                        '+' => l + r,
                        '-' => l - r,
                        '*' => l * r,
                        '/' if *r != 0.0 => l / r,
                        '^' => l.powf(*r),
                        _ => {
                            return ASTNode::BinaryOp {
                                op: *op,
                                left: Box::new(left_opt),
                                right: Box::new(right_opt),
                            };
                        }
                    };
                    return ASTNode::Number(result);
                }

                // Algebraic simplification
                match (*op, &left_opt, &right_opt) {
                    // x + 0 = x
                    ('+', _, ASTNode::Number(0.0)) => left_opt,
                    ('+', ASTNode::Number(0.0), _) => right_opt,
                    // x - 0 = x
                    ('-', _, ASTNode::Number(0.0)) => left_opt,
                    // x * 0 = 0
                    ('*', _, ASTNode::Number(0.0)) | ('*', ASTNode::Number(0.0), _) => {
                        ASTNode::Number(0.0)
                    }
                    // x * 1 = x
                    ('*', _, ASTNode::Number(1.0)) => left_opt,
                    ('*', ASTNode::Number(1.0), _) => right_opt,
                    // x / 1 = x
                    ('/', _, ASTNode::Number(1.0)) => left_opt,
                    // x ^ 0 = 1
                    ('^', _, ASTNode::Number(0.0)) => ASTNode::Number(1.0),
                    // x ^ 1 = x
                    ('^', _, ASTNode::Number(1.0)) => left_opt,
                    _ => ASTNode::BinaryOp {
                        op: *op,
                        left: Box::new(left_opt),
                        right: Box::new(right_opt),
                    },
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                let operand_opt = operand.optimize();
                if let ASTNode::Number(n) = operand_opt {
                    if op == "sqrt" {
                        return ASTNode::Number(n.powf(0.5));
                    }
                }
                ASTNode::UnaryOp {
                    op: op.clone(),
                    operand: Box::new(operand_opt),
                }
            }
        }
    }
}
