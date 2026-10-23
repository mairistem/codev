# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — beyond the material's size limit.
Summarized: docs/en/src/reference/cli.md — beyond the material's size limit.
Summarized: docs/fr/src/reference/cli.md — beyond the material's size limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 `stamp_checksum` hides where the line goes behind offset arithmetic and a garbled doc sentence

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:421-433"
evidence: |
  /// the version stamp, replacing any earlier one. `codev update` tells a
  /// skill it rendered from a hand edit by it, whatever build rendered it.
  fn stamp_checksum(skill: &str) -> String {
      let unsigned = without_checksum(skill);
      let line = format!("  {CHECKSUM_KEY} \"{}\"\n", content_checksum(&unsigned));
      let Some(version_at) = unsigned.find("\n  version:") else {
          return unsigned;
      };
      let insert_at = unsigned[version_at + 1..]
          .find('\n')
          .map_or(unsigned.len(), |end| version_at + 1 + end + 1);
recommendation: Rewrite the doc sentence as the reason ("the checksum lets `codev update` tell a skill codev rendered from one edited by hand, whatever build rendered it"); replace the `version_at + 1 + end + 1` arithmetic by building the output line by line, as `without_checksum` does, pushing the checksum line right after the `version:` line; say when a skill without a version stamp comes back without a checksum (never for `render`'s output).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `edited_within_version`: the doc comment is hard to follow, and `checksum` vs `content_checksum` look alike at the call site

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:201-216"
evidence: |
  /// another version is never asked: it is rewritten, so that no hand edit
  /// survives an upgrade.
  ///
  /// Its checksum decides: a match means codev rendered it — an earlier
  /// build of this version, as every build from source is — so it is
  /// rewritten. Without a checksum, the skill predates checksums and keeps
  /// the rule of that time: codev's own only if it is this version's
  /// rendering, for this review mode or the other one (the mode changed),
  /// compared without the checksum line the old rendering lacks.
  fn edited_within_version(&self, workflow: &Workflow, version: &str, current: &str) -> bool {
      match checksum(current) {
          Some(recorded) => recorded != content_checksum(current),
          None => [&self.ctx, &self.ctx.other_review_mode()]
              .into_iter()
              .all(|ctx| current != without_checksum(&Self::render(workflow, version, ctx))),
recommendation: State the cause directly (builds from source between releases keep the version number while their skill text changes) and say the function is only called for a skill stamped with the running version; rename `checksum` / `content_checksum` to `recorded_checksum` / `computed_checksum`; replace the negated `.all(... != ...)` by a named `matches_a_rendering` built with `.any(... == ...)`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `without_checksum`: a negated compound condition with `i > 0` repeated

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:404-410"
evidence: |
      for (i, line) in contents.split_inclusive('\n').enumerate() {
          if i > 0 && line.trim_end_matches(['\n', '\r']) == "---" {
              in_frontmatter = false;
          }
          if !(in_frontmatter && i > 0 && is_checksum_line(line)) {
recommendation: Push the opening `---` line before the loop so the index is no longer needed, then name the drop condition (`is_frontmatter_checksum`).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs"
  - status: fixed
    by: newcomer
```
