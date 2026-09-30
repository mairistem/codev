# Demo for the README, recorded with vhs (https://github.com/charmbracelet/vhs).
#
# Prerequisites: vhs, git, and a `codev` binary on PATH.
# Record from the repository root:
#
#   vhs docs/assets/demo.tape
#
# The output, docs/assets/demo.gif, is referenced from README.md once recorded.

Output docs/assets/demo.gif

Require git
Require codev

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 45ms
Set Theme "Catppuccin Mocha"

# Start from a fresh, empty repository, off camera.
Hide
Type "export PS1='$ ' && rm -rf /tmp/codev-demo && mkdir -p /tmp/codev-demo/acme-app && cd /tmp/codev-demo/acme-app && git init -q && clear"
Enter
Show

Type "codev init --yes --language en"
Sleep 400ms
Enter
Sleep 3s

Type "codev new change add-dark-mode --goal 'Let users switch the UI to a dark theme'"
Sleep 400ms
Enter
Sleep 2500ms

Type "codev status"
Sleep 400ms
Enter
Sleep 3s

# A fresh change has no spec delta yet: validation says what is missing.
Type "codev validate"
Sleep 400ms
Enter
Sleep 4s
