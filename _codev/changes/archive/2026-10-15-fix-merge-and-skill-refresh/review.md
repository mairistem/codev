# Review record: fix-merge-and-skill-refresh

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 A hand-edited checksummed skill is preserved across upgrades, and doctor then reports it outdated with no way out but --force

```yaml
severity: medium
confidence: high
location: "specs/skills/spec.md#Requirement: `codev update` regenerates the skills of the configured workflows"
evidence: |
  - a skill whose content no longer matches its checksum is edited by hand
    and is preserved, unless `--force` is given, in which case it is
    rewritten;
recommendation: "Keep the version rule for any skill stamped with another version and apply the checksum rule only when the stamp matches the running version; or state cross-version preservation and fix doctor."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "skills delta: a skill stamped with another version is rewritten as before; the checksum decides only among the running version's skills; scenario \"A hand edit survives no upgrade\"; design and task 2.1 updated"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 The legacy rule for skills without a checksum contradicts the review-mode requirement

```yaml
severity: low
confidence: high
location: "specs/skills/spec.md#Requirement: `codev update` regenerates the skills of the configured workflows"
evidence: |
  - a skill without a checksum, rendered by an earlier codev, is rewritten
    when it is stamped with another codev version and preserved otherwise,
    as edited by hand.
recommendation: "Add the review-mode exception to the legacy rule and to task 2.1, comparing without the checksum line."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "the legacy rule keeps the review-mode exception, the checksum line left out of the comparison; task 2.1 test `a_legacy_skill_of_the_other_review_mode_is_rewritten`"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 Two scenarios give opposite outcomes for the same skill from an earlier build

```yaml
severity: low
confidence: high
location: "specs/skills/spec.md#Requirement: `codev update` regenerates the skills of the configured workflows"
evidence: |
  - **GIVEN** a project whose `codev-apply` skill was rendered by a build
    of the same version whose skill text differed, untouched since
recommendation: "Say in that GIVEN that the skill carries a checksum line matching its content."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "the GIVEN now says the skill carries a checksum line matching its content"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 Domains says None, but the plan changes a documented file format and what an upgrade does

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  None
recommendation: "Tick migration so its expert reviews the transition, or state why none is needed."
history:
  - status: open
    by: skeptic
  - status: fixed
    by: product
    ref: "Domains ticks `migration`; its expert reviews the plan"
  - status: fixed
    by: skeptic
```

### migration

#### R-migration-1 Specs already cut by earlier codev archives get no path: nothing tells users, and a re-sync does not repair them

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Fixed: Archiving a change that modifies a spec's last requirement and adds requirements no longer cuts that requirement in two: the added ones are written after it.
recommendation: "Add a release-note item and a line in the docs for existing projects: earlier codev versions could cut a spec's last requirement when one delta modified it and added requirements, `codev validate` does not detect it, re-syncing does not repair it; check each spec archived from such a delta and repair it by hand or with a new MODIFIED delta."
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: "release note item for existing projects; FAQ entry (en, fr) in task 3.1 on spotting and repairing a cut spec"
  - status: fixed
    by: migration
```

#### R-migration-2 The documented update rule leaves out unchecksummed skills of the running version, and the one-time `--force` step appears only in the design

```yaml
severity: low
confidence: high
location: "tasks.md#3. Documentation"
evidence: |
  in both `reference/cli.md`, say that a skill codev rendered is refreshed whatever its version and only a skill whose content no longer matches its checksum is preserved
recommendation: "Have the docs state the transitional rule for unchecksummed skills of the running version and the one-time `codev update --force`, and repeat it in the release note."
history:
  - status: open
    by: migration
  - status: fixed
    by: lead
    ref: "task 3.1 documents the full rule, the transitional case and the one-time `--force`; the release note repeats it"
  - status: fixed
    by: migration
```

## code.review

Mode: isolated.
Summarized: crates/codev-agents/src/claude.rs — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: docs/en/src/reference/cli.md — over the material budget; its diff fit in two hundred lines and was returned verbatim.
Summarized: docs/fr/src/reference/cli.md — over the material budget; its diff fit in two hundred lines and was returned verbatim.

### migration

#### R-migration-3 The one-time `codev update --force` step discards the user's real hand edits in skills without a checksum, and the docs do not warn about it

```yaml
severity: low
confidence: low
location: "docs/en/src/reference/cli.md:183-188"
evidence: |
  hashes to its `checksum` — codev's rendering, from an earlier build or for
  the other review mode — and preserves one that no longer does, edited by
  hand, unless you pass `--force`. A skill without a checksum, rendered
  before checksums existed, is preserved when it carries this version's
  stamp and differs from the rendering by more than the review mode: run
  `codev update --force` once to bring such skills under the checksum. A preserved skill is
recommendation: "Say wherever the one-time step appears, and in the release note, that `--force` also overwrites real hand edits: check `git diff` first and apply them again after."
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: "docs (en, fr: cli.md, file-formats.md, skills.md) and the release note say `--force` also overwrites real edits: check `git diff` first and apply them again after"
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 The hand-edit decision in plan_skills sits under a stale comment, inside an inverted match

