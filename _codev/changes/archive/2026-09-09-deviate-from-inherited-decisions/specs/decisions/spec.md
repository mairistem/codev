## ADDED Requirements

### Requirement: `deviates_from` field recognized in an ADR's frontmatter

The ADR parser SHALL recognize an optional `deviates_from` field in the
YAML frontmatter — a list of qualified identifiers (`<origin>/<id>`)
pointing to the decisions this new ADR locally departs from. The field
is additive: its absence keeps the current ADR semantics. A value that
is not a list, or whose entries are not strings, is reported.

#### Scenario: ADR with `deviates_from`

- **GIVEN** a local ADR whose frontmatter carries
  `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the parser reads the file
- **THEN** the result exposes the list `["path:~/shared/0100"]` on the
  `deviates_from` field
- **AND** no finding is emitted for this field

#### Scenario: Malformed `deviates_from`

- **GIVEN** an ADR whose frontmatter carries `deviates_from: "not-a-list"`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_field_type_mismatch` reports
  the `deviates_from` field

### Requirement: `codev decision deviate` command creates a deviation ADR

`codev decision deviate <qualified-id> <title>` MUST create a local
`accepted` ADR with `deviates_from: ["<qualified-id>"]` and a valid
frontmatter (`id`, `title`, `status: accepted`, `date`), sealed by the
same effect plan — consistent with K3.

The command MUST refuse:

- if `<qualified-id>` designates a local decision (`project/…`): stable
  code `cannot_deviate_from_local`, with a message pointing to
  `codev decision supersede`.
- if `<qualified-id>` matches no indexed decision: stable code
  `unknown_decision_id` (an existing code).
- if `<title>` is empty: stable code `empty_title` (an existing code).

#### Scenario: Deviation from an inherited `path:` decision

- **GIVEN** a project inheriting from a source `path: ~/shared`
  containing an ADR `path:~/shared/0100`
- **WHEN** the user runs
  `codev decision deviate path:~/shared/0100 "Our local alternative"`
- **THEN** a new local ADR is created under
  `_codev/decisions/NNNN-our-local-alternative.md` with
  `deviates_from: ["path:~/shared/0100"]` and `status: accepted`
- **AND** an entry is added to `_codev/decisions/seal.yaml` for this
  new ADR

#### Scenario: Refusal to deviate from a local decision

- **GIVEN** a project containing a local ADR `0003 accepted`
- **WHEN** the user runs `codev decision deviate project/0003 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code
  `cannot_deviate_from_local` and suggests `codev decision supersede`

#### Scenario: Refusal of an unknown target

- **GIVEN** a project without an inherited source
- **WHEN** the user runs
  `codev decision deviate path:~/unknown/0100 "…"`
- **THEN** no file is written
- **AND** the error message names the stable code `unknown_decision_id`

### Requirement: The index hides deviated inherited decisions and exposes `deviated_by`

When the decision index computes the entries in effect, each inherited
decision referenced by a `deviates_from` of a local `accepted` ADR MUST
be marked `deviated_by: <local-qualified-id>` in the index, removed
from the `in_effect` array, and appear neither in the `decisions[]`
array of the `design` instructions, nor in the human "Decisions in
effect" section of their rendering.

The inherited entry stays visible in `codev decision list` —
transparency prevails over hiding.

#### Scenario: Design instructions do not carry the deviated decision

- **GIVEN** a project inheriting from a source containing
  `path:~/shared/0100 accepted`, and a local ADR `0007 accepted` whose
  frontmatter carries `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the user runs
  `codev instructions design --change <name> --json`
- **THEN** the `decisions` array of the response contains `project/0007`
- **AND** the `decisions` array does NOT contain `path:~/shared/0100`

#### Scenario: `decision list` exposes the deviation

- **GIVEN** the same context
- **WHEN** the user runs `codev decision list --json`
- **THEN** the `path:~/shared/0100` entry carries `deviatedBy:
  "project/0007"` and `inEffect: false`
- **AND** the `project/0007` entry carries `deviatesFrom:
  ["path:~/shared/0100"]` and `inEffect: true`

#### Scenario: A `proposed` local ADR does not cause a deviation

- **GIVEN** a local ADR `0007 proposed` with `deviates_from:
  ["path:~/shared/0100"]`
- **WHEN** the index is computed
- **THEN** `path:~/shared/0100` stays in effect (the proposed ADR is
  not yet committed, it cannot deviate)
- **AND** the `path:~/shared/0100` entry carries no `deviatedBy`

### Requirement: `validate` detects degenerate deviations

`codev validate` MUST emit two new findings with stable codes:

- `decision_dangling_deviation` — **warning** — when a local ADR has a
  `deviates_from: ["<qualified-id>"]` whose target does not exist or no
  longer exists in the index (source removed, SHA moved, id changed).
- `decision_conflicting_deviations` — **error** — when two local
  `accepted` ADRs reference the same target in their `deviates_from`.
  The rule is: "one target, one deviation".

#### Scenario: Orphan deviation

- **GIVEN** a project where a local ADR `0007 accepted` carries
  `deviates_from: ["path:~/unknown/9999"]`, with no source exposing
  this `qualified-id`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a finding with code
  `decision_dangling_deviation` naming `0007` and its missing target
- **AND** the command's exit code is zero (warning)

#### Scenario: Two deviations on the same target

- **GIVEN** a project with two local `accepted` ADRs, `0007` and
  `0008`, both with `deviates_from: ["path:~/shared/0100"]`
- **WHEN** the user runs `codev validate`
- **THEN** the output contains a
  `decision_conflicting_deviations` finding naming both ADRs and the
  conflicting target
- **AND** the command's exit code is non-zero (error)
