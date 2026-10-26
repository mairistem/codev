## MODIFIED Requirements

### Requirement: Skills write artifact prose in the configured language

The skills that write planning content — `propose`, `update` and
`configure` — SHALL write their prose in the language set by `language:` in
`_codev/config.yaml` (`en` when absent), whatever language the conversation
is in. `codev instructions --json` MUST expose that language as `language`.
The `propose` and `update` skills SHALL write the structural keywords —
the template's headings, delta section headings, requirement and scenario
headings, the scenario lines and the normative words — exactly as
`template` and `keywords` give them, in `keywordLanguage`, and never
translate a keyword themselves. The citation line of a ticket at the top
of a proposal SHALL be written in `language` as well. When `update` revises
an artifact that already holds structural keywords, it SHALL keep the
language of that artifact's keywords, so that a change planned before the
project's keyword language changed stays in one language;
`keywordLanguage` applies to a file it creates.

#### Scenario: A plan entirely in French

- **GIVEN** a project whose config contains `language: fr`
- **WHEN** the user runs `/codev-propose` in an English conversation
- **THEN** the proposal's sentences are written in French
- **AND** its headings are those of the French template, among them
  `## Pourquoi`, `## Changements` and `## Domaines`
- **AND** its delta specs use `## Exigences AJOUTÉES`, `### Exigence :`,
  `#### Scénario :` and `DOIT`, and `codev validate` reports no finding

#### Scenario: The ticket citation in the project's language

- **GIVEN** a project whose config contains `language: fr`
- **WHEN** the user runs `/codev-propose` with a ticket `PROJ-12` whose
  content is retrieved
- **THEN** the proposal starts with `# Proposition :` and its next line is
  the citation of `PROJ-12` written in French

#### Scenario: An artifact already written keeps its keywords

- **GIVEN** a project whose config contains `language: fr`, and an active
  change whose delta spec holds `## ADDED Requirements` and
  `### Requirement:` headings
- **WHEN** the user runs `/codev-update` to add a requirement to that delta
- **THEN** the new requirement is written `### Requirement:` with `SHALL`,
  in the delta's own language, and its prose is in French

#### Scenario: A plan entirely in English

- **GIVEN** a project whose config contains `language: en`, or no
  `language:`
- **WHEN** the user runs `/codev-propose` in a French conversation
- **THEN** the proposal's sentences and headings are in English, with
  `## Why` and `## What Changes`
- **AND** its delta specs use `## ADDED Requirements`, `### Requirement:`
  and `SHALL`

#### Scenario: A language without a catalog

- **GIVEN** a project whose config contains `language: pt-BR`
- **WHEN** the user runs `/codev-propose`
- **THEN** the sentences are written in Brazilian Portuguese and the
  structural keywords in English, as `keywordLanguage` says