```yaml
severity: medium
confidence: high
location: "crates/codev-agents/src/claude.rs:235-260"
evidence: |
              // Same stamped version, different content: someone edited the
              // file by hand. Overwriting it would lose their work without
              // warning; report it and move on. A skill rendered by this
              // version for the other review mode is codev's own: the mode
              // changed, regenerate it.
              let edited_by_hand = !force
                  && fs.exists(&path)
                  && fs.read_to_string(&path).is_ok_and(|current| {
                      current != desired
                          && stamped_version(&current).as_deref() == Some(version)
                          && match matches_its_checksum(&current) {
                              // Rendered by codev, by an earlier build of this
                              // version or for the other review mode.
                              Some(true) => false,
                              Some(false) => true,
                              // Rendered before checksums: only a difference
                              // of review mode is codev's own.
                              None => {
                                  current
                                      != without_checksum(&Self::render(
                                          workflow,
                                          version,
                                          &self.ctx.other_review_mode(),
                                      ))
                              }
                          }
recommendation: "Move the decision into a named helper with early returns and rewrite the comment to state the checksum rule."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-agents/src/claude.rs: `is_hand_edited` with early returns and a comment stating the checksum rule; `plan_skills` calls it"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Test name a_hand_edit_survives_no_upgrade says the opposite of what it asserts

```yaml
severity: medium
confidence: high
location: "crates/codev-agents/src/claude.rs:631-637"
evidence: |
      #[test]
      fn a_hand_edit_survives_no_upgrade() {
          let rendered = ClaudeCode::render(propose(), "0.5.0", &RenderCtx::default());
          let planned = plan_for(format!("{rendered}\nMy own line.\n"), RenderCtx::default());
          assert_eq!(planned.plan.writes.len(), 1, "another version: rewritten");
          assert!(planned.preserved.is_empty());
      }
recommendation: "Name it after the behavior, for example `a_hand_edit_of_a_skill_from_another_version_is_rewritten`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "renamed `a_hand_edit_of_a_skill_from_another_version_is_rewritten`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 Test name refers to "the version rule", a term only the plan defines

```yaml
severity: low
confidence: low
location: "crates/codev-agents/src/claude.rs:607-617"
evidence: |
      #[test]
      fn a_skill_without_checksum_keeps_the_version_rule() {
          let rendered = ClaudeCode::render(propose(), VERSION, &RenderCtx::default());
          let legacy = format!("{}\nAn older text.\n", without_checksum(&rendered));
          let planned = plan_for(legacy, RenderCtx::default());
          assert!(
              planned.plan.writes.is_empty(),
              "same version, no checksum: preserved"
          );
          assert_eq!(planned.preserved.len(), 1);
      }
recommendation: "Name it after what it asserts, for example `a_same_version_skill_without_checksum_that_differs_is_preserved`."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "renamed `a_same_version_skill_without_checksum_that_differs_is_preserved`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The checksum sealing is written twice, and the test copy hard-codes the key

```yaml
severity: low
confidence: high
location: "crates/codev-agents/src/claude.rs:551-560"
evidence: |
      fn rendered_by_another_build(rendered: &str, body_edit: &str) -> String {
          let unsealed = format!("{}{body_edit}", without_checksum(rendered));
          let checksum = sha256_hex(unsealed.as_bytes());
          let version_line = format!("  version: \"{VERSION}\"\n");
          unsealed.replacen(
              &version_line,
              &format!("{version_line}  checksum: \"{checksum}\"\n"),
              1,
          )
      }
recommendation: "Extract a `seal(unsealed, version)` used by `render` and the test helper."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "`seal(unsealed, version)` used by `render` and the test helper, with `CHECKSUM_KEY`"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 without_checksum tracks the frontmatter with an unexplained toggle

```yaml
severity: low
confidence: low
location: "crates/codev-agents/src/claude.rs:398-410"
evidence: |
  fn without_checksum(contents: &str) -> String {
      let mut out = String::with_capacity(contents.len());
      let mut in_frontmatter = false;
      for (i, line) in contents.split_inclusive('\n').enumerate() {
          if line.trim_end() == "---" {
              in_frontmatter = i == 0;
          }
          if in_frontmatter && line.trim().starts_with(CHECKSUM_KEY) {
              continue;
          }
          out.push_str(line);
      }
      out
recommendation: "Say in a comment that only the opening `---` starts the frontmatter, or restructure."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "a comment in `without_checksum` says only the first line's `---` opens the frontmatter"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 The FAQ says "up to codev 0.6.1" while the fixed build is itself 0.6.1

```yaml
severity: low
confidence: low
location: "docs/en/src/faq.md:163-164"
evidence: |
  Up to codev 0.6.1, archiving a change that both modified the last
  requirement of a spec and added requirements could write the added ones in
recommendation: "Say which builds are affected without naming a release that does not exist yet; same in French."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "the FAQ says codev 0.6.1 and earlier, builds made before this fix, in both languages"
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 The rewritten file-formats paragraph mixes overlong and normal line wraps

```yaml
severity: low
confidence: high
location: "docs/en/src/reference/file-formats.md:477-480"
evidence: |
  `metadata.checksum` is the SHA-256 of the whole skill without that line.
  With `metadata.version`, it is how `codev update` tells codev's renderings
  from your edits. `codev update` rewrites a skill stamped with another version; among the
  skills stamped with its own version, it rewrites one whose content still
recommendation: "Rewrap the paragraph to the chapter's width, in both languages."
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "the rewritten paragraphs are wrapped at the chapters' width, in both languages"
  - status: fixed
    by: newcomer
```
