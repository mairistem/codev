# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 The ADR write is queued before the seal conflict refusal

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:581-584"
evidence: |
  plan.write(entry.path.clone(), accepted, WriteMode::Overwrite);
  match existing_seal.find(&local_id) {
recommendation: Decide the seal entry first (conflict, keep or add), then build the plan, so a reader sees every refusal happen before any write is queued — as the doc comment promises.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — plan_accept decides `new_seal` before building the plan"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 One test name hides a second behavior

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:1616-1633"
evidence: |
  fn plan_accept_refuses_a_leftover_seal_for_another_body() {
recommendation: Move the "leftover seal for the same body is kept" half into its own test named after that behavior, so the test list describes both outcomes.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — new test plan_accept_keeps_a_leftover_seal_for_the_same_body"
  - status: fixed
    by: newcomer
```
