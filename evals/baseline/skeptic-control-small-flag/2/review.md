# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The flag's position on the command line is left unspecified

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  State in the spec what `ledger import data.csv --quiet` (flag after the file) does, and add a scenario
  for it: either it is accepted like `--quiet data.csv`, or it is a usage error with exit code 2. Today
  `src/main.rs` matches the arguments positionally (`[command, path] if command == "import"`), so a
  nightly script that writes the flag after the file would get the usage line and exit 2 instead of a
  quiet import; the spec's only scenarios put the flag before the file.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Task 1.1 adds the flag to `FLAGS` before its documentation exists, leaving the docs test red

```yaml
severity: low
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.1 `--quiet` read by `src/main.rs`, added to `FLAGS` in `src/cli/import.rs` and passed to `Progress::new(quiet)`, verified by tests `a_quiet_import_prints_nothing` ("A quiet import") and `a_quiet_import_still_prints_errors` ("A quiet import that fails")
recommendation: >-
  Move the `FLAGS` entry into task 1.2 with the documentation, or have task 1.1's verification include
  `cargo test docs_mention_every_flag`. As ordered, once 1.1 is done `tests/docs.rs` iterates over
  `FLAGS`, finds `--quiet` absent from `docs/cli.md` and fails, so 1.1 cannot be checked as done with a
  green test suite.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The usage line printed by `src/main.rs` is not updated to mention `--quiet`

```yaml
severity: low
confidence: high
location: "src/main.rs:8"
evidence: |
          eprintln!("usage: ledger import <file>");
recommendation: >-
  Add to task 1.1 updating the usage line to `usage: ledger import [--quiet] <file>`, so that a user who
  gets the usage error (for instance by writing the flag in an unaccepted position, see R-skeptic-1)
  learns the flag exists; the plan updates `docs/cli.md` but not the only help the binary itself prints.
history:
  - status: open
    by: skeptic
```
