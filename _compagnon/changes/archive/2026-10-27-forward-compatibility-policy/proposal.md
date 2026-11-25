# Proposal: A forward compatibility policy for the files compagnon reads

## Why

One repository is read by several compagnon versions at once — teammates who
updated at different times, a CI pinned to a release — and nothing ever
decided what a compagnon does with a file written for a newer one. Each change
chose on its own: `_compagnon/config.yaml` warns on an unknown key, but
`change.yaml`, a project `schema.yaml`, a role's settings and an ADR
frontmatter refuse it, and the migration reviewer raised the same problem
in four changes (`R-migration-*` of `expert-triggers-and-stack-filter`,
`review-record`, `isolated-review`, `fix-merge-and-skill-refresh`). The
next key added to any of these files breaks every teammate who has not
updated yet; deciding the policy once, as a decision, ends the case by
case.

## What Changes

- A new decision, proposed for the author to accept, sets the policy:
  hand-written shared files tolerate unknown keys with a warning; a key
  whose ignoring would weaken a check fails closed through `compagnon_version`;
  files compagnon writes carry a format marker; content shipped in the binary
  stays strict; every change states what an older compagnon does; an update
  never rewrites the project's files.
- An unknown key in a change's `change.yaml` (top level or under
  `review:`) is ignored with a `change_unknown_key` warning instead of
  failing every command that reads the change.
- An unknown key in a project's `schema.yaml`, an artifact role of it
  included, is ignored with a `schema_unknown_key` warning instead of
  making the schema unusable. The built-in schema stays strict.
- An unknown key inside a role's mapping under `roles:` in
  `_compagnon/config.yaml` is ignored with an `unknown_config_key` warning
  naming `roles.<id>.<key>`, instead of failing every command.
- An unknown field in an ADR frontmatter is reported as a
  `decision_unknown_field` warning, and the decision is read, instead of
  the decision being rejected; `compagnon decision accept` keeps that field
  when it rewrites the frontmatter.
- Every warning names the key, quotes the project's `compagnon_version` when
  set and points to `compagnon self-update`; `compagnon validate --strict` fails
  on them, so a typo stays caught in CI.
- `compagnon validate`, `compagnon sync` and `compagnon archive` — the commands whose
  success is a verdict on the project — refuse to run with the code
  `compagnon_version_unsatisfied` when the running compagnon does not satisfy the
  project's `compagnon_version`. Other commands keep working.
- A `seal.yaml` of a newer format is refused with
  `seal_version_unsupported`, whose message now points to
  `compagnon self-update`, by every command that reads it.
- **BREAKING** A team whose `compagnon_version` the running compagnon does not
  satisfy can no longer validate, sync or archive with it.

## Capabilities

### New Capabilities

### Modified Capabilities

- changes
- configuration
- schemas
- roles
- review-mode
- decisions
- self-update
- doctor

### Removed Capabilities

## Impact

- Code: `crates/compagnon-engine/src/metadata.rs` (`change.yaml`),
  `crates/compagnon-core/src/schema.rs` and the `ArtifactRole` of
  `crates/compagnon-core/src/roles.rs` (project schemas),
  `RoleSettingMapping` in `roles.rs` and `crates/compagnon-engine/src/config.rs`
  (role settings), `crates/compagnon-core/src/decisions/parser.rs` and
  `crates/compagnon-engine/src/decisions_actions.rs` (ADR frontmatter),
  `crates/compagnon-core/src/decisions/seal.rs` (message), the
  `compagnon_version` fix of `crates/compagnon-core/src/doctor.rs`, the commands of
  `crates/compagnon-cli/src/commands.rs` that gate, and the validate report
  (`crates/compagnon-engine/src/validate/`).
- JSON contract: no field added or removed; new codes in the existing
  `status` arrays and validate findings (`change_unknown_key`,
  `schema_unknown_key`, `decision_unknown_field`), and an existing code
  (`compagnon_version_unsatisfied`) now also fails commands.
- Decisions: a new proposed ADR, to be accepted by the author.
- Documentation: in `docs/en` and `docs/fr`, `reference/configuration.md`,
  `faq.md`, `reference/file-formats.md`, `reference/cli.md`,
  `reference/json-output.md` and `guides/custom-schemas.md`; and
  `CONTRIBUTING.md` for what a change states about older compagnons.

## Domains

- [ ] security — no new input or secret; inherited sources keep their strict keys
- [ ] personal-data
- [ ] user-interface — no interface change; the trigger matched `role=` in schema test text
- [ ] performance — no hot path changed; the trigger matched the existing update cache, not new code
- [ ] operations
- [x] migration

## Release note

- Changed: An unknown key in a change's `change.yaml`, a project
  `schema.yaml`, a role's settings in `_compagnon/config.yaml` or an ADR
  frontmatter is now ignored with a warning naming it and pointing to
  `compagnon self-update`, instead of failing; `compagnon validate --strict` still
  fails on it. A file written for a newer compagnon no longer stops an older
  one.
- Changed: **BREAKING** `compagnon validate`, `compagnon sync` and `compagnon archive`
  refuse to run when the running compagnon does not satisfy the project's
  `compagnon_version`; run `compagnon self-update`. Projects without
  `compagnon_version` are unaffected. With an upper bound (`^0.6`), widen
  `compagnon_version` before upgrading past it, or install the version
  `compagnon doctor` names. A CI job pinned with `COMPAGNON_VERSION=` needs its pin
  raised whenever `compagnon_version` is.
- Changed: `compagnon decision accept` keeps the frontmatter fields it does
  not know when it rewrites an ADR.
