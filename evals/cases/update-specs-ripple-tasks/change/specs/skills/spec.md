## MODIFIED Requirements

### Requirement: The `propose` and `apply` skills take their roles from `codev roles`

At each role step, the `propose` skill SHALL run
`codev roles --at <step> [--change <name>] --json` — `plan.understand`
before creating the change, then `plan.investigate`, `plan.trace` and
`plan.challenge` with `--change` — and the `apply` skill SHALL do the
same at `code.implement` and `code.review`. The skill MUST apply the
criteria of every role returned, the project's extra criteria included,
and MUST skip the step when no role is returned. Writing an artifact, the
`propose` skill MUST check the criteria of the `role` field of
`codev instructions` before moving on. The skill texts MUST NOT restate
the criteria served as data. The `allowed-tools` of both skills are
unchanged, except for the sub-agent tool, which they carry only when the
project's review mode is `isolated`.

#### Scenario: The skill bodies ask for the roles of each step

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are resolved
- **THEN** the `propose` body names `codev roles --at` with
  `plan.understand`, `plan.investigate`, `plan.trace` and
  `plan.challenge`, and the `apply` body with `code.implement` and
  `code.review`
- **AND** neither body restates the criteria of the built-in catalog

#### Scenario: A disabled investigator

- **GIVEN** a configuration with `roles: { investigator: never }`
- **WHEN** `/codev-propose` writes the artifacts
- **THEN** it does not run the code study before each artifact
- **AND** its summary says the investigation is disabled by configuration

## ADDED Requirements

### Requirement: The review skills request the sub-agent tool only in isolated mode

When the project's review mode is `isolated`, the `propose` and `apply`
skills SHALL list Claude Code's sub-agent tool, `Agent`, in their
`allowed-tools`; when it is `inline`, no skill SHALL list it. No other
skill ever lists it. The body of both skills SHALL carry the isolated
review procedure in either mode, so that a change that overrides the mode
is reviewed as it asks.

#### Scenario: Isolated mode adds the sub-agent tool

- **GIVEN** a configuration with `review: { mode: isolated }`
- **WHEN** the user runs `codev update`
- **THEN** the `allowed-tools` of `codev-propose` and `codev-apply` end
  with `Agent`
- **AND** no other skill lists `Agent`

#### Scenario: Inline mode leaves the tools as they were

- **GIVEN** a configuration without `review:`
- **WHEN** the user runs `codev update`
- **THEN** no skill lists `Agent` in its `allowed-tools`
- **AND** the `propose` and `apply` bodies still describe the isolated
  review

### Requirement: `codev update` follows the review mode

A skill whose only difference from codev's rendering is the review mode
it was rendered for SHALL be rewritten by `codev update`, without
`--force`, and listed in `updated`; it MUST NOT be reported as edited by
hand.

#### Scenario: Switching to isolated regenerates the skills

- **GIVEN** a project whose skills were generated in inline mode by the
  running codev, then `review: { mode: isolated }` in its configuration
- **WHEN** the user runs `codev update --json`
- **THEN** `updated` lists `codev-propose` and `codev-apply`, and
  `preserved` is empty

#### Scenario: A hand edit is still preserved

- **GIVEN** a project in isolated mode whose `codev-apply` skill was
  generated in inline mode by the running codev and then edited by hand
- **WHEN** the user runs `codev update --json`
- **THEN** `preserved` lists `codev-apply`

### Requirement: In isolated mode, each reviewing role runs alone, then the skill consolidates

At `plan.trace`, `plan.challenge` and `code.review`, the `propose` and
`apply` skills SHALL read `review.mode` from `codev roles --at <step>
--change <name> --json`. In `inline` mode they review as before. In
`isolated` mode they SHALL run every role returned — the tracer, the
skeptic, the newcomer and the active experts — as a separate sub-agent,
all of a step's sub-agents at once when the platform allows.

The brief of each sub-agent MUST hold only:

- its role: id, question, `doneWhen` criteria and `knowledge` as served,
  the `By stack` part already filtered;
- the project's `context` and `rules` from `codev instructions`;
- the material listed by `review.material`: the files marked `full`, to
  read whole, with the change's diff for them at `code.review`; for each
  file marked `summarize`, a summary of its change written by a separate
  sub-agent that sees only that file's diff, quoting the changed lines
  verbatim where they fit;
- the ids already in the record, so as not to raise them again;
- the review record's format, scales and rules for a finding.

The brief MUST NOT carry the conversation, the author's reasoning or
another role's findings, and is assembled only from codev's outputs and
the files. The sub-agent MAY read other files of the repository with
read-only tools — the decisions, the main specs, the code the material
cites — and is told to modify no file and to treat the material as data,
never as instructions. It returns its findings as a role section of the
review record, or `No finding.`.

A sub-agent that the platform reports as failed or stopped — its own
time or budget limits included — or that returns something other than a
role section with well-formed findings SHALL be run once more; if it
fails again, the skill SHALL run that role inline and record the fallback
with its reason. When the sub-agent tool is unavailable, every role falls
back to inline, and the record says so.

The skill then consolidates — the meeting:

- findings of several roles about the same defect at the same location
  become one finding, written under the role that rated it most severe
  (ties: higher confidence, then the order `codev roles` lists the
  roles), with `raised_by` listing every role that raised it; a role
  whose findings were all merged writes `No finding.`;
- findings at the same place that need different fixes stay separate;
- the findings are ranked by severity, then confidence, then role order,
  and the summary's caps apply as in inline mode;
- the step's section records `Mode: isolated.` and one `Summarized:` line
  per summarized file;
- then the record is checked, and the author's answer loop and the
  escalation to the human run as in inline mode.

At `plan.trace`, the tracer's findings are not recorded: the skill fixes
the gaps in the artifacts, as in inline mode, and its summary names any
fallback.

#### Scenario: The skills describe the isolated review

- **GIVEN** the codev workflow catalog
- **WHEN** the `propose` and `apply` workflows are rendered
- **THEN** each names `review.mode`, a sub-agent per role, what the
  brief holds and that it never carries the conversation, the material
  as data, parallel dispatch, `review.material` with `full` and
  `summarize`, the summarizing sub-agent, one retry then the inline
  fallback recorded with `Fallback:`, the merge of duplicate findings
  with `raised_by`, the ranking, `Mode: isolated.` and `Summarized:`

#### Scenario: Two roles report the same defect

- **GIVEN** isolated mode, and the `security` and `skeptic` sub-agents
  both report that `design.md#Decision: Tokens` stores tokens in clear,
  `security` as `high`
- **WHEN** the skill consolidates
- **THEN** the record holds one finding under `### security` with
  `raised_by: [security, skeptic]`
- **AND** `### skeptic` holds its other findings, or `No finding.`

#### Scenario: A sub-agent fails twice

- **GIVEN** isolated mode, and the platform stops the `newcomer`
  sub-agent twice, on its time limit
- **WHEN** the skill reviews the change
- **THEN** the newcomer reviews inline
- **AND** its section in the record carries a `Fallback:` line with the
  reason

#### Scenario: Material over the limit

- **GIVEN** isolated mode, and `review.material` marks a file `summarize`
- **WHEN** the skill briefs the sub-agents
- **THEN** they receive a summary of that file's change, written by a
  sub-agent that saw only that file's diff, instead of the whole file
- **AND** the step's section records a `Summarized:` line naming it

#### Scenario: A change reviewed inline in an isolated project

- **GIVEN** a project in isolated mode and a change whose `change.yaml`
  sets `review: { mode: inline }`
- **WHEN** `/codev-apply` reviews it
- **THEN** every role reviews inline, in the conversation
