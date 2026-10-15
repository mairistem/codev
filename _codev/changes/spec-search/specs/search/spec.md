## Purpose

Let a person or an agent find, in a few hundred tokens, the requirements and
decisions a request concerns, and read one requirement without its whole
spec, so that planning modifies what exists instead of adding beside it.

## ADDED Requirements

### Requirement: `codev search` ranks the requirements and decisions that match a query

`codev search <query>` SHALL rank every requirement of the project's main
specs — its name, its description and its scenarios — and every decision of
the decision index, the inherited ones included, by how well they match the
words of the query, and print the best matches, the most relevant first.
Matching SHALL ignore case, accents and common words without meaning, and
SHALL count a rare word of the query for more than a frequent one. Words
SHALL match through their stem in the language the specs are written in —
the language their structural keywords are written in, English when they
mix several —, so that `decisions` finds `decision` and, in a French
project, `décisions acceptées` finds `décision acceptée`.

Each result SHALL give its kind (`requirement` or `decision`), its
identifier — `<capability>::<requirement name>` for a requirement, the
decision's identifier, title and status for a decision, with its origin
when it is inherited and, for a decision no longer in effect, the decision
that supersedes it — and an excerpt of at most 160 characters taken from its
description or its body. The command SHALL print 10 results unless
`--limit <n>` asks for another number from 1 to 50, so that its output does
not grow with the project. Results of equal relevance SHALL be ordered by
their identifier, so that the same query on the same files always prints
the same results. A decision that is not in effect — proposed, rejected,
deprecated, or superseded — SHALL be printed with its status and never
presented as in effect.

A query that matches nothing SHALL print that nothing matched, name
`codev list --specs`, and exit with code 0. An empty query, or one made
only of common words, SHALL fail with `search_query_empty` and exit with
code 1. A `--limit` outside 1 to 50 SHALL fail with `search_limit_invalid`
and exit with code 1, the value checked by codev so that `--json` still
prints its document. The command SHALL read the project's files only: it
SHALL write no file and reach no network.

#### Scenario: A query finds the requirement it describes

- **GIVEN** a project whose spec `decisions` holds a requirement
  `Creating a decision through the CLI`
- **WHEN** the user runs `codev search "create a new decision from the command line"`
- **THEN** the results include `decisions::Creating a decision through the CLI`,
  with its kind and an excerpt of its description

#### Scenario: A word matches through its stem

- **GIVEN** a project whose specs are written in English and hold a
  requirement whose text says `decision` but never `decisions`
- **WHEN** the user runs `codev search decisions`
- **THEN** that requirement is among the results

#### Scenario: A French project

- **GIVEN** a project whose specs are written with French keywords and hold a
  requirement about `une décision acceptée`
- **WHEN** the user runs `codev search "decisions acceptees"`
- **THEN** that requirement is among the results

#### Scenario: An inherited decision

- **GIVEN** a project that inherits from a source `path:/shared` holding a
  decision `0100` titled `Errors are typed`
- **WHEN** the user runs `codev search "typed errors"`
- **THEN** the decision is among the results with its identifier, its
  title and its origin `path:/shared`

#### Scenario: A superseded decision is not presented as in effect

- **GIVEN** a decision `0003` superseded by an accepted decision `0009`,
  both about the planning folder
- **WHEN** the user runs `codev search "planning folder"`
- **THEN** `0003` is printed with the status `superseded` and the
  decision `0009` that supersedes it
- **AND** `0009` is printed as in effect

#### Scenario: The output stays bounded

- **GIVEN** a project of 300 requirements, 120 of which contain the word
  `config`
- **WHEN** the user runs `codev search config`
- **THEN** exactly 10 results are printed
- **AND** with `--limit 3`, exactly 3

#### Scenario: Nothing matches

- **WHEN** the user runs `codev search "xylophone quantum"`
- **THEN** it says nothing matched, names `codev list --specs`, and exits
  with code 0

#### Scenario: An empty query

- **WHEN** the user runs `codev search "the of"`
- **THEN** it fails with `search_query_empty` and exits with code 1

#### Scenario: A limit out of range

- **WHEN** the user runs `codev search config --limit 0`
- **THEN** it fails with `search_limit_invalid` and exits with code 1

### Requirement: Archived changes are searched apart, on request

