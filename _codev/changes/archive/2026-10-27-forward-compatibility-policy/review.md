# Review record: forward-compatibility-policy

<!-- codev-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The verdict gate also refuses a codev that is newer than the requirement, which the Why does not cover

```yaml
severity: medium
confidence: high
location: "specs/self-update/spec.md#Requirement: The commands that give a verdict refuse a codev outside `codev_version`"
evidence: |
  `codev doctor` gives for the same case — `codev self-update`, or
  `codev self-update --version <X.Y.Z>` when the running codev is above the
  requirement. With `--json`, the command writes its usual document, the
recommendation: Either gate only below the requirement's lower bound, or state in the design why an upper bound is a verdict gate too, and add that case to the release note's BREAKING line. A project with `^0.6` upgrading to 0.7, or a CI tracking the latest release, would otherwise break without being told why.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (Decision: Only `validate`, `sync` and `archive` refuse…, paragraph on upper bounds; Risks) and proposal.md (Release note, BREAKING item)"
    note: The upper bound is the team's explicit exclusion of a release; the refusal is kept and justified, and the release note and docs give the fix (widen `codev_version`, or `codev self-update --version`).
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 ADR 0013 gives review.md and corrections.md two contradicting rules, and the code follows neither of them

```yaml
severity: medium
confidence: high
location: "_codev/decisions/0013-a-file-written-for-a-newer-codev-warns-unless-ignoring-it-would-weaken-a-check.md:19-60"
evidence: |
  `schema.yaml`, an ADR frontmatter, `review.md` and `corrections.md`, a
recommendation: Say which changes to review.md and corrections.md fall under point 1 (a field an older codev may ignore) and which under point 3 (a format change), and word point 3 as the current behavior — a newer format is not checked, the gate refuses it — or add tasks aligning the code.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "ADR 0013, points 1 and 3 reworded: point 1 covers fields of a known format, point 3 covers what an older codev would misread, and describes the current behavior (not checked, said so, the gate refuses)"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 An unknown field in an inherited ADR is not specified, and it would fail the inheriting project's strict CI

```yaml
severity: medium
confidence: low
location: "specs/decisions/spec.md#Requirement: ADR format recognized"
evidence: |
  A frontmatter field the parser does not know SHALL NOT reject the
  decision: the decision is read from the fields it knows, and the field is
recommendation: Decide whether an inherited decision's unknown fields are reported; a source maintained with a newer codev would otherwise fail every inheriting project's `codev validate --strict` on a file it cannot edit (decision 0005). Add a scenario and a test.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/decisions/spec.md (requirement text and scenario \"An inherited decision's unknown field is not reported\"), tasks.md 5.1, ADR 0013 point 1"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 ADR point 2 says an older codev refuses, but the gate it describes refuses only in three commands

```yaml
severity: low
confidence: high
location: "_codev/decisions/0013-a-file-written-for-a-newer-codev-warns-unless-ignoring-it-would-weaken-a-check.md:19-60"
evidence: |
  `codev archive`) refuse to run when the running codev does not satisfy
recommendation: Reword point 2 to say what is guaranteed — the verdict commands refuse, other commands may proceed without the key — and that a key misleading non-verdict commands needs per-command strictness.
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "ADR 0013 point 2 reworded"
  - status: fixed
    by: skeptic
```

### migration

#### R-migration-1 The BREAKING release note gives one remedy, and it fixes neither a pinned CI nor a requirement this release is above

```yaml
severity: medium
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: **BREAKING** `codev validate`, `codev sync` and `codev archive`
    refuse to run when the running codev does not satisfy the project's
recommendation: Add to the BREAKING item the two cases where `codev self-update` is not the fix — an upper bound (`^0.6`, widen it before upgrading, or `codev self-update --version`) and a CI pinned with `CODEV_VERSION=` by `codev init` (raise the pin with `codev_version`) — and give the same advice in task 7.1 for both docs editions.
history:
  - status: open
    by: migration
  - status: fixed
    by: product
    ref: "proposal.md (Release note, BREAKING item) and tasks.md 7.1"
  - status: fixed
    by: migration
```

#### R-migration-2 Installed skills still answer "Later → continue the action", which now runs into the refusal of sync and archive

```yaml
severity: medium
confidence: high
location: "design.md#Decisions in force"
evidence: |
  - 0008 — this change modifies no skill, workflow, role text or JSON field;
