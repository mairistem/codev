# Review record: forward-compatible-config
<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

#### R-newcomer-1 The `config.keys` fix suggests a refused upgrade when codev is above the requirement

```yaml
severity: medium
confidence: high
location: "crates/codev-core/src/doctor.rs:219"
evidence: |
  let requirement_unsatisfied = matches!(version.requirement, Some((_, false)));
recommendation: >-
  With `codev_version: "<0.7"`, codev 0.7.1 running and a misspelled
  top-level key, `config.keys` gives the fix `codev self-update`: the keys
  cannot come from a newer codev (the running one is already above the
  requirement), and `codev self-update` refuses the implicit downgrade —
  the very defect this change fixes for the `codev_version` check. Base
  the fix on `requirement_side(...) == Some(Below)` instead of "not
  satisfied". The doctor spec currently says "`codev self-update` when the
  running codev does not satisfy `codev_version`", so this is a behavior
  change: amend the spec through `/codev-update` first, then the code.
history:
  - status: open
    by: newcomer
  - status: open
    by: developer
    note: >-
      The implementation follows the doctor spec's `config.keys` row word
      for word; changing it changes specified behavior, which only the
      person owning the plan can decide.
```

#### R-newcomer-2 The known-key test depends on the order of the struct's fields

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:737"
evidence: |
  sorted(struct_fields::<ProjectConfig>()),
recommendation: >-
  `every_known_key_is_recognized` compared serde's field list with
  `PROJECT_KEYS` in order, so a field added in the middle of the struct
  but appended to the list would fail with a confusing diff. Compare the
  two lists sorted: only membership matters.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs — every_known_key_is_recognized compares sorted lists"
  - status: fixed
    by: newcomer
```
