# Design: Declarative roles — one catalog, configurable activation and criteria

## Context

Today a role is prose in three places: the `Role:` / `Done when:` block at
the top of each `instruction` in `assets/schemas/spec-driven/schema.yaml`,
the steps of `assets/workflows/propose.md` (understand, study, trace,
challenge) and `assets/workflows/apply.md` (implement, review), and the two
`roles.md` chapters. `codev instructions` hands the schema text to the skill
unchanged; nothing in the binary knows that a role exists.

The configuration is read by `codev-engine::config` (`ProjectConfig`,
tolerant top level, strict `inherits` and `changelog`); a change's
`change.yaml` by `codev-engine::metadata` (`deny_unknown_fields`). Schemas
are parsed and validated in `codev-core::schema`. The JSON contract lives
in `codev-cli::contract`.

## Goals / Non-Goals

**Goals:**

- One source of truth per role, served by the binary, checked against the
  documentation by a test.
- A project switches a role on or off and extends its criteria; a change
  overrides the activation.
- The skills keep the procedure and stop restating criteria.

**Non-Goals:**

- Expert roles (security, privacy, accessibility, performance, ops,
  migration) and domain-triggered activation (`on_domain`) — next change.
- Custom or inherited step roles; roles contributed by inherited sources.
- Per-change extra criteria: `change.yaml` overrides `when` only.

## Decisions

### Decision: The catalog is data; resolution is a pure function of the core

The built-in step roles live in `assets/roles.yaml`, embedded at compile
time and parsed by a new `codev-core::roles` module with `serde_norway`
(decision 0006). The same module holds the setting types, the resolution
(`catalog + project settings + change settings → resolved roles`) and the
findings. It touches no file: the engine reads `config.yaml`,
`change.yaml` and the schema, then calls the core — decision 0001 (deciding
is not executing) and decision 0002 (the core sits at the bottom of the
crate graph; the CLI only translates to the JSON contract).

**Alternatives considered**: hard-coding the step roles as Rust constants
(a role edit would need a code review of Rust rather than of text, against
the "schemas are data" principle stated in `codev-core::schema`); resolving
in the engine (puts decisions next to I/O, against 0001).

### Decision: Step roles in the catalog, artifact roles in the schema

An artifact role belongs to the schema that defines the artifact: a custom
schema defines its own, as it already does in prose. A step role belongs
to the skills' workflow, which the binary ships: it goes in the catalog.
`Artifact` gains an optional `role:` (`id`, `name`, `question`,
`done_when`), parsed with `deny_unknown_fields` like the rest of the
schema. Schema validation refuses a role id that is not kebab-case, is
declared twice, collides with a built-in step role, or whose `name` lacks
`en`.

**Alternatives considered**: every role in the catalog, artifacts
referring to it by id (a custom schema could not define a role without a
codev release); keeping artifact roles as prose (they could not be
extended or listed).

### Decision: Steps are named, artifact roles sit at `plan.write`

The steps are `plan.understand`, `plan.investigate`, `plan.write`,
`plan.trace`, `plan.challenge`, `code.implement`, `code.review` — a closed
list in the core, in workflow order. Every artifact role is at
`plan.write` and carries its `artifact` id. Listing roles orders them by
step, then by catalog or schema order, which gives the reading order of a
change (analyst … lead, tracer … newcomer) without a separate rank.

**Alternatives considered**: one step per artifact (`plan.write.proposal`
— grows with every schema); no step for artifact roles (they could not be
listed in workflow order, nor asked for with `--at`).

### Decision: Settings are text first, validated second

`roles:` deserializes each entry from a YAML scalar (short form) or a
mapping (`when`, `done_when`, `deny_unknown_fields`). The activation is
kept as text and validated by the core, so that `newcomer: off` or
`newcomer: false` yields `invalid_role_activation` naming the accepted
values rather than a serde type error. An unknown key inside a role's
mapping fails the configuration (`invalid`), like `changelog`: a misspelled
`done-when` would silently drop criteria. `change.yaml`'s `roles:` accepts
the scalar form only.

