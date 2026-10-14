# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The position of `--quiet` after the file is left unspecified

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: State in the spec what `ledger import data.csv --quiet` does — accepted like `ledger import --quiet data.csv`, or rejected with the usage message and exit code 2 — and add a scenario for it. Today `src/main.rs` matches only `[command, path]`, so a script appending the flag after the file gets the usage error, a behavior users see that neither the requirement nor its scenarios decide.
history:
  - status: open
    by: skeptic
```
