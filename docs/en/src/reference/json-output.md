# JSON output

Every command that works on a project accepts `--json`. The output is a
public, versioned contract — version 1 — that the skills rely on, and that
you can use in scripts and CI.

## Guarantees

- **One document.** stdout carries exactly one JSON document, pretty-printed,
  including when the command fails. Nothing else is printed to stdout.
- **A `status` array.** Every document has a `status` array, empty when all
  went well, listing warnings and errors otherwise.
- **The same shape on failure.** A failed command returns the shape of its
  successful output, with empty or `null` fields, plus the error in `status`.
  A consumer parses both cases with the same code.
- **Stable names.** Field names are camelCase. Codes are stable and meant to
  be tested; messages are meant to be read and may be reworded.
- **Portable paths.** Paths use `/` separators on every OS. Paths inside
  findings are relative to the project root.
- **Additive evolution.** Within version 1, fields may be added, never renamed
  or removed. Some optional fields are omitted when empty, as noted below.

The exit status follows the same rules as in human mode: `0` on success, `1`
on failure, and for `codev validate`, `1` when there are errors (or any
finding with `--strict`).

## The status array

```json
{
  "status": [
    {
      "level": "error",
      "code": "ambiguous_change",
      "message": "several active changes: add-audit-log, rework — specify which one"
    }
  ]
}
```

| Field | Values |
|---|---|
| `level` | `error` or `warning` |
| `code` | A stable identifier, such as `no_codev_root` |
| `message` | Human-readable text |

Common error codes:

| Code | Meaning |
|---|---|
| `no_codev_root` | No `_codev/` in the current directory or its parents |
| `no_active_change` | The project has no active change |
| `ambiguous_change` | Several active changes and no `--change` |
| `unknown_change` | The named change does not exist |
| `change_exists` | `new change` with a name already taken |
| `schema_not_found` | Unknown schema name |
| `no_artifact_ready` | `instructions` without an artifact, and none is ready |
| `template_not_found` | A schema references a missing template |
| `validation_failed` | `sync` or `archive` refused a change whose validation reports errors |
| `invalid` | An invalid file, or a delta that cannot be merged into its main spec |
| `unreadable` | A file could not be read |
| `write_failed` | A file could not be written |

Warnings reuse the codes of the condition they report, for example
`inherit_unresolved`, `git_source_unlocked`, `unknown_workflow` or
`artifact_skipped`.

## Documents by command

### init and update

`codev init --json` and `codev update --json`:

```json
{
  "root": "/home/you/acme-app",
  "created": ["…"],
  "updated": ["…"],
  "untouched": ["…"],
  "preserved": ["/home/you/acme-app/.claude/skills/codev-apply/SKILL.md"],
  "skills": ["codev-propose", "codev-explore", "…"],
  "status": []
}
```

`preserved` lists the skills left in place because they were edited by hand.

### new change

```json
{
  "changeName": "add-audit-log",
  "schemaName": "spec-driven",
  "changeRoot": "/home/you/acme-app/_codev/changes/add-audit-log",
  "created": [
    "/home/you/acme-app/_codev/changes/add-audit-log/change.yaml"
  ],
  "status": []
}
```

### list

```json
{
  "changes": ["add-audit-log"],
  "root": "/home/you/acme-app",
  "status": []
}
```

With `--specs`, the array is named `specs` and lists capability paths.

### status

```json
{
  "changeName": "add-audit-log",
  "schemaName": "spec-driven",
  "planningHome": "/home/you/acme-app",
  "changeRoot": "/home/you/acme-app/_codev/changes/add-audit-log",
  "applyRequires": ["tasks"],
  "isPlanningComplete": false,
  "artifacts": [
    {
      "id": "proposal",
      "outputPath": "proposal.md",
      "status": "ready",
      "requires": [],
      "missingDeps": []
    },
    {
      "id": "specs",
      "outputPath": "specs/**/*.md",
      "status": "blocked",
      "requires": ["proposal"],
      "missingDeps": ["proposal"]
    }
  ],
  "status": []
}
```

An artifact's `status` is `done`, `ready`, `blocked` or `skipped`.
`planningHome` is the project root; skills use it rather than assuming the
current directory.

### instructions

```json
{
  "changeName": "add-dark-mode",
  "schemaName": "spec-driven",
  "artifact": "design",
  "description": "Technical approach and decisions",
  "resolvedOutputPath": "/home/you/acme-app/_codev/changes/add-dark-mode/design.md",
  "instruction": "Write the document that explains HOW to implement the change.\n…",
  "template": "# Design: <change title>\n…",
  "language": "en",
  "context": [
    { "origin": "path:~/src/acme-standards", "text": "All services log in JSON." },
    { "origin": "project", "text": "TypeScript monorepo." }
  ],
  "rules": [
    { "origin": "project", "text": "Cite the decisions that constrain the approach." }
  ],
  "dependencies": [
    { "id": "proposal", "path": "/home/you/acme-app/_codev/changes/add-dark-mode/proposal.md", "done": true }
  ],
  "unlocks": ["tasks"],
  "decisions": [
    {
      "id": "0003",
      "qualifiedId": "project/0003",
      "title": "Use SQLite for persistence",
      "status": "accepted",
      "tags": [],
      "path": "_codev/decisions/0003-use-sqlite-for-persistence.md",
      "origin": "project"
    }
  ],
  "skipped": false,
  "status": []
}
```

