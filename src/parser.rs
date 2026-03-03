use crate::error::CalcError;
use crate::lexer::{Op, Token};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// Parses infix tokens into an AST using the Shunting-Yard algorithm.
pub fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    if tokens.is_empty() {
        return Err(CalcError::InvalidExpression);
    }

    let mut output: Vec<Expr> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => {
                output.push(Expr::Num(*n));
            }
            Token::Operator(op) => {
                while let Some(top) = ops.last() {
                    match top {
                        Token::Operator(top_op) => {
                            let should_pop = if op.is_right_associative() {
                                top_op.precedence() > op.precedence()
                            } else {
                                top_op.precedence() >= op.precedence()
                            };
                            if should_pop {
                                let top_op = match ops.pop().unwrap() {
                                    Token::Operator(o) => o,
                                    _ => unreachable!(),
                                };
                                apply_op(&mut output, top_op)?;
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                ops.push(Token::Operator(op.clone()));
            }
            Token::LeftParen => {
                ops.push(Token::LeftParen);
            }
            Token::RightParen => {
                let mut found_paren = false;
                while let Some(top) = ops.pop() {
                    match top {
                        Token::LeftParen => {
                            found_paren = true;
                            break;
                        }
                        Token::Operator(op) => {
                            apply_op(&mut output, op)?;
                        }
                        _ => return Err(CalcError::InvalidExpression),
                    }
                }
                if !found_paren {
                    return Err(CalcError::MissingParen);
                }
            }
        }
    }

    while let Some(top) = ops.pop() {
        match top {
            Token::Operator(op) => {
                apply_op(&mut output, op)?;
            }
            Token::LeftParen => return Err(CalcError::MissingParen),
            _ => return Err(CalcError::InvalidExpression),
        }
    }

    if output.len() != 1 {
        return Err(CalcError::InvalidExpression);
    }

    Ok(output.pop().unwrap())
}

fn apply_op(output: &mut Vec<Expr>, op: Op) -> Result<(), CalcError> {
    if output.len() < 2 {
        return Err(CalcError::InvalidExpression);
    }
    let right = output.pop().unwrap();
    let left = output.pop().unwrap();
    output.push(Expr::BinOp {
        op,
        left: Box::new(left),
        right: Box::new(right),
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn parse_simple_addition() {
        let tokens = tokenize("3 + 4").unwrap();
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::BinOp {
                op: Op::Add,
                left: Box::new(Expr::Num(3.0)),
                right: Box::new(Expr::Num(4.0)),
            }
        );
    }

    #[test]
    fn parse_precedence() {
        // 3 + 4 * 2 should parse as 3 + (4 * 2)
        let tokens = tokenize("3 + 4 * 2").unwrap();
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::BinOp {
                op: Op::Add,
                left: Box::new(Expr::Num(3.0)),
                right: Box::new(Expr::BinOp {
                    op: Op::Mul,
                    left: Box::new(Expr::Num(4.0)),
                    right: Box::new(Expr::Num(2.0)),
                }),
            }
        );
    }

    #[test]
    fn parse_parentheses() {
        let tokens = tokenize("(3 + 4) * 2").unwrap();
        let ast = parse(&tokens).unwrap();
        assert_eq!(
            ast,
            Expr::BinOp {
                op: Op::Mul,
                left: Box::new(Expr::BinOp {
                    op: Op::Add,
                    left: Box::new(Expr::Num(3.0)),
                    right: Box::new(Expr::Num(4.0)),
                }),
                right: Box::new(Expr::Num(2.0)),
            }
        );
    }

    #[test]
    fn parse_missing_paren() {
        let tokens = tokenize("(3 + 4").unwrap();
        assert_eq!(parse(&tokens), Err(CalcError::MissingParen));
    }
}
