use crate::error::CalcError;
use crate::lexer::Op;
use crate::parser::Expr;

pub fn eval(expr: &Expr) -> Result<f64, CalcError> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::BinOp { op, left, right } => {
            let l = eval(left)?;
            let r = eval(right)?;
            apply_op(op, l, r)
        }
    }
}

pub fn apply_op(op: &Op, l: f64, r: f64) -> Result<f64, CalcError> {
    match op {
        Op::Add => Ok(l + r),
        Op::Sub => Ok(l - r),
        Op::Mul => Ok(l * r),
        Op::Div => {
            if r == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(l / r)
            }
        }
        Op::Mod => {
            if r == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(l % r)
            }
        }
        Op::Pow => Ok(l.powf(r)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn eval_str(input: &str) -> Result<f64, CalcError> {
        let tokens = tokenize(input)?;
        let ast = parse(&tokens)?;
        eval(&ast)
    }

    #[test]
    fn eval_addition() {
        assert_eq!(eval_str("3 + 4").unwrap(), 7.0);
    }

    #[test]
    fn eval_precedence() {
        assert_eq!(eval_str("3 + 4 * 2").unwrap(), 11.0);
    }

    #[test]
    fn eval_parens() {
        assert_eq!(eval_str("(3 + 4) * 2").unwrap(), 14.0);
    }

    #[test]
    fn eval_power_right_assoc() {
        assert_eq!(eval_str("2 ^ 3 ^ 2").unwrap(), 512.0);
    }

    #[test]
    fn eval_division_by_zero() {
        assert_eq!(eval_str("10 / 0"), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn eval_modulo() {
        assert_eq!(eval_str("10 % 3").unwrap(), 1.0);
    }
}
