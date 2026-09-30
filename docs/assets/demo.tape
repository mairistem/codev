# Demo for the README, recorded with vhs (https://github.com/charmbracelet/vhs).
#
# Prerequisites: vhs, git, and a `codev` binary on PATH.
# Record from the repository root:
#
#   vhs docs/assets/demo.tape
#
# The plan of the demo change (proposal, spec delta, design, tasks) lives in
# docs/assets/demo/add-dark-mode/: it stands for what `/codev-propose` writes
# in Claude Code, so the recording needs no model and is reproducible.

Output docs/assets/demo.gif

Require git
Require codev

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 720
Set Padding 24
Set TypingSpeed 40ms
Set Theme "Catppuccin Mocha"

# Start from a fresh, empty repository, off camera.
Hide
Type `export PS1='$ ' LANG=en_US.UTF-8 DEMO_PLAN="$PWD/docs/assets/demo/add-dark-mode" && rm -rf /tmp/acme-app && mkdir /tmp/acme-app && cd /tmp/acme-app && git init -q && clear`
Enter
Show

Type "codev init --yes"
Sleep 300ms
Enter
Sleep 3s

Hide
Type "clear"
Enter
Show

Type "codev new change add-dark-mode"
Sleep 300ms
Enter
Sleep 1500ms

Type "# In Claude Code, /codev-propose now writes the plan: proposal, spec delta, design, tasks."
Sleep 300ms
Enter
Sleep 1500ms

Hide
Type `cp -R "$DEMO_PLAN/." _codev/changes/add-dark-mode/`
Enter
Show

Type "codev status"
Sleep 300ms
Enter
Sleep 2500ms

Hide
Type "clear"
Enter
Show

Type "codev validate"
Sleep 300ms
Enter
Sleep 2500ms

Type "codev archive --change add-dark-mode"
Sleep 300ms
Enter
Sleep 3s

Hide
Type "clear"
Enter
Show

# The delta is now part of the living spec.
Type "cat _codev/specs/ui/theme/spec.md"
Sleep 300ms
Enter
Sleep 5s
