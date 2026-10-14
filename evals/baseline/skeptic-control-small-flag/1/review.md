# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 `--quiet` after the file is a usage error, and no scenario says so

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  State in the spec where `--quiet` may appear and what happens otherwise:
  either accept it before or after the file, or add a scenario where
  `ledger import data.csv --quiet` exits 2 with a usage line. Update the
  usage line in `src/main.rs:8` (`usage: ledger import <file>`) to mention
  `--quiet`, so a script that puts the flag last learns why it failed.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Task 1.1 breaks `docs_mention_every_flag` until task 1.2 is done

```yaml
severity: low
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.1 `--quiet` read by `src/main.rs`, added to `FLAGS` in `src/cli/import.rs` and passed to `Progress::new(quiet)`, verified by tests `a_quiet_import_prints_nothing` ("A quiet import") and `a_quiet_import_still_prints_errors` ("A quiet import that fails")
recommendation: >-
  Adding `--quiet` to `FLAGS` makes `tests/docs.rs` fail (`docs/cli.md does
  not mention --quiet`) as soon as 1.1 is done, so the suite is red between
  the two tasks. Document the flag in the same task, or move task 1.2 first.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 No scenario keeps the progress lines when `--quiet` is absent

```yaml
severity: low
confidence: high
location: "specs/cli-import/spec.md#Requirement: `--quiet` silences progress"
evidence: |
  `ledger import --quiet` SHALL print no progress line on standard error; errors SHALL still be printed, and the exit code SHALL be unchanged.
recommendation: >-
  Add a scenario where `ledger import data.csv` on 2 500 rows still prints
  two progress lines, and a test for it in task 1.1, so that an
  implementation that silences progress unconditionally does not pass.
history:
  - status: open
    by: skeptic
```