| Field | Meaning |
|---|---|
| `resolvedOutputPath` | Where to write; may be a glob such as `…/specs/**/*.md` |
| `instruction`, `template` | From the schema; `null` when the schema defines none |
| `language` | The artifact language, `en` by default |
| `context`, `rules` | Blocks ordered from the most general source to the project, each with its `origin` (`project`, `path:<path>` or `git:<url>`) |
| `dependencies` | The artifacts this one requires, with their path and whether they are done |
| `unlocks` | Artifacts that become writable once this one exists |
| `decisions` | The decisions in effect — filled for `design`, empty for other artifacts |
| `skipped` | `true` when `skip_specs` disables this artifact: it must not be written |

### validate

```json
{
  "root": "/home/you/acme-app",
  "items": [
    {
      "kind": "change",
      "name": "add-audit-log",
      "path": "_codev/changes/add-audit-log",
      "findings": [
        {
          "path": "_codev/changes/add-audit-log/change.yaml",
          "line": 1,
          "severity": "error",
          "code": "zero_delta_without_marker",
          "message": "no delta file under `specs/` and `skip_specs: true` is not declared; add a delta or set `skip_specs: true` in change.yaml"
        }
      ]
    }
  ],
  "hasWarnings": false,
  "status": []
}
```

`kind` is `change`, `spec` or `decisions`. Findings are reported in `items`,
not in `status`: `status` only carries a failure of the command itself. See
[Validation codes](file-formats.md#validation-codes).

### sync and archive

```json
{
  "changeName": "add-dark-mode",
  "root": "/home/you/acme-app",
  "updated": [],
  "created": ["/home/you/acme-app/_codev/specs/ui/theme/spec.md"],
  "unchanged": [],
  "deleted": [],
  "movedTo": "/home/you/acme-app/_codev/changes/archive/2026-09-30-add-dark-mode",
  "status": []
}
```

`codev sync --json` returns the same document without `movedTo`. `deleted`
lists main specs removed by a change that retires a capability.

### schemas

```json
{
  "schemas": [
    { "name": "spec-driven", "origin": "built-in", "flow": ["proposal", "specs", "design", "tasks"] }
  ],
  "root": "/home/you/acme-app",
  "status": []
}
```

`origin` is `project` or `built-in`.

### decision

`codev decision list --json` returns `{ "root", "decisions": [...], "status" }`,
and `codev decision show --json` returns `{ "root", "decision", "content",
"status" }`, where `content` is the full Markdown of the file. A decision is:

```json
{
  "id": "0001",
  "qualifiedId": "project/0001",
  "title": "Use PostgreSQL for persistence",
  "status": "superseded",
  "date": "2026-09-30",
  "tags": [],
  "supersedes": [],
  "deviatesFrom": [],
  "path": "…/_codev/decisions/0001-use-postgresql-for-persistence.md",
  "origin": "project",
  "inEffect": false,
  "supersededBy": "project/0003"
}
```

`deviatedBy` is added for an inherited decision set aside by a local
deviation. `origin` is `project`, `path:<path>` or `git:<url>`.

The commands that create or seal decisions return:

| Command | Fields besides `root` and `status` |
|---|---|
| `decision new` | `decision`, `path`, `bodySha256` (only with `--status accepted`) |
| `decision accept` | `decision`, `path`, `bodySha256` |
| `decision supersede` | `newDecision`, `newPath`, `oldId`, `oldQualifiedId`, `oldPath` |
| `decision seal` | `seal` (`id`, `bodySha256`, `sealedAt`), `wasNoop` |
| `decision deviate` | `decision`, `path`, `targetQualifiedId`, `bodySha256` |
| `decision promote` | `decision`, `path`, `bodySha256`, `sourceChange`, `designPath` |

For example, `codev decision accept 0001 --json`:

```json
{
  "root": "/home/you/acme-app",
  "decision": {
    "id": "0001",
    "qualifiedId": "project/0001",
    "title": "Use PostgreSQL for persistence",
    "status": "accepted",
    "date": "2026-09-30",
    "tags": [],
    "supersedes": [],
    "deviatesFrom": [],
    "path": "_codev/decisions/0001-use-postgresql-for-persistence.md",
    "origin": "project",
    "inEffect": true,
    "supersededBy": null
  },
  "path": "/home/you/acme-app/_codev/decisions/0001-use-postgresql-for-persistence.md",
  "bodySha256": "sha256:167c548eb2750fa6f7b25dd836916adf29d02a95adcf6510a7cc11873e994deb",
  "status": []
}
```

A refused acceptance keeps the same shape, with `decision` and `path` set to
`null` and the code in `status`: `unknown_decision_id`,
`cannot_accept_inherited` or `decision_not_proposed`.

### sources

`codev sources list --json` returns `{ "root", "sources": [...], "status" }`,
where a source is:

```json
{
  "type": "git",
  "address": "git@github.com:acme/codev-standards.git",
  "state": "locked",
  "gitRef": "main",
  "sha": "12958274cbed0eb66985f57d3272c6d3134b672d",
  "resolvedPath": "/home/you/.cache/codev/content/12958274cbed0eb66985f57d3272c6d3134b672d"
}
```

`type` is `path` or `git`; `state` is one of `resolved`, `unreadable`,
`unlocked`, `locked`, `needs_update`. `gitRef`, `subpath`, `sha` and
`resolvedPath` are omitted when they do not apply.

`codev sources show --json` returns `{ "source", "filesExposed", "root",
"status" }`. `codev sources update --json` returns `{ "root", "changes",
"lockWritten", "status" }`, where each change has a `kind` (`added`, `moved`
or `unchanged`), `url`, `gitRef`, `from` (for `moved`) and `to`.
