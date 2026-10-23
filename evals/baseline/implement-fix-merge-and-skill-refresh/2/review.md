# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — 38650 bytes, over the material limit.
Summarized: docs/en/src/reference/cli.md — 43620 bytes, over the material limit.
Summarized: docs/fr/src/reference/cli.md — 46882 bytes, over the material limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 The hand-edit check is a nested closure that ends in a double negative

```yaml
severity: medium
confidence: high
location: "crates/codev-agents/src/claude.rs:235-250"
evidence: |
  None => [&self.ctx, &self.ctx.other_review_mode()]
      .into_iter()
      .map(|ctx| split_checksum(&Self::render(workflow, version, ctx)))
      .all(|rendered| rendered.is_none_or(|(_, bare)| bare != current)),
recommendation: >
  Move the decision out of `plan_skills` into a named helper, such as
  `is_hand_edited(&self, workflow, version, current) -> bool`, with one
  early return per rule: another stamp returns false, and a checksum that is
  present returns `sha256 != checksum`. Write the legacy case in the
  positive (`!any(rendering == current)`) rather than `.all(is_none_or(!=))`.
  Keep `!force && fs.exists(..)` at the call site. As it stands, a reader
  asking "when is a legacy skill preserved?" must apply De Morgan by hand
  through four levels (`&&` chain, closure, `match`, iterator).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — the decision moved to ClaudeCode::is_hand_edited, early returns, legacy case written as !any(rendering == current)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The checksum line format is split between `render` and the constant that names it

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:195-197"
evidence: |
  let tail = format!("---\n\n{}", body.trim_end());
  let checksum = sha256_hex(format!("{head}{tail}").as_bytes());
  format!("{head}  {CHECKSUM_KEY} \"{checksum}\"\n{tail}")
recommendation: >
  Give the line's shape one home, a `checksum_line(checksum)` next to
  `split_checksum`, and say in `render` that the hash covers `head + tail`
  because `split_checksum` removes exactly that line. The doc of
  `CHECKSUM_KEY` says "indented by two spaces" while the constant is the
  bare `checksum:`; the indent and quotes live in the `format!` here.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — checksum_line() shared by render and the tests, why-comment in render, CHECKSUM_KEY doc corrected"
  - status: fixed
    by: newcomer
```
