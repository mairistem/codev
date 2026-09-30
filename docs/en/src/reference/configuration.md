# Configuration

A codev project is configured by `_codev/config.yaml`. Every key is optional:
a project can even run without the file. Unknown keys are rejected with an
error that lists the valid ones, so a typo never goes unnoticed.

`codev init` generates the file with the values it detected, each preceded by
a comment giving its source. You can edit it by hand at any time. After
changing `workflows` or `mcp`, run `codev update` so the skills reflect the
change.

## Complete example

```yaml
schema: spec-driven

language: en

workflows:
  - propose
  - explore
  - onboard
  - apply
  - sync
  - archive
  - update
  - configure

context: |
  TypeScript monorepo: a Next.js web app and a Node.js API.
  Errors are typed; never throw raw strings.

rules:
  specs:
    - Describe observable behavior, never implementation.
  design:
    - Cite the decisions in _codev/decisions/ that constrain the approach.
  tasks:
    - "Each task states how to verify it: a named test, a command, or an observable behavior."

inherits:
  - path: ~/src/acme-standards
  - git: git@github.com:acme/codev-standards.git
    ref: main
    subpath: standards

mcp:
  jira_tool: mcp__atlassian__getJiraIssue
```

## schema

The workflow schema used by new changes. Default: `spec-driven`, the schema
embedded in codev. The name of a schema defined under `_codev/schemas/` is
also accepted. A change records its schema in its own `change.yaml`, so
changing this key does not affect changes in progress. See
[Custom schemas](../guides/custom-schemas.md).

## language

The language skills write artifact prose in, as an ISO 639 code: two or three
lowercase letters, optionally followed by one subtag (`en`, `fr`, `pt-BR`,
`zh-Hant`). Default: `en`. Structural keywords always stay in English. See
[Artifact language](../guides/artifact-language.md).

## workflows

The workflows to install as Claude Code skills, by id: `propose`, `explore`,
`onboard`, `configure`, `apply`, `sync`, `archive`, `update`. When the key is
absent, all eight are installed. An unknown id produces an `unknown_workflow`
warning and is ignored. See [Claude Code skills](../skills.md).

## context

Free text given to the agent with the instructions of **every** artifact:
your stack, conventions, constraints — what cannot be inferred from the code.
It is a constraint on the agent, never content to copy into the files.
`/codev-configure` can draft it for you.

## rules

Rules for one artifact, as a map from artifact id to a list of strings. With
the `spec-driven` schema, the ids are `proposal`, `specs`, `design` and
`tasks`. Each rule is handed to the agent only when it writes that artifact.

YAML reads `key: value` inside a plain scalar as a map, so quote a rule that
contains a colon followed by a space, as in the `tasks` rule above.

While `rules` is empty, `codev init` and `codev status` suggest running
`/codev-configure`.

## inherits

Read-only sources to inherit context, rules and decisions from, in order from
the most general to the most specific. Each entry is either a local folder or
a git repository:

| Key | Applies to | Meaning |
|---|---|---|
| `path` | local | Folder containing a codev project; absolute or starting with `~` |
| `git` | git | Repository URL, as `git` understands it |
| `ref` | git | Branch or tag to follow — required |
| `subpath` | git | Folder of the repository that contains the codev project |

An entry declares exactly one of `path` and `git`; `ref` and `subpath` are
rejected on a `path` entry. Git sources are pinned in `_codev/codev.lock` by
`codev sources update`. See [Inherited sources](../guides/inherited-sources.md).

## mcp

Names of the MCP tools the skills may call. These are specific to each Claude
Code setup, so they are never inherited.

| Key | Meaning |
|---|---|
| `jira_tool` | Tool that fetches a Jira issue, used by `/codev-propose` when a ticket id is mentioned |

See [Jira and other MCP servers](../guides/mcp.md).

## Environment variables

| Variable | Used by | Effect |
|---|---|---|
| `LC_ALL`, `LC_MESSAGES`, `LANG` | `codev init` | Default for `language:` when `--language` is not given |
| `XDG_CACHE_HOME` | git sources | Cache location: `$XDG_CACHE_HOME/codev/`, else `~/.cache/codev/` |
| `VISUAL`, `EDITOR` | `codev init` | Editor opened when the context question is left empty |
| `CODEV_VERSION` | `install.sh`, `install.ps1` | Version to install instead of the latest |
