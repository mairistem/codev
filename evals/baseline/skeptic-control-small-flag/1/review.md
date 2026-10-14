# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 `--quiet` after the file is rejected, and no scenario says so

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  The design accepts the flag only before the file, and `src/main.rs` matches
  the arguments by position (`[command, path] if command == "import"`), so
  `ledger import data.csv --quiet` falls into the usage branch and exits 2
  without importing. Either accept the flag in any position, or state the
  order in the requirement and add a scenario for the flag placed after the
  file (and for an unknown flag), with a test in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The usage line printed by `ledger` is not updated

```yaml
severity: low
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.2 Document `--quiet` in `docs/cli.md`, verified by `cargo test docs_mention_every_flag`
recommendation: >-
  The only in-program help, `eprintln!("usage: ledger import <file>");` in
  `src/main.rs:8`, keeps omitting the new flag, and the docs test checks
  `docs/cli.md` alone. Add the usage line to task 1.1 or 1.2 (for example
  `usage: ledger import [--quiet] <file>`), or state that it stays unchanged.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Every domain is unticked with no reason

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] user-interface
  - [ ] performance
  - [ ] operations
recommendation: >-
  The change adds a flag to the public command line and changes the output
  of nightly jobs, which touches `user-interface` (the CLI contract) and
  possibly `operations` (what the nightly reports show). No suggestion was
  raised from the diff, but an unticked box carries no reason. Tick the
  domains the change touches, or give each unticked one a reason on its line.
history:
  - status: open
    by: skeptic
```
