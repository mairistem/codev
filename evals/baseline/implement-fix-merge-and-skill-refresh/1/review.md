# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — the material exceeds the review size limit (272154 bytes for 163840).
Summarized: docs/en/src/reference/cli.md — the material exceeds the review size limit (272154 bytes for 163840).
Summarized: docs/fr/src/reference/cli.md — the material exceeds the review size limit (272154 bytes for 163840).

### migration

No finding.

### newcomer

#### R-newcomer-1 `is_hand_edit` hides its legacy branch behind a dense docstring and a negated `all`

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:202-217"
evidence: |
              None => [self.ctx.clone(), self.ctx.other_review_mode()]
                  .iter()
                  .all(|ctx| {
                      unchecksummed != split_checksum(&Self::render(workflow, version, ctx)).0
                  }),
recommendation: Write the docstring as the two cases (with a checksum, a hand edit when the content no longer matches it; without one, a skill rendered before checksums, a hand edit unless it equals the rendering of either review mode), express the legacy arm positively as `!…any(… == …)`, and move the method into the `impl ClaudeCode` block holding `render` instead of a one-method block of its own.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — is_hand_edit moved next to render, docstring as two cases, legacy arm as `!renderings.contains(&unchecksummed)`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `split_checksum` leaves its loop guard unexplained and rebuilds the string inline

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:380-400"
evidence: |
          if index > 0 && bare == "---" {
recommendation: Say why line 0 is skipped (it is the opening delimiter; the scan stops at the closing one), and name the rebuilt string, for example `without_checksum_line`, so the returned tuple reads as content without the line and the checksum it states.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — split_checksum comments the opening delimiter and names `without_checksum_line` and `checksum`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 Test helper `checksummed` has an unclear docstring, and three tests repeat the same legacy-rendering expression

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:588-590"
evidence: |
      /// rendered it, as a build whose skill text differed would have.
      fn checksummed(contents: &str) -> String {
recommendation: Say what the helper does (inserts after the version stamp the checksum line codev would write for `contents`, as a skill rendered by another build of the same version), and factor the repeated `split_checksum(&ClaudeCode::render(propose(), VERSION, &RenderCtx::default())).0` of lines 613, 651 and 665 into a helper such as `rendering_without_checksum()`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `checksummed` docstring rewritten, `rendering_without_checksum()` helper used by the three tests"
  - status: fixed
    by: newcomer
```
