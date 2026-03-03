use clap::Parser;

use rpncalc::eval::eval;
use rpncalc::lexer::tokenize;
use rpncalc::parser::parse;
use rpncalc::repl::run_repl;
use rpncalc::rpn::eval_rpn;

#[derive(Parser)]
#[command(name = "rpncalc", version, about = "CLI calculator with RPN support")]
struct Cli {
    /// Expression to evaluate (one-shot mode)
    expression: Option<String>,

    /// Evaluate in Reverse Polish Notation
    #[arg(long)]
    rpn: bool,
}

fn format_result(value: f64) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("= {}", value as i64)
    } else {
        format!("= {value}")
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.expression {
        Some(expr) => {
            let result = if cli.rpn {
                eval_rpn(&expr)
            } else {
                let tokens = match tokenize(&expr) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
                let ast = match parse(&tokens) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("{e}");
                        std::process::exit(1);
                    }
                };
                eval(&ast)
            };

            match result {
                Ok(value) => println!("{}", format_result(value)),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
        None => run_repl(),
    }
}
