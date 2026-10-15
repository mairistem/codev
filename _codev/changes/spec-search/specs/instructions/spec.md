## ADDED Requirements

### Requirement: `codev instructions` names the existing requirements the change likely touches

Once the change folder holds its `proposal.md`, `codev instructions` SHALL
give, for every artifact other than the proposal itself, the requirements
of the project's main specs most relevant to the proposal's text — its
`Why` and `What Changes` sections, its whole text when it has neither —,
ranked as `codev search` ranks them, in the language the specs are written
in, at most 10. With `--json`, the document SHALL carry `related`: one
entry per requirement, the most relevant first, with `capability`,
`requirement` and `excerpt`, the excerpt at most 160 characters of the
requirement's description. `related` SHALL be an empty array for the
proposal itself, when `proposal.md` does not exist yet, when the project
has no main spec, or when nothing matches. The decisions in effect stay in
`decisions` and are not repeated in `related`.

Without `--json`, the instructions SHALL print, after `Read before
writing:` and only when not empty, `Existing requirements this change
likely touches:`, one line per requirement as `<capability>::<requirement>
— <excerpt>`, followed by one line saying that a requirement the change
alters is modified, in a `MODIFIED` block copied from its spec, rather than
added again.

The command reads files and writes none, as before.

#### Scenario: The specs of a change about decisions

- **GIVEN** a project whose spec `decisions` holds the requirement
  `Creating a decision through the CLI`, and a change whose `proposal.md`
  says it lets a user create a decision as a draft from the command line
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` lists `decisions` / `Creating a decision through the CLI`
  with an excerpt
- **AND** it holds at most 10 entries

#### Scenario: The proposal itself

- **WHEN** the user runs `codev instructions proposal --change draft-decisions --json`
- **THEN** `related` is an empty array

#### Scenario: A proposal not written yet

- **GIVEN** a change whose folder holds no `proposal.md`
- **WHEN** the user runs `codev instructions specs --change draft-decisions --json`
- **THEN** `related` is an empty array

#### Scenario: The human output

- **GIVEN** the change of the first scenario
- **WHEN** the user runs `codev instructions design --change draft-decisions`
- **THEN** the output has the section `Existing requirements this change
  likely touches:` with the line
  `decisions::Creating a decision through the CLI — …`
- **AND** a line saying that a requirement the change alters is modified,
  not added again

#### Scenario: A project without specs

- **GIVEN** a project whose `_codev/specs/` is empty, and a change whose
  `proposal.md` exists
- **WHEN** the user runs `codev instructions specs --change first --json`
- **THEN** `related` is an empty array and the human output has no
  `Existing requirements` section
