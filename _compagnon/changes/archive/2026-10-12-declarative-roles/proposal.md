# Proposal: Declarative roles — one catalog, configurable activation and criteria

## Why

Roles are prose scattered across the default schema's instructions, the
`propose` and `apply` skills and both Roles chapters: adding or tuning one
means editing several prompts and two documents, and they drift each time.
A team also has no way to turn a review step off, or to add its own "done
when" criteria, without forking the skills.

## What Changes

- Roles become data. Ten roles, each with an id, a display name (English
  and French), the question it owns and its "done when" criteria:
  `analyst`, `investigator`, `tracer`, `skeptic`, `developer` and
  `newcomer` in a built-in catalog of step roles embedded in codev, each
  bound to a workflow step (`plan.understand`, `plan.investigate`,
  `plan.trace`, `plan.challenge`, `code.implement`, `code.review`);
  `product`, `qa`, `architect` and `lead` as artifact roles declared in the
  default schema, bound to the step `plan.write`. Today's criteria move
  verbatim into the data.
- A schema artifact may declare a structured `role:` (id, name, question,
  done when). The default schema's instructions no longer start with a
  `Role:` / `Done when:` prose block: the role comes as data.
- New `roles:` key in `_codev/config.yaml`: per role, `always` or `never`
  (short form), or `when:` plus extra `done_when:` criteria added to the
  built-in ones, with the origin `project`. Artifact roles and the
  `developer` cannot be disabled; their criteria can be extended. Inherited
  sources do not contribute roles.
- New `roles:` key in a change's `change.yaml`, overriding `when` for that
  change only.
- Errors: an unknown role id (`unknown_role`), an activation other than
  `always` or `never` (`invalid_role_activation`), disabling a role that
  cannot be disabled (`role_not_disableable`). `codev validate` reports them
  as errors; `codev status`, `codev instructions` and `codev roles` fail
  with the same code.
- `codev instructions <artifact> --json` gains an additive `role` field: the
  artifact's role with its merged criteria and their origins; the human
  output gains a `Role:` section.
- New command `codev roles [--at <step>] [--change <name>] [--json]`: the
  roles active at a step with their merged criteria, or every role with its
  activation when `--at` is omitted.
- The `propose` and `apply` skills ask `codev roles --at <step>` at each
  role step, apply the criteria it returns, and skip a step whose role is
  disabled, saying so in their summary. They no longer restate the
  criteria.
- `codev doctor` gains an informational `config.roles` check listing the
  active and disabled roles.
- The Roles chapter is organized around the catalog — ids, display names,
  steps — and gains a "Configuring roles" section; a test checks that both
  editions list exactly the catalog's roles.

## Capabilities

### New Capabilities

- `roles` — the catalog of roles, their configuration per project and per
  change, the error codes, and the `codev roles` command.

### Modified Capabilities

- `instructions` — the `role` field and the `Role:` section of the human
  output.
- `skills` — artifact roles as schema data; the `propose` and `apply`
  skills apply the roles served by `codev roles` and skip a disabled step.
- `schemas` — a schema artifact may declare a `role:`, validated with the
  schema.
- `doctor` — the `config.roles` check.
- `docs` — the Roles chapter lists exactly the catalog's roles, in both
  languages.

### Removed Capabilities

None.

## Impact

- `assets/roles.yaml` (new), `assets/schemas/spec-driven/schema.yaml`,
  `assets/workflows/propose.md`, `assets/workflows/apply.md`, and the
  regenerated skills under `.claude/skills/`.
- `codev-core`: new `roles` module (catalog, settings, resolution,
  findings), `schema` (artifact `role:`), `doctor` (new check).
- `codev-engine`: configuration (`roles:` key), change metadata (`roles:`),
  role resolution against a change's schema, instructions, validation.
- `codev-cli`: the `codev roles` command, its JSON contract v1 document,
  the `role` field of `InstructionsV1`, human renderings, doctor facts.
- Documentation in `docs/en` and `docs/fr`: `roles.md` (catalog, steps,
  "Configuring roles"), `skills.md`, `reference/configuration.md`
  (`roles:`), `reference/cli.md` (`codev roles`), `reference/json-output.md`,
  `reference/file-formats.md` (schema `role`, `change.yaml` `roles`),
  `guides/custom-schemas.md`.
- JSON contract v1: additive only — a new document, a new field, a new
  doctor check id.
- A custom schema whose instructions carry roles as prose keeps working
  unchanged.

## Release note

- Added: Roles are configurable: `roles:` in `_codev/config.yaml` turns a
  review role on or off (`always` / `never`) and adds your own "done when"
  criteria; `roles:` in a change's `change.yaml` overrides the activation
  for that change.
- Added: `codev roles` lists the roles active at a workflow step with their
  criteria, or every role with its activation; `codev instructions --json`
  returns the artifact's role in a `role` field.
- Added: A schema artifact may declare its role as data (`role:` with id,
  name, question and "done when" criteria).
- Changed: The default schema's instructions no longer start with a
  `Role:` prose block; the role comes in the `role` field of
  `codev instructions`, and `/codev-propose` and `/codev-apply` take their
  review criteria from `codev roles`.
- Added: `codev doctor` lists the active and disabled roles.
