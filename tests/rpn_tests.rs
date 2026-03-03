use rpncalc::error::CalcError;
use rpncalc::rpn::eval_rpn;

#[test]
fn rpn_simple_addition() {
    assert_eq!(eval_rpn("3 4 +").unwrap(), 7.0);
}

#[test]
fn rpn_multiplication_then_add() {
    assert_eq!(eval_rpn("3 4 2 * +").unwrap(), 11.0);
}

#[test]
fn rpn_complex() {
    assert_eq!(eval_rpn("5 1 2 + 4 * + 3 -").unwrap(), 14.0);
}

#[test]
fn rpn_insufficient_operands() {
    assert_eq!(eval_rpn("+"), Err(CalcError::InsufficientOperands));
}

#[test]
fn rpn_single_number() {
    assert_eq!(eval_rpn("42").unwrap(), 42.0);
}

#[test]
fn rpn_subtraction() {
    assert_eq!(eval_rpn("10 3 -").unwrap(), 7.0);
}

#[test]
fn rpn_division() {
    assert_eq!(eval_rpn("10 2 /").unwrap(), 5.0);
}

#[test]
fn rpn_division_by_zero() {
    assert_eq!(eval_rpn("10 0 /"), Err(CalcError::DivisionByZero));
}

#[test]
fn rpn_modulo() {
    assert_eq!(eval_rpn("10 3 %").unwrap(), 1.0);
}

#[test]
fn rpn_power() {
    assert_eq!(eval_rpn("2 3 ^").unwrap(), 8.0);
}

#[test]
fn rpn_too_many_values() {
    assert_eq!(eval_rpn("1 2 3 +"), Err(CalcError::InvalidExpression));
}
