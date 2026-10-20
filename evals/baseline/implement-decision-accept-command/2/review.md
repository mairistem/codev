# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Fallback: `codev roles` and `codev review` are not available in the installed codev (0.6.1); the newcomer's review was applied inline from the skill's description, and no domain expert is activated (the proposal has no `## Domains`).

#### R-newcomer-1 The ADR is written before its seal, so a failed seal write leaves an accepted, unsealed decision

```yaml
severity: medium
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:591-596"
evidence: |
  plan.write(entry.path.clone(), accepted, WriteMode::Overwrite);
  plan.write(
      layout.decisions_seal_file(),
recommendation: >-
  `apply::execute` runs the writes in order and does not roll back, so the
  docstring's "the decision is never accepted without its seal" only holds
  if the seal is written first. With the ADR first, a seal write that fails
  (a write-locked `seal.yaml`) leaves `status: accepted` with no seal, and a
  retry of `decision accept` is then refused with `decision_not_proposed`.
  Write `seal.yaml` first: an interrupted run then leaves a seal entry that
  matches the body of a still `proposed` decision, which `plan_accept`
  already keeps, so a retry completes the acceptance. Test the write order
  and the retry.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — plan_accept writes seal.yaml before the ADR; tests plan_accept_writes_the_seal_before_the_adr and plan_accept_completes_after_a_run_interrupted_after_the_seal (both fail without the fix)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `DecisionNotProposed` picks its message by comparing a status string

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:137"
evidence: |
  Self::DecisionNotProposed { id, status } if status == "accepted" => format!(
recommendation: >-
  Carry the `DecisionStatus` in the variant and match on
  `DecisionStatus::Accepted`, so the message dispatch is checked by the
  compiler rather than by a string literal.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — DecisionNotProposed carries a DecisionStatus, matched on DecisionStatus::Accepted"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 Accepting re-renders the frontmatter, dropping YAML comments a user wrote in it

```yaml
severity: low
confidence: low
location: "crates/codev-engine/src/decisions_actions.rs:573"
evidence: |
  let accepted = rewrite_frontmatter_status(&source, &entry.decision, DecisionStatus::Accepted);
recommendation: >-
  `rewrite_frontmatter_status` regenerates the frontmatter from the parsed
  fields; unknown keys are already rejected by the parser, but a YAML comment
  in a hand-written proposed decision would be lost on accept. Whether users
  put comments there is not known; a targeted rewrite of the `status:` line
  would keep them, for `supersede` too, in a later change.
history:
  - status: open
    by: newcomer
```
