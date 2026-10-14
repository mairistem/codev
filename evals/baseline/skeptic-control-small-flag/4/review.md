# Review record: add-quiet-flag

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 The position of `--quiet` on the command line is left unspecified

```yaml
severity: low
confidence: high
location: "design.md#Decision: The progress reporter takes a `quiet` switch"
evidence: |
  `src/main.rs` accepts `--quiet` before the file and passes it to
recommendation: >-
  The requirement and its scenarios only cover `ledger import --quiet <file>`.
  `src/main.rs` matches the arguments by exact shape (`[command, path] if command == "import"`),
  so a script that writes `ledger import data.csv --quiet` would, as designed, fall
  into the usage branch and exit 2 without importing anything. Either state in the
  requirement where the flag is accepted and add a scenario for the other placement
  (accepted, or rejected with the usage error and exit code 2), or accept it in both
  positions and say so.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 No domain is ticked or justified in the proposal

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] user-interface
  - [ ] performance
  - [ ] operations
recommendation: >-
  The change adds a user-visible option to the `ledger import` command line and
  targets nightly scripts' mail reports, yet every domain is left unticked with
  no reason on its line. Whether a CLI flag falls under `user-interface` and a
  scheduled job's output under `operations` depends on how the project defines
  those domains; tick them so their experts review the plan, or keep them
  unticked with a reason (`- [ ] operations — <why>`).
history:
  - status: open
    by: skeptic
```
