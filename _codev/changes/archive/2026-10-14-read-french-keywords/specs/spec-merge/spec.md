## ADDED Requirements

### Requirement: A merge writes keywords in the language of the spec it edits

When a sync or an archive writes into a main spec, it SHALL write every
structural keyword — requirement and scenario headings, section headings —
in the language of that spec, so that a French spec stays French. The
language of an existing main spec is the language of its requirements
section heading (`## Requirements` or `## Exigences`); a spec whose
heading matches no catalog language is written in English, as today. A
new main spec takes the language of the delta section heading that
creates it. The requirement names, descriptions and scenario bodies are
copied verbatim, whatever their language.

#### Scenario: French requirement merged into a French spec

- **GIVEN** a main spec whose sections are `## Objet` and `## Exigences`
- **AND** a delta holding `## Exigences MODIFIÉES` with
  `### Exigence : Annulation`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the spec's requirement is written `### Exigence : Annulation`
  with its scenarios as `#### Scénario : <name>`

#### Scenario: English delta merged into a French spec

- **GIVEN** a main spec whose requirements heading is `## Exigences`
- **AND** a delta holding `## ADDED Requirements` with
  `### Requirement: Export`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the new requirement is written `### Exigence : Export` in the
  spec

#### Scenario: New capability created from a French delta

- **GIVEN** no main spec under `_codev/specs/conges/`
- **AND** a delta holding `## Objet` and `## Exigences AJOUTÉES` with
  `### Exigence : Annulation`
- **WHEN** the user runs `codev sync <change>`
- **THEN** the new spec holds `## Objet`, the Purpose text, then
  `## Exigences` and `### Exigence : Annulation`

#### Scenario: English specs are written as before

- **GIVEN** a main spec whose requirements heading is `## Requirements`
- **AND** an English delta
- **WHEN** the user runs `codev sync <change>`
- **THEN** the spec's bytes are those an earlier codev would have written
