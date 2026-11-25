# Proposal: Forward-compatible configuration

## Why

`_compagnon/config.yaml` is rejected as a whole as soon as it holds a key the
running compagnon does not know, so a teammate whose compagnon is older than the
project's configuration gets a hard failure on every command — at the very
moment compagnon should explain that a newer version is required (`compagnon_version`
is the first such key). And when the running compagnon is newer than an upper
bound of `compagnon_version`, `compagnon doctor` suggests `compagnon self-update`, which
refuses the implicit downgrade: the advertised fix does not work.

## What Changes

- An unknown top-level key of `_compagnon/config.yaml` no longer fails the
  command: it is ignored, the rest of the configuration is used, and a
  warning `unknown_config_key` names the key, says the project may need a
  newer compagnon — quoting the project's `compagnon_version` when set — and points
  to `compagnon self-update`.
- Unknown keys inside the `mcp` section are tolerated the same way. Unknown
  keys inside an `inherits` entry or the `changelog` section still fail the
  command, because ignoring them would silently change which source is read
  or what `compagnon changelog --write` writes; that error now also quotes the
  project's `compagnon_version` when set.
- Unknown keys of an inherited source's configuration are ignored without a
  warning: only its context and rules are read.
- The warning appears in `compagnon status` (and every command that already
  reports configuration warnings), as a `warning` finding of a new `config`
  item in `compagnon validate` (so `compagnon validate --strict` fails on it), and as
  a new `config.keys` check of `compagnon doctor`.
- `compagnon doctor`'s `compagnon_version` check, when the running compagnon is above
  the requirement's upper bound, gives the fix
  `compagnon self-update --version <X.Y.Z>` naming a release that satisfies the
  requirement; below the requirement, the fix stays `compagnon self-update`.
- Documentation: configuration reference (unknown keys, forward
  compatibility, compagnon 0.5 and earlier still reject unknown keys), FAQ entry
  for a teammate whose compagnon fails on the configuration, doctor check tables
  and samples, in English and French.

## Capabilities

### New Capabilities

- `configuration` — how `_compagnon/config.yaml` treats keys the running compagnon
  does not know, and where that is reported.

### Modified Capabilities

- `doctor` — the new `config.keys` check, and the `compagnon_version` fix that
  depends on whether the running compagnon is above or below the requirement.

### Removed Capabilities

None.

## Impact

- `compagnon-core`: the unknown-key message, the upper-bound analysis of a
  version requirement and the downgrade target, the `config.keys` doctor
  check and the `compagnon_version` fix.
- `compagnon-engine`: configuration loading (tolerant top level and `mcp`,
  strict `inherits` entries and `changelog`, error enriched with
  `compagnon_version`), the configuration warning, the `config` item of the
  validation report.
- `compagnon-cli`: doctor facts (unknown keys, cached latest release), the
  `config` item kind in the human and JSON validation reports (additive
  value of `kind`).
- Documentation in English and French: reference/configuration.md,
  reference/cli.md (doctor table), reference/json-output.md (doctor table,
  validate item kinds), guides/repository-integration.md and quickstart.md
  (doctor samples), faq.md.

## Release note

- Changed: A key of `_compagnon/config.yaml` that your compagnon does not know —
  typically one added by a newer compagnon — is now a warning `unknown_config_key`
  instead of an error, naming the key and the project's `compagnon_version`, and
  pointing to `compagnon self-update`; `compagnon validate --strict` and the new
  `config.keys` check of `compagnon doctor` report it. compagnon 0.5 and earlier still
  reject such keys.
- Fixed: When the running compagnon is newer than the project's `compagnon_version`
  allows, `compagnon doctor` now suggests `compagnon self-update --version <X.Y.Z>`
  with a version that satisfies it, instead of a `compagnon self-update` that
  refuses to downgrade.
