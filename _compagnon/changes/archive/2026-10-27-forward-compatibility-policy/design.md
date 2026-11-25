# Design: A forward compatibility policy for the files compagnon reads

## Context

Each file compagnon reads chose its own answer to a key or a value it does not
know (proposal, Why). What the code does today:

| File | Written by | Unknown key today | Where |
|---|---|---|---|
| `_compagnon/config.yaml`, top level, `mcp`, `review` | hand | warning `unknown_config_key` | `crates/compagnon-engine/src/config.rs` (`PROJECT_KEYS`, `SECTION_KEYS`) |
| `config.yaml`, an `inherits` entry, `changelog` | hand | error `invalid`, with the `compagnon_version` hint | `InheritSource`, `ChangelogConfig` (`deny_unknown_fields`) |
| `config.yaml`, a role's mapping under `roles:` | hand | error `invalid` | `RoleSettingMapping` in `crates/compagnon-core/src/roles.rs` |
| `change.yaml` | `compagnon new change`, then hand | error `invalid` | `ChangeMetadata` in `crates/compagnon-engine/src/metadata.rs` |
| a project `schema.yaml` | hand | error `schema_unreadable` | `Schema`, `Artifact`, `Apply` in `crates/compagnon-core/src/schema.rs`, `ArtifactRole` in `roles.rs` |
| ADR frontmatter | `compagnon decision …`, then hand | error `decision_missing_field`, ADR not read | `RawFrontmatter` in `crates/compagnon-core/src/decisions/parser.rs` |
| `review.md`, `corrections.md` | the skills | warning; a newer format marker is not checked, the archive refuses it | `crates/compagnon-core/src/review/` |
| `seal.yaml` | compagnon | error `seal_invalid`; a newer `version` fails with `seal_version_unsupported`, with no way out in the message | `crates/compagnon-core/src/decisions/seal.rs` |
| built-in schema, built-in roles | the binary | error | same structs, `assets/` |

`compagnon_version` exists (specs `self-update`, `doctor`): `compagnon status`
reports whether the running compagnon satisfies it, `compagnon doctor` gives the
fix, and the skills offer to update. No command refuses on it.

`compagnon decision accept` rewrites a frontmatter by rendering the known
fields again (`rewrite_frontmatter_status` in
`crates/compagnon-engine/src/decisions_actions.rs`): a field it does not know
would be lost the day it is tolerated.

## Goals / Non-Goals

**Goals:**

- One written policy, as a decision, that the next change adding a key,
  a value or a format follows instead of re-deciding.
- Align today's hand-written files with it, and give `compagnon_version` the
  refusal the policy relies on.

**Non-Goals:**

- Making a compagnon that predates this change behave differently: it is
  released. The policy protects from the first release that ships it on.
- A command that migrates formats: none is needed today (point 6 of the
  policy says how one would be introduced).
- Changing the strictness of inherited sources' `inherits` entries and of
  `changelog`: they already fail closed.
- Unknown *values* of a closed set in a hand-written file (`review.mode`,
  a role activation, an ADR `status`): they stay errors, since a value
  cannot be ignored without guessing what the author meant. Pointing
  their messages to `compagnon_version` is left to a later change.

## Decisions

### Decision: A file written for a newer compagnon warns, unless ignoring it would weaken a check

> Promoted to ADR **0013** — see `_compagnon/decisions/0013-a-file-written-for-a-newer-compagnon-warns-unless-ignoring-it-would-weaken-a-check.md`.

### Decision: Each hand-written file gets its own warning code, and the existing reporting channels

`change_unknown_key`, `schema_unknown_key` and `decision_unknown_field`,
next to the existing `unknown_config_key`; a key of a role's mapping is an
`unknown_config_key` named `roles.<id>.<key>`, since it lives in
`config.yaml`. Each code follows its file's existing naming —
`decision_*` and "field" for ADRs (`decision_missing_field`),
`<file>_unknown_*` like `review_unknown_field`. The message is built by one
function shared with `unknown_key_message`, taking the file and the key,
so the wording and the `compagnon_version` quote cannot drift.

They travel where warnings already travel: the `status` array of the
commands that load the file (`Warning` in `compagnon-engine`), and a
`warning` finding in `compagnon validate` at the key's line — the change's
item for `change.yaml`, the `decisions` item for an ADR, the `config`
item for a project schema (it describes the project, not a change, and a
schema no change uses yet must still fail `--strict` in CI). No JSON field
is added: the codes are values of existing fields.

