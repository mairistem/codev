## MODIFIED Requirements

### Requirement: `codev roles` serves the roles of a step

`codev roles [--at <step>] [--change <name>] [--knowledge] [--all-stacks]
[--json]` SHALL resolve the roles against the change's schema, its
`change.yaml` and the domains of its proposal with `--change`, otherwise
against the configured schema and the project configuration alone. With
`--at`, it SHALL list the roles active at that step; without it, every
role of the catalog with its activation. It reads files, writes none, and
exits with code 0.

With `--json`, the document SHALL carry `changeName` (`null` without
`--change`), `schemaName`, `at` (`null` without `--at`), `language`,
`stacks`, `diff`, `suggestions`, `roles` and `status`. `diff` and
`suggestions` are defined by the `domain-suggestions` capability. Each
role SHALL carry `id`, `name` (its display name in the configured
`language`, English when the role has none in that language), `kind`
(`step` or `artifact`), `at` (the step asked with `--at`, otherwise the
role's first step), `steps` (every step the role serves, in workflow
order), `artifact` (the artifact id of an artifact role, `null`
otherwise), `domains` (the domains that activate it, empty for a role bound
to none), `question`, `when` (`always`, `never` or `on_domain`),
`whenOrigin` (`default`, `project` or `change`), `activatedBy`,
`disableable`, `knowledge`, and `doneWhen`, a list of criteria each with
its `origin` (`built-in`, `schema:<name>` or `project`) and its `text`.

`activatedBy` SHALL say why an active role is active: `always` for the
default activation, `project` or `change` when that file set `always`, and
`domain:<name>` for an `on_domain` role, naming its first domain the
proposal ticks. It is `null` for an inactive role. `knowledge` SHALL be the
role's knowledge text for an active role that has one, its `By stack`
part filtered as `stacks.filter` says, and `null` otherwise.

Without `--json`, with `--at`, it SHALL print each active role's id and
name, its question, and its criteria each prefixed with its origin in
parentheses — or a line saying no role is active at that step — and, with
`--knowledge`, the stacks the knowledge is filtered for and each role's
knowledge after its criteria. Without `--at`, it SHALL print one line per
role: id, name, step or artifact, and activation with its origin when not
the default, marking the roles that cannot be disabled.

An `--at` that is not one of the steps SHALL fail with `unknown_step`,
naming the valid steps. With `--json`, a failure keeps the document's
shape, with `roles` and `suggestions` empty and `stacks` and `diff`
`null`.

#### Scenario: The roles of a step, as JSON

- **GIVEN** a project with no `roles:` key and a change `add-auth`
- **WHEN** the user runs `codev roles --at plan.trace --change add-auth
  --json`
- **THEN** `roles` holds one entry, `tracer`, with `kind: "step"`,
  `at: "plan.trace"`, `steps: ["plan.trace"]`, `artifact: null`,
  `domains: []`, `when: "always"`, `whenOrigin: "default"`,
  `activatedBy: "always"`, `knowledge: null` and its criteria with the
  origin `built-in`

#### Scenario: Artifact roles at `plan.write`

- **WHEN** the user runs `codev roles --at plan.write --json`
- **THEN** `roles` holds `product`, `qa`, `architect` and `lead`, with
  `artifact` `proposal`, `specs`, `design` and `tasks`, and criteria with
  the origin `schema:spec-driven`

#### Scenario: Display names follow the configured language

- **GIVEN** a configuration with `language: fr`
- **WHEN** the user runs `codev roles --at plan.challenge --json`
- **THEN** the `skeptic` entry has `name: "Contradicteur"`
- **AND** with `language: de`, it has `name: "Skeptic"`

#### Scenario: An unknown step

- **WHEN** the user runs `codev roles --at plan.review --json`
- **THEN** the exit code is 1 and `status[0].code` is `unknown_step`
- **AND** its message lists the valid steps, `roles` and `suggestions`
  are empty, and `stacks` and `diff` are `null`

