# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — the review material exceeds its size limit.
Summarized: docs/en/src/reference/cli.md — the review material exceeds its size limit.
Summarized: docs/fr/src/reference/cli.md — the review material exceeds its size limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 `is_edited_by_hand` answers only part of its name, and its legacy branch reads as a double negative

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:205-210"
evidence: |
  fn is_edited_by_hand(&self, workflow: &Workflow, version: &str, current: &str) -> bool {
  .all(|ctx| current != without_checksum(&Self::render(workflow, version, ctx))),
recommendation: Name the helper after the question it answers (`is_codev_rendering`, the caller negating it), and write the legacy branch positively with `.any(|ctx| current == …)` over `[&self.ctx, &other]`, which also drops the clone.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — renamed to `is_codev_rendering`, legacy branch `.any(|ctx| current == …)` without clone"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `with_checksum` finds the end of the frontmatter by offset arithmetic, each helper by its own rule

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:424-426"
evidence: |
  match unchecksummed.get(1..).and_then(|rest| rest.find("\n---\n")) {
  let close = at + 2; // past the `-` skipped and the `\n` before `---`
recommendation: Extract one `frontmatter_end(contents) -> Option<usize>` (the byte offset of the closing `---` line) used by `frontmatter_field`, `without_checksum` and `with_checksum`, so that the checksum's writer and reader agree on the frontmatter by construction, and `with_checksum` is a plain insertion.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `frontmatter_end` shared by `frontmatter_field`, `without_checksum` and `with_checksum`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The merge test's expected output adds blank lines the test does not explain

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/merge/mod.rs:549-555"
evidence: |
  "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\n\nThe system MUST emit
recommendation: Say in a comment that the merge renders a blank line after each heading, or write the inputs already normalized, so the only difference left is the modified and added text.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/merge/mod.rs — comment on the heading normalization in the test"
  - status: fixed
    by: newcomer
```
