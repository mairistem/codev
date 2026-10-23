# Configuration

A codev project is configured by `_codev/config.yaml`. Every key is optional:
a project can even run without the file. A key codev does not know is
reported — as a warning at the top level, in `mcp` and in `review`, as an
error elsewhere —
so a typo never goes unnoticed; see [Unknown keys](#unknown-keys).

`codev init` generates the file with the values it detected, each preceded by
a comment giving its source. You can edit it by hand at any time. After
changing `workflows`, `mcp` or `review`, run `codev update` so the skills
reflect the change.

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

changelog:
  exclude:
    - 2026-03-02-legacy-import

codev_version: ">=0.5"

roles:
  newcomer: never
  skeptic:
    done_when:
      - The rollback plan is stated

triggers:
  - domain: personal-data
    paths: ["src/billing/**"]

review:
  mode: isolated
```

## schema

The workflow schema used by new changes. Default: `spec-driven`, the schema
embedded in codev. The name of a schema defined under `_codev/schemas/` is
also accepted. A change records its schema in its own `change.yaml`, so
changing this key does not affect changes in progress. See
[Custom schemas](../guides/custom-schemas.md).

## language

The language skills write artifacts in, as an ISO 639 code: two or three
lowercase letters, optionally followed by one subtag (`en`, `fr`, `pt-BR`,
`zh-Hant`). Default: `en`. It sets the prose, and the structural keywords
when codev ships the language (English and French); another language keeps
English keywords. codev reads the keywords in English or French whatever
this setting. See [Artifact language](../guides/artifact-language.md).

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

`/codev-propose` applies the rules a second time when it challenges the
finished plan, so `rules` is also the way to add a review lens specific to
your project:

```yaml
rules:
  design:
    - "Assess the accessibility (RGAA) impact of any user-interface change."
```

Testing rules go under `tasks`: `/codev-propose` writes the plan with them,
and `/codev-apply` follows them when it writes the tests. Describe what your
existing tests do — `/codev-configure` drafts the rule from them:

```yaml
rules:
  tasks:
    # One rule for your stack, for example:
    # Rust
    - "Unit tests in #[cfg(test)] mod tests next to the code, integration tests in tests/; cargo test, cargo clippy -- -D warnings, cargo fmt --check."
    # TypeScript
    - "Vitest, *.test.ts next to the code, Testing Library for components; tsc --noEmit and the linter in CI."
    # Python
    - "pytest, tests/ mirroring the package, fixtures and parametrize; ruff check."
    # Go
    - "Table-driven tests in _test.go files of the same package; go test ./..., go vet."
    # Java
    - "JUnit 5 under src/test/java mirroring the main package, Mockito only at boundaries; mvn verify."
```

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

`codev init --inherit <SOURCE>` writes this key for you when it creates the
configuration, one entry per flag, under a `# set with codev init --inherit`
comment, and pins the git sources at once:

```bash
codev init --yes --inherit git@github.com:acme/codev-standards.git@v2 --inherit ~/src/acme-standards
```

```yaml
# set with codev init --inherit
inherits:
  - git: git@github.com:acme/codev-standards.git
    ref: v2
  - path: ~/src/acme-standards
```

## mcp

Names of the MCP tools the skills may call. These are specific to each Claude
Code setup, so they are never inherited.

| Key | Meaning |
|---|---|
| `jira_tool` | Tool that fetches a Jira issue, used by `/codev-propose` when a ticket id is mentioned |

See [Jira and other MCP servers](../guides/mcp.md).

## changelog

Settings of [`codev changelog`](cli.md#codev-changelog). Project only, never
inherited.

| Key | Meaning |
|---|---|
| `exclude` | Archive folder names (`<date>-<name>`) of changes already described by hand in `CHANGELOG.md`. `codev changelog` leaves them out, without a warning |

Use it when adopting `codev changelog` in a project whose `CHANGELOG.md`
already describes changes archived without a release note, so they are not
listed twice. See [Release notes and CHANGELOG](../guides/release-notes.md).

## codev_version

The codev versions the team accepts, as a semantic version requirement in
Cargo's syntax: `">=0.5"`, `"^0.6"`, `">=0.5, <0.8"`. Quote it, since YAML
reads `>` specially. Project only, never inherited.

When the running codev does not satisfy it, `codev doctor` reports the
`codev_version` check as missing and the skills offer to update before
acting. The fix is `codev self-update` when the running codev is too old. When
it is too new — above an upper bound such as `<0.7` — a plain
`codev self-update` would refuse to downgrade, so the fix names a version:
`codev self-update --version 0.6.4`, taken from the latest release the update
cache knows when it satisfies the requirement, otherwise from the requirement
itself (`<=0.6.2` gives `0.6.2`, `^0.5` gives `0.5.0`). A value that is not a
requirement makes every command fail with `invalid_codev_version`.

codev 0.5 and earlier do not know this key and reject the file: add it once
everyone runs a version that does.

## roles

Which [roles](../roles.md) run, and the criteria your project adds to them.
Project only, never inherited. Each key is a role id; its value is either an
activation, or a mapping:

| Key | Meaning |
|---|---|
| `when` | `always`, `never` or `on_domain`; the default is `on_domain` for the experts, `always` for every other role. `on_domain` activates a role only for a change whose proposal ticks one of its domains. A step whose roles are all inactive is skipped, and the skill says so |
| `done_when` | Criteria added after the role's own, with the origin `project`; blank entries are ignored |
| `model` | The model the role's sub-agent runs on in the `isolated` [review mode](#review): `opus`, `sonnet` or `haiku`, or `inherit` for the session's model. No role sets one by default — see [The reviewers' models](../roles.md#the-reviewers-models). In the `inline` mode every role runs on the session's model |

```yaml
roles:
  newcomer: never            # short form
  security: always           # the security expert reviews every change
  skeptic:
    when: always
    done_when:
      - The rollback plan is stated
  privacy:
    model: sonnet            # the privacy expert's sub-agent runs on Sonnet
  migration:
    model: inherit           # back to the session's model
```

The ids are the built-in step roles (`analyst`, `investigator`, `tracer`,
`skeptic`, `developer`, `newcomer`), the experts (`security`, `privacy`,
`accessibility`, `performance`, `ops`, `migration` — see
[Experts and domains](../roles.md#experts-and-domains)) and the artifact
roles of the schema in use (`product`, `qa`, `architect`, `lead` with
`spec-driven`). The artifact roles, the `analyst` and the `developer`
cannot be disabled; their criteria can be extended. A change's `change.yaml` can override the activation for that
change only — see [change.yaml](file-formats.md#changeyaml).

`codev validate` reports an unknown id as `unknown_role`, an activation
other than `always`, `never` or `on_domain` — or `on_domain` on a role
bound to no domain — as `invalid_role_activation`, and `never` on a role
that cannot be disabled as `role_not_disableable` — errors of the
`config` item, at the line of the role. `codev status`,
`codev instructions` and `codev roles` fail with the same code. Any other
key inside a role's mapping, such as a misspelled `done-when`, fails every
command with `invalid`: ignoring it would drop criteria silently.
A `model` other than `opus`, `sonnet`, `haiku` or `inherit` is an
`invalid_role_model` error, reported and failing the same way. A `model`
on a role that serves no reviewing step (`plan.trace`, `plan.challenge`,
`code.review`) and is not the `developer` never applies: it is a
`role_model_unused` warning. The `developer`'s model implements the tasks
of `/codev-apply` in a sub-agent, in either review mode (decision 0012).

**`model` and older codev versions.** A codev older than the release that
introduced `model` refuses the key, like any unknown key inside a role:
every command fails with `invalid`. Raise
[`codev_version`](#codev_version) to that release before you set it, so
every codev that reads the repository, CI included, understands it. A
source shared through [`inherits`](#inherits) should not set it until
every project inheriting from it runs that release: from that release on
a source's `roles:` is skipped, never read, but an older codev still
parses it and would drop the source's context and rules.

## triggers

Your project's own [triggers](../roles.md#triggers-and-suggested-domains):
the paths and patterns only your team knows to be sensitive. Project only,
never inherited. A list of groups, each naming the domain it suggests and
holding at least one of `paths`, `manifests` or `patterns`:

| Key | Meaning |
|---|---|
| `domain` | `security`, `personal-data`, `user-interface`, `performance`, `operations` or `migration` |
| `paths` | Globs over the changed files, relative to the project root, ignoring case; `*` stays within a folder, `**` crosses folders |
| `manifests` | File names of dependency manifests, matched at any depth |
| `patterns` | Regular expressions over the lines the diff adds |

```yaml
triggers:
  - domain: personal-data
    paths: ["src/billing/**", "src/crm/**"]
    patterns: ['\bcustomer_id\b']
  - domain: migration
    paths: [api/public.yaml]
```

Quote a glob that starts with `*`: in YAML, `*` starts an alias. Their
matches have the source `project`, and they are evaluated whatever your
`roles:` settings. Expressions run in linear time, whatever their shape,
and their compiled size is capped.

`codev validate` reports, at the line of `triggers:`, an `invalid_trigger`
error for a group without a known `domain`, without any trigger, or with
a glob or an expression that does not compile — the group is then ignored
by every command, which says so in its `status` — and an `invalid_trigger`
warning for a key it does not know, which is ignored while the rest of the
group is used.

codev 0.6 and earlier do not know this key: they warn
`unknown_config_key`, which fails their `codev validate --strict`. When you
add it, raise `codev_version` to the release that introduced it.

## review

How the reviewing roles run. Project only, never inherited.

| Key | Meaning |
|---|---|
| `mode` | `inline` (default): the skill reviews in its own conversation, one role after the other. `isolated`: each reviewing role runs alone as a Claude Code sub-agent that never sees the conversation, then the skill merges their findings |

```yaml
review:
  mode: isolated
```

A change can override it in its `change.yaml`, with the same key:
`review: { mode: inline }` for a typo fix in an isolated project, or
`isolated` for one sensitive change in an inline project.
`codev roles --json` gives the mode in effect and where it comes from
(`review.mode`, `review.origin`). Any other value fails every command
with `invalid_review_mode`, naming `inline` and `isolated`; an unknown
key under `review` is an `unknown_config_key` warning (`review.<key>`).

**Switching.** Set the key, then run `codev update`: in isolated mode the
`/codev-propose` and `/codev-apply` skills request Claude Code's `Agent`
tool, in inline mode they do not, and `codev doctor` reports
`skills_review_mode` until they are regenerated. A skill you edited by
hand is kept; `codev update --force` rewrites it.

**Why inline is the default.** Isolated review multiplies the reading of
the material by the number of roles, and its benefit has not been
measured yet: codev's role evaluations run one role at a time, with no
authoring conversation, so they cannot compare the two modes. See
[Inline or isolated review](../roles.md#inline-or-isolated-review).

codev 0.6 and earlier do not know this key: they warn
`unknown_config_key` — which fails their `codev validate --strict` — and
refuse a `change.yaml` that sets it. When you set it, raise
`codev_version` to the release that introduced it, so every codev that
reads the repository, CI included, understands it.

## Unknown keys

A newer codev may add keys an older one does not know. So that an older codev
keeps working on a newer configuration and tells you what it needs, unknown
keys are treated according to what ignoring them would do:

| Where | Unknown key | Why |
|---|---|---|
| top level | warning `unknown_config_key`, key ignored | each key is an independent feature: the rest of the configuration still applies |
| `mcp` | warning `unknown_config_key` (`mcp.<key>`), key ignored | an unknown tool only leaves that integration unconfigured |
| `review` | warning `unknown_config_key` (`review.<key>`), key ignored | an unknown review setting leaves the review as it was |
| an `inherits` entry | error `invalid` | ignoring it would read another source than the one you described |
| `changelog` | error `invalid` | ignoring it would make `codev changelog --write` write another file |
| a `roles` entry | error `invalid` | ignoring it would drop criteria or an activation |
| a `triggers` group | warning `invalid_trigger` from `codev validate`, key ignored | the rest of the group still suggests its domain |

The warning names the key, says the project may need a newer codev, quotes
`codev_version` when the file sets it, and points to `codev self-update`; the
error does the same when `codev_version` is set. The warning appears:

- in the output of `codev status`, `codev new change`, `codev init` and
  `codev update`, and in the `status` array of their `--json` output;
- in `codev validate`, as a `warning` finding of a `config` item at the line of
  the key, so `codev validate --strict` — the CI case — fails on it while plain
  `codev validate` passes;
- in `codev doctor`, as the `config.keys` check.

The keys of an inherited source's configuration are not reported: only its
`context` and `rules` are read.

This tolerance starts with the codev release that introduced it: **codev 0.5
and earlier reject any unknown key**, at any level. When you add a key that
only recent versions know, raise `codev_version` too, so that teammates on an
older codev learn from `codev doctor` and the skills which version to install.

## Environment variables

| Variable | Used by | Effect |
|---|---|---|
| `LC_ALL`, `LC_MESSAGES`, `LANG` | `codev init` | Default for `language:` when `--language` is not given |
| `XDG_CACHE_HOME` | git sources, update check | Cache location: `$XDG_CACHE_HOME/codev/`, else `~/.cache/codev/` |
| `CI` | `codev status` | Any value disables the update check |
| `CODEV_NO_UPDATE_CHECK` | `codev status` | `1` disables the update check |
| `VISUAL`, `EDITOR` | `codev init` | Editor opened when the context question is left empty |
| `CODEV_VERSION` | `install.sh`, `install.ps1` | Version to install instead of the latest |
