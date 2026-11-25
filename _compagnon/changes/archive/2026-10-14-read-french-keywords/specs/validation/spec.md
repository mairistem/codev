## MODIFIED Requirements

### Requirement: Structural rules beyond the parser

The validator MUST reject as errors the defects that the parser lets
through because they require inspecting the content of a requirement or a
spec: a requirement without a normative keyword of the catalog in its
description — `SHALL`, `MUST`, `DOIT` or `DOIVENT`, in upper case — a
requirement with no scenario, a main spec with no extractable
requirement.

#### Scenario: Requirement without SHALL or MUST

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: X`
  whose descriptive text is "The system does something"
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_shall` reports the line of
  the header and names the requirement `X`

#### Scenario: French requirement with DOIT

- **GIVEN** an `## Exigences AJOUTÉES` delta with a
  `### Exigence : Annulation` whose descriptive text is
  "Le portail DOIT permettre d'annuler une demande en attente." and a
  `#### Scénario : Dans les temps`
- **WHEN** the validator inspects the file
- **THEN** no `requirement_no_shall` finding is reported

#### Scenario: Requirement with no scenario at all

- **GIVEN** an `## ADDED Requirements` delta with a `### Requirement: Y`
  followed by its descriptive text but no `#### Scenario:`
- **WHEN** the validator inspects the file
- **THEN** a finding with code `requirement_no_scenario` reports the line
  of the requirement

#### Scenario: Main spec with no requirement

- **GIVEN** a main spec `_codev/specs/x/spec.md` with `## Purpose`
  but whose `## Requirements` is empty
- **WHEN** the validator inspects the spec
- **THEN** a finding with code `spec_no_requirement` reports that the spec
  has no extractable requirement

## ADDED Requirements

### Requirement: The proposal sections are checked in every catalog language

The checks of a proposal's release note and domains SHALL apply to the
section written with any catalog language's heading — `## Note de version`
and `## Domaines` in French — with the same findings and codes. A release
note category and the single line `None` are accepted in any catalog
language (`Ajouté`, `Aucune`…); domain names stay the identifiers
`security`, `personal-data`, `user-interface`, `performance`, `operations`
and `migration` in every language. A proposal that has the section in one
language is not reported as missing it. When a proposal holds the same
section in two languages, the first one in the file is read and the other
is ignored, as a second section of the same heading is today.

#### Scenario: French release note and domains

- **GIVEN** an active change whose proposal holds `## Note de version`
  with `- Ajouté : un export.` and `## Domaines` with `- [x] migration`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` and no domain finding is reported

#### Scenario: Internal change written in French

- **GIVEN** an active change whose `## Note de version` is `Aucune` and
  whose `## Domaines` is `Aucun`
- **WHEN** the user runs `codev validate <change>`
- **THEN** no `release_note_*` and no domain finding is reported

#### Scenario: Unknown French category

- **GIVEN** an active change whose `## Note de version` holds
  `- Amélioré : la vitesse.`
- **WHEN** the user runs `codev validate <change>`
- **THEN** a `release_note_invalid` error names `Amélioré` and its line

#### Scenario: Translated domain name

- **GIVEN** an active change whose `## Domaines` holds `- [x] sécurité`
- **WHEN** the user runs `codev validate <change>`
- **THEN** an `unknown_domain` error names `sécurité` and its line

#### Scenario: The same section in two languages

- **GIVEN** an active change whose proposal holds `## Domains` with
  `- [x] migration`, then `## Domaines` with `- [x] sécurité`
- **WHEN** the user runs `codev validate <change>`
- **THEN** the domains read are those of `## Domains`, and no
  `unknown_domain` finding names `sécurité`
