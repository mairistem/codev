# Demo for the README, recorded with vhs (https://github.com/charmbracelet/vhs)
# in the Mairistem brand colors.
#
# Do not run this tape directly: run docs/assets/record-demo.sh, which records
# it and then stamps the Mairistem logo on the frame (vhs cannot draw images).
#
# The plan of the demo change (proposal, spec delta, design, tasks) lives in
# docs/assets/demo/add-dark-mode/: it stands for what `/codev-propose` writes
# in Claude Code, so the recording needs no model and is reproducible.

Output docs/assets/demo-raw.gif

Require git
Require codev

Set Shell "bash"
Set FontSize 16
Set Width 1280
Set Height 880
Set Padding 20
Set Margin 64
Set MarginFill "#004C66"
Set WindowBar Colorful
Set WindowBarSize 36
Set BorderRadius 12
Set TypingSpeed 40ms
# Terminal colors from the Mairistem palette: deep-blue card, white text,
# coral / green / yellow accents.
Set Theme { "name": "Mairistem", "background": "#00394D", "foreground": "#FFFFFF", "cursor": "#FFAB03", "selection": "#004C66", "black": "#00394D", "red": "#FF5331", "green": "#00C39B", "yellow": "#FFAB03", "blue": "#9CBAC4", "magenta": "#FF5331", "cyan": "#00C39B", "white": "#E7EDEF", "brightBlack": "#6E858D", "brightRed": "#FF5331", "brightGreen": "#00C39B", "brightYellow": "#FFAB03", "brightBlue": "#B9CDD3", "brightMagenta": "#FF5331", "brightCyan": "#00C39B", "brightWhite": "#FFFFFF" }

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
