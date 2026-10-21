# Review record: fix-merge-and-skill-refresh
<!-- codev-review: 1 -->

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — beyond the material limit (38701 bytes).
Summarized: docs/en/src/reference/cli.md — beyond the material limit (43864 bytes).
Summarized: docs/fr/src/reference/cli.md — beyond the material limit (47153 bytes).

### migration

#### R-migration-1 The main legacy path, a same-version skill without a checksum that matches the rendering, has no test

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:220-223"
evidence: |
              None => {
                  let other = Self::render(workflow, version, &self.ctx.other_review_mode());
                  unstamped != split_checksum(desired).1 && unstamped != split_checksum(&other).1
              }
recommendation: >-
  Every skill installed before this change has no checksum line; when its content
  still matches the rendering, this branch rewrites it and it gains a checksum —
  the path existing installs take into the checksum scheme without `--force` — but
  no test checks it. Add a test that plans over `without_checksum` of the current
  rendering and asserts one write carrying `metadata.checksum` and no preserved
  skill, so that a later change to how the checksum line is inserted cannot make
  every existing install report all its skills as edited by hand.
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — test a_legacy_skill_matching_the_rendering_is_rewritten_with_its_checksum"
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 "stamped"/"unstamped" means two things, and `.1` hides what is compared

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:216-222"
evidence: |
          let (checksum, unstamped) = split_checksum(current);
          match checksum {
              Some(checksum) => sha256_hex(unstamped.as_bytes()) != checksum,
              None => {
                  let other = Self::render(workflow, version, &self.ctx.other_review_mode());
                  unstamped != split_checksum(desired).1 && unstamped != split_checksum(&other).1
recommendation: >-
  In this file "stamped" already means "carries the version stamp"
  (`stamped_version`); the change reuses it for the checksum (`unstamped` in
  `render`, the `stamped` flag in `with_checksum`, `unstamped` here, one line below
  a check on the version stamp), and `split_checksum(desired).1` sends the reader
  to the tuple order. Rename to names about the checksum and add a
  `strip_checksum` helper so the legacy branch reads as a comparison of contents
  without their checksum line.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `content`/`inserted` names, `strip_checksum` helper"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 `split_checksum` hides its one skip in a `continue` inside a let-chain

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:409-421"
evidence: |
      if let Some(first) = lines.next() {
          rest.push_str(first);
          let mut in_frontmatter = first.trim_end() == "---";
          for line in lines {
              if in_frontmatter && line.trim_end() == "---" {
                  in_frontmatter = false;
              } else if in_frontmatter
                  && checksum.is_none()
                  && let Some(value) = line.trim().strip_prefix(CHECKSUM_KEY)
              {
                  checksum = Some(value.trim().trim_matches('"').to_string());
                  continue;
              }
              rest.push_str(line);
recommendation: >-
  The one line the function exists to drop is handled by a `continue` at the end
  of an `else if` let-chain three levels deep, and the first line is handled apart.
  Flatten it: treat every line in one loop and make "skip the checksum line" the
  visible first step of the loop body.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs — `split_checksum` as one flat loop, the checksum line skipped first"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 The regression test puts its spec and delta on one unreadable line each

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/merge/mod.rs:550-554"
evidence: |
          let spec_source = "## Purpose\n\nx.\n\n## Requirements\n\n### Requirement: Login\nThe system SHALL x.\n\n#### Scenario: S\n- **WHEN** a\n- **THEN** b\n";
recommendation: >-
  The shape that matters — MODIFIED before ADDED, `Login` closing the
  `Requirements` section — is buried in strings of 150 to 250 characters, and
  nothing says why the expected `login` has blank lines after its headings. Write
  each document one Markdown line per source line (`concat!`), and say in a
  comment that `login` is the normalized rendering the merge produces.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/merge/mod.rs — the test documents written with `concat!`, a comment on `login`"
  - status: fixed
    by: newcomer
```
