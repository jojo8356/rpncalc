// Shunting-Yard: infix tokens → AST with operator precedence

fn parse(tokens: &[Token]) -> Result<Expr, CalcError> {
    let mut output: Vec<Expr> = Vec::new();
    let mut ops: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(n) => output.push(Expr::Num(*n)),
            Token::Operator(op) => {
                while should_pop(&ops, op) {
                    apply_op(&mut output, ops.pop())?;
                }
                ops.push(Token::Operator(op.clone()));
            }
            Token::LeftParen => ops.push(Token::LeftParen),
            Token::RightParen => pop_until_paren(&mut ops, &mut output)?,
        }
    }
    drain_remaining(&mut ops, &mut output)?;
    Ok(output.pop().unwrap())
}