#### Scenario: Human output of every role

- **GIVEN** a configuration with `roles: { newcomer: never }`
- **WHEN** the user runs `codev roles`
- **THEN** it prints one line per role, the `newcomer` line saying `never`
  and `project`, the `security` line saying `on_domain`, and the `product`
  line saying it cannot be disabled

#### Scenario: Knowledge in the human output

- **GIVEN** a change `add-auth` whose proposal ticks `security`
- **WHEN** the user runs `codev roles --at code.review --change add-auth`
- **THEN** it prints the criteria of `newcomer` and `security` without
  their knowledge
- **AND** with `--knowledge`, it also prints the stacks detected and the
  knowledge of each

## ADDED Requirements

### Requirement: The `By stack` knowledge follows the project's languages

`codev roles` SHALL detect the project's languages from the dependency
manifests found at the project root and in its directories up to three
levels below it, skipping hidden directories and the directories named
`node_modules`, `vendor`, `third_party`, `target`, `dist` and `build`, and
visiting at most 2000 directories: `Cargo.toml` for Rust, `package.json`
for TypeScript and JavaScript, `pyproject.toml`, `setup.py`,
`requirements.txt` or `Pipfile` for Python, `go.mod` for Go, `pom.xml`,
`build.gradle` or `build.gradle.kts` for Java. A monorepo has every
language found.

In the knowledge it serves, the `By stack` section SHALL keep only the
items whose label names a detected language — an item labelled
`TypeScript/JavaScript` or `Rust, Python, Go, Java` names each of those —
and the items whose label names no language codev detects; the section is
left out when no item remains. When no language is detected, or with
`--all-stacks`, the section SHALL be served whole.

`stacks` in the JSON document SHALL carry `detected`, the languages found
(`Rust`, `TypeScript`, `JavaScript`, `Python`, `Go`, `Java`, in that
order; a `package.json` counts as both `TypeScript` and `JavaScript`), and
`filter`: `detected` when the knowledge is filtered, `none-detected` when
no language was found, `all-stacks` when `--all-stacks` was given.

#### Scenario: A Rust project receives the Rust pitfalls only

- **GIVEN** a project with a `Cargo.toml` at its root and a change whose
  proposal ticks `security`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** `stacks` is `{ "detected": ["Rust"], "filter": "detected" }`
- **AND** the `security` knowledge's `By stack` names Rust, and neither
  Python, Go, Java nor TypeScript/JavaScript

#### Scenario: A monorepo receives each of its stacks

- **GIVEN** a project with `services/api/go.mod` and `web/package.json`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json` for a change ticking `security`
- **THEN** `stacks.detected` is `["TypeScript", "JavaScript", "Go"]`
- **AND** the `By stack` section keeps the Go and the
  TypeScript/JavaScript items only

#### Scenario: An item naming several languages

- **GIVEN** a Python project and a change ticking `user-interface`
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json`
- **THEN** the `accessibility` knowledge keeps the item labelled
  `Rust, Python, Go, Java` and drops the `TypeScript/JavaScript` one

#### Scenario: No language detected

- **GIVEN** a project with no dependency manifest
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --json` for a change ticking `security`
- **THEN** `stacks` is `{ "detected": [], "filter": "none-detected" }`
- **AND** the `By stack` section names the five languages

#### Scenario: Every stack on request

- **GIVEN** a project with a `Cargo.toml` at its root
- **WHEN** the user runs `codev roles --at code.review --change <name>
  --all-stacks --json`
- **THEN** `stacks.filter` is `all-stacks` and the `By stack` section names
  the five languages

#### Scenario: Manifests in skipped directories do not count

- **GIVEN** a Rust project with `node_modules/left-pad/package.json` and
  `.cache/pyproject.toml`
- **WHEN** the user runs `codev roles --json`
- **THEN** `stacks.detected` is `["Rust"]`
