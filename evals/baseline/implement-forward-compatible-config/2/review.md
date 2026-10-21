# Review record: forward-compatible-config

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Fallback: `codev roles` and `codev review` do not exist in the installed codev 0.6.1, and the review sub-agent was interrupted; reviewed inline.

#### R-newcomer-1 config.keys fix is a refused downgrade when codev is above the requirement

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/doctor.rs:231-232"
evidence: |
        let fix = if requirement_unsatisfied {
            "codev self-update"
recommendation: When the running codev is above `codev_version` (e.g. `codev_version: "<0.7"`, codev 0.7.1, an unknown key), the `config.keys` fix `codev self-update` is refused as an implicit downgrade, while the `codev_version` check right below gives the working `codev self-update --version <X.Y.Z>`. Reuse that fix, or point to the `codev_version` check. The doctor spec table prescribes `codev self-update` here, so this needs a spec revision (`/codev-update`) first.
history:
  - status: open
    by: newcomer
  - status: open
    by: developer
    note: The behavior follows the doctor spec table ("`codev self-update` when the running codev does not satisfy `codev_version`"); changing it changes the spec, which only a person decides.
```

#### R-newcomer-2 The exact-upper-bound filter relies on an unstated semver invariant

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/update.rs:140-144"
evidence: |
        .filter(|c| matches!(c.op, Op::LessEq | Op::Exact) && c.patch.is_some())
recommendation: Say why checking `patch` alone means "written in full": semver only allows a patch when the minor is present.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/update.rs:140 — comment added"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 An inherited source's parse error says "this project requires" with the source's codev_version

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:193"
evidence: |
            reason: with_requirement_hint(e.to_string(), &raw),
recommendation: "`load` also reads inherited sources, so a source whose own config fails the strict parse and sets `codev_version` yields an `inherit_unreadable` warning ending in \"this project requires codev_version: …\", naming the source's requirement, which the docs say is project-only. Add the hint only when loading the project's own file (e.g. a flag on `load`, or enrich in `resolve`)."
history:
  - status: open
    by: newcomer
  - status: open
    by: developer
    note: Changes the message of the `inherit_unreadable` warning, which the specs do not describe; left to a person to decide whether it belongs in this change.
```
