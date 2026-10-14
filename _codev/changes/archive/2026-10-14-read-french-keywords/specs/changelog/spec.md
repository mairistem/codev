## ADDED Requirements

### Requirement: A release note in any catalog language feeds the same categories

`codev changelog` SHALL read a release note written under any catalog
language's heading (`## Note de version`), with categories and the single
line `None` in any catalog language, and SHALL report each item under the
category's English name — `Added`, `Changed`, `Deprecated`, `Removed`,
`Fixed` or `Security` — in every output format, so that one changelog
gathers changes written in different languages. A change archived without
a release note falls back to the bullets of its `## What Changes` section,
in any catalog language (`## Changements`).

#### Scenario: French items grouped with English ones

- **GIVEN** two archived changes, one whose note is `- Added: a flag.` and
  one whose `## Note de version` is `- Ajouté : un export.`
- **WHEN** the user runs `codev changelog --format json`
- **THEN** both items have the category `Added`

#### Scenario: A French internal change is omitted

- **GIVEN** an archived change whose `## Note de version` is `Aucune`
- **WHEN** the user runs `codev changelog`
- **THEN** no item of that change appears and no warning names it

#### Scenario: A French change without a release note

- **GIVEN** an archived change whose proposal has no release note and
  whose `## Changements` holds `- Un export mensuel.`
- **WHEN** the user runs `codev changelog --format json`
- **THEN** the item `Un export mensuel.` is listed under `Changed`
