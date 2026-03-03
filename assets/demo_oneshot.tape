Output assets/demo_oneshot.gif

Set Shell "zsh"
Set FontFamily "FiraCode Nerd Font"
Set FontSize 16
Set Width 900
Set Height 400
Set Theme "Dracula"
Set Padding 20

Type "# Mode one-shot infixe"
Enter
Sleep 500ms

Type `cargo run -- "3 + 4 * 2"`
Enter
Sleep 2s

Type `cargo run -- "(3 + 4) * 2"`
Enter
Sleep 2s

Type `cargo run -- "2 ^ 3 ^ 2"`
Enter
Sleep 2s

Type "# Mode one-shot RPN"
Enter
Sleep 500ms

Type `cargo run -- --rpn "3 4 2 * +"`
Enter
Sleep 2s

Type `cargo run -- --rpn "5 1 2 + 4 * + 3 -"`
Enter
Sleep 3s