`codev search <query> --archive` SHALL also rank the archived changes —
each as one document made of its proposal and its design — and print the
best of them in a section of their own, after the requirements and
decisions, each with the archived change's directory name and an excerpt.
The archived changes SHALL never be ranked together with the requirements
and decisions: the requirements and decisions printed, and their order,
SHALL be the same with and without `--archive`. Without `--archive`, the
archive SHALL not be read.

#### Scenario: The archive in its own section

- **GIVEN** a project with an archived change `2026-09-01-add-seals`
  whose proposal is about decision seals
- **WHEN** the user runs `codev search "decision seal" --archive`
- **THEN** the output has a section of archived changes listing
  `2026-09-01-add-seals`
- **AND** the requirements and decisions listed before it are the same,
  in the same order, as those of `codev search "decision seal"`

#### Scenario: A long archived design does not push the requirements out

- **GIVEN** an archived design of several thousand words that repeats the
  word `review`
- **WHEN** the user runs `codev search review --archive`
- **THEN** the requirements about review are listed as they are without
  `--archive`

### Requirement: `codev show` prints one requirement, or a capability's requirement names

`codev show <capability>::<requirement name>` SHALL print the requirement as
its main spec holds it — its heading, its description and its scenarios,
verbatim — and nothing else of the spec. The requirement name SHALL be
compared ignoring case and runs of whitespace. `codev show <capability>`
SHALL print the capability's purpose and the names of its requirements, in
the order of the spec, one per line, in the `<capability>::<name>` form
`codev show` accepts.

The capability SHALL be one of the project's main specs, as
`codev list --specs` names them; any other text — a name that does not
exist, a path that leads outside `_codev/specs/` such as `../other` or an
absolute path — SHALL fail with `unknown_capability`, naming
`codev list --specs`, read no file outside `_codev/specs/`, and exit with
code 1. A requirement the
capability does not hold SHALL fail with `unknown_requirement`, listing the
capability's requirement names, and exit with code 1. The command SHALL
write no file.

#### Scenario: One requirement

- **GIVEN** a spec `decisions` holding, among others, the requirement
  `Creating a decision through the CLI` with two scenarios
- **WHEN** the user runs `codev show "decisions::creating a decision  through the CLI"`
- **THEN** the output is that requirement's heading, description and two
  scenarios, as the spec holds them
- **AND** no other requirement of the spec is printed

#### Scenario: A capability's index

- **GIVEN** a spec `decisions` with 20 requirements
- **WHEN** the user runs `codev show decisions`
- **THEN** the output is the capability's purpose and its 20 requirement
  names, each as `decisions::<name>`, in the spec's order

#### Scenario: A requirement that does not exist

- **WHEN** the user runs `codev show "decisions::Deleting a decision"`
- **THEN** it fails with `unknown_requirement`, lists the requirement names
  of `decisions`, and exits with code 1

#### Scenario: A capability that does not exist

- **WHEN** the user runs `codev show billing`
- **THEN** it fails with `unknown_capability`, names `codev list --specs`,
  and exits with code 1

#### Scenario: A path outside the specs

- **GIVEN** a file `spec.md` two directories above `_codev/specs/`
- **WHEN** the user runs `codev show ../../`, or `codev show` with an
  absolute path
- **THEN** it fails with `unknown_capability` and prints nothing of that
  file

### Requirement: `codev search` and `codev show` answer in JSON

With `--json`, each command SHALL write exactly one JSON document, under the
rules of the codev contract: a root `status` array for errors, paths with
`/` separators. `codev search --json` SHALL carry `query`, `language` (the
language the words were stemmed in), `results` — each with `kind`, `id`,
`capability` and `requirement` for a requirement, `decision`, `title`,
`origin`, `status`, `inEffect` and `supersededBy` for a decision, and
`excerpt` — and, with `--archive`, `archived`,
each with `change` and `excerpt`. `codev show --json` SHALL carry
`capability`, and either `requirement` — its `name` and its `text`, verbatim
— or `requirements`, the capability's requirement names, with `purpose`.
A failure SHALL carry its code in `status` and `null` for the missing
object.

#### Scenario: Results in JSON

- **WHEN** the user runs `codev search "decision seal" --json`
- **THEN** stdout is one JSON document with `query`, `language`, `results`
  and `status: []`
- **AND** each result has `kind`, `id` and `excerpt`

#### Scenario: A failure in JSON

- **WHEN** the user runs `codev show billing --json`
- **THEN** stdout is one JSON document with `capability: null` and
  `status: [{ code: "unknown_capability", … }]`
