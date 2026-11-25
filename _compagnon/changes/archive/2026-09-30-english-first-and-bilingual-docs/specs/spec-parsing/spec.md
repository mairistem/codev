## MODIFIED Requirements

### Requirement: Delta operations recognized

The parser MUST recognize the four operations of a delta and associate
each with its payload: the full requirement block for `ADDED` and
`MODIFIED`, the name accompanied by `**Reason**` and `**Migration**`
for `REMOVED`, and the `FROM:` / `TO:` pair for `RENAMED`. A `FROM:` or
`TO:` value MAY be the bare name or the heading form
`` `### Requirement: <name>` ``; both designate the same requirement.

#### Scenario: ADDED block with requirement and scenario

- **GIVEN** a delta containing `## ADDED Requirements` then
  `### Requirement: Two-Factor Authentication` with a `#### Scenario: Enrolment`
- **WHEN** the parser reads the delta
- **THEN** the `ADDED` operation carries the requirement `Two-Factor Authentication`
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

- **GIVEN** a delta that starts with `## Purpose` followed by a sentence, then
  contains an `## ADDED Requirements` block
- **WHEN** the parser reads the delta
- **THEN** the result carries the Purpose text, to be copied verbatim when
  creating a new main spec

#### Scenario: RENAMED written with the heading form

- **GIVEN** a delta containing `## RENAMED Requirements` then the lines
  ``- FROM: `### Requirement: Old Name` `` and
  ``- TO: `### Requirement: New Name` ``
- **WHEN** the parser reads the delta
- **THEN** the `RENAMED` operation associates `Old Name` with `New Name`

### Requirement: Localized structural defects

When faced with a malformed file, the parser MUST produce a report naming the
line concerned, the defect type, and a readable message; it SHALL NOT
fail wholesale on a partially recoverable file.

#### Scenario: Scenario written with three hashes

- **GIVEN** a requirement whose scenario is written `### Scenario:` instead of
  `#### Scenario:`
- **WHEN** the parser reads the file
- **THEN** a structural defect reports the line and indicates that the
  scenario must carry four hashes
- **AND** the requirement still appears in the result, without that scenario

#### Scenario: Duplicate requirement within the same section

- **GIVEN** an `## ADDED Requirements` delta containing two requirements with
  exactly the same name
- **WHEN** the parser reads the delta
- **THEN** a defect reports both lines of the duplicate requirements
- **AND** names the `ADDED` section as the location of the conflict

#### Scenario: Delta header in a main spec

- **GIVEN** a main spec file that mistakenly contains an
  `## ADDED Requirements`
- **WHEN** the parser reads the file
- **THEN** a defect reports the line and specifies that delta headers
  belong only in change files

#### Scenario: Unexpected heading inside a delta section

- **GIVEN** an `## ADDED Requirements` section containing a heading
  `### Exigence : Mode sombre` instead of `### Requirement: …`
- **WHEN** the parser reads the delta
- **THEN** an error `delta_unexpected_heading` reports the line
- **AND** the section is also reported as empty with the warning
  `delta_section_empty`