Unknown keys are collected the way `config.rs` already does — the typed
parse without `deny_unknown_fields`, then a second read of the generic
YAML mapping against a list of known keys per level, with a
`every_known_key_is_recognized` test per struct keeping the list in sync.
The same type parses the built-in schema: `Schema::parse` returns the
unknown keys, and the built-in schema has a test asserting there are none
(policy point 4), rather than a second, strict parse path.

**Alternatives considered**:

- One code for every file (`unknown_key`): fewer codes, but the agent and
  the user lose which file to open, and `unknown_config_key` cannot be
  renamed without breaking installed skills and CI scripts.
- `#[serde(flatten)] extra: BTreeMap<…>` to capture unknown keys: no line
  numbers and a second mechanism beside `config.rs`'s.

### Decision: Only `validate`, `sync` and `archive` refuse a compagnon outside `compagnon_version`

They are the commands whose success is a verdict on the project — CI's
`compagnon validate --strict`, the archive gate — or which write the main
specs on that verdict. The refusal happens right after the configuration
is read, before any check or write, with `compagnon_version_unsatisfied` and
the fix `compagnon doctor` already computes (reused, not reimplemented). Every
other command keeps working, so that `compagnon status`, `compagnon doctor` and
the skills can still say what to install, and `compagnon self-update` can
install it.

The refusal covers a compagnon above an upper bound too (`^0.6` with 0.7
running). An upper bound is the team's explicit statement that it has not
moved to a newer release — typically because that release writes files
(a `review.md` of a newer format, a seal) its other members cannot check
yet. A verdict given by a compagnon the team excluded is the case point 2
closes. The fix is the one `compagnon doctor` already gives: widen
`compagnon_version` once the team moves, or install the version it names with
`compagnon self-update --version <X.Y.Z>`; the release note says so.

**Alternatives considered**:

- Refuse every command: the user could no longer run `compagnon status` or
  `compagnon doctor` to learn what is wrong, and skills would fail at their
  first step.
- Refuse `compagnon archive` only: CI's `compagnon validate --strict` would pass on
  an old compagnon that ignored a gating key — the case the policy closes.
- Also refuse the sealing commands (`decision accept`, `decision seal`):
  they run no check, and sealing records the body's hash, which an older
  compagnon computes the same way.
- A `--ignore-compagnon-version` escape hatch: it reopens what point 2 closes;
  the fix is one `compagnon self-update`.

### Decision: Role settings, `change.yaml`, project schemas and ADR frontmatter move to warnings; `seal.yaml` stays strict

Each was checked against point 2 — would ignoring an unknown key weaken a
check?

- **Role settings.** The keys added so far (`done_when`, `model`) change
  which criteria and which model a reviewer gets; the strictness was meant
  for typos (`done-when`). A typo is now a warning in every command and a
  failure under `--strict`. A future key whose ignoring would drop a
  review (for instance a role made blocking) is a point 2 key and comes
  with a `compagnon_version` bump. Decision 0012 already records the cost of
  the current strictness: an older case tree rejects the appended
  `roles:` key.
- **`change.yaml`.** The markers there fail closed by construction:
  without `retire_capabilities`, an older compagnon refuses to delete a spec;
  without `skip_specs`, it requires a delta. An unknown key under
  `review:` leaves the mode as written. The values (`review.mode`, a role
  activation) stay strict.
