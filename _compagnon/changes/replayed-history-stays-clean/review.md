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
