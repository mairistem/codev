# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — the material exceeds the size limit.
Summarized: docs/en/src/reference/cli.md — the material exceeds the size limit.
Summarized: docs/fr/src/reference/cli.md — the material exceeds the size limit.

### migration

#### R-migration-1 A change synced with an earlier codev and archived after the upgrade keeps the cut text

```yaml
severity: low
confidence: low
location: "docs/en/src/faq.md:178-182"
evidence: |
  To find out whether you are affected, look at the diff of the archive of each
  change that both modified a spec's last requirement and added requirements to
  that spec (`git log -p -- _codev/specs/`). `codev validate` does not detect
  it — the cut text is still valid Markdown — and syncing the change again does
  not repair it: its requirements are already in the spec.
recommendation: >-
  A change synced with codev 0.6.1 or earlier, not yet archived, whose delta
  modifies a spec's last requirement and adds requirements, already cut the
  spec. Archiving it after the upgrade replaces the modified requirement
  whole, but skips the ADDED one whose name already exists
  (merge/mod.rs, "an ADDED whose name already exists in the main spec is
  treated as a silent no-op"): the tail of the cut text stays under the added
  requirement's heading, unnoticed. In both FAQ entries, also cover changes
  synced but not yet archived, and say to check, after archiving, that the
  added requirement holds no text of the one before it. The third
  release-note item could say the same (the proposal is a planning
  artifact: `/codev-update`).
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: "docs/en/src/faq.md, docs/fr/src/faq.md: the entry covers a change synced before the upgrade and archived after it"
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 `edited_by_hand` answers only after its caller's own checks, and its `None` branch negates inside `all`

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:259-265"
evidence: |
      fn edited_by_hand(&self, workflow: &Workflow, version: &str, current: &str) -> bool {
          let content = without_checksum(current);
          match stamped_checksum(current) {
              Some(checksum) => sha256_hex(content.as_bytes()) != checksum,
              None => [self.ctx.clone(), self.ctx.other_review_mode()]
                  .iter()
                  .all(|ctx| content != without_checksum(&Self::render(workflow, version, ctx))),
recommendation: >-
  The method shares its name with the local `edited_by_hand` at the call
  site, which holds the whole decision, while the method is right only after
  the caller checked the version and the difference from the rendering. Give
  it a name that states that precondition (`same_version_skill_was_edited`),
  and write the `None` branch as `!…any(|ctx| content == …)` with a comment:
  a skill without a checksum is codev's when it matches its rendering in
  either review mode.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs: renamed same_version_skill_was_edited, its None branch written with any and a comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `checksum_line` skips the opening `---` line without saying so

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:400-404"
evidence: |
      let mut lines = contents.split_inclusive('\n');
      let mut start = lines.next().map_or(0, str::len);
      for line in lines {
          if line.trim_end() == "---" {
              break;
recommendation: >-
  `lines.next().map_or(0, str::len)` both consumes the opening delimiter, so
  that the `---` test in the loop finds the closing one, and sets the offset
  past it. Name the step (`let opening = lines.next()?;`) and say why in a
  comment.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs: checksum_line names the opening line and says why it is skipped"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `CHECKSUM_KEY` holds the key and its colon

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:390-392"
evidence: |
  /// The frontmatter key, under `metadata`, of a skill's checksum: the
  /// SHA-256 of the whole skill without that line.
  const CHECKSUM_KEY: &str = "checksum:";
recommendation: >-
  The YAML key is `checksum`; the constant carries the colon too, so a reader
  writing `"{CHECKSUM_KEY}: …"` would produce `checksum::`. Rename it
  `CHECKSUM_PREFIX`, or keep the key alone and put the colon at its uses.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs: CHECKSUM_KEY renamed CHECKSUM_PREFIX, its comment says it holds the colon"
  - status: fixed
    by: newcomer
```
