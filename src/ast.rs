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
                    _ => 0.0,
                }
            }
            ASTNode::UnaryOp { op, operand } => {
                let val = operand.evaluate();
                match op.as_str() {
                    "sqrt" => val.sqrt(),
                    _ => val,
                }
            }
        }
    }

    pub fn to_tree_lines(&self, prefix: &str, is_last: bool) -> Vec<String> {
        let mut lines = Vec::new();
        let connector = if is_last { "└─ " } else { "├─ " };

        match self {
            ASTNode::Number(n) => {
                lines.push(format!("{}{}{}", prefix, connector, n));
            }
            ASTNode::Identifier(_name, idx) => {
                lines.push(format!("{}{}id{}", prefix, connector, idx));
            }
            ASTNode::BinaryOp { op, left, right } => {
                lines.push(format!("{}{}{}", prefix, connector, op));
                let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
                lines.extend(left.to_tree_lines(&new_prefix, false));
                lines.extend(right.to_tree_lines(&new_prefix, true));
            }
            ASTNode::UnaryOp { op, operand } => {
                lines.push(format!("{}{}{}", prefix, connector, op));
                let new_prefix = format!("{}{}   ", prefix, if is_last { " " } else { "│" });
                lines.extend(operand.to_tree_lines(&new_prefix, true));
            }
        }

        lines
    }
}
