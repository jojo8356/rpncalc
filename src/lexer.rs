use crate::error::CalcError;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl Op {
    pub fn precedence(&self) -> u8 {
        match self {
            Op::Add | Op::Sub => 1,
            Op::Mul | Op::Div | Op::Mod => 2,
            Op::Pow => 3,
        }
    }

    pub fn is_right_associative(&self) -> bool {
        matches!(self, Op::Pow)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Op),
    LeftParen,
    RightParen,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' => {
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let n: f64 = num_str.parse().map_err(|_| CalcError::InvalidExpression)?;
                tokens.push(Token::Number(n));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Operator(Op::Add));
            }
            '-' => {
                chars.next();
                // Negative number: after operator, left paren, or at start
                let is_unary = tokens.is_empty()
                    || matches!(
                        tokens.last(),
                        Some(Token::Operator(_)) | Some(Token::LeftParen)
                    );
                if is_unary {
                    if let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() || ch == '.' {
                            let mut num_str = String::from("-");
                            while let Some(&ch) = chars.peek() {
                                if ch.is_ascii_digit() || ch == '.' {
                                    num_str.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            let n: f64 =
                                num_str.parse().map_err(|_| CalcError::InvalidExpression)?;
                            tokens.push(Token::Number(n));
                            continue;
                        }
                    }
                    tokens.push(Token::Operator(Op::Sub));
                } else {
                    tokens.push(Token::Operator(Op::Sub));
                }
            }
            '*' => {
                chars.next();
                tokens.push(Token::Operator(Op::Mul));
            }
            '/' => {
                chars.next();
                tokens.push(Token::Operator(Op::Div));
            }
            '%' => {
                chars.next();
                tokens.push(Token::Operator(Op::Mod));
            }
            '^' => {
                chars.next();
                tokens.push(Token::Operator(Op::Pow));
            }
            '(' => {
                chars.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RightParen);
            }
            _ => return Err(CalcError::UnknownChar(c)),
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let tokens = tokenize("3 + 4").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(3.0),
                Token::Operator(Op::Add),
                Token::Number(4.0),
            ]
        );
    }

    #[test]
    fn tokenize_negative() {
        let tokens = tokenize("-7 + 3").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(-7.0),
                Token::Operator(Op::Add),
                Token::Number(3.0),
            ]
        );
    }

    #[test]
    fn tokenize_decimal() {
        let tokens = tokenize("3.14").unwrap();
        assert_eq!(tokens, vec![Token::Number(3.14)]);
    }

    #[test]
    fn tokenize_parens() {
        let tokens = tokenize("(3 + 4)").unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParen,
                Token::Number(3.0),
                Token::Operator(Op::Add),
                Token::Number(4.0),
                Token::RightParen,
            ]
        );
    }

    #[test]
    fn tokenize_unknown_char() {
        assert_eq!(tokenize("3 & 4"), Err(CalcError::UnknownChar('&')));
    }
}
