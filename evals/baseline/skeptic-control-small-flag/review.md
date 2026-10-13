# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Task 1.2's check passes even if `--quiet` is never declared

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.2 Document `--quiet` in `docs/cli.md`, verified by `cargo test docs_mention_every_flag`
recommendation: >-
  Make task 1.1 add "--quiet" to `FLAGS` in `src/cli/import.rs` (today
  `pub const FLAGS: &[&str] = &[];`). `tests/docs.rs` only loops over
  `ledger::cli::import::FLAGS`, so while that list stays empty the test
  passes whether or not `docs/cli.md` mentions `--quiet`.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The "A quiet import" scenario passes without `--quiet` on a small file

```yaml
severity: medium
confidence: high
location: "specs/cli-import/spec.md#Scenario: A quiet import"
evidence: |
  - **WHEN** the user runs `ledger import --quiet data.csv` and the import succeeds
  - **THEN** no progress line is printed and the exit code is 0
recommendation: >-
  Say in the scenario that `data.csv` has at least 1 000 data rows.
  `Progress::row` only prints every 1 000 rows (`every: 1000`), so with a
  smaller file the test `a_quiet_import_prints_nothing` passes even when
  `--quiet` does nothing.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The plan does not say how `--quiet` gets from the command line to `run`

```yaml
severity: low
confidence: low
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `Progress::new(quiet)` does nothing when `quiet` is set; errors keep their
recommendation: >-
  Say how `quiet` reaches `import::run`. Its signature today is
  `pub fn run(path: &str) -> i32`, and the crate has no argument parser or
  binary (no `src/main.rs`). Either change the signature or name the
  caller that parses argv, and add that file to Impact if it is in this
  repository.
history:
  - status: open
    by: skeptic
```
