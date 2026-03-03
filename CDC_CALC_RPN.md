# Cahier des Charges - Calculatrice Rust avec Notation Polonaise Inverse

## 1. Presentation du projet

**Nom du projet** : `rpncalc`
**Langage** : Rust
**Type** : Application CLI (terminal)

Calculatrice en ligne de commande supportant deux modes de saisie :
- **Mode classique** (infixe) : `3 + 4 * 2`
- **Mode RPN** (notation polonaise inverse / postfixe) : `3 4 2 * +`

---

## 2. Objectifs

- Fournir une calculatrice fiable et rapide en Rust
- Supporter la notation infixe classique et la notation polonaise inverse (RPN)
- Fonctionner en mode interactif (REPL) et en mode one-shot (argument CLI)
- Gerer les erreurs proprement (division par zero, syntaxe invalide, etc.)

---

## 3. Fonctionnalites

### 3.1 Operations supportees

| Operateur | Description         | Priorite (infixe) |
|-----------|---------------------|--------------------|
| `+`       | Addition            | 1                  |
| `-`       | Soustraction        | 1                  |
| `*`       | Multiplication      | 2                  |
| `/`       | Division            | 2                  |
| `%`       | Modulo              | 2                  |
| `^`       | Puissance           | 3                  |
| `(`, `)`  | Parentheses (infixe)| -                  |

### 3.2 Modes de fonctionnement

#### Mode REPL (interactif)
```
$ rpncalc
rpncalc> 3 + 4
= 7
rpncalc> mode rpn
Mode: RPN
rpncalc> 5 3 + 2 *
= 16
rpncalc> quit
```

#### Mode one-shot
```
$ rpncalc "3 + 4 * 2"
= 11
$ rpncalc --rpn "3 4 2 * +"
= 11
```

### 3.3 Commandes REPL

| Commande   | Description                              |
|------------|------------------------------------------|
| `mode rpn` | Bascule en mode RPN                      |
| `mode std` | Bascule en mode infixe (defaut)          |
| `stack`    | Affiche la pile courante (mode RPN)      |
| `clear`    | Vide la pile                             |
| `history`  | Affiche l'historique des calculs         |
| `quit`     | Quitte le programme                      |

### 3.4 Types numeriques

- Nombres entiers : `42`, `-7`
- Nombres decimaux : `3.14`, `-0.5`
- Representation interne en `f64`

---

## 4. Architecture

```
rpncalc/
├── Cargo.toml
├── src/
│   ├── main.rs          # Point d'entree, CLI args, boucle REPL
│   ├── lib.rs           # Re-exports publics
│   ├── lexer.rs         # Tokenisation de l'input
│   ├── parser.rs        # Parsing infixe -> AST (Shunting-Yard)
│   ├── rpn.rs           # Evaluateur RPN (pile)
│   ├── eval.rs          # Evaluation de l'AST
│   ├── error.rs         # Types d'erreurs
│   └── repl.rs          # Boucle interactive
└── tests/
    ├── infix_tests.rs
    └── rpn_tests.rs
```

### 4.1 Pipeline de traitement

**Mode infixe** :
```
Input -> Lexer -> Tokens -> Parser (Shunting-Yard) -> AST -> Eval -> Resultat
```

**Mode RPN** :
```
Input -> Lexer -> Tokens -> Evaluateur RPN (pile) -> Resultat
```

### 4.2 Structures principales

```rust
// Tokens produits par le lexer
enum Token {
    Number(f64),
    Operator(Op),
    LeftParen,
    RightParen,
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

// AST pour le mode infixe
enum Expr {
    Num(f64),
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}
```

---

## 5. Gestion des erreurs

| Erreur                    | Message                                    |
|---------------------------|--------------------------------------------|
| Division par zero         | `Erreur: division par zero`                |
| Syntaxe invalide          | `Erreur: expression invalide`              |
| Parentheses non fermees   | `Erreur: parenthese manquante`             |
| Pile insuffisante (RPN)   | `Erreur: operandes insuffisants sur la pile` |
| Token inconnu             | `Erreur: caractere inconnu '{c}'`          |

---

## 6. Arguments CLI

```
rpncalc [OPTIONS] [EXPRESSION]

Arguments:
  [EXPRESSION]    Expression a evaluer (mode one-shot)

Options:
  --rpn           Evalue en notation polonaise inverse
  -h, --help      Affiche l'aide
  -V, --version   Affiche la version
```

Crate suggeree : `clap` pour le parsing des arguments.

---

## 7. Dependances

| Crate       | Usage                        |
|-------------|------------------------------|
| `clap`      | Parsing des arguments CLI    |
| `rustyline` | Ligne de commande interactive (historique, edition) |

Aucune autre dependance externe requise. Le lexer, le parser et l'evaluateur sont implementes from scratch.

---

## 8. Tests

### 8.1 Tests unitaires
- Lexer : tokenisation correcte de chaque type de token
- Parser infixe : respect de la priorite des operateurs et des parentheses
- Evaluateur RPN : evaluation correcte avec pile
- Gestion des erreurs : chaque cas d'erreur produit le bon type

### 8.2 Cas de test

```
// Infixe
"3 + 4"           -> 7.0
"3 + 4 * 2"       -> 11.0
"(3 + 4) * 2"     -> 14.0
"2 ^ 3 ^ 2"       -> 512.0  (associativite droite)
"10 / 0"          -> Erreur
"10 % 3"          -> 1.0

// RPN
"3 4 +"           -> 7.0
"3 4 2 * +"       -> 11.0
"5 1 2 + 4 * + 3 -" -> 14.0
"+"               -> Erreur (pile insuffisante)
```

---

## 9. Contraintes techniques

- **Rust edition** : 2021
- **MSRV** : 1.70+
- Pas de `unsafe`
- `clippy` sans warnings
- Formattage avec `rustfmt`

---

## 10. Extensions futures (hors scope v1)

- Support des variables (`x = 5`, puis `x + 3`)
- Fonctions mathematiques (`sin`, `cos`, `sqrt`, `log`)
- Mode fichier : lire des expressions depuis un fichier
- Historique persistant (sauvegarde sur disque)
