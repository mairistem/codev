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
`stacks`, `diff`, `suggestions`, `roles`, `disabled` and `status`. `diff` and
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

With `--at`, `disabled` SHALL list, in workflow order, the roles that
serve the step and whose activation, once the project's and then the
change's settings are applied, is `never`, each with `id`, `name` and
`whenOrigin` (`project` or `change`, the file whose setting won), so that a
skill writes `Disabled by configuration.` for them without a second call;
it is empty without `--at`, and an `on_domain` role whose domain is not
ticked is not listed: it is inactive, not disabled.

`activatedBy` SHALL say why an active role is active: `always` for the
default activation, `project` or `change` when that file set `always`, and
`domain:<name>` for an `on_domain` role, naming its first domain the
proposal ticks. It is `null` for an inactive role. `knowledge` SHALL be the
role's knowledge text for an active role that has one, its `By stack`
part filtered as `stacks.filter` says, and `null` otherwise.

Without `--json`, with `--at`, it SHALL print each active role's id and
name, its question, and its criteria each prefixed with its origin in
parentheses — or a line saying no role is active at that step — then a
line naming the roles disabled at the step with the origin of the
setting, when there are any, and, with
`--knowledge`, the stacks the knowledge is filtered for and each role's
knowledge after its criteria. Without `--at`, it SHALL print one line per
role: id, name, step or artifact, and activation with its origin when not
the default, marking the roles that cannot be disabled.

An `--at` that is not one of the steps SHALL fail with `unknown_step`,
naming the valid steps. With `--json`, a failure keeps the document's
shape, with `roles`, `disabled` and `suggestions` empty and `stacks` and
`diff` `null`.

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
- **AND** its message lists the valid steps, `roles`, `disabled` and
  `suggestions` are empty, and `stacks` and `diff` are `null`

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

#### Scenario: The roles disabled at a step

- **GIVEN** a project whose configuration sets `newcomer: never` and a
  change `add-auth` whose proposal ticks `security`
- **WHEN** the user runs `codev roles --at code.review --change add-auth
  --json`
- **THEN** `roles` holds `security` and not `newcomer`, and `disabled`
  holds one entry, `newcomer`, with `whenOrigin: "project"`
- **AND** without `--json`, the output names `newcomer` as disabled by
  the project's configuration

#### Scenario: A change turns a role back on

- **GIVEN** a configuration with `newcomer: never` and a change whose
  `change.yaml` sets `newcomer: always`
- **WHEN** the user runs `codev roles --at code.review --change <change>
  --json`
- **THEN** `newcomer` is in `roles` and not in `disabled`

#### Scenario: An inactive expert is not disabled

- **GIVEN** a change whose proposal ticks no domain
- **WHEN** the user runs `codev roles --at code.review --change <change>
  --json`
- **THEN** `disabled` does not list `security`