recommendation: Either update `assets/workflows/fragments/version-check.md` so that an unsatisfied requirement stops `/codev-sync`, `/codev-archive` and validate steps (a workflow change under decision 0008, with its evals step, and `codev update` in the release note), or document the dead end in the release note and in `docs/*/src/skills.md` as a known limit.
history:
  - status: open
    by: migration
  - status: disputed
    by: architect
    note: "The documentation part is done (design.md Risks; tasks.md 7.2, skills.md Update notice), and the refusal message gives the fix. Changing the skill text is a workflow change under decision 0008 with a codev-evals run; the author proposes a follow-up change (`version-check-stops-verdict-actions`) rather than widening this one."
  - status: disputed
    by: migration
    note: "Documenting the dead end does not remove it: installed skills keep offering an answer that cannot succeed. Whether to accept it as a known limit until the follow-up change, or pull the skill change into this one, is a human decision."
  - status: deferred
    by: human:Ludovic
    to: the follow-up change `version-check-stops-verdict-actions` (the update check stops `/codev-sync` and `/codev-archive` on an unsatisfied requirement)
    note: Accepted as a known limit of this change; until the follow-up change, the refusal message and the documented Update notice give the fix.
```

## code.review

Mode: isolated.

### migration

No finding.

### newcomer

#### R-newcomer-1 `line_of` packs a two-level scan into one function with one-letter names, so a reader cannot follow how it finds the next step

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/yaml_lines.rs:95-154"
evidence: |
          let mut child: Option<usize> = None;
          let mut items = 0;
          let mut hit = None;
recommendation: Name what the scan computes and explain the indentation rule — rename `names` and `found`, and comment the `get_or_insert` that fixes the children's indentation from the first line scanned.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/yaml_lines.rs (`starts_with_key`, `deepest_found_line`, a comment on the children's indentation); the loop is kept whole, its four steps being short"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 The "Keep it in sync … fails otherwise" comments promise a guard that the tests do not provide

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/metadata.rs:13-16"
evidence: |
  /// [`ChangeMetadata`] — `every_known_key_is_recognized` fails otherwise.
recommendation: Reword the comments to what the tests really guard (a key removed from the list, or a count change), in metadata.rs, schema.rs and decisions/parser.rs.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/metadata.rs, crates/codev-core/src/schema.rs, crates/codev-core/src/decisions/parser.rs: the comments say what the tests guard"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 `require_codev_version` matches on a bare `false` in a tuple whose meaning lives in another function

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/commands.rs:1125-1129"
evidence: |
      let Some((requirement, false)) = version_requirement(ctx, config.codev_version.as_deref())
recommendation: Bind the flag by name — `let Some((requirement, satisfied)) = … else { return Ok(()) }; if satisfied { return Ok(()); }`.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/commands.rs (`require_codev_version` binds `satisfied`)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 The `unknown_fields` closure works with two line offsets and two `while` loops, but does not name what it computes

```yaml
severity: low
confidence: high
location: "crates/codev-core/src/decisions/parser.rs:144-165"
evidence: |
              let start = unknown.line - 1;
              let mut end = start + 1;
recommendation: Extract `fn field_text(lines, start) -> String`, and comment the `+ 1` (the frontmatter starts after the opening `---`).
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-core/src/decisions/parser.rs (`field_text`, comment on the line offset)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 Three new near-identical blocks in validate rebuild the relative path by hand, while the change adds `relative_text` for exactly that

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/validate/mod.rs:250-256"
evidence: |
      let file = codev_core::layout::slashed(&relative);
recommendation: Extract a helper building the located warning, and reuse `change::relative_text` for the file name.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/validate/mod.rs (`unknown_key_finding`, used by the change, schema and ADR checks)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-6 `config::unknown_keys` now locates keys with two different text scanners, with no explanation

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/config.rs:362-381"
evidence: |
                  unknown.extend(unknown_role_keys(raw, roles));
recommendation: Move the top-level and section lookups to `yaml_lines`, or say why `key_line` stays.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/config.rs (`key_line`'s comment says why it stays and that new lookups use `yaml_lines`)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-7 Two change.yaml tests suggest different behaviors but check the same thing, and neither runs `--strict`

```yaml
severity: low
confidence: high
location: "crates/codev-cli/src/commands.rs:6032-6058"
evidence: |
      fn validate_strict_fails_on_a_misspelled_change_key() {
recommendation: Name the tests after what they assert (a warning at the key's line, no error); the same for the schema and role tests.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-cli/src/commands.rs (tests renamed `validate_warns_at_the_line_of_a_misspelled_*_key` and `validate_reports_an_unknown_change_key_as_a_warning_only`; the `--strict` exit code stays the main.rs rule shared by every warning)"
  - status: fixed
    by: newcomer
```

#### R-newcomer-8 `ActionError::SealVersionUnsupported` renames `actual`/`expected` to `found`/`read`, and `read` sounds like a verb

```yaml
severity: low
confidence: high
location: "crates/codev-engine/src/decisions_actions.rs:33-36"
evidence: |
                  read: expected,
recommendation: Use `found`/`supported`, and say the variant exists only to add `codev_version` to the message.
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/codev-engine/src/decisions_actions.rs (`supported`, and a comment on the variant)"
  - status: fixed
    by: newcomer
```
