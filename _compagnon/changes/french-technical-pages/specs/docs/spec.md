## MODIFIED Requirements

### Requirement: The French edition speaks French

Every French chapter listed in `docs/fr/src/SUMMARY.md`, in all its parts,
and `README.fr.md`, SHALL use the French word wherever one exists,
following this lexicon: change → modification, skill → compétence, spec →
spécification, workflow → déroulé, review → relecture, commit →
enregistrement, pull request and merge request → demande de fusion,
design → conception, template → modèle, and, checked by hand because
« issue » is also French, issue → ticket. The English word
MAY appear only between parentheses, placed after its French equivalent so
that a reader can match the screen ("une compétence (skill)"), and stays
untouched in inline code, fenced code, "Under the hood" blocks, command
names, flags, configuration keys, JSON fields, error codes and paths
(`/compagnon-apply`, `_compagnon/changes/`), and in headings that name an
identifier — a file name, a configuration key, a `compagnon` command.
« artefact » and « delta », defined in the French glossary, are French
terms of the edition; Compagnon's review is « relecture », not « revue ». The
example requests the French edition gives to say to Claude follow the same
lexicon (« réalise la modification », not « applique le change »).

#### Scenario: The French book holds no anglicism of the lexicon

- **GIVEN** every French chapter listed in `docs/fr/src/SUMMARY.md` and
  `README.fr.md`
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** none holds, in prose or headings and outside parentheses, the words
  change, changes, skill, skills, spec, specs, workflow, workflows, review,
  reviews, commit, commits, pull request, merge request, design, template
  or templates

#### Scenario: A merge request in prose is caught

- **GIVEN** a French chapter saying « Ouvrez une merge request. »
- **WHEN** the check runs on it
- **THEN** it fails naming « merge request »

#### Scenario: An anglicism in prose is caught

- **GIVEN** a French chapter saying « Relisez le change avant de
  l'archiver »
- **WHEN** the check runs on it
- **THEN** it fails naming « change »

#### Scenario: A change counted after a quantifier or in a heading is caught

- **GIVEN** a French chapter saying « aucun change n'est actif » and
  holding the heading `## Changes`
- **WHEN** the check runs on it
- **THEN** it fails naming « change » and « changes »

#### Scenario: A heading naming an identifier is not prose

- **GIVEN** a French chapter with the headings `## review`, `## design.md`
  and `### compagnon review`
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: An anglicism of a technical page is caught

- **GIVEN** a French chapter saying « le design et le template du schéma »
- **WHEN** the check runs on it
- **THEN** it fails naming « design » and « template »

#### Scenario: The French verb « changer » is not an anglicism

- **GIVEN** a French chapter saying « voici ce qui change »
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: The English word between parentheses, in code or in a path is allowed

- **GIVEN** a French chapter saying « une compétence (skill) », and
  naming `/compagnon-apply` and `_compagnon/changes/`
- **WHEN** the check runs on it
- **THEN** it passes

#### Scenario: A renamed heading keeps its links

- **GIVEN** a French heading renamed from « Écrire un template » to
  « Écrire un modèle », and a chapter linking to its old anchor
- **WHEN** the test suite of `compagnon-cli` runs
- **THEN** the check that every link of the offline manual lands on a
  heading fails until the link names the new anchor
