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
  The design accepts the flag only before the file, and `src/main.rs` matches
  the arguments by exact position (`[command, path] if command == "import"`),
  so `ledger import data.csv --quiet` falls to the usage branch and exits 2
  without importing — a nightly script that writes the flag last fails instead
  of running quietly. Either accept `--quiet` in any position, or keep the
  single position and add a scenario to `specs/cli-import/spec.md` stating what
  happens when it comes after the file (usage error, exit code 2), with the
  usage line naming `--quiet`.
history:
  - status: open
    by: skeptic
```
