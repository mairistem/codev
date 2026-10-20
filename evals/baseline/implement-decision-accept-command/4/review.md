# Review record: decision-accept-command
<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `decision new` points a rejected or deprecated decision to `accept`

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/render.rs:454-473"
evidence: |
  if outcome.body_sha256.is_some() {
recommendation: Show the `codev decision accept` hint only for a `proposed` decision; `codev decision new X --status rejected` otherwise prints a next step that fails with `decision_not_proposed`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/render.rs — the hint is limited to `proposed`; test decision_new_points_only_a_proposed_decision_to_accept"
  - status: fixed
    by: newcomer
```
