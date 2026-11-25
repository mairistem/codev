## Purpose

Give codev a structured, reliable reading of main specs and deltas written
in markdown, on which validation, sync and archive can rely without risking
a destructive rewrite.

## ADDED Requirements

### Requirement: Structure of a main spec extracted

The parser SHALL extract from a main spec markdown file its `## Purpose`
section, its `## Requirements` section, and for each requirement its name,
its descriptive text and its scenarios.

#### Scenario: Well-formed Purpose and requirements

- **GIVEN** a file containing `## Purpose`, a sentence, then `## Requirements`,
  then a `### Requirement: Session Expiration` followed by a
  `#### Scenario: Idle` with **WHEN** / **THEN** lines
- **WHEN** the parser reads the file
- **THEN** the result exposes the Purpose text
- **AND** exposes a requirement named `Session Expiration` carrying its
  scenario named `Idle`

#### Scenario: Missing Purpose on a main spec

- **GIVEN** a main spec file without a `## Purpose` section
- **WHEN** the parser reads the file
- **THEN** the result reports the missing Purpose as a structural defect
  naming what is missing
- **AND** the rest of the requirements remain extractable

### Requirement: Delta operations recognized

The parser MUST recognize the four operations of a delta and associate
each with its payload: the complete requirement block for `ADDED` and
`MODIFIED`, the name along with `**Reason**` and `**Migration**` for
`REMOVED`, and the `FROM:` / `TO:` pair for `RENAMED`.

#### Scenario: ADDED block with requirement and scenario

- **GIVEN** a delta containing `## ADDED Requirements` then
  `### Requirement: Two-Factor Authentication` with a `#### Scenario: Enrolment`
- **WHEN** the parser reads the delta
- **THEN** the `ADDED` operation carries the requirement
  `Two-Factor Authentication`
- **AND** that requirement carries its scenario `Enrolment`

#### Scenario: REMOVED block with reason and migration

- **GIVEN** a delta containing `## REMOVED Requirements` then
  `### Requirement: Remember Me` followed by `**Reason**: <text>` and
  `**Migration**: <text>`
- **WHEN** the parser reads the delta
- **THEN** the `REMOVED` operation carries the name `Remember Me`, its reason
  and its migration

#### Scenario: RENAMED block with FROM and TO

- **GIVEN** a delta containing `## RENAMED Requirements` then the lines
  `FROM: Old Name` and `TO: New Name`
- **WHEN** the parser reads the delta
- **THEN** the `RENAMED` operation associates the old name `Old Name` with
  the new name `New Name`

#### Scenario: Delta for a new capability with Purpose

- **GIVEN** a delta that starts with `## Purpose` followed by a sentence,
  then contains a `## ADDED Requirements` block
- **WHEN** the parser reads the delta
- **THEN** the result carries the Purpose text, to be copied verbatim when
  creating a new main spec

### Requirement: Literal zones ignored

The parser MUST ignore any structure — headings, requirements, scenarios,
delta headers — that appears inside a code block delimited by ` ``` ` or
`~~~`, or inside an HTML comment `<!-- … -->`.

#### Scenario: Example requirement inside a code block

- **GIVEN** a file whose Purpose section contains a ` ``` ` block holding
  the line `### Requirement: Example`
- **WHEN** the parser reads the file
- **THEN** no requirement named `Example` appears in the result

#### Scenario: Delta header inside a comment

- **GIVEN** a delta containing a comment `<!-- ## ADDED Requirements … -->`
  followed, further down, by a real `## ADDED Requirements` with a
  requirement
- **WHEN** the parser reads the delta
- **THEN** the `ADDED` operation is counted only once, with the requirement
  from the real block

### Requirement: Source position preserved

Each extracted element — Purpose, requirement, scenario, delta block — SHALL
carry the exact `[start, end)` interval it occupies in the source text, in
bytes and in lines.

#### Scenario: Rewriting one block without touching the rest

- **GIVEN** a main spec file containing two consecutive requirements
- **WHEN** a consumer replaces the source text occupied by the first
  requirement with a new block of different length
- **THEN** the second requirement, its scenarios and the whitespace around
  them remain identical down to the character

#### Scenario: Line position of a scenario

- **GIVEN** a file where the `Idle` scenario of a requirement starts on
  line 42
- **WHEN** the parser reads the file
- **THEN** the element representing that scenario exposes line 42 as its
  start

### Requirement: Localized structural defects

Faced with a malformed file, the parser MUST produce a report naming the
line concerned, the type of defect, and a readable message; it SHALL NOT
fail wholesale on a partially recoverable file.

#### Scenario: Scenario written with three hashes

- **GIVEN** a requirement whose scenario is written `### Scenario:` instead
  of `#### Scenario:`
- **WHEN** the parser reads the file
- **THEN** a structural defect reports the line and states that the
  scenario must carry four hashes
- **AND** the requirement still appears in the result, without that
  scenario

#### Scenario: Duplicate requirement within one section

- **GIVEN** a `## ADDED Requirements` delta containing two requirements
  bearing exactly the same name
- **WHEN** the parser reads the delta
- **THEN** a defect reports both lines of the duplicated requirements
- **AND** names the `ADDED` section as the location of the conflict

#### Scenario: Delta header in a main spec

- **GIVEN** a main spec file that mistakenly contains a
  `## ADDED Requirements`
- **WHEN** the parser reads the file
- **THEN** a defect reports the line and specifies that delta headers
  belong only in change files
