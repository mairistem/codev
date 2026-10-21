# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — the material exceeded the review budget (273604 of 163840 bytes).
Summarized: docs/en/src/reference/cli.md — the material exceeded the review budget (273604 of 163840 bytes).
Summarized: docs/fr/src/reference/cli.md — the material exceeded the review budget (273604 of 163840 bytes).

### migration

No finding.

### newcomer

#### R-newcomer-1 The hand-edit decision is a nested closure that strips a checksum the skill does not have

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:248-265"
evidence: |
                            None => {
                                let legacy = without_checksum(&current);
                                [&self.ctx, &self.ctx.other_review_mode()]
                                    .iter()
                                    .all(|ctx| {
                                        legacy
                                            != Self::render_without_checksum(workflow, version, ctx)
                                    })
recommendation: Move the decision into a named method with early returns (identical to the rendering, another stamp, checksum present, legacy comparison), so each rule of the comment sits next to its line; drop the `without_checksum(&current)` call in the `None` arm, a no-op since `None` means the skill has no checksum line; and say "matches neither review mode's rendering" positively with `!any(==)` rather than `all(!=)`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — the decision moved to `ClaudeCode::is_edited_by_hand`, with early returns, without the no-op `without_checksum` call, with `!any(==)`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The test helper repeats the checksum insertion with a hard-coded version

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:636-641"
evidence: |
          let checksum = sha256_hex(unchecked.as_bytes());
          unchecked.replacen(
              "  version: \"0.1.0\"\n",
              &format!("  version: \"0.1.0\"\n  checksum: \"{checksum}\"\n"),
              1,
          )
recommendation: Extract the insertion of the checksum line from `render` into one function, `with_checksum(unchecked, version)`, and call it from the test helper with `VERSION`, removing the copy and the literal "0.1.0" a reader may take for an earlier version.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `with_checksum(unchecked, version)` shared by `render` and the test helper, called with `VERSION`"
  - status: fixed
    by: newcomer
```
