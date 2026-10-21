# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### developer

Fallback: `codev roles` and `codev review` are unavailable in the installed codev 0.6.1; the developer (newcomer) role was applied inline.

#### R-developer-1 A failed seal write leaves an accepted, unsealed decision that `accept` cannot finish

```yaml
severity: medium
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:612"
evidence: |
  plan.write(entry.path.clone(), accepted, WriteMode::Overwrite);
recommendation: >-
  `apply::execute` runs the writes in order with no rollback, so the plan's
  order decides the half-applied state. With the ADR first, a failed
  `seal.yaml` write (write-locked file, full disk) leaves `status: accepted`
  without a seal, and re-running `decision accept` is refused with
  `decision_not_proposed`. Write the seal first: a failed ADR write then
  leaves a seal on a still `proposed` decision, which a re-run of
  `decision accept` completes through the matching-seal branch. A real
  transaction in `apply::execute` is shared with `decision new` and outside
  this change.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs — seal written before the ADR; test plan_accept_writes_the_seal_before_the_adr_so_a_retry_completes (fails with the old order)"
  - status: fixed
    by: developer
```
