# Custom schemas

A schema defines the planning workflow of a change: which artifacts it has,
which ones depend on which, and the instruction and template the agent
follows for each. codev embeds one schema, `spec-driven` — proposal, specs,
design, tasks — which suits most projects. When it does not, you can define
your own, as plain data files: no code, no rebuild.

## Where schemas live

A project schema is a folder under `_codev/schemas/`:

```text
_codev/schemas/
└── lite/
    ├── schema.yaml
    └── templates/
        ├── brief.md
        ├── spec.md
        └── tasks.md
```

`codev schemas` lists what is available:

```text
Available schemas:
  lite (project)
    brief → specs → tasks
  spec-driven (built-in)
    proposal → specs → design → tasks
```

A project schema named `spec-driven` replaces the built-in one, without
changing anything else.

## Writing a schema

```yaml
# _codev/schemas/lite/schema.yaml
name: lite
description: Small changes — a short brief, spec deltas and a task list
artifacts:
  - id: brief
    generates: brief.md
    description: What changes, and why
    template: brief.md
    instruction: |
      Explain in a few sentences what changes and why.
  - id: specs
    generates: "specs/**/*.md"
    description: Spec deltas
    template: spec.md
    requires: [brief]
  - id: tasks
    generates: tasks.md
    template: tasks.md
    requires: [specs]
apply:
  requires: [tasks]
  tracks: tasks.md
```

| Key | Required | Meaning |
|---|---|---|
| `name` | yes | Schema name, in kebab-case |
| `version` | no | Schema format version, `1` by default |
| `description` | no | One-line description |
| `artifacts` | yes | The artifacts of a change, at least one |
| `artifacts[].id` | yes | Artifact id, in kebab-case, unique |
| `artifacts[].generates` | yes | Output path relative to the change folder; may be a glob such as `specs/**/*.md` |
| `artifacts[].description` | no | Short description, shown to the agent |
| `artifacts[].template` | no | Template file name, looked up in the schema's `templates/` folder |
| `artifacts[].instruction` | no | Guidance for the agent writing this artifact |
| `artifacts[].requires` | no | Ids of the artifacts that must exist first |
| `apply.requires` | yes | Artifacts that must exist before implementation starts |
| `apply.tracks` | yes | The file whose checkboxes track implementation progress |
| `apply.instruction` | no | Guidance for the implementation phase |

Unknown keys are rejected, so a typo cannot silently produce a workflow that
behaves differently from what you wrote. codev also rejects a schema whose
dependencies reference a missing artifact or form a cycle, and names the
artifacts involved.

An artifact is **done** when its file exists — or, for a glob, when at least
one file matches. It is **ready** when all the artifacts it requires are done.

## Using a schema

Choose a schema per change:

```bash
codev new change quick-fix --schema lite
```

or make it the project default in `_codev/config.yaml`:

```yaml
schema: lite
```

The schema is recorded in each change's `change.yaml`, so changing the default
does not affect changes already in progress.

The skills work with any schema: `/codev-propose` writes whatever artifacts
are ready, in dependency order, using `codev instructions`. Per-artifact
`rules:` in the configuration are keyed by artifact id, so rules written for
`brief` apply to the `brief` artifact of the `lite` schema.

## Behaviors tied to conventions

A few behaviors rely on conventions rather than configuration. Keep them in
mind when you design a schema:

- **Spec deltas are files under `specs/`.** Validation, sync and archive read
  deltas from `specs/` in the change folder. An artifact that writes elsewhere
  is not merged into the main specs.
- **`skip_specs: true`** skips every artifact whose `generates` starts with
  `specs/`, whatever its id.
- **Decisions are injected into the artifact whose id is `design`.** Name your
  design artifact `design` if you want the agent to receive the decisions in
  effect while writing it.
