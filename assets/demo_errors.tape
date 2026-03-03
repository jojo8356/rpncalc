Output assets/demo_errors.gif

Set Shell "zsh"
Set FontFamily "FiraCode Nerd Font"
Set FontSize 16
Set Width 900
Set Height 400
Set Theme "Dracula"
Set Padding 20

Type "# Gestion des erreurs"
Enter
Sleep 500ms

Type `cargo run -- "10 / 0"`
Enter
Sleep 2s

Type `cargo run -- "(3 + 4"`
Enter
Sleep 2s

Type `cargo run -- --rpn "+"`
Enter
Sleep 2s

Type `cargo run -- "3 & 4"`
Enter
Sleep 3s
