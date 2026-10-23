# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 Refusal message branches on a status string

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:139"
evidence: |
  Self::DecisionNotProposed { id, status } if status == "accepted" => format!(
recommendation: Hold the status as a `DecisionStatus` in `DecisionNotProposed` and match on `DecisionStatus::Accepted`, so the branch reads as a domain case instead of a magic string.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — `DecisionNotProposed.status` is a `DecisionStatus`, the message matches `DecisionStatus::Accepted`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Empty match arm for a matching leftover seal reads as a forgotten case

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:586"
evidence: |
  Some(leftover) if leftover.body_sha256 == body_sha256 => {}
recommendation: Say in the arm why nothing is written — the recorded seal already matches the body, so only the status changes — so a reader does not have to go back to the doc comment.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — comment on the matching-seal arm"
  - status: fixed
    by: newcomer
```
