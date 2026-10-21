# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### developer

Fallback: `codev roles` and `codev review` are not available in the installed codev; the newcomer review ran inline with the criteria of the skill.

#### R-developer-1 Accept report types an always-present hash as optional

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/contract.rs:361-370"
evidence: |
  /// Shaped like `DecisionCreatedV1`, except that `bodySha256` is always
  /// present: an accepted decision is sealed.
recommendation: Type `body_sha256` as `String` — the failure output goes through `decision_accepted_shape`, never this struct, so the `Option` only misleads a reader about when the hash can be missing. Serialization is unchanged.
history:
  - status: open
    by: developer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/contract.rs — doc comment reworded: the field keeps the `Option` convention of DecisionPromotedV1 and DecisionDeviatedV1, and now states it is always set on success"
  - status: fixed
    by: developer
    note: The misleading doc is gone; keeping `Option` matches the sibling reports, so the reworded comment resolves the confusion without breaking the convention.
```
