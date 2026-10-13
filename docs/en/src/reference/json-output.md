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
- **Portable paths.** Every path, absolute or relative, uses `/` separators
  on every OS — on Windows, `C:/work/acme-app`. Paths inside findings are
  relative to the project root.
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
| `invalid_change_id` | A change name that is not kebab-case, or the reserved `archive`; the message names the rule it breaks |
| `change_exists` | `new change` with a name already taken |
| `schema_not_found` | Unknown schema name |
| `schema_invalid`, `schema_unreadable` | A schema file that codev refuses: a structural defect, or invalid YAML or an unknown key |
| `unknown_artifact` | `instructions` with an artifact that the change's schema does not have |
| `no_artifact_ready` | `instructions` without an artifact, and none is ready |
| `template_not_found` | A schema references a missing template |
| `validation_failed` | `sync` or `archive` refused a change whose validation reports errors |
| `review_blocked` | `archive` refused a change whose review record is missing for a reviewed step, holds a high-severity finding still open or disputed, or comes from a newer codev; the message names each reason |
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
  "integration": [
    {
      "step": "claude_md",
      "action": "appended",
      "path": "CLAUDE.md",
      "reason": null
    },
    {
      "step": "ci",
      "action": "skipped",
      "path": ".github/workflows/codev.yml",
      "reason": ".github/workflows/ci.yml already runs codev validate"
    },
    {
      "step": "pr_template",
      "action": "created",
      "path": ".github/PULL_REQUEST_TEMPLATE.md",
      "reason": null
    }
  ],
  "status": []
}
```

`preserved` lists the skills left in place because they were edited by hand.
`integration` has one entry per repository integration step: `step` is
`claude_md`, `ci` or `pr_template`; `action` is `created`, `appended`,
`refreshed`, `unchanged` or `skipped`; `path` is relative to the project, or
`null`; `reason` explains a skipped step, or is `null`. `codev update` only
lists the managed files it found. The `codev doctor` hint and the GitLab
snippet are human output only.

Warnings specific to `init`: `inherit_not_written` (`--inherit` with an
existing `config.yaml`), `inherit_pin_failed` (a git source could not be
pinned) and `managed_section_malformed` (unbalanced codev markers; the file
is left untouched). An invalid `--inherit` value fails with
`invalid_inherit`.

### doctor

`codev doctor --json`:

```json
{
  "root": "/home/you/acme-app",
  "checks": [
    {
      "id": "config.rules",
      "group": "configuration",
      "status": "missing",
      "code": "rules_missing",
      "message": "no rules: in _codev/config.yaml",
      "fix": "/codev-configure"
    },
    {
      "id": "config.language",
      "group": "configuration",
      "status": "ok",
      "code": "ok",
      "message": "language: is set",
      "fix": null
    },
    "…"
  ],
  "summary": {
    "total": 14,
    "missing": 2,
    "warnings": 1
  },
  "status": []
}
```

Each check has a stable `id`, its `group`, a `status` (`ok`, `missing` or
`warning`), a stable `code` (`ok` when the check passes), a `message`, and a
`fix` — the command or skill to run — or `null` when `ok`. Checks come in the
order of the table below; `pr_template` is only present for GitHub
repositories. `summary.missing` drives the exit status of `--strict`.

| Check `id` | `group` | Codes when not `ok` |
|---|---|---|
| `config.rules` | `configuration` | `rules_missing` |
| `config.language` | `configuration` | `language_missing` |
| `config.keys` | `configuration` | `unknown_config_key` (warning) |
| `config.roles` | `configuration` | `roles_invalid` (warning); when `ok`, `message` lists the active roles, the roles active on domain with their domains, and the disabled ones |
| `skills` | `configuration` | `skills_missing`, `skills_outdated` |
| `sources` | `configuration` | `source_unpinned`, `lock_stale` (warning) |
| `codev_version` | `configuration` | `codev_version_unsatisfied`, `update_available` (warning) |
| `specs` | `planning` | `no_main_spec` |
| `decisions` | `planning` | `no_accepted_decision` (warning) |
| `validate` | `planning` | `validate_errors`, `validate_warnings` (warning) |
| `claude_md` | `repository` | `claude_md_section_missing` |
| `ci` | `repository` | `ci_validate_missing` |
| `pr_template` | `repository` | `pr_template_section_missing` |
| `changelog` | `repository` | `changelog_missing` (warning), `changelog_outdated` (warning) |

### changelog

`codev changelog --format json` (the `--json` of this command):

```json
{
  "root": "/home/you/acme-app",
  "since": "v1.3.0",
  "until": null,
  "release": null,
  "entries": [
    {
      "category": "Added",
      "text": "`--format json` on `codev export`.",
      "breaking": false,
      "change": "2026-10-02-export-json",
      "archivePath": "_codev/changes/archive/2026-10-02-export-json",
      "source": "release_note"
    }
  ],
  "markdown": "## [Unreleased]\n\n### Added\n\n- `--format json` on `codev export`.\n",
  "written": null,
  "status": []
}
```

- `since` — the ref the range starts after, explicit or the default tag;
  `null` when every archived change is in range. `until` — `null` for the
  working tree.
- `release` — `{ "version", "date" }` with `--release`, else `null`.
- `entries` — in display order. `text` is the item as written, `**BREAKING**`
  marker included. `source` is `release_note`, or `what_changes` for a change
  archived without a release note.
- `markdown` — the entries rendered as Keep a Changelog.
- `written` — with `--write`, `{ "path", "action" }`, `action` being
  `created`, `updated` or `unchanged`; else `null`.

Warnings in `status`: `release_note_missing`, `release_note_invalid`,
`release_note_inconsistent`, `not_a_git_repository`,
`archived_change_unreadable`. Error codes: `unknown_ref`,
`not_a_git_repository` (with `--since` or `--until` outside git),
`invalid_release_version`, `invalid_date`, `invalid_argument`,
`changelog_malformed`.

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
  "update": {
    "current": "0.5.0",
    "latest": "0.6.0",
    "url": "https://github.com/mairistem/codev/releases/tag/v0.6.0",
    "available": true,
    "snoozed": false
  },
  "versionRequirement": {
    "requirement": ">=0.5",
    "satisfied": true
  },
  "status": []
}
```

