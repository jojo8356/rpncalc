use crate::error::CalcError;
use crate::eval::apply_op;
use crate::lexer::{Token, tokenize};

pub fn eval_rpn(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    eval_rpn_tokens(&tokens)
}

pub fn eval_rpn_tokens(tokens: &[Token]) -> Result<f64, CalcError> {
    if tokens.is_empty() {
        return Err(CalcError::InvalidExpression);
    }

    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(CalcError::InsufficientOperands);
                }
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                let result = apply_op(op, l, r)?;
                stack.push(result);
            }
            Token::LeftParen | Token::RightParen => {
                return Err(CalcError::InvalidExpression);
            }
        }
    }

    if stack.len() != 1 {
        return Err(CalcError::InvalidExpression);
    }

    Ok(stack.pop().unwrap())
}

pub fn get_stack(input: &str, stack: &mut Vec<f64>) -> Result<Option<f64>, CalcError> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Ok(None);
    }

    for token in &tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(op) => {
                if stack.len() < 2 {
                    return Err(CalcError::InsufficientOperands);
                }
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                let result = apply_op(op, l, r)?;
                stack.push(result);
            }
            Token::LeftParen | Token::RightParen => {
                return Err(CalcError::InvalidExpression);
            }
        }
    }

    Ok(stack.last().copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rpn_simple() {
        assert_eq!(eval_rpn("3 4 +").unwrap(), 7.0);
    }

    #[test]
    fn rpn_complex() {
        assert_eq!(eval_rpn("3 4 2 * +").unwrap(), 11.0);
    }

    #[test]
    fn rpn_longer() {
        assert_eq!(eval_rpn("5 1 2 + 4 * + 3 -").unwrap(), 14.0);
    }

    #[test]
    fn rpn_insufficient_operands() {
        assert_eq!(eval_rpn("+"), Err(CalcError::InsufficientOperands));
    }

    #[test]
    fn rpn_stack_persistent() {
        let mut stack = Vec::new();
        get_stack("5 3", &mut stack).unwrap();
        assert_eq!(stack, vec![5.0, 3.0]);
        get_stack("+", &mut stack).unwrap();
        assert_eq!(stack, vec![8.0]);
    }
}
