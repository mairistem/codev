# Review record: rename-to-compagnon

<!-- compagnon-review: 1 -->

## plan.challenge

Mode: isolated.

### skeptic

#### R-skeptic-1 The sweep's exception list leaves out the text and tests that must keep saying "codev"

```yaml
severity: high
confidence: high
location: 'design.md#Decision: The legacy names live in one module, the only place "codev" survives in code'
evidence: |
  The deliberate exceptions to "no codev left" are therefore: this module
  and its tests, the migration spec and guide, the 0.7.0 CHANGELOG entry
recommendation: 'List every file that must keep the former name (delta bodies, rename notice, migrate reference page, tests needing former names) or build former names in tests only from legacy constants; rename the one MODIFIED heading by hand.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (legacy-names decision: exceptions extended, tests use legacy constants, changelog delta heading renamed by hand; sweep decision) and tasks.md 5.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-2 `compagnon init` in a codev project is not specified, and the current init creates a second root beside `_codev/`

```yaml
severity: medium
confidence: high
location: "specs/migration/spec.md#Requirement: A codev project is detected and never used silently"
evidence: |
  SHALL fail with the code `legacy_root`, naming the directory found
  and telling the user to run `compagnon migrate`.
recommendation: 'Specify that `compagnon init` where `_codev/` exists and `_compagnon/` does not fails with `legacy_root` and writes nothing; add a scenario and a CLI test.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/migration/spec.md (init clause, scenario Init in a codev project), design.md root-discovery decision, tasks.md 1.4, proposal.md Impact"
  - status: fixed
    by: skeptic
```

#### R-skeptic-3 A migrate run that stops partway cannot be resumed: the next run says "already migrated"

```yaml
severity: medium
confidence: high
location: "specs/migration/spec.md#Requirement: `compagnon migrate` converts a codev project in place"
raised_by: [skeptic, ops]
evidence: |
  nothing. Run where `_compagnon/` exists and `_codev/` does not, it SHALL do nothing, exit 0,
  and say the project is already migrated.
recommendation: 'Decide "already migrated" from every item the migration touches so a second run completes an interrupted one; on a failed operation exit 1 with `migration_incomplete`, naming the path, the error and the operations applied; add scenario and tests. (Folded from R-ops-3.)'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: qa
    ref: "specs/migration/spec.md (completion and migration_incomplete clauses, scenario An interrupted migration is completed by a second run), design.md migrate decision, tasks.md 2.2-2.3"
  - status: fixed
    by: skeptic
```

#### R-skeptic-4 Re-sealing 0003 after the sweep changes what that decision says, contrary to what ADR 0014 will claim

```yaml
severity: medium
confidence: high
location: "design.md#Decision: The repository is converted by dogfooding `compagnon migrate`, then a mechanical sweep"
evidence: |
     Compagnon", accepted, which states that the bodies of 0001–0013 were
     re-sealed for the rename only and that no decision's substance changed.
recommendation: 'Have ADR 0014 supersede 0003 on the name and leave 0003''s body unchanged, or state explicitly that its content is rewritten.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md (sweep decision, step 5) and tasks.md 5.3-5.4: ADR 0014 supersedes 0003 on the name; 0003's body stays as written"
  - status: fixed
    by: skeptic
```

#### R-skeptic-5 Task 5.2 expects `compagnon_version: ">=0.7"`, but the migrate spec keeps the old value

```yaml
severity: low
confidence: high
location: "tasks.md#5. Renaming the repository's contents"
evidence: |
  `compagnon migrate` on this repository: `_codev/` becomes `_compagnon/` (this
    change included), `compagnon.lock`, `compagnon_version: ">=0.7"`, skills
recommendation: 'Expect the value kept by migrate, then bump it by hand.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: lead
    ref: "tasks.md 5.2"
  - status: fixed
    by: skeptic
```

#### R-skeptic-6 The bridge for codev 0.6 users is first tested end to end only after v0.7.0 is published, with no rollback if it fails

```yaml
severity: low
confidence: low
location: "design.md#Migration Plan"
evidence: |
  Rollback: before tagging, revert the merge. After tagging, a project
  migrated by mistake goes back with `git checkout` of the files listed by
  `compagnon migrate` (its output names every path).
recommendation: 'State the fallback (v0.7.1) or rehearse the 0.6.1 self-update before tagging.'
history:
  - status: open
    by: skeptic
  - status: fixed
    by: architect
    ref: "design.md Migration Plan (rehearsal against a draft release, v0.7.1 fallback)"
  - status: fixed
    by: skeptic
```

### security

No finding.

### accessibility

#### R-accessibility-1 The mark's text alternative is put in SVG `<title>`, which `<img>` and `<use>` do not reliably expose, and no test checks it

```yaml
severity: medium
confidence: low
location: "tasks.md#4. Compagnon mark"
evidence: |
    `compagnon-logo.svg`, `compagnon-logo-dark.svg`, each with a `<title>`
    naming Compagnon and only palette colors; replace `docs/theme/favicon.svg`;