An artifact's `status` is `done`, `ready`, `blocked` or `skipped`.
`planningHome` is the project root; skills use it rather than assuming the
current directory.

`update` is present when the latest codev release is known: `current` is the
running version, `latest` the latest release, `url` its release page,
`available` whether `latest` is newer, and `snoozed` whether
`codev self-update --snooze` silenced it. It is absent when the check is
disabled (`CI`, `CODEV_NO_UPDATE_CHECK=1`) or has never succeeded.
`versionRequirement` is present when `_codev/config.yaml` sets
`codev_version`: `requirement` is its value, `satisfied` whether the running
version matches it. Both objects are additive: a consumer that ignores them
reads the document as before.

### self-update

`codev self-update --json`, which implies `--yes`:

```json
{
  "current": "0.5.0",
  "latest": "0.6.0",
  "url": "https://github.com/mairistem/codev/releases/tag/v0.6.0",
  "action": "updated",
  "installMethod": "standalone",
  "path": "/home/you/.local/bin/codev",
  "hint": null,
  "projectUpdated": true,
  "status": []
}
```

`action` is `updated`, `up_to_date`, `available` (with `--check`),
`snoozed`, `nothing_to_snooze` or `managed_elsewhere`. `installMethod` is
`standalone`, `cargo`, `build` or `package_manager`; for the last three,
`hint` holds the command that upgrades codev. `latest` and `url` are `null`
when unknown. Failures keep this shape and carry their code in `status` —
see the [codes](cli.md#codev-self-update).

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
  "role": {
    "id": "architect",
    "name": "Architect",
    "kind": "artifact",
    "at": "plan.write",
    "steps": ["plan.write"],
    "artifact": "design",
    "domains": [],
    "question": "How will it be built?",
    "when": "always",
    "whenOrigin": "default",
    "activatedBy": "always",
    "disableable": false,
    "knowledge": null,
    "doneWhen": [
      { "origin": "schema:spec-driven", "text": "each decision names the alternative it rejects and why" },
      "…"
    ]
  },
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
| `role` | The role that owns the artifact, in the shape of [`codev roles`](#roles); `null` when the schema declares none |

A failure keeps this shape, with `language` still set — the configured
language, or `en` when the configuration cannot be read — and the other
fields `null` or empty.

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

`kind` is `change`, `spec`, `decisions` or `config` — the last one only when
`_codev/config.yaml` holds keys this codev does not know (see
[Unknown keys](configuration.md#unknown-keys)). Findings are reported in `items`,
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

### review

```json
{
  "changeName": "add-sessions",
  "path": "_codev/changes/add-sessions/review.md",
  "exists": true,
  "format": 1,
  "steps": [
    {
      "step": "plan.challenge",
      "recorded": true,
      "activeRoles": ["skeptic", "security"],
      "roles": [
        { "role": "skeptic", "state": "no-finding" },
        { "role": "security", "state": "findings" }
      ]
    },
    {
      "step": "code.review",
      "recorded": false,
      "activeRoles": ["newcomer", "security"],
      "roles": []
    }
  ],
  "findings": [
    {
      "id": "R-security-1",
      "title": "Session tokens are stored in clear",
      "role": "security",
      "step": "plan.challenge",
      "line": 13,
      "severity": "high",
      "confidence": "high",
      "reported": true,
      "location": "design.md#Decision: Sessions live in the database",
      "evidence": "The session token is stored in the `sessions` table.\n",
      "recommendation": "Store a SHA-256 of the token and compare digests.",
      "status": "disputed",
      "awaiting": "human",
      "history": [
        { "status": "open", "by": "security", "note": null, "ref": null, "to": null },
        { "status": "disputed", "by": "architect", "note": "Tokens expire after 5 minutes.", "ref": null, "to": null },
        { "status": "disputed", "by": "security", "note": "Five minutes is enough to replay a stolen token.", "ref": null, "to": null }
      ]
    }
  ],
  "issues": [],
  "archive": {
    "ready": false,
    "blockers": [
      { "code": "review_step_missing", "message": "the review record has no `## code.review` section, while roles were active at it" },
      { "code": "review_unresolved", "message": "R-security-1 (Session tokens are stored in clear) is high and still disputed; fix it, or a human accepts the risk or defers it" }
    ]
  },
  "status": []
}
```

| Field | Meaning |
|---|---|
| `exists` | `false` without `review.md`; `format`, `findings` are then empty or `null` |
| `steps[].activeRoles` | The roles active at the step for the change, as `codev roles --at <step> --change <name>` returns them |
| `steps[].roles[].state` | `findings`, `no-finding`, `disabled`, `empty` (a section with nothing usable), or `missing` (an active role without a section, in a recorded step) |
| `findings[].reported` | `true` when the confidence is `high`: the skills present only those |
| `findings[].status` | The last history entry's status |
| `findings[].awaiting` | Who acts next: `author` (open), `reviewer` (a fix to verify, a dispute to answer), `human` (a dispute the reviewer maintained), or `null` |
| `issues` | The record's defects, in the shape of `validate`'s findings, with the `review_*` codes |
| `archive.blockers[].code` | `review_record_missing`, `review_step_missing`, `review_unresolved` or `review_format_unsupported` |

The exit status is `1` only when `issues` holds an error. Fields the file
leaves out or gets wrong are `null`.

### roles

`codev roles --at plan.challenge --change add-dark-mode --json`:

```json
{
  "changeName": "add-dark-mode",
  "schemaName": "spec-driven",
  "at": "plan.challenge",
  "language": "en",
  "stacks": { "detected": ["TypeScript", "JavaScript"], "filter": "detected" },
  "diff": {
    "available": true,
    "reason": null,
    "base": "origin/main",
    "files": 14,
    "scanned": 11,
    "skipped": { "lockfile": 1, "prose": 2 },
    "truncated": false
  },
  "suggestions": [
    {
      "domain": "user-interface",
      "state": "ticked",
      "justification": null,
      "matchCount": 6,
      "matches": [
        { "source": "accessibility", "kind": "paths", "trigger": "**/*.{html,htm,css,scss,sass,less,jsx,tsx,vue,svelte,astro}", "path": "web/src/theme.css", "line": null },
        "…"
      ]
    },
    {
      "domain": "performance",
      "state": "justified",
      "justification": "the cache is the browser's, not ours",
      "matchCount": 1,
      "matches": [
        { "source": "performance", "kind": "patterns", "trigger": "(?i)\\b(cache|caching|memoi[sz]e|lru)\\b", "path": "web/src/theme.ts", "line": 12 }
      ]
    }
  ],
  "roles": [
    {
      "id": "skeptic",
      "name": "Skeptic",
      "kind": "step",
      "at": "plan.challenge",
      "steps": ["plan.challenge"],
      "artifact": null,
      "domains": [],
      "question": "Would the plan survive a reviewer who did not write it?",
      "when": "always",
      "whenOrigin": "project",
      "activatedBy": "project",
      "disableable": true,
      "knowledge": "## Signals\n\nPlanning smells — each one is a question to raise, not a verdict.\n…",
      "doneWhen": [
        { "origin": "built-in", "text": "**Need** — does the change solve the Why? Is there a simpler solution, or one without code?" },
        "…",
        { "origin": "project", "text": "The rollback plan is stated" }
      ]
    },
    {
      "id": "accessibility",
      "name": "Accessibility",
      "kind": "step",
      "at": "plan.challenge",
      "steps": ["plan.challenge", "code.review"],
      "artifact": null,
      "domains": ["user-interface"],
      "question": "Can everyone use what this change adds, with a keyboard or assistive technology?",
      "when": "on_domain",
      "whenOrigin": "default",
      "activatedBy": "domain:user-interface",
      "disableable": true,
      "knowledge": "## Scope\n\nLook at the user interface the change adds or modifies…",
      "doneWhen": [
        { "origin": "built-in", "text": "Every interactive element is reachable and operable with the keyboard alone, in a logical focus order, with a visible focus" },
        "…"
      ]
    }
  ],
  "status": []
}
```

| Field | Meaning |
|---|---|
| `changeName` | The change given with `--change`; `null` without it |
| `schemaName` | The schema the artifact roles come from |
| `at` | The step given with `--at`; `null` lists every role, active or not |
| `stacks` | The languages detected from the project's dependency manifests (`detected`: `Rust`, `TypeScript`, `JavaScript`, `Python`, `Go`, `Java`) and the `filter` applied to the knowledge's `By stack` part: `detected`, `none-detected` (everything kept) or `all-stacks` (`--all-stacks`) |
| `diff` | With `--change`, the change's diff: `available`, `reason` when it is not, the default branch `base`, the `files` changed, the files `scanned` by the patterns, the files `skipped` per category (`planning`, `vendored`, `lockfile`, `generated`, `binary`, `prose`, `large`), and `truncated` when a cap was reached; `null` without `--change` |
| `suggestions` | With `--change`, the domains the diff suggests, in the order of the domains list: `state` is `ticked`, `justified` (with its `justification`) or `unticked`; `matchCount` counts the matches and `matches` lists the first five, each with its `source` (the role, or `project`), `kind` (`paths`, `manifests`, `patterns`), `trigger` as written, `path`, and `line` for a pattern; empty without `--change` |
| `roles[].name` | The display name in the configured `language`, English when the role has none in it |
| `roles[].kind` | `step`, or `artifact` with the artifact's id in `artifact` |
| `roles[].at` | The step given with `--at`, otherwise the role's first step; `plan.write` for every artifact role |
| `roles[].steps` | Every step the role serves, in workflow order: an expert serves `plan.challenge` and `code.review` |
| `roles[].domains` | The domains that activate an expert, such as `["personal-data"]`; empty for the other roles |
| `roles[].when`, `roles[].whenOrigin` | `always`, `never` or `on_domain`, set by `default`, `project` or `change` |
| `roles[].activatedBy` | Why the role is active: `always`, `project`, `change`, or `domain:<name>` for an expert whose domain the proposal ticks; `null` when it is inactive |
| `roles[].disableable` | `false` for the artifact roles, the `analyst` and the `developer` |
| `roles[].knowledge` | The role's knowledge, as markdown in the sections Scope, Signals, By stack, References and Reporting, for an active role that has some, its `By stack` part filtered as `stacks.filter` says; `null` otherwise |
| `roles[].doneWhen` | The role's criteria, then the project's, each with its `origin`: `built-in`, `schema:<name>` or `project` |

With `--at`, `roles` holds only the active roles, possibly none: the step is
then skipped. Without `--change`, no proposal is read, so the experts are
inactive. A failure — `unknown_step`, `unknown_role`,
`invalid_role_activation`, `role_not_disableable` — keeps this shape, with
`roles` and `suggestions` empty, `stacks` and `diff` `null`, and `language`
set as for `instructions`. A project trigger that cannot be used is left
out with an `invalid_trigger` warning in `status`.

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

`origin` is `project` or `built-in`. A project schema that cannot be used is
left out of `schemas`, with a warning `schema_unusable` in `status`. A project
schema whose `name:` differs from its folder is listed under its folder name,
with a warning `schema_name_mismatch`.

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
deviation. `origin` is `project`, `path:<path>` or `git:<url>`. `path` is
relative to the project for a decision inside it, absolute otherwise.

The commands that create or seal decisions return:

| Command | Fields besides `root` and `status` |
|---|---|
| `decision new` | `decision`, `path`, `bodySha256` (only with `--status accepted`) |
| `decision accept` | `decision`, `path`, `bodySha256`, `superseded` |
| `decision supersede` | `newDecision`, `newPath`, `oldId`, `oldQualifiedId`, `oldPath` |
| `decision seal` | `seal` (`id`, `bodySha256`, `sealedAt`), `wasNoop` |
| `decision deviate` | `decision`, `path`, `targetQualifiedId` |
| `decision promote` | `decision`, `path`, `sourceChange`, `designPath` |

`bodySha256` is present only when the command sealed the decision:
`decision supersede`, `decision deviate` and `decision promote` create a
`proposed` decision and seal nothing. For `decision supersede`, `oldId`,
`oldQualifiedId` and `oldPath` name the decision that accepting the new one
will supersede; it is not modified. `superseded` lists the predecessors that
`decision accept` set to `superseded` — each with its `id`, `qualifiedId` and
`path` — and is empty when the decision supersedes nothing.

For example, `codev decision accept 0002 --json` for a decision created by
`codev decision supersede 0001 "Use SQLite"`:

```json
{
  "root": "/home/you/acme-app",
  "decision": {
    "id": "0002",
    "qualifiedId": "project/0002",
    "title": "Use SQLite",
    "status": "accepted",
    "date": "2026-09-30",
    "tags": [],
    "supersedes": [
      "0001"
    ],
    "deviatesFrom": [],
    "path": "_codev/decisions/0002-use-sqlite.md",
    "origin": "project",
    "inEffect": true,
    "supersededBy": null
  },
  "path": "/home/you/acme-app/_codev/decisions/0002-use-sqlite.md",
  "bodySha256": "sha256:c8873b83c178dbf8434045f43a722911b9d9913ee9f13915ccda893539b788c1",
  "superseded": [
    {
      "id": "0001",
      "qualifiedId": "project/0001",
      "path": "/home/you/acme-app/_codev/decisions/0001-use-postgresql.md"
    }
  ],
  "status": []
}
```

A refused acceptance keeps the same shape, with `decision` and `path` set to
`null`, `superseded` empty and the code in `status`: `unknown_decision_id`,
`cannot_accept_inherited`, `decision_not_proposed`,
`cannot_supersede_inherited` or `predecessor_not_accepted`.

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