`unknown_role`, `invalid_role_activation` and `role_not_disableable` are
findings, not parse errors: `codev validate` reports them all, at the line
of the role's key, as errors of the `config` item or of the change's item.
`codev status`, `codev instructions` and `codev roles` resolve the roles and
fail with the first finding's code, through a dedicated `EngineError`
variant whose `code()` is that code.

**Alternatives considered**: failing the configuration load on any role
error (every command, `codev validate` and `codev doctor` included, would
stop at the first error instead of reporting them).

### Decision: Which roles cannot be disabled

Artifact roles cannot be disabled (their artifact is still written), and
neither can the `developer`, marked `required: true` in the catalog:
without it `/codev-apply` has nothing to do. Every other step role can be.

**Alternatives considered**: every step role disableable, as the request's
"default for every current role" allows (turning off the developer would
make `/codev-apply` a no-op, a configuration that is only ever a mistake).

### Decision: Origins and names

A criterion carries its origin: `built-in` (catalog), `schema:<name>`
(schema), `project` (config). `whenOrigin` is `default`, `project` or
`change`. `name` is a mapping from language codes to display names; the
served `name` is the configured `language`'s, else English. Inherited
sources' `roles:` are not read: their keys are already ignored apart from
context and rules (decision 0005 keeps them read-only and partial).

**Alternatives considered**: a plain string `name` (the French book could
not be checked against the data).

### Decision: Serving roles — `codev roles` and `role` in instructions

`codev roles` is a new command with a v1 JSON document (`RolesV1`); one
`RoleV1` shape is shared by `codev roles` and the additive `role` field of
`InstructionsV1`. No new skill and no command file: the skills already
allow `Bash(codev:*)` (decision 0004).

**Alternatives considered**: roles only through `codev instructions` (step
roles have no artifact); a `roles` array in `codev status` (status is
called often, and would carry criteria it does not need).

### Decision: Skills ask, then apply or skip

`propose.md` and `apply.md` each get one short rule — "at a role step, run
`codev roles --at <step> …`; apply every criterion returned; no role
returned → skip the step and say so in the summary" — and each role step
names its step id. The criteria leave the skill texts; the procedure
(outputs, caps, guardrails, the Jira rules, the "never change behavior"
of the review) stays. `plan.understand` runs before the change exists, so
it is asked without `--change`.

**Alternatives considered**: one `codev roles --json` call at the start
(simpler, but a per-step call keeps
the criteria next to where they are applied).

### Decision: Doctor reports, validate judges

The doctor's new `config.roles` check is `ok` with a message listing the
active and disabled roles; when settings are invalid it is a `warning`
(`roles_invalid`, fix `codev validate`) — the `validate` check already
counts the errors as `missing`, so the strict exit code is driven once.

### Decision: The documentation is checked against the catalog

`roles.md` gets a "Built-in roles" table (`id`, name, step). A test in
`codev-cli::docs` reads both embedded editions and compares the table's ids
and names with the catalog plus the default schema's artifact roles. The
other skills' stances (explore, onboard, configure, update, sync/archive)
stay documented but are no longer called roles.

## Risks / Trade-offs

- [The skills lose their criteria if `codev roles` fails] → it fails only
  on an invalid setting, which `codev status` (run by both skills first)
  already reports; the skills stop on a failing `codev` call.
- [Criteria drift while moving] → texts move verbatim (list punctuation
  aside); tests assert key phrases on the catalog and the schema roles, as
  they did on the skill bodies.
- [Older installed skills with a newer binary lose the schema's prose role
  block] → the skills' version stamp already triggers the update notice;
  `codev update` regenerates them.
- [A project setting checked against the configured schema while a change
  uses another] → rare; the change's commands report the finding with its
  code, and `codev validate` names it.

## Open Questions

None.
