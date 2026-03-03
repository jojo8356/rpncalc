use rpncalc::error::CalcError;
use rpncalc::eval::eval;
use rpncalc::lexer::tokenize;
use rpncalc::parser::parse;

fn eval_str(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    let ast = parse(&tokens)?;
    eval(&ast)
}

#[test]
fn infix_simple_addition() {
    assert_eq!(eval_str("3 + 4").unwrap(), 7.0);
}

#[test]
fn infix_operator_precedence() {
    assert_eq!(eval_str("3 + 4 * 2").unwrap(), 11.0);
}

#[test]
fn infix_parentheses() {
    assert_eq!(eval_str("(3 + 4) * 2").unwrap(), 14.0);
}

#[test]
fn infix_power_right_associative() {
    assert_eq!(eval_str("2 ^ 3 ^ 2").unwrap(), 512.0);
}

#[test]
fn infix_division_by_zero() {
    assert_eq!(eval_str("10 / 0"), Err(CalcError::DivisionByZero));
}

#[test]
fn infix_modulo() {
    assert_eq!(eval_str("10 % 3").unwrap(), 1.0);
}

#[test]
fn infix_negative_numbers() {
    assert_eq!(eval_str("-5 + 3").unwrap(), -2.0);
}

#[test]
fn infix_decimal() {
    let result = eval_str("3.14 * 2").unwrap();
    assert!((result - 6.28).abs() < 1e-10);
}

#[test]
fn infix_nested_parens() {
    assert_eq!(eval_str("((2 + 3) * (4 - 1))").unwrap(), 15.0);
}

#[test]
fn infix_missing_paren() {
    assert_eq!(eval_str("(3 + 4"), Err(CalcError::MissingParen));
}

#[test]
fn infix_unknown_char() {
    assert_eq!(eval_str("3 & 4"), Err(CalcError::UnknownChar('&')));
}

#[test]
fn infix_complex_expression() {
    assert_eq!(eval_str("2 + 3 * 4 - 6 / 2").unwrap(), 11.0);
}