recommendation: 'Put `alt="Compagnon"` on README images and `role="img"` with `aria-label` on inline SVG; test both.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: lead
    ref: "tasks.md 4.2 and specs/docs/spec.md (text alternative rule, README scenario)"
  - status: fixed
    by: accessibility
```

#### R-accessibility-2 The plan does not say whether each placement of the mark is decorative or informative, or what its link is named

```yaml
severity: low
confidence: low
location: "specs/docs/spec.md#Requirement: Compagnon's mark identifies the product in every documentation surface"
evidence: |
  Every mark carries a text alternative naming Compagnon.
recommendation: 'State the rule per placement: decorative next to the word or inside a named link, named when alone; rename the brand links.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: qa
    ref: "specs/docs/spec.md (placement rule, brand link names), tasks.md 4.2"
  - status: fixed
    by: accessibility
```

### ops

#### R-ops-1 Merging the new installer breaks the CI of every codev project that has not migrated

```yaml
severity: high
confidence: high
location: "tasks.md#3. Changelog, bridge, update URLs"
raised_by: [ops, migration]
evidence: |
  - [ ] 3.5 Update `install.sh` and `install.ps1`: repository
    `mairistem/compagnon`, `COMPAGNON_VERSION`, archives `compagnon-*`, installed as
recommendation: 'Honor `CODEV_VERSION` as a fallback; below 0.7 fetch `codev-*` and install `codev`; with only `CODEV_VERSION` install as `codev`; test a 0.6 and a 0.7 pin; release note and guide on CI files migrate does not touch. (Merged with R-migration-1.)'
history:
  - status: open
    by: ops
  - status: fixed
    by: qa
    ref: "specs/distribution/spec.md (ADDED Installers and self-update still reach the releases published as codev), design.md (Below 0.7.0 decision), tasks.md 3.5 and 7.1, proposal.md What Changes, Impact and Release note"
  - status: fixed
    by: ops
```

#### R-ops-2 The rollback plan covers project files but gives no way to go back to the codev 0.6 binary

```yaml
severity: medium
confidence: high
location: "design.md#Migration Plan"
evidence: |
  Rollback: before tagging, revert the merge. After tagging, a project
  migrated by mistake goes back with `git checkout` of the files listed by
  `compagnon migrate` (its output names every path).
recommendation: 'Below 0.7.0, pick the codev asset names from `legacy`; document the downgrade; test it.'
history:
  - status: open
    by: ops
  - status: fixed
    by: architect
    ref: "design.md (Below 0.7.0 decision, Migration Plan rollback), specs/distribution/spec.md scenario Going back to codev 0.6, tasks.md 3.3 and 7.1"
  - status: fixed
    by: ops
```

### migration

#### R-migration-1 The managed section of the pull request template is not migrated and becomes an orphan

```yaml
severity: medium
confidence: high
location: "specs/migration/spec.md#Requirement: `compagnon migrate` converts a codev project in place"
evidence: |
  - replace the managed block of `CLAUDE.md` and the generated
    `.github/workflows/codev.yml` by their Compagnon versions
recommendation: 'Add the PR template managed section to the migrate facts and plan, with a scenario and a CLI test.'
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    ref: "specs/migration/spec.md (PR template clause and scenario), design.md, tasks.md 2.2-2.3, proposal.md"
  - status: fixed
    by: migration
```

#### R-migration-2 A version requirement with an upper bound is copied as is and makes the migrated project unsatisfiable

```yaml
severity: medium
confidence: high
location: "specs/migration/spec.md#Requirement: `compagnon migrate` converts a codev project in place"
evidence: |
  - rename the key `codev_version` to `compagnon_version` in
    `_compagnon/config.yaml`, keeping its value, its comments and the rest of
    the file byte for byte;
recommendation: 'Check the requirement against the running version before writing; refuse with a code naming it, or rewrite it; test and document.'
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    ref: "specs/migration/spec.md (migration_version_unsatisfied clause and scenario), design.md, tasks.md 2.2-2.3 and 7.1"
  - status: fixed
    by: migration
```

#### R-migration-3 A git source pinned to a pre-migration commit is refused with a fix that does not apply

```yaml
severity: medium
confidence: high
location: "specs/migration/spec.md#Requirement: An inherited source still laid out for codev is refused with its fix"
evidence: |
  sources SHALL fail with `legacy_source`, naming the source and
  saying that its maintainers must run `compagnon migrate` in it. Compagnon MUST
recommendation: 'Name both causes and fixes (`compagnon migrate` in the source, `compagnon sources update`); have migrate say to run sources update; guide; test.'
history:
  - status: open
    by: migration
  - status: fixed
    by: qa
    ref: "specs/migration/spec.md (legacy_source message, sources update after migrate), tasks.md 1.5, 2.3 and 7.1"
  - status: fixed
    by: migration
