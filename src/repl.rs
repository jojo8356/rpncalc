use rustyline::DefaultEditor;

use crate::error::CalcError;
use crate::eval::eval;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::rpn::get_stack;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Std,
    Rpn,
}

fn format_result(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("= {}", value as i64)
    } else {
        format!("= {value}")
    }
}

fn eval_infix(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    let ast = parse(&tokens)?;
    eval(&ast)
}

pub fn run_repl() {
    let mut rl = DefaultEditor::new().expect("Failed to initialize rustyline");
    let mut mode = Mode::Std;
    let mut rpn_stack: Vec<f64> = Vec::new();
    let mut history: Vec<String> = Vec::new();

    loop {
        let prompt = "rpncalc> ";
        let line = match rl.readline(prompt) {
            Ok(line) => line,
            Err(_) => break,
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let _ = rl.add_history_entry(trimmed);

        match trimmed {
            "quit" | "exit" => break,
            "mode rpn" => {
                mode = Mode::Rpn;
                println!("Mode: RPN");
                continue;
            }
            "mode std" => {
                mode = Mode::Std;
                println!("Mode: Standard");
                continue;
            }
            "stack" => {
                if mode == Mode::Rpn {
                    println!("Stack: {:?}", rpn_stack);
                } else {
                    println!("Command only available in RPN mode");
                }
                continue;
            }
            "clear" => {
                rpn_stack.clear();
                println!("Stack cleared");
                continue;
            }
            "history" => {
                if history.is_empty() {
                    println!("No history");
                } else {
                    for entry in &history {
                        println!("  {entry}");
                    }
                }
                continue;
            }
            _ => {}
        }

        match mode {
            Mode::Std => match eval_infix(trimmed) {
                Ok(result) => {
                    let output = format_result(result);
                    println!("{output}");
                    history.push(format!("{trimmed} {output}"));
                }
                Err(e) => println!("{e}"),
            },
            Mode::Rpn => match get_stack(trimmed, &mut rpn_stack) {
                Ok(Some(result)) => {
                    let output = format_result(result);
                    println!("{output}");
                    history.push(format!("{trimmed} {output}"));
                }
                Ok(None) => {}
                Err(e) => println!("{e}"),
            },
        }
    }
}