- **Project `schema.yaml`.** A future key that changes what
  `apply.requires` gates would be a point 2 key. Ignoring a descriptive
  one (`description`, a role's extra field) changes nothing checked.
- **ADR frontmatter.** No field drives a check except `status` and
  `supersedes`, both known. `compagnon decision accept` keeps unknown fields
  when it rewrites a frontmatter: it renders the known fields as today,
  then appends the unknown ones as written, in their order — the body,
  hence the seal, is untouched (decision 0001: the plan is computed
  whole before writing).
- **`seal.yaml`** is written by compagnon and has a `version` (point 3): an
  unknown field in a known version is corruption, and stays
  `seal_invalid`. A newer `version` keeps `seal_version_unsupported`; its
  message gains the way out, and `compagnon validate` reports that code
  instead of a generic `unreadable`.

**Alternatives considered**:

- Keep role settings strict as "fail closed": they would be the only
  hand-written section still breaking every command for a teammate one
  release behind, which is what the four `R-migration` findings were
  about.
- Rewrite only the `status:` line of a frontmatter instead of rendering
  it: smaller, but `decision accept` would stop normalizing a hand-written
  frontmatter (quoting, field order) as the current tests expect.

### Decision: No tasks rule; the policy is applied through this decision

The project rule for designs already makes every design cite the
decisions in force, so the next change that adds a key meets this one. A
`tasks:` or `design:` rule restating it would duplicate a decision, which
a rule must never do. The migration expert's built-in text is not
changed: it is shared by every project, and changing it is a workflow
change under decision 0008. The decisions in force reach the plan
through `compagnon instructions` (`decisions`), and a reviewer may read
`_compagnon/decisions/` while it challenges the plan.

**Alternatives considered**:

- A project `done_when` on `migration` in this repository's
  `_compagnon/config.yaml`: explicit, but a restatement of the decision; left
  as an open question for the author.

### Decisions in force

- 0001 — the frontmatter rewrite stays a pure function inside the
  accept plan.
- 0002 — warnings are typed in `compagnon-engine` (`Warning`) and in the core's
  findings; the CLI only renders them.
- 0005 — inherited sources stay strict and read-only; their `roles:` and
  `review:` are still not read.
- 0006 — `serde_norway` stays the parser; unknown keys are read from its
  generic `Value`.
- 0008 — this change modifies no skill, workflow, role text or JSON field;
  it adds code values to existing `status` arrays and findings. No
  `compagnon-evals` step covers it, and none is needed.
- 0012 — its consequence on `--fixture-config` (an old case tree rejects
  appended keys) is what the role-settings warning removes for future
  trees.

## Risks / Trade-offs

- [A teammate relies on the old hard error to catch a typo locally] →
  The warning is printed by every command that reads the file, and
  `compagnon validate --strict`, which `compagnon init` puts in CI, fails on it.
- [A team adds a point 2 key without raising `compagnon_version`, and an
  older compagnon ignores it] → The change that introduces such a key says so
  in its release note with the minimum version (point 5), and the
  migration expert checks it; a check by compagnon itself is an open question.
- [The CI job `compagnon init` writes pins the binary (`COMPAGNON_VERSION=<version>`)
  and starts failing once the team raises `compagnon_version` above it] → It is
  the intended fail-closed behavior: the pinned CI is exactly the older
  reader the policy guards against. The message names the requirement and
  the running version, the release note marks it **BREAKING**, and the
  documentation says to bump the CI pin with `compagnon_version`.
- [This repository raises `compagnon_version` before the release that
  satisfies it ships, and its own `compagnon validate` refuses] → Raise it
  only once the release is out, as the archived `R-migration-2` of
  `expert-triggers-and-stack-filter` already noted for `doctor --strict`.
- [A project with an upper bound (`^0.6`) upgrades compagnon and its verdict
  commands start refusing] → Intended (see the decision above); the
  release note's **BREAKING** item and the `compagnon_version` documentation
  tell the team to widen the requirement when it moves.
- [Installed skills answer **Later** to an unsatisfied requirement by
  continuing the action, and `/compagnon-sync` or `/compagnon-archive` then meet
  the refusal] → The refusal message gives the fix, and the documentation
  of the update notice says so; making the skills stop the action is a
  change to the workflow under decision 0008, left to a follow-up change
  (review record, `R-migration-2`).
- [Compagnons from 0.6.1 back keep refusing these files] → Unavoidable; the
  documentation's FAQ says which versions refuse, as it already does for
  `config.yaml`.
- [An ADR frontmatter rewritten by `decision accept` changes the order of
  unknown fields relative to known ones] → Unknown fields keep their
  relative order after the known ones; no field is lost, and the body and
  its seal are untouched.

## Migration Plan

Nothing to migrate: every file accepted today is accepted the same way,
and files refused today for an unknown key are now read with a warning.
The only behavior that becomes stricter is the refusal of
`validate`/`sync`/`archive` outside `compagnon_version`, which affects only a
project that set a requirement its compagnon does not meet; the fix is
`compagnon self-update`. Rollback is a revert: no file is written in a new
format.

## Open Questions

- Should a newer compagnon itself report a point 2 key used while
  `compagnon_version` admits releases older than the key — a `validate`
  error such as `compagnon_version_too_low` — instead of relying on the
  release note and the review? No such key exists today, so it can wait
  for the first one.
- Should this repository give the `migration` role a project `done_when`
  pointing to this decision, or is the decision being in force enough?
