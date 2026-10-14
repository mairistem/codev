# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The spec never states that progress still prints without `--quiet`

```yaml
severity: low
confidence: high
location: "specs/cli-import/spec.md#Requirement: `--quiet` silences progress"
evidence: |
  `ledger import --quiet` SHALL print no progress line on standard error; errors SHALL still be printed, and the exit code SHALL be unchanged.
recommendation: Add a scenario for `ledger import data.csv` on 2 500 rows that prints exactly two progress lines and exits 0, so an implementation that silences progress unconditionally fails a test; cover it in task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Where `--quiet` goes on the command line is decided only in the design, and the usage line is not updated

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: "State in the spec whether `ledger import data.csv --quiet` is accepted or rejected (today `src/main.rs:6-9` matches only `[command, path]` and falls to the usage message `ledger import <file>` with exit code 2), add a scenario for it, and include the usage message in task 1.1 so it names `--quiet`."
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 `operations` left unticked with no reason although the change targets nightly jobs' output

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] operations
recommendation: Either tick `operations`, since the change alters what scheduled jobs report by mail, or keep it unticked with the reason on its line (`- [ ] operations — <reason>`).
history:
  - status: open
    by: skeptic
```
