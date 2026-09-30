## Purpose

Make architecture decisions usable by codev: read them, index them, resolve
the supersession chain, and inject them into the instructions for the
`design` artifact so that a drafting agent sees the already settled
choices right away.

## ADDED Requirements

### Requirement: Recognized ADR format

The parser SHALL recognize an ADR written as YAML frontmatter followed by
free-form markdown sections. The frontmatter carries at least the fields
`id`, `title`, `status`, `date`, and optionally `tags` (list), `supersedes`
(identifier or array of identifiers). A file without frontmatter, or whose
frontmatter lacks a mandatory field, is reported, not silently parsed.

#### Scenario: Well-formed ADR

- **GIVEN** a file starting with `---`, containing YAML frontmatter
  with `id: 0007`, `title: "…"`, `status: accepted`, `date: 2026-09-08`,
  `tags: [architecture]`, then a `---` separator, then free-form markdown
- **WHEN** the parser reads the file
- **THEN** the result exposes the identifier `0007`, the title, the status
  `accepted`, the date and the tag `architecture`

#### Scenario: ADR without frontmatter

- **GIVEN** a markdown file without a `---` header
- **WHEN** the parser reads the file
- **THEN** a finding with the stable code `decision_missing_frontmatter`
  reports that the file does not have the expected format

#### Scenario: Missing mandatory field

- **GIVEN** a file whose frontmatter has no `title`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_missing_field` names the missing
  field

### Requirement: Recognized statuses and effect

The validator MUST recognize the statuses `accepted`, `superseded`,
`proposed`, `deprecated`, `rejected`; only `accepted` and `superseded`
are taken into account when computing the effect — the other three are
exposed as is in the index and never considered "in effect".

#### Scenario: Unknown status reported

- **GIVEN** an ADR whose `status` is `pending`
- **WHEN** the parser reads the file
- **THEN** a finding with code `decision_unknown_status` reports the value
  and recalls the list of recognized statuses

#### Scenario: Proposed status does not take effect

- **GIVEN** an ADR with status `proposed`, with no supersession link
- **WHEN** the index is computed
- **THEN** this decision does not appear among the "decisions in effect"

### Requirement: Supersession resolved as a chain

The index MUST resolve the `supersedes` field: each decision that an ADR
supersedes is marked `superseded_by(<id>)` in the index, and is not in
effect. A chain `A ← B ← C` leaves `A` and `B` superseded; only `C`
remains in effect.

#### Scenario: Direct supersession

- **GIVEN** an ADR `0003 accepted` and an ADR `0007 accepted supersedes: [0003]`
- **WHEN** the index is computed
- **THEN** `0003` is marked `superseded_by(0007)` and does not appear among
  the decisions in effect
- **AND** `0007` appears among the decisions in effect

#### Scenario: Three-link chain

- **GIVEN** three `accepted` ADRs where `B` supersedes `A` and `C`
  supersedes `B`
- **WHEN** the index is computed
- **THEN** only `C` is in effect

#### Scenario: Missing supersession target

- **GIVEN** an ADR `0007 supersedes: [9999]` while `9999` does not exist
- **WHEN** the index is computed
- **THEN** a finding with code `decision_supersedes_unknown` reports the
  phantom identifier
- **AND** `0007` remains in effect (the lost link does not disqualify it)

### Requirement: Inherited decisions taken into account

When a project declares `inherits: path: <path>` in its
`_codev/config.yaml`, the validator MUST also parse the ADRs in
`<path>/_codev/decisions/` and merge them into the index with their
`origin` visible.

#### Scenario: ADR from an inherited source appears in the index

- **GIVEN** a project inheriting from a source `path: ~/shared` containing
  an ADR `0100 accepted`
- **WHEN** the index is computed
- **THEN** the entry carries the `origin` `path:~/shared`
- **AND** its qualified identifier, to avoid collisions, is
  `path:~/shared/0100`

#### Scenario: Id collision between project and source

- **GIVEN** a local ADR `0007 accepted` **and** an ADR `0007 accepted` in
  an inherited source
- **WHEN** the index is computed
- **THEN** a finding with code `decision_id_collision` reports the
  duplicate
- **AND** the project's version wins (it is closer to the author)

### Requirement: Injection into the design instructions

The call `codev instructions design --change <name>` MUST enrich its
response with a `decisions[]` field carrying the decisions in effect. Each
entry exposes `id`, `title`, `status`, `tags`, `path` (relative to the
project) and `origin`. The full content stays in the file — no duplication
in the response.

#### Scenario: Design instructions carry the decisions in effect

- **GIVEN** a project with 6 `accepted` ADRs and no supersession
- **WHEN** the user runs `codev instructions design --change <name>
  --json`
- **THEN** the JSON response contains a `decisions` array with exactly
  6 entries, each carrying `id`, `title`, `status`, a `path` relative to
  the project and `origin: "project"`

#### Scenario: Superseded decisions absent from the instructions

- **GIVEN** a project where `0003` is superseded by `0007`
- **WHEN** the user runs `codev instructions design --change <name>
  --json`
- **THEN** `0003` does not appear in the `decisions` array
- **AND** `0007` does appear in it

#### Scenario: Human rendering lists the decisions

- **GIVEN** the same context
- **WHEN** the user runs `codev instructions design --change <name>`
  (without `--json`)
- **THEN** the rendering contains a "Decisions in effect" section listing
  each decision on its own line with its `id` and its `title`

#### Scenario: No decision, no section

- **GIVEN** a new project without any ADR
- **WHEN** the user runs `codev instructions design --change <name>`
- **THEN** no "Decisions in effect" section is added to the rendering
- **AND** the `decisions[]` field in the JSON response is present and empty
