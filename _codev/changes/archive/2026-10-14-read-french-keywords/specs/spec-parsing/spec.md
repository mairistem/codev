## ADDED Requirements

### Requirement: Delta operations are recognized with French keywords

The parser SHALL recognize the four delta operations written with the
French keywords of the catalog, with the same payloads as in English: the
full requirement block for `## Exigences AJOUTÉES` and
`## Exigences MODIFIÉES`, the name with `**Raison**` and `**Migration**`
for `## Exigences SUPPRIMÉES`, and the `DE :` / `VERS :` pair for
`## Exigences RENOMMÉES`. A `DE :` or `VERS :` value MAY be the bare name
or the heading form `` `### Exigence : <name>` ``.

#### Scenario: SUPPRIMÉES block with reason and migration

- **GIVEN** a delta holding `## Exigences SUPPRIMÉES` then
  `### Exigence : Se souvenir de moi` followed by `**Raison** : <text>`
  and `**Migration** : <text>`
- **WHEN** the parser reads the delta
- **THEN** the `REMOVED` operation carries the name
  `Se souvenir de moi`, its reason and its migration

#### Scenario: RENOMMÉES block in the heading form

- **GIVEN** a delta holding `## Exigences RENOMMÉES` then the lines
  ``- DE : `### Exigence : Ancien nom` `` and
  ``- VERS : `### Exigence : Nouveau nom` ``
- **WHEN** the parser reads the delta
- **THEN** the `RENAMED` operation associates `Ancien nom` with
  `Nouveau nom`

#### Scenario: A French scenario at the wrong level is reported

- **GIVEN** a delta whose requirement holds `### Scénario : Trop haut`
- **WHEN** the parser reads the delta
- **THEN** a structural defect reports the wrong-level scenario and its
  line, as it does for `### Scenario:`

#### Scenario: Delta for a new capability with a French Purpose

- **GIVEN** a delta that starts with `## Objet` followed by a sentence,
  then holds a `## Exigences AJOUTÉES` block
- **WHEN** the parser reads the delta
- **THEN** the result carries the Purpose text, to be copied verbatim when
  creating the new main spec

## MODIFIED Requirements

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
