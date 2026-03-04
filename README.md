# rpncalc

Dual-mode CLI calculator in Rust — infix (Shunting-Yard) and RPN (stack-based).

## Features

- **Infix evaluation** with full operator precedence (Shunting-Yard algorithm)
- **RPN evaluation** with a stack-based engine
- **Interactive REPL** with readline support (rustyline)
- **One-shot mode** via command-line arguments
- Supports `+`, `-`, `*`, `/` and parentheses

## Install

```bash
git clone https://github.com/jojo8356/rpncalc.git
cd rpncalc
cargo build --release
```

## Usage

### Infix (default)

```bash
rpncalc "3 + 4 * 2"
# = 11

rpncalc "(1 + 2) * (3 + 4)"
# = 21
```

### RPN

```bash
rpncalc --rpn "3 4 2 * +"
# = 11
```

### REPL

```bash
rpncalc
> 10 / 3
= 3.3333333333333335
> exit
```

## License

MIT
