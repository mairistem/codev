# Review record: decision-accept-command

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 `DecisionAcceptedV1.body_sha256` is optional although always present

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/contract.rs:229-230"
evidence: |
  #[serde(skip_serializing_if = "Option::is_none")]
  pub body_sha256: Option<String>,
recommendation: >-
  The doc comment says the hash is always present, and failures are emitted
  through `decision_accepted_shape()`, never through this struct — so the
  `Option` and its `skip_serializing_if` make a reader look for a case where
  it is absent that does not exist. Declare it `String` and drop the
  `Some(...)` at the call site; the JSON output stays identical.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/contract.rs, crates/codev-cli/src/main.rs — `body_sha256: String`"
  - status: fixed
    by: newcomer
```
