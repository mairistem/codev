# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — beyond the material size limit.
Summarized: docs/en/src/reference/cli.md — beyond the material size limit.
Summarized: docs/fr/src/reference/cli.md — beyond the material size limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 The hand-edit check is a three-level boolean expression with a `match` inside a closure

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:232-243"
evidence: |
  let edited_by_hand = !force
      && fs.exists(&path)
      && fs.read_to_string(&path).is_ok_and(|current| {
recommendation: Move the closure body into a named method (`is_hand_edit`) with early returns in the order the comment states the rule, so the call site reads as one condition.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `ClaudeCode::is_hand_edit`, early returns, called from `plan_skills`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `edited_before_checksums` uses a double negative (`all` + `is_none_or` + `!=`)

```yaml
severity: medium
confidence: high
location: "crates/codev-agents/src/claude.rs:264-269"
evidence: |
  [self.ctx.clone(), self.ctx.other_review_mode()]
      .iter()
      .map(|ctx| Self::render(workflow, version, ctx))
      .all(|rendered| {
          split_checksum(&rendered).is_none_or(|(unstamped, _)| unstamped != current)
      })
recommendation: State the rule positively — codev rendered it if it equals either unstamped rendering — and negate once; render the skill without its checksum through a helper that cannot fail, so the `Option` disappears.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `edited_before_checksums` computes `rendered_by_codev` with `any` over `render_unstamped`, then returns its negation"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 In `split_checksum`, `start` is the offset of the current line, not a fixed start

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:387-396"
evidence: |
  let mut start = contents.find('\n')? + 1;
  for line in contents[start..].split_inclusive('\n') {
recommendation: Rename it `line_offset`, keep the loop's start in its own binding, and say in a comment that the offset tracks `line`, so the cut reads as "everything except this line".
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `split_checksum` iterates from `after_opening` and tracks `line_offset`, cutting `before` and `after`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The test copies `render`'s checksum stamping by hand and reuses the name `earlier`

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:616-620"
evidence: |
  let earlier = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
  let unstamped = without_checksum(&earlier).replace("Planning boundary", "Plan boundary");
  let checksum = sha256_hex(unstamped.as_bytes());
  let (head, tail) = unstamped.split_at(unstamped.find("---\n\n").unwrap());
  let earlier = format!("{head}{CHECKSUM_PREFIX}\"{checksum}\"\n{tail}");
recommendation: Extract a `stamp_checksum` used by both `render` and the test, and name the simulated skill `earlier_build`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `stamp_checksum` and `render_unstamped`, used by `render` and by the test, which binds `current` then `earlier_build`"
  - status: fixed
    by: newcomer
```
