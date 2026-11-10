## MODIFIED Requirements

### Requirement: Localized structural defects

When faced with a malformed file, the parser MUST produce a report naming the
line concerned, the defect type, and a readable message; it SHALL NOT
fail wholesale on a partially recoverable file. A requirement name defined
twice is such a defect, in a delta section and in the requirements section
of a main spec alike: the parser MUST report it as `duplicate_requirement`
with both lines.

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

#### Scenario: Duplicate requirement in a main spec

- **GIVEN** a main spec whose `## Requirements` holds `### Requirement: Login`
  at line 12 and again at line 80, as a text merge of two branches that
  each added it leaves it
- **WHEN** the parser reads the file
- **THEN** a `duplicate_requirement` defect names `Login` and both lines
  12 and 80
- **AND** both requirements still appear in the result

#### Scenario: Two requirements with distinct names in a main spec

- **GIVEN** a main spec holding `### Requirement: Login` and
  `### Requirement: Login with SSO`
- **WHEN** the parser reads the file
- **THEN** no `duplicate_requirement` defect is reported

#### Scenario: Delta header in a main spec

- **GIVEN** a main spec file that mistakenly contains an
  `## ADDED Requirements`
- **WHEN** the parser reads the file
- **THEN** a defect reports the line and specifies that delta headers
  belong only in change files

#### Scenario: French delta header in a main spec

- **GIVEN** a main spec file whose sections are `## Objet` and
  `## Exigences`, and that mistakenly contains an `## Exigences AJOUTÉES`
- **WHEN** the parser reads the file
- **THEN** a defect reports the line of `## Exigences AJOUTÉES` and
  specifies that delta headers belong only in change files
- **AND** `## Exigences AJOUTÉES` is not taken for the requirements section

#### Scenario: Unexpected heading inside a delta section

- **GIVEN** an `## ADDED Requirements` section containing a heading
  `### Notes` instead of `### Requirement: …`
- **WHEN** the parser reads the delta
- **THEN** an error `delta_unexpected_heading` reports the line
- **AND** the section is also reported as empty with the warning
  `delta_section_empty`
