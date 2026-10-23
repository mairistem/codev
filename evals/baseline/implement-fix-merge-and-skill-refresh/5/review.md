# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — its diff and file exceed the material limit.
Summarized: docs/en/src/reference/cli.md — its file exceeds the material limit.
Summarized: docs/fr/src/reference/cli.md — its file exceeds the material limit.

### migration

No finding.

### newcomer

#### R-newcomer-1 The doc comment of `is_hand_edited` names only the other review mode, but the code checks both

```yaml
severity: medium
confidence: high
location: "crates/codev-agents/src/claude.rs:270-280"
evidence: |
      /// is codev's only when it equals a rendering of this version for the
      /// other review mode — the mode changed — the checksum line left out of
      /// the comparison.
recommendation: |
  The comment says a skill without a checksum is codev's only when it matches the other review mode, while the code compares it with both renderings, the current mode (`&self.ctx`) and the other one. A newcomer trusting the comment takes the `&self.ctx` entry for a mistake and may "fix" it, bringing a legacy same-mode skill back to being preserved. Rewrite the comment to cover both modes, and replace the double negation `.all(|ctx| current != …)` with `!….any(|ctx| current == …)`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs:264-277 — the comment names both review modes; `!….any(|ctx| current == …)`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The test helper `signed` copies `render`'s checksum insertion but drops the leading newline

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:599-608"
evidence: |
      /// Inserts the checksum of `unsigned` after its version line, as
      /// `render` does.
      fn signed(unsigned: &str, version: &str) -> String {
          let version_line = format!("  version: \"{version}\"\n");
recommendation: |
  `render` matches `"\n  version: …\n"` and `signed` matches it without the leading newline, though its comment says "as `render` does": a reader cannot tell whether the tests sign files the way production does. Move the insertion into one private function, called by `render` and by the tests.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs:384-394 — one `insert_checksum`, called by `render` and the tests; `signed` removed"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The test `a_legacy_skill_of_the_other_review_mode_is_rewritten` also tests a case its name does not mention

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:683-686"
evidence: |
          // The same mode, without its checksum line: codev's too.
          let planned = plan_over(legacy, RenderCtx::default());
recommendation: |
  The second half checks that a legacy skill of the same review mode is rewritten; only an inline comment says so, not the test name, so a failure report or a scan of the names hides that rule. Move it into its own test named after that behavior.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs:671-689 — the same-mode case moved to `a_legacy_skill_of_the_same_review_mode_is_rewritten`"
  - status: fixed
    by: newcomer
```
