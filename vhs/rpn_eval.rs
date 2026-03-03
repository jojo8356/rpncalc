// RPN evaluator: push numbers, apply operators from the stack

fn eval_rpn(tokens: &[Token]) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => stack.push(*n),
            Token::Operator(op) => {
                let r = stack.pop().ok_or(InsufficientOperands)?;
                let l = stack.pop().ok_or(InsufficientOperands)?;
                stack.push(apply(op, l, r)?);
            }
        }
    }
    Ok(stack.pop().unwrap())
}

// "3 4 2 * +" → 3 + (4 * 2) → 11
