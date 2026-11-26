## ADDED Requirements

### Requirement: The everyday pages explain the technical terms they use

In the chapters of the "Start here" and "Everyday use" parts, glossary
excepted, the first use of a listed technical term SHALL be a link to its
glossary entry; the later uses of that term in the chapter need no link.
The listed terms are, in English, delta, artifact, capability, schema,
workflow, frontmatter and kebab-case, and in French, delta, artefact,
capacité, schéma, déroulé, frontmatter and kebab-case. The phrase "main
specs" ("specs principales") MUST NOT appear in these chapters: they say
"the project's specs" ("les spécifications du projet"). Headings, inline code,
fenced code blocks and "Under the hood" blocks are not concerned: a
configuration key such as `workflows:` stays as it is.

#### Scenario: Every everyday chapter explains its terms

- **GIVEN** the chapters of the "Start here" and "Everyday use" parts of
  both editions
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** in each, the first use of every listed term is a link to the
  glossary
- **AND** none holds "main specs" or "specs principales"

#### Scenario: A term used without a link is caught

- **GIVEN** a chapter whose first prose use of "delta" is plain text, and
  whose second use is a link to the glossary
- **WHEN** the check runs on it
- **THEN** it fails naming "delta"

#### Scenario: A link to the top of the glossary is caught

- **GIVEN** a chapter whose first use of "delta" links to `glossary.md`
  without an anchor, and another whose link names an anchor no glossary
  heading produces
- **WHEN** the check runs on them
- **THEN** each fails naming "delta"

#### Scenario: A term in code or under the hood is not concerned

- **GIVEN** a chapter that uses "schema" only in inline code, in a fenced
  block and in a `<details>` block
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The phrase "main specs" is caught

- **GIVEN** a chapter that says "merges the deltas into the main specs"
- **WHEN** the check runs on it
- **THEN** it fails naming "main specs"

### Requirement: The French edition speaks French

The French chapters of the "Start here" and "Everyday use" parts, and
`README.fr.md`, SHALL use the French word wherever one exists, following
this lexicon: change → modification, skill → compétence, spec →
spécification, workflow → déroulé, review → relecture, commit →
enregistrement, pull request → demande de fusion. The English word MAY
appear only between parentheses, placed after its French equivalent so that
a reader can match the screen ("une compétence (skill)"), and stays
untouched in inline code, fenced code, "Under the hood" blocks, command
names and paths (`/compagnon-apply`, `_compagnon/changes/`). The example
requests the French edition gives to say to Claude follow the same lexicon
(« réalise la modification », not « applique le change »).

#### Scenario: The French everyday pages hold no anglicism of the lexicon

- **GIVEN** the French chapters of the "Start here" and "Everyday use"
  parts and `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds, in prose or headings and outside parentheses, the words
  change, changes, skill, skills, spec, specs, workflow, review, commit or
  pull request

#### Scenario: An anglicism in prose is caught

- **GIVEN** a French chapter saying « Relisez le change avant de
  l'archiver »
- **WHEN** the check runs on it
- **THEN** it fails naming « change »

#### Scenario: The French verb « changer » is not an anglicism

- **GIVEN** a French chapter saying « voici ce qui change »
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The English word between parentheses, in code or in a path is allowed

- **GIVEN** a French chapter saying « une compétence (skill) », and
  naming `/compagnon-apply` and `_compagnon/changes/`
- **WHEN** the check runs on it
- **THEN** it passes

## MODIFIED Requirements

### Requirement: The glossary defines every term of the Start here path

`glossary.md` SHALL hold one `## <Term>` section per term, defining, in
plain words, at least: agent, Claude Code, skill, change, plan, proposal,
spec, requirement, scenario, delta, capability, design, tasks, archive,
decision, role, review, MCP, artifact, schema, workflow, frontmatter,
kebab-case — under the French names the French edition uses, following
the lexicon of the French edition (Modification, Compétence,
Spécification, Déroulé, Relecture…). Each definition MUST hold at most 60 words in English and 75 in
French.

#### Scenario: Every required term is defined in both editions

- **GIVEN** `docs/en/src/glossary.md` and `docs/fr/src/glossary.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** each holds a section for every required term, each under its
  cap

#### Scenario: A missing or overlong entry is caught

- **GIVEN** a glossary without a "Delta" section and with a 90-word
  definition of "Spec"
- **WHEN** the check runs on it
- **THEN** it fails naming "Delta" and "Spec"