```

## code.review

Mode: isolated.

### security

#### R-security-1 Fixture rename writes through symlinks to files outside the extracted tree

```yaml
severity: low
confidence: low
location: "evals/src/runner.rs:2508-2512"
evidence: |
        if let Ok(text) = fs::read_to_string(&from) {
            let through = legacy::through_rename(&text);
recommendation: 'Skip symlinks so they are only renamed, never read or written.'
history:
  - status: open
    by: security
  - status: fixed
    by: developer
    ref: "evals/src/runner.rs collect_files reports links; bring_through_rename never reads or writes through one"
  - status: fixed
    by: security
```

### accessibility

#### R-accessibility-3 The offline manual hides the visible word and names the mark beside it

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/docs.rs:34-38"
evidence: |
  const MARK: &str = "<svg class=\"mark\" viewBox=\"0 0 250 250\" role=\"img\" aria-label=\"Compagnon\">\
recommendation: 'Make the mark aria-hidden and expose the visible title.'
history:
  - status: open
    by: accessibility
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/docs.rs MARK is aria-hidden, the title span is exposed; offline_manual_inlines_the_mark_without_external_images asserts both"
  - status: fixed
    by: accessibility
```

### ops

#### R-ops-3 A refresh that fails after the operations ran hides the migration, and a second run says nothing to do

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/commands.rs:508-509"
evidence: |
      engine_migrate::execute(ctx.fs, &root, &ops)?;
      outcome.refresh = Some(update(ctx, false)?);
recommendation: 'Report a failed refresh as part of the migration with the applied operations; treat an unresolvable configuration as a pending refresh.'
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/commands.rs migrate wraps a refresh failure as migration_incomplete listing the applied operations; skills_need_refresh counts an unresolvable configuration as pending; test a_failed_refresh_is_reported_and_not_taken_for_done"
  - status: fixed
    by: ops
```

#### R-ops-4 compagnon migrate drops the refresh warnings

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/main.rs:752-758"
evidence: |
          Command::Migrate { dry_run, json } => match commands::migrate(&ctx, dry_run) {
recommendation: 'Print the refresh warnings and put them in the top-level status.'
history:
  - status: open
    by: ops
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/main.rs prints them; contract.rs MigrateV1 status carries them"
  - status: fixed
    by: ops
```

### migration

#### R-migration-4 Skills installed by a released codev carry no checksum, so migrate keeps every one of them

```yaml
severity: high
confidence: high
location: "crates/compagnon-agents/src/claude.rs:452-458"
evidence: |
  pub fn is_generated_rendering(contents: &str) -> bool {
recommendation: 'Classify former skills as update does; treat a skill without checksum from the former generator as generated; test with a skill exactly as 0.6.1 wrote it.'
history:
  - status: open
    by: migration
  - status: fixed
    by: developer
    ref: "crates/compagnon-agents/src/claude.rs is_generated_rendering; specs/migration/spec.md (rule and scenario A skill installed by codev 0.6 is replaced); both migration guides; tests migrate_replaces_a_skill_installed_by_codev_0_6 and released_former_skills_without_checksum_count_as_generated"
    note: 'An edit to a 0.6 skill cannot be detected (no checksum): such skills are replaced and the guide says to recover an edit from git, after a --dry-run.'
  - status: fixed
    by: migration
```

### newcomer

#### R-newcomer-1 MigrateOutcome sits between update doc comment and update

```yaml
severity: medium
confidence: high
location: "crates/compagnon-cli/src/commands.rs:403-411"
evidence: |
  /// Regenerates the skills of an already initialized project.
recommendation: 'Move MigrateOutcome and migrate so the doc comment sits on update again.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/commands.rs migrate block moved above update's doc comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-2 Unexplained skills_need_refresh check inside migrate

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/src/commands.rs:443-461"
evidence: |
      let skills_need_refresh = !ctx.fs.exists(&layout.planning_dir())
recommendation: 'Extract fn skills_need_refresh with a why-comment.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/src/commands.rs fn skills_need_refresh"
  - status: fixed
    by: newcomer
```

#### R-newcomer-3 rust_sources doc comment says the opposite of what the function does

```yaml
severity: low
confidence: high
location: "crates/compagnon-cli/tests/repository.rs:22-33"
evidence: |
  /// Every `.rs` file under `dir`, as `/`-separated paths relative to the
recommendation: 'Fix the comment.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-cli/tests/repository.rs comment"
  - status: fixed
    by: newcomer
```

#### R-newcomer-4 Op::Keep doc names a kept_modified report that does not exist

```yaml
severity: low
confidence: high
location: "crates/compagnon-core/src/migrate.rs:93-97"
evidence: |
      /// Leaves a file under its former name, edited by hand: reported as
recommendation: 'Say what actually happens.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-core/src/migrate.rs Op::Keep doc"
  - status: fixed
    by: newcomer
```

#### R-newcomer-5 Bare boolean argument to refuse_former_layout

```yaml
severity: low
confidence: high
location: "crates/compagnon-engine/src/config.rs:538"
evidence: |
                  refuse_former_layout(fs, &source_layout, &raw, false)?;
recommendation: 'Pass a SourceKind instead of a bool.'
history:
  - status: open
    by: newcomer
  - status: fixed
    by: developer
    ref: "crates/compagnon-engine/src/config.rs and error.rs use sources::SourceKind"
  - status: fixed
    by: newcomer
```
