# Review record: replayed-history-stays-clean

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 rustfmt given a path also formats the module files it declares

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Only the Rust files the rename changed are formatted, with rustfmt"
evidence: |
  `rustfmt --edition <edition> --emit stdout` on standard input, run from
recommendation: 'Format each changed file through standard input, which does not follow `mod x;`, and declare the untouched file as a module of the renamed one in the test.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md decision on rustfmt; specs scenario A file the rename does not touch; tasks.md 1.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The edition and the files differ from what cargo fmt formats

```yaml
severity: low
confidence: low
location: "specs/implementation-evaluation/spec.md#Requirement: A tree replayed from before the rename stays as clean as it was"
evidence: |
  format with rustfmt each `.rs` file the rename changed that a crate
recommendation: 'Use each crate''s edition and only the files cargo fmt formats, or say the limitation is accepted.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md: crate edition, target files only; risk recorded"
  - status: fixed
    by: skeptic
```

## code.review

Mode: isolated.

### newcomer

#### R-newcomer-1 edition_of hid its rules behind a flag and a sentinel string

```yaml
severity: medium
confidence: high
location: "evals/src/runner.rs:2660"
evidence: |
  fn edition_of(dir: &Path, file: &Path) -> String {
recommendation: 'Read the edition with a named function returning an Edition enum (Set or FromWorkspace), a DEFAULT_EDITION constant, and say why each fallback.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs: Edition, edition_line, DEFAULT_EDITION; a manifest without edition takes 2021, as cargo"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The seal and rustfmt checks sit in the pinned-case test

```yaml
severity: low
confidence: high
location: "evals/tests/fixture.rs:368"
evidence: |
  fn a_case_pinned_before_the_rename_is_migrated_before_it_runs() {
recommendation: 'Move the seal and the rustfmt fixtures to tests of their own calling bring_through_rename directly.'
history:
  - status: open
    by: newcomer
  - status: disputed
    by: developer
    note: 'tasks.md 1.1 and 1.2 extend this test on purpose: it drives setup_plan, so it shows the reseal and the format land in the fixture''s base commit, which a direct call of bring_through_rename would not; the edge cases (unparsable seal, current names, no rustfmt) have tests of their own.'
  - status: rejected
    by: newcomer
```
