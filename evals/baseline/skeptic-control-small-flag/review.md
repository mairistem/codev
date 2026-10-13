# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 The plan targets CLI files that do not exist in this repository

```yaml
severity: high
confidence: high
location: "proposal.md#Impact"
evidence: |
  - `src/cli/import.rs`, `src/cli/progress.rs`, `docs/cli.md`.
recommendation: Verify where `ledger import` and its progress output actually live (the crate holds only `src/lib.rs`, `src/api.rs` and `src/db.rs`, a library with no CLI and no `docs/` folder) and re-anchor the Impact, the design's Context and the tasks on the real files, or state that the CLI must be created first.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The design rests on an unverified description of the progress and error paths

```yaml
severity: medium
confidence: high
location: "design.md#Context"
evidence: |
  `ledger import` prints one progress line per 1 000 rows on standard error
  through `src/cli/progress.rs`; errors go through `src/cli/errors.rs`, a
  separate path.
recommendation: Confirm that progress and errors really take separate paths in the code (neither `src/cli/progress.rs` nor `src/cli/errors.rs` exists); the decision that errors "cannot be silenced by mistake" holds only if this is true.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Task 1.2 is verified by a test that does not exist and that no task creates

```yaml
severity: medium
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.2 Document `--quiet` in `docs/cli.md`, verified by `cargo test docs_mention_every_flag`
recommendation: Add a task that writes `docs_mention_every_flag`, or name an observable check that exists today; as written the task cannot be verified.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The scenario "nothing is printed" is broader than the requirement

```yaml
severity: low
confidence: high
location: "specs/cli-import/spec.md#Scenario: A quiet import"
evidence: |
  - **THEN** nothing is printed and the exit code is 0
recommendation: Align the scenario with the requirement ("no progress line on standard error"), or state that standard output is also silent; otherwise a summary line on stdout, or a warning, would make the test fail or the requirement ambiguous.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 Warnings are classified neither as progress nor as errors

```yaml
severity: low
confidence: low
location: "specs/cli-import/spec.md#Requirement: `--quiet` silences progress"
evidence: |
  `ledger import --quiet` SHALL print no progress line on standard error; errors SHALL still be printed, and the exit code SHALL be unchanged.
recommendation: Say whether non-fatal messages (skipped rows, warnings) are silenced by `--quiet`; nightly scripts likely still want them, and the spec covers only success and a fatal error.
history:
  - status: open
    by: skeptic
```
