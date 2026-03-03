Output vhs/demo_oneshot.gif

Set Theme "Catppuccin Mocha"
Set FontFamily "FiraCode Nerd Font"
Set FontSize 20
Set Width 1080
Set Height 420
Set Padding 40
Set TypingSpeed 30ms

Hide
Type "alias rpncalc='./target/release/rpncalc'"
Enter
Sleep 500ms
Type "clear"
Enter
Sleep 500ms
Show

Type `rpncalc "3 + 4 * 2"`
Enter
Sleep 800ms

Type `rpncalc "(3 + 4) * 2"`
Enter
Sleep 800ms

Type `rpncalc "2 ^ 3 ^ 2"`
Enter
Sleep 800ms

Type `rpncalc --rpn "3 4 2 * +"`
Enter
Sleep 800ms

Type `rpncalc --rpn "5 1 2 + 4 * + 3 -"`
Enter
Sleep 1s

Screenshot vhs/demo_oneshot.png
Sleep 500ms
