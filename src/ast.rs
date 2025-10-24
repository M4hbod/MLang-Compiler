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
}
