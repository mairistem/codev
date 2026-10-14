# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 Flag position is unspecified; the design rejects `--quiet` after the file

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  State in the spec where `--quiet` may appear and add a scenario for the other
  position (`ledger import data.csv --quiet`): either accept it, or name the
  outcome (today `src/main.rs:6-10` falls to the usage error, exit code 2, and
  nothing is imported). A nightly script that appends the flag would otherwise
  stop importing, which is worse than the noise the Why complains about.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 The usage line is not updated to mention `--quiet`

```yaml
severity: low
confidence: high
location: "tasks.md#1. Flag"
evidence: |
  - [ ] 1.1 `--quiet` read by `src/main.rs`, added to `FLAGS` in `src/cli/import.rs` and passed to `Progress::new(quiet)`
recommendation: >-
  `src/main.rs:8` prints `usage: ledger import <file>`; no task or decision
  changes it, so the CLI's own help would keep hiding the new flag while
  `docs/cli.md` documents it. Add the update of the usage line to task 1.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains all unticked with no reason

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] user-interface
recommendation: >-
  The change alters the command line's user-visible output and the output of
  scheduled jobs (the Why: "Nightly scripts run `ledger import`"). Either tick
  `user-interface` / `operations`, or keep them unticked with the reason on
  their line (`- [ ] <domain> — <reason>`), so that no expert's domain is left
  to nobody silently.
history:
  - status: open
    by: skeptic
```
