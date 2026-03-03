use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    InvalidExpression,
    MissingParen,
    InsufficientOperands,
    UnknownChar(char),
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Error: division by zero"),
            CalcError::InvalidExpression => write!(f, "Error: invalid expression"),
            CalcError::MissingParen => write!(f, "Error: missing parenthesis"),
            CalcError::InsufficientOperands => {
                write!(f, "Error: insufficient operands on stack")
            }
            CalcError::UnknownChar(c) => write!(f, "Error: unknown character '{c}'"),
        }
    }
}

impl std::error::Error for CalcError {}
