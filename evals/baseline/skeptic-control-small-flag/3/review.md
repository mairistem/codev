# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The flag's position is fixed by the design but no scenario covers it elsewhere

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: State in the spec what `ledger import data.csv --quiet` does — accepted like `--quiet data.csv`, or rejected with the usage message and exit code 2 as `src/main.rs` does today for any other argument list — and add a scenario for it, so a nightly script that puts the flag last does not fail without the spec saying so.
history:
  - status: open
    by: skeptic
```
