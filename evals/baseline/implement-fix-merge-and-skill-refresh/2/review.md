# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — the material exceeds the size limit (273841 of 163840 bytes).
Summarized: docs/en/src/reference/cli.md — the material exceeds the size limit (273841 of 163840 bytes).
Summarized: docs/fr/src/reference/cli.md — the material exceeds the size limit (273841 of 163840 bytes).

### migration

No finding.

### newcomer

#### R-newcomer-1 The checksum line's format is spelled out in several places that must agree

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:171-176"
evidence: |
  let stamp = format!("{VERSION_LINE_PREFIX}\"{version}\"\n");
  unsigned.replacen(
      &stamp,
      &format!("{stamp}{CHECKSUM_LINE_PREFIX}\"{checksum}\"\n"),
      1,
  )
recommendation: >
  `render` anchors the checksum line on a version line rebuilt from
  `VERSION_LINE_PREFIX`, while `render_unsigned` writes that line from its own
  format literal; nothing shows they must match, and `replacen` silently does
  nothing if they drift, shipping a skill without a checksum. The test helper
  `signed` spells both lines a third time with dense index arithmetic. Extract
  one function that adds the checksum line, used by `render` and the tests, and
  build the version line of `render_unsigned` from `VERSION_LINE_PREFIX`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `render` calls a single `sign` function, also used by the tests; `render_unsigned` builds the version line from `VERSION_LINE_PREFIX`; the test helper `signed` is removed"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `is_edited_by_hand` returns false for a hand-edited skill of another version

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:243-247"
evidence: |
  let edited_by_hand = !force
      && fs.exists(&path)
      && fs.read_to_string(&path).is_ok_and(|current| {
          current != desired && self.is_edited_by_hand(&current, workflow, version)
      });
recommendation: >
  The name says "edited by hand" while the method answers "must be preserved":
  it returns false for a skill of another version even when edited by hand, and
  the call-site comment that explained the rule was removed. Rename the method
  and the variable after the decision (`must_preserve`), or restore a one-line
  comment at the call site: a hand edit is kept only within the running
  version; an upgrade rewrites it.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — method renamed `must_preserve`, variable `preserve`, and the call-site comment restored: a hand edit is kept only within the running version"
  - status: fixed
    by: newcomer
```
