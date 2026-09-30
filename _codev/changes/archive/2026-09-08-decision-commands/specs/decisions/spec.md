## ADDED Requirements

### Requirement: Creating a decision via the CLI

`codev decision new <title>` SHALL create a new ADR in the project, with
an automatically generated numeric `id` (the largest numeric `id` found in
the project + 1, on four digits), a file named `NNNN-<slug>.md` where
`<slug>` is derived from the title in kebab-case, and a valid frontmatter
including `status: accepted` and today's date.

#### Scenario: Creation in a project without ADRs

- **GIVEN** a project whose `_codev/decisions/` folder is empty
- **WHEN** the user runs `codev decision new "A first choice"`
- **THEN** the file `_codev/decisions/0001-a-first-choice.md` is created
- **AND** its frontmatter carries `id: "0001"`, `title: "A first choice"`,
  `status: accepted`, and a date in the `YYYY-MM-DD` format

#### Scenario: Sequential numbering

- **GIVEN** a project whose `_codev/decisions/` folder already contains six
  ADRs numbered `0001` to `0006`
- **WHEN** the user runs `codev decision new "A seventh choice"`
- **THEN** the file `_codev/decisions/0007-a-seventh-choice.md` is created
- **AND** its `id` is `"0007"`

#### Scenario: Customizable status

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "Lead to explore"
  --status proposed`
- **THEN** the created ADR carries `status: proposed`

#### Scenario: Empty title refused

- **GIVEN** a project
- **WHEN** the user runs `codev decision new ""`
- **THEN** no file is written
- **AND** an error message names the stable code `empty_title` and
  asks for a non-empty title

### Requirement: Listing decisions via the CLI

`codev decision list` MUST display all local and inherited decisions, each
with its identifier, its title, its status, its effect state, and its
origin. The `--json` mode MUST return exactly one JSON document whose shape
is frozen per version.

#### Scenario: List with local decisions and one supersession

- **GIVEN** a project containing three `accepted` ADRs where `0007`
  supersedes `0003`
- **WHEN** the user runs `codev decision list --json`
- **THEN** the `decisions` array contains the three entries
- **AND** the `0003` entry carries `inEffect: false` and `supersededBy:
  "project/0007"`
- **AND** the `0007` entry carries `inEffect: true`

#### Scenario: Concise human rendering

- **GIVEN** the same context
- **WHEN** the user runs `codev decision list`
- **THEN** each decision appears on its own line with, in order, an effect
  marker (`•` for in effect, `–` otherwise), its `id`, its title, and its
  status

### Requirement: Showing a decision via the CLI

`codev decision show <id>` MUST display the full content of the requested
decision. Resolution accepts a short `id` when it is unambiguous, or a
qualified identifier `<origin>/<id>` in case of a collision.

#### Scenario: Show of an id without collision

- **GIVEN** a project containing a single ADR `0001`
- **WHEN** the user runs `codev decision show 0001`
- **THEN** the rendering contains the frontmatter header (`id`, `title`,
  `status`, `date`) followed by the decision's markdown body

#### Scenario: Show of an ambiguous id

- **GIVEN** a project containing a local ADR `0007` **and** an ADR `0007`
  in an inherited source `path:~/shared`
- **WHEN** the user runs `codev decision show 0007`
- **THEN** no content is displayed
- **AND** the `ambiguous_decision_id` error message lists both qualified
  identifiers (`project/0007`, `path:~/shared/0007`) and asks the user to
  specify

#### Scenario: Show with a qualified identifier

- **GIVEN** the same context
- **WHEN** the user runs `codev decision show path:~/shared/0007`
- **THEN** the inherited decision is displayed

### Requirement: Superseding a decision via the CLI

`codev decision supersede <old-id> <new-title>` MUST create a new decision
that references the old one in its `supersedes`, and rewrite the old one's
frontmatter so that its `status` becomes `superseded`. Both writes happen
in the same plan — either both succeed, or neither is applied.

#### Scenario: Local supersession

- **GIVEN** a project containing an ADR `0003 accepted` titled "Old
  choice"
- **WHEN** the user runs `codev decision supersede 0003 "New
  choice"`
- **THEN** a new ADR `0007-new-choice.md` (or the next available `id`) is
  created with `supersedes: ["0003"]` and `status: accepted`
- **AND** the frontmatter of the `0003` file now has `status:
  superseded`
- **AND** the body of the `0003` file — Context, Decision, everything that
  follows the frontmatter — has remained identical down to the character

#### Scenario: Superseding an unknown id refuses everything

- **GIVEN** a project in which `9999` does not exist
- **WHEN** the user runs `codev decision supersede 9999 "X"`
- **THEN** no file is written
- **AND** the `unknown_decision_id` error message names `9999`

#### Scenario: Superseding an inherited decision refused

- **GIVEN** a project inheriting from a source containing `path:~/shared/0100`
- **WHEN** the user runs `codev decision supersede path:~/shared/0100
  "Our alternative"`
- **THEN** no file is written
- **AND** the `cannot_supersede_inherited` error message explains that an
  inherited decision is read-only and suggests "deviating" from it — the
  name of the action K6 will deliver

### Requirement: Stable JSON contract for all commands

When `--json` is requested, each command MUST write to stdout exactly one
JSON document whose shape is frozen per version, with a root `status`
array for execution errors — the same rules as the rest of codev's
contract.

#### Scenario: Shape of a `decision new --json`

- **GIVEN** a project
- **WHEN** the user runs `codev decision new "X" --json`
- **THEN** stdout carries a JSON document containing `changeName: null`
  (this command does not act on a change), `decision: { id: "0001",
  qualifiedId: "project/0001", path: "…", title: "X", status: "accepted"
  }`, and a `status: []`

#### Scenario: Shape of a `decision show` failure on an unknown id

- **GIVEN** a project
- **WHEN** the user runs `codev decision show 9999 --json`
- **THEN** stdout carries a single JSON document containing
  `decision: null` and `status: [{ code: "unknown_decision_id", … }]`
