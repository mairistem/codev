## Purpose

Let a team write the structural keywords of its specs, deltas, proposals and
designs in its own language, by giving every keyword a stable key and each
supported language its own text for every key.

## ADDED Requirements

### Requirement: Every structural keyword has a key and a text per language

codev SHALL know a catalog of structural keywords, each identified by a
stable key, and SHALL ship the languages English (`en`) and French (`fr`),
each giving a text for every key. A key MAY have several texts in a
language — alternative spellings or grammatical forms — and the first one
is the form codev writes. The shipped keywords are:

| Key | `en` | `fr` |
|---|---|---|
| `spec.title` | `# {title} Specification` | `# Spécification {title}` |
| `spec.purpose` | `## Purpose` | `## Objet` |
| `spec.requirements` | `## Requirements` | `## Exigences` |
| `requirement` | `### Requirement:` | `### Exigence :` |
| `scenario` | `#### Scenario:` | `#### Scénario :` |
| `delta.added` | `## ADDED Requirements` | `## Exigences AJOUTÉES` |
| `delta.modified` | `## MODIFIED Requirements` | `## Exigences MODIFIÉES` |
| `delta.removed` | `## REMOVED Requirements` | `## Exigences SUPPRIMÉES` |
| `delta.renamed` | `## RENAMED Requirements` | `## Exigences RENOMMÉES` |
| `removed.reason` | `**Reason**` | `**Raison**` |
| `removed.migration` | `**Migration**` | `**Migration**` |
| `renamed.from` | `FROM:` | `DE :` |
| `renamed.to` | `TO:` | `VERS :` |
| `normative` | `SHALL`, `MUST` | `DOIT`, `DOIVENT` |
| `proposal.what_changes` | `## What Changes` | `## Changements` |
| `proposal.domains` | `## Domains` | `## Domaines` |
| `proposal.release_note` | `## Release note` | `## Note de version` |
| `none` | `None` | `Aucun`, `Aucune` |
| `category.added` | `Added` | `Ajouté` |
| `category.changed` | `Changed` | `Modifié` |
| `category.deprecated` | `Deprecated` | `Obsolète` |
| `category.removed` | `Removed` | `Supprimé` |
| `category.fixed` | `Fixed` | `Corrigé` |
| `category.security` | `Security` | `Sécurité` |
| `design.decisions` | `## Decisions` | `## Décisions` |
| `design.decision` | `### Decision:` | `### Décision :` |

Domain names (`security`, `personal-data`…), finding codes and the
`**WHEN**` / `**THEN**` scenario lines are not keywords of the catalog:
codev does not read the scenario lines, and the other two are
identifiers.

#### Scenario: Every shipped language is complete

- **GIVEN** the languages shipped with codev
- **WHEN** the catalog is loaded
- **THEN** each of `en` and `fr` gives at least one text for every key of
  the table

#### Scenario: A key with several texts accepts each of them

- **GIVEN** the key `normative`, whose `fr` texts are `DOIT` and `DOIVENT`
- **WHEN** codev reads a French requirement
- **THEN** either word makes it normative

### Requirement: A keyword is recognized in every catalog language

Wherever codev reads a structural keyword — in a main spec, a delta, a
proposal or a design — it SHALL recognize the text of that keyword's key in
every language of the catalog, whatever the project's `language:`
setting, and give it the same meaning. A document MAY mix languages: each
keyword is recognized on its own. Recognition is unchanged for English.

#### Scenario: A French main spec is read

- **GIVEN** a main spec holding `## Objet`, a sentence, `## Exigences`,
  then `### Exigence : Expiration de session` with a
  `#### Scénario : Inactivité`
- **WHEN** codev reads the spec
- **THEN** it exposes the Purpose text and a requirement named
  `Expiration de session` carrying its scenario named `Inactivité`

#### Scenario: A project configured in English reads a French spec

- **GIVEN** a project whose config has `language: en`
- **AND** a delta written with `## Exigences AJOUTÉES`
- **WHEN** the user runs `codev validate`
- **THEN** the delta is read as an `ADDED` section, with no finding about
  its headings

#### Scenario: A mixed document

- **GIVEN** a delta holding `## ADDED Requirements` then
  `### Exigence : Annulation`
- **WHEN** codev reads the delta
- **THEN** the `ADDED` operation carries the requirement `Annulation`

#### Scenario: A word that is no keyword stays text

- **GIVEN** a delta whose level-2 heading is `## Exigences AJOUTEES PLUS TARD`
- **WHEN** codev reads the delta
- **THEN** the heading is not taken for an `ADDED` section

### Requirement: Keywords of new languages are matched with tolerance; English keeps its exact rules

When codev compares a heading or a labelled line with a keyword text of a
language other than English, it SHALL ignore letter case, SHALL treat an
accented letter as its unaccented form, and SHALL accept the text with or
without a space before its colon — an ordinary space, a no-break space
(U+00A0) or a narrow no-break space (U+202F). The English texts keep the
rules codev applies today, so that no existing English document is read
differently. A section heading matches only as a whole line; a heading
that carries a name (requirement, scenario, decision) matches as the start
of its line. The normative keywords are case-sensitive in every language,
because the same words in lower case are ordinary prose. A line that only
this tolerance makes a keyword of another language is read as that
keyword: `#### Scenario : y`, with a space before the colon, is the French
scenario heading written without its accent.

#### Scenario: Accents and colon spacing

- **GIVEN** a delta holding `## exigences ajoutees` then
  `### Exigence: Annulation` with a `#### Scenario : Dans les temps`
- **WHEN** codev reads the delta
- **THEN** the `ADDED` operation carries `Annulation` and its scenario
  `Dans les temps`

#### Scenario: No-break space before the colon

- **GIVEN** a delta holding `## Exigences AJOUTÉES` then
  `### Exigence` followed by a no-break space (U+00A0) and `: Annulation`
- **WHEN** codev reads the delta
- **THEN** the `ADDED` operation carries the requirement `Annulation`

#### Scenario: English edge cases are read as today

- **GIVEN** a delta whose section heading is `## ADDED   Requirements`,
  holding a requirement whose body has the line `### requirement: x`
- **WHEN** codev reads the delta
- **THEN** the findings and the requirements are those an earlier codev
  reports for the same file

#### Scenario: Lower-case "doit" is not normative

- **GIVEN** an added requirement whose only sentence is
  `Le portail doit afficher la date.`
- **WHEN** the user runs `codev validate`
- **THEN** a `requirement_no_shall` finding names the requirement
