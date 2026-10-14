# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The spec leaves `--quiet` after the file undefined, and the planned parser rejects it

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  Add a scenario to `specs/cli-import/spec.md` for `ledger import data.csv --quiet`
  (accepted, or rejected with a stated error and exit code), and make the design's
  parsing in `src/main.rs` match it. Today `src/main.rs:6` only matches
  `[command, path]`, so with the design as written `ledger import data.csv --quiet`
  falls to the `_` arm, prints the usage line and exits 2 without importing: a
  nightly script that puts the flag last skips its import that night. Cover the
  chosen behaviour with a test in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The usage line keeps omitting `--quiet`

```yaml
severity: low
confidence: high
location: "src/main.rs:8"
evidence: |
  eprintln!("usage: ledger import <file>");
recommendation: >-
  Add to task 1.1 (or 1.2) updating the usage line, and the `docs/cli.md`
  heading `ledger import <file>`, to show `[--quiet]`; the design only adds the
  flag to `FLAGS`, which `docs_mention_every_flag` checks against `docs/cli.md`
  alone, so nothing catches a stale usage message.
history:
  - status: open
    by: skeptic
```
