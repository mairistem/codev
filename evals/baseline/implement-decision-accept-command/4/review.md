# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 The refusal message branches on a status string

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:137-140"
evidence: |
            Self::DecisionNotProposed {
                qualified_id,
                status,
            } if status == "accepted" => format!(
recommendation: Keep the status as a `DecisionStatus` in `DecisionNotProposed` and match on `DecisionStatus::Accepted`, so the reader sees an enum case rather than a string literal, and render it with `as_str()` in the generic message.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — `DecisionNotProposed.status` is a `DecisionStatus`, matched on `DecisionStatus::Accepted`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 An empty match arm keeps a matching seal without saying why

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:598"
evidence: |
        Some(existing_entry) if existing_entry.body_sha256 == body_sha256 => {}
recommendation: Add a one-line comment in the arm saying the existing entry already seals this body, so there is nothing to write.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — comment in the matching-seal arm"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `PROPOSED_PATH` also names an accepted decision in the tests

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/commands.rs:1737"
evidence: |
    const PROPOSED_PATH: &str = "/p/_codev/decisions/0001-a-first-choice.md";
recommendation: Rename it `ADR_0001_PATH`; `decision_accept_refuses_an_accepted_decision` reads an accepted decision through it, which a reader would misread as a proposed one.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/commands.rs — constant renamed `ADR_0001_PATH`"
  - status: fixed
    by: newcomer
```
