# Design: Fix the core-command edge cases found while specifying them

## Context

The five defects are independent and each touches a different layer:

- the capability listing (`codev-engine::specs::list`) derives a capability
  from any walked file whose path ends with the text `spec.md`;
  `validate --all/--specs` and `validate <name>` iterate that list, then read
  `<capability>/spec.md`, which does not exist for `notes/myspec.md`;
- `codev new change` records `ResolvedSchema::name()`, the `name:` inside
  `schema.yaml`, while resolution goes by folder; `codev status` and
  `codev instructions` also print that internal name (`schemaName`, the
  `unknown_artifact` message), because the core status takes it from the
  parsed graph;
- `ChangeId::parse` accepts `archive`, and `change::load` only checks that
  the folder exists;
- the failure shape of `instructions` is a static JSON value without
  `language`;
- JSON documents are filled in two places — the `From` conversions of
  `codev-cli::contract` and the `emit` calls of `main.rs` — and about forty
  path fields go through `display().to_string()`, keeping `\` on Windows.
  `contract.rs` already has a private `to_slash` used by `validate`, `sync`
  and `archive`, and `commands.rs` a `portable` doing the same for the paths
  written into `config.yaml`; `codev-engine::changelog` repeats it inline.

Constraints: decision 0001 (the core stays pure — a string transformation
of a path is pure) and 0002 (the crate graph is the dependency rule: a
helper shared by the engine and the CLI lives in `codev-core`). The JSON
contract v1 only evolves additively.

## Goals / Non-Goals

Goals: fix the five defects with a regression test each, written first and
seen failing; make the JSON path rule hard to break again.

Non-goals: validating project schemas in `codev validate`; reporting stray
files under `_codev/specs/`; reworking the human output's paths (they stay
native, which is what a person expects to copy into their shell);
re-parsing `change.yaml` files already written with a wrong schema name
(see Migration Plan).

## Decisions

### Decision: A capability is a folder whose file is named exactly `spec.md`; other files are ignored silently

`specs::list` keeps a walked file only when its last path segment is
`spec.md` (the walk already returns `/`-separated relative paths), and the
root `spec.md` still names no capability. `validate` keeps iterating the
same list, so both commands agree by construction.

A stray file is ignored without a finding. A `README.md` or notes kept next
to the specs are legitimate — the existing spec already says a `README.md`
is not a spec — and a warning would make `codev validate --strict` and
`codev doctor --strict` fail in CI for them, with no way to silence it short
of deleting the file. Main specs are written by `sync`/`archive`, always as
`spec.md`, so a misnamed spec is unlikely to need a warning to be noticed.

**Alternatives considered**: a `validate` warning `stray_spec_file` —
rejected for the CI noise above; listing every `*.md` as a capability —
rejected, it contradicts the spec-merge layout (`<capability>/spec.md`).

### Decision: The folder is the schema's name everywhere; a differing `name:` is a `codev schemas` warning

`ResolvedSchema` carries the name it was resolved by, and `name()` returns
it. `new change` records it, `instructions` uses it, and `change::status`
sets it as the status' schema name instead of the graph's `name:`. One
source of truth, so `change.yaml`, `schemaName` and the messages can no
longer disagree with what resolution accepts.

`codev schemas` adds a `schema_name_mismatch` warning when a resolved
project schema's `name:` differs from its folder — the command people run to
understand their schemas, and the guide already says the two should match.
The schema is still listed and usable: after this fix the mismatch is only
misleading, never breaking.

**Alternatives considered**: refusing the mismatch as `schema_invalid` —
rejected, it would break projects whose changes work today with the fixed
code, for a cosmetic field; a `validate` warning — rejected, `validate`
checks planning content (changes, specs, decisions, config), not schemas,
and adding schemas to it widens the command beyond this fix; resolving by
`name:` — rejected, it would need scanning every folder and contradicts the
documented layout.

### Decision: `archive` is reserved in `ChangeId::parse`, with `invalid_change_id`

`ChangeId::parse` refuses `archive` with the reason "`archive` is reserved
for the archived changes". The check sits in the change-name rule, not in
the generic kebab-case check, which also validates schema and artifact ids.
Every entry point goes through `ChangeId::parse` — `new change`, `--change`
of `status`, `instructions`, `sync`, `archive`, `decision promote` — so they
all refuse it with the same code, and `change::list` no longer needs its own
`archive` filter to be correct (it keeps it, harmless).

**Alternatives considered**: `unknown_change` for `--change archive` —
rejected: the folder exists, so "does not exist, see `codev list`" would be
false, and it would leave `new change archive` and `--change archive` with
two codes for one rule; a dedicated `reserved_change_name` code — rejected:
the existing spec already maps every name breaking the naming rule to
`invalid_change_id`, skills already handle it, and the message carries the
reason.

### Decision: The `instructions` failure shape takes the language resolved best-effort

`instructions_shape` takes the language; `main` computes it with a new
`commands::configured_language`, which discovers the project and resolves
the configuration, falling back to the default language when either fails.
The language is resolved again only on the failure path, so the success
path is unchanged.

**Alternatives considered**: always `en` in the failure — rejected, the
brief asks for the configured one when resolvable and a skill would
otherwise switch languages on an error; carrying the language inside the
`Failure` — rejected, it couples every failure type to one command's field.

### Decision: One path helper in `codev-core`, used by every JSON field

`codev_core::layout::slashed(&Path) -> String` writes a path with `/`
separators (`to_string_lossy` then `\` → `/`, which keeps a single leading
`/` on Unix). It replaces `contract::to_slash`, `contract::paths_slash`,
`main::paths`, `commands::portable` and the inline copy in
`codev-engine::changelog`. Every `display().to_string()` feeding a JSON
field in `contract.rs` and `main.rs` goes through it, and `StatusV1::new`
takes `&Path` arguments so the conversion lives in the contract layer.

A guard test reads `main.rs` and `contract.rs` (with `include_str!`) and
fails if `display().to_string()` reappears in them: the human output lives
in `render.rs`, so nothing in those two files legitimately renders a native
path. Contract tests also build documents from paths containing `\`, which
proves the conversion on every platform, and the Windows CI job exercises
the real paths.

**Alternatives considered**: a `JsonPath` newtype for every path field —
rejected for now: it changes about forty fields and their tests for the same
guarantee the guard test gives; converting only the fields the brief named —
rejected, the documentation promises `/` for every path.

## Risks / Trade-offs

- [A Windows script parsing `\` in today's JSON] → the documentation has
  always promised `/`, and Windows accepts `/` in paths; the release note
  says so.
- [A Unix path containing a literal `\` is altered] → already the behavior
  of `validate`, `sync` and `archive`; such names are not portable anyway.
- [A change created by the buggy `new change` records the internal `name:`]
  → it fails with `schema_not_found` naming that schema, as today; the
  migration note below explains the one-line fix.
- [The guard test is textual] → it targets an exact pattern in two files;
  its message says why, so a reviewer can tell a false positive.
- [Reserving `archive` breaks a project with a change folder named
  `archive`] → impossible: that folder is the archive itself.

## Migration Plan

No migration step. A change created before this fix with a schema whose
`name:` differs from its folder keeps the wrong name in `change.yaml`;
`codev status` fails with `schema_not_found` as before, and the user edits
`schema:` to the folder name. `codev schemas` now points the mismatch out.
