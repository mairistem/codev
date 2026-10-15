# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The spec does not say where `--quiet` goes, and the parser accepts it only before the file

```yaml
severity: low
confidence: high
location: "specs/cli-import/spec.md#Requirement: `--quiet` silences progress"
evidence: |
  `ledger import --quiet` SHALL print no progress line on standard error; errors SHALL still be printed, and the exit code SHALL be unchanged.
recommendation: >-
  State in the requirement where the flag is accepted, and add a scenario for
  the other position. The design has `src/main.rs` accept "`--quiet` before the
  file", and `src/main.rs:6` matches an exact slice
  (`[command, path] if command == "import"`), so `ledger import data.csv --quiet`
  would fall to the usage branch and exit 2 — a changed exit code, which the
  requirement forbids, for a script that puts the flag last. Either accept the
  flag in both positions or specify the usage error as the expected outcome.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 `Progress::new(quiet)` "does nothing", but the lines are printed by `row()`

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `Progress::new(quiet)` does nothing when `quiet` is set;
recommendation: >-
  Say that `Progress::row` prints nothing when the reporter was built quiet.
  `Progress::new` prints nothing today (`src/cli/progress.rs:12-14`); the line
  is written in `row()` (`src/cli/progress.rs:20`), so the decision as worded
  describes no change to the code that prints.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The usage message is not part of the plan

```yaml
severity: low
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.1 `--quiet` read by `src/main.rs`, added to `FLAGS` in `src/cli/import.rs` and passed to `Progress::new(quiet)`, verified by tests `a_quiet_import_prints_nothing` ("A quiet import") and `a_quiet_import_still_prints_errors` ("A quiet import that fails")
recommendation: >-
  Include updating the usage line printed by `src/main.rs:8`
  (`eprintln!("usage: ledger import <file>");`) to show `[--quiet]`, so a
  wrong invocation points at the new flag.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Every domain is unticked with no reason

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] user-interface
  - [ ] performance
  - [ ] operations
recommendation: >-
  The change adds a flag to a command scripts call and changes what nightly
  jobs log; whether `user-interface` (the CLI surface) or `operations` (the
  nightly scripts' reports) applies depends on how the project scopes those
  domains. Tick them, or give the reason on their line, so no expert's review
  is skipped silently.
history:
  - status: open
    by: skeptic
```
