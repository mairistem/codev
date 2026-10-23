# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `AcceptPlan.title` is never read

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:249"
evidence: |
    pub title: String,
recommendation: Remove the field; the CLI builds its summary from the index read back after the write, so a reader looking for where the title is used finds nothing.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/decisions_actions.rs — `title` removed from `AcceptPlan`
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `DecisionNotProposed` branches its message on a magic string

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:137"
evidence: |
            Self::DecisionNotProposed { id, status } if status == "accepted" => format!(
recommendation: Carry the `DecisionStatus` itself in the variant and match `DecisionStatus::Accepted`, so the special case reads as a status, not as a string comparison a reader must trust.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: crates/codev-engine/src/decisions_actions.rs — `DecisionNotProposed.status` is a `DecisionStatus`, matched on `DecisionStatus::Accepted`
  - status: fixed
    by: newcomer
```
