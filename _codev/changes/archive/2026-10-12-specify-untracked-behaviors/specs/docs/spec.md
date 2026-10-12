## ADDED Requirements

### Requirement: Warning callouts stand out from notes in the offline manual

In the HTML that `codev docs` produces, a blockquote that opens on a bold
warning title — `**Warning**` in English, `**Attention**` in French — MUST
be rendered as a warning callout, visually distinct from a note (a coral
rule instead of the green one). Other blockquotes keep the note style.

#### Scenario: Warning and note callouts differ

- **GIVEN** a chapter containing `> **Warning**` and `> **Note**` callouts
- **WHEN** the user runs `codev docs --write out.html`
- **THEN** the warning blockquote carries a distinct style class
- **AND** the note blockquote does not

### Requirement: `codev docs --print` stops silently when its reader exits

When the output of `codev docs --print` is closed before everything is
written (for example `codev docs --print | head`), the command MUST stop
without printing an error and exit with code 0. Any other write error is
reported and exits with a non-zero code.

#### Scenario: Piping into `head`

- **WHEN** the user runs `codev docs --print | head -1`
- **THEN** the first line of the manual is printed
- **AND** nothing is printed on stderr
