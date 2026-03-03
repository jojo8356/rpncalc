# LinkedIn Post — rpncalc

I built **rpncalc** — a dual-mode CLI calculator in Rust that handles both standard math and Reverse Polish Notation, from scratch.

The idea: one tool that parses infix with a **Shunting-Yard algorithm** and evaluates RPN with a pure **stack machine** — no eval crate, no shortcuts.

**What it does:**
- Parses infix expressions with full operator precedence and parentheses
- Evaluates RPN using a **15-line** stack-based engine
- Handles right-associativity for power (`2^3^2 = 512`, not 64)
- Switches between infix and RPN mode live in the REPL
- Supports **6 operators**: `+`, `-`, `*`, `/`, `%`, `^`
- Tracks calculation history across both modes
- **43 tests** across **8 modules** — zero clippy warnings, no unsafe

**Built with:** Rust, clap, rustyline

Source: [lien GitHub]

Feedback welcome — do you use RPN in your daily workflow, or is infix the only way?

`#Rust` `#CLI` `#OpenSource` `#DevTools` `#BuildInPublic` `#Algorithms` `#RPN`

---

## Alternative Hook

I implemented the **Shunting-Yard algorithm** from scratch in Rust — rpncalc is a CLI calculator that speaks both infix and RPN, backed by **43 tests**.

---

## Images (ordre recommandé pour LinkedIn)

| # | Fichier | Outil | Description |
|---|---------|-------|-------------|
| 1 | `vhs/shunting_yard.png` | silicon | Code du parser Shunting-Yard |
| 2 | `vhs/rpn_eval.png` | silicon | Évaluateur RPN à pile |
| 3 | `vhs/demo_oneshot.png` | vhs | Terminal one-shot : infixe + RPN |
| 4 | `vhs/demo_repl.png` | vhs | REPL interactif avec mode switch + history |

## Timing recommandé

Mardi-jeudi, 8h-10h CET
