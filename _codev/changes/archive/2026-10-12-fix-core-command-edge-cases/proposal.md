# Proposal: Fix the core-command edge cases found while specifying them

## Why

Specifying the core commands (change `specify-core-commands`) surfaced five
defects that were deliberately left out of the specs: a stray Markdown file
under `_codev/specs/` breaks `codev validate`, a project schema whose `name:`
differs from its folder produces a change no command can load, `archive` is
accepted as a change name, a failed `codev instructions --json` loses a
field, and JSON paths keep native separators on Windows although the
documentation promises `/`. They should be fixed before the real-world
workflows start depending on these commands.

## What Changes

- `codev list --specs` lists a capability only for a file named exactly
  `spec.md`; any other Markdown file under `_codev/specs/` (`notes/myspec.md`,
  a `README.md`) is ignored instead of producing a phantom capability
  (`notes/my`). `codev validate` (`--all`, `--specs`, a named item) validates
  exactly the capabilities that `codev list --specs` lists, so a stray file no
  longer makes it fail with an unreadable `spec.md`.
- A project schema is identified by its folder everywhere: `codev new change`
  records the folder name in `change.yaml`, and `schemaName` in
  `codev status` and `codev instructions` is that name. A project schema whose
  `name:` differs from its folder stays usable and `codev schemas` reports it
  with a new warning `schema_name_mismatch`.
- `archive` becomes a reserved change name: `codev new change archive` and
  `--change archive` fail with `invalid_change_id` and a message saying the
  name is reserved for archived changes; the archive folder is never listed,
  inferred nor loaded as a change.
- A failed `codev instructions --json` keeps the `language` field, set to the
  configured language when the configuration can be read, `en` otherwise.
- Every path in a JSON document uses `/` separators on every operating
  system, as the JSON documentation already promises — notably
  `planningHome`, `changeRoot`, `created`, `resolvedOutputPath`, dependency
  paths, the path lists of `init`/`update`, decision paths and `root`.

## Capabilities

### New Capabilities

- `json-output` — the portable paths of every JSON document.

### Modified Capabilities

- `changes` — which files are capabilities for `codev list --specs`; the
  reserved name `archive`.
- `schemas` — a project schema is identified by its folder; the
  `schema_name_mismatch` warning of `codev schemas`.
- `instructions` — `language` in the failure document.
- `validation` — the specs validated are the capabilities listed.

### Removed Capabilities

None.

## Impact

- `codev-core`: the change name rule (reserved `archive`); the shared helper
  that writes a path with `/` separators.
- `codev-engine`: the capability listing, the resolved schema's name, the
  status' schema name.
- `codev-cli`: `new change`, `schemas`, the `instructions` failure shape and
  every JSON document carrying a path (`contract`, `main`).
- JSON contract v1: no field added or removed except the warning code
  `schema_name_mismatch`; `language` joins the `instructions` failure
  document, which the success document already had. On Linux and macOS no
  JSON output changes; on Windows, paths switch from `\` to `/`.
- Documentation, English and French: reference/cli.md (reserved name,
  `schema_name_mismatch`, what `list --specs` lists), reference/json-output.md
  (the failure shape of `instructions`, `schema_name_mismatch`, portable
  paths), guides/custom-schemas.md (the folder is the schema's name),
  reference/file-formats.md (what is a capability under `_codev/specs/`, the
  reserved change name).

## Release note

- Fixed: `codev validate` no longer fails when `_codev/specs/` holds a
  Markdown file other than `spec.md`, and `codev list --specs` no longer
  shows such a file as a capability.
- Fixed: A change created with a project schema whose `name:` differs from
  its folder can now be used; `codev schemas` warns about the mismatch with
  `schema_name_mismatch`.
- Fixed: `archive` is refused as a change name, by `codev new change` and by
  `--change`, instead of being treated as a change.
- Fixed: A failed `codev instructions --json` now carries `language`, like a
  successful one.
- Fixed: Paths in JSON output use `/` separators on Windows too, as
  documented.
