## MODIFIED Requirements

### Requirement: A planning case pins a past state of the repository, a request and what the real change did

A planning case SHALL be a directory of `evals/cases/` holding only a
`case.yaml` with `step: plan.write`, `kind` (`seeded`), `ref` (a commit
of this repository: the state the request is planned against), `request`
(the user's words, in any language), `summary`, and `expected`:
`modified` and `collateral`, each a list of
`<capability>::<requirement name>`, and `decisions`, a list of entries
with `id` and `expect` (`follow` or `supersede`); and, optionally,
`keyword_language`, the language whose structural keywords the plan must
be written in (`en` or a language of the keyword catalog). A case MAY
also hold `config`, a YAML mapping of configuration keys — such as
`language: fr` — appended to the fixture's `_codev/config.yaml` before the
skills are installed; it may not set `review:`. `modified` holds the
requirements the request is about; `collateral` the ones the real change
also modified in passing. A case SHALL expect at least one requirement
or one decision.

Loading a case SHALL fail, naming the case and the cause, when `ref` is
not a commit of the repository's history; when a requirement of
`modified` or `collateral` is not in `_codev/specs/<capability>/spec.md`
at `ref`; when a decision id is not one of `_codev/decisions/` at `ref`;
when the tree at `ref` holds `evals/cases/<case id>/`, which would hand
the agent the answer; when `config` is not a YAML mapping or sets
`review:`; when `keyword_language` is not `en` nor a language of the
keyword catalog; or when an unknown key or an empty request is found. `codev-evals list` SHALL show planning cases with their step, and
`codev-evals run` and `score` SHALL exit with code 2 on an invalid case,
as for the other cases.

#### Scenario: A case replaying an archived change

- **GIVEN** a `case.yaml` with `ref` the commit before the change
  `decision-accept-command` was merged, a request in French, and
  `modified: ["decisions::Creating a decision through the CLI"]`
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `plan.write` case

#### Scenario: A requirement that did not exist yet

- **GIVEN** a case whose `modified` names a requirement absent from the
  specs at its `ref`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the requirement

#### Scenario: A commit outside the history

- **GIVEN** a case whose `ref` is not a commit of the repository, as in a
  shallow clone
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and the `ref`, and saying the full
  history is needed

#### Scenario: A state that holds the answer

- **GIVEN** a case whose `ref` is a commit where `evals/cases/<its id>/`
  already exists
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

#### Scenario: A case planned in French

- **GIVEN** a planning case with `config: { language: fr }` and
  `keyword_language: fr`
- **WHEN** the suite loads its cases and the runner builds its fixture
- **THEN** the case loads, and the fixture's `_codev/config.yaml` ends
  with `language: fr` before the skills are installed

### Requirement: The suite covers planning with past changes and with decisions

The repository's suite SHALL hold at least 17 planning cases that replay
archived changes, each expecting at least one requirement under
`modified`, and at least 4 planning cases that each expect a decision,
among them one to supersede; and at least one planning case whose
configuration sets a catalog language other than English, expecting the
keywords in it, and one whose configuration sets a language the catalog
does not ship, expecting English keywords. A request SHALL be written as
a user would ask, not copied from the archived proposal.

#### Scenario: The repository's planning suite

- **GIVEN** `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if there are fewer than 17 planning cases with a
  `modified` requirement, or fewer than 4 with a decision, or none with
  a decision to supersede, or none planned in a catalog language other
  than English, or none planned in a language without catalog

#### Scenario: A request copied from the proposal

- **GIVEN** a planning case whose `request` repeats a sentence of more
  than twelve words of the archived change's proposal
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails, naming the case

### Requirement: A plan is scored on the requirements it modifies, its added duplicates, its decisions and its validity

Scoring a planning run SHALL read the deltas of the change it created
with codev's own parser, whatever the language of their keywords. An
expected requirement SHALL count as modified when a delta of its
capability holds it under `MODIFIED` or `REMOVED`, or as the `FROM` of a
`RENAMED`, its name compared ignoring case and runs of whitespace. Each
run SHALL be given:

- the primary recall: the share of `modified` the plan modifies;
- the collateral recall: the share of `collateral` it modifies, reported
  apart and never counted as a miss of the primary recall;
- the probable duplicates: the requirements the plan adds under `ADDED`
  in a capability where an expected requirement is left unmodified, or
  whose name is the name of a requirement the capability already had at
  `ref`;
- the decisions ignored: each expected decision the proposal and the
  design do not cite, by its id or its title; a decision to `supersede`
  also counts as ignored when no paragraph citing it says the plan
  supersedes or deviates from it;
- whether the plan validates: `codev validate` reports no error on it;
- for a case with `keyword_language`, whether its keywords are in that
  language: every structural keyword line of its proposal, delta specs
  and design that the keyword catalog recognizes is written in that
  language.

A run that created no change SHALL be a failed run (`no-plan`), with
every expected requirement missed; a run that created several changes is
scored on all of them together. The planning measures SHALL be computed
per repetition over the planning runs, then given as their median with
their minimum and maximum, as the role measures are — the share of the
runs with `keyword_language` whose keywords are in that language among
them; they have no
threshold of their own and are judged by `codev-evals compare` against
the baseline.

#### Scenario: A requirement modified, another added beside it

- **GIVEN** a case expecting `decisions::Creating a decision through the
  CLI` and a plan whose delta of `decisions` modifies that requirement
  and adds `Accepting a decision through the CLI`
- **WHEN** the run is scored
- **THEN** its primary recall is 1.00 and it has no probable duplicate

#### Scenario: An addition where a modification was due

- **GIVEN** the same case and a plan whose delta of `decisions` only
  adds `Creating a draft decision`
- **WHEN** the run is scored
- **THEN** its primary recall is 0.00 and it has one probable duplicate

#### Scenario: Keywords in French

- **GIVEN** a plan whose delta writes `## Exigences MODIFIÉES` and
  `### Exigence : Creating a decision through the CLI`
- **WHEN** the run is scored
- **THEN** the requirement counts as modified

#### Scenario: A decision to supersede, only cited

- **GIVEN** a case expecting decision `0007` to be superseded, and a
  design that cites `0007` without saying it supersedes or deviates from
  it
- **WHEN** the run is scored
- **THEN** one decision is ignored

#### Scenario: A run that plans nothing

- **GIVEN** a planning run that ended without creating a change
- **WHEN** the run is scored
- **THEN** it is a failed run, `no-plan`, and the suite exits with 1
  naming the case and the repetition

#### Scenario: A French plan with an English heading

- **GIVEN** a case with `keyword_language: fr`, and a run whose delta
  spec holds `## Exigences MODIFIÉES` and `### Requirement: Login`
- **WHEN** the run is scored
- **THEN** its keywords are not in its language, naming the English line

### Requirement: The report gives the planning measures by step

`report.md` and `report.json` SHALL give, for the step `plan.write`, the
cases and the median, minimum and maximum over the repetitions of the
primary recall, the collateral recall, the probable duplicates, the
decisions ignored, the plans that do not validate and, when a case sets
`keyword_language`, the share of plans whose keywords are in it, next to
the roles;
and per planning run, its outcome, the expected requirements found and
missed, the duplicates, the decisions ignored, and its consumption,
tool calls and orientation included. A suite without planning case SHALL
give no planning section. `codev-evals compare` SHALL fail on the
planning step, besides its other rules, when the candidate's median
share of plans whose keywords are in their language is under the
baseline's minimum.

#### Scenario: A suite with both kinds of cases

- **GIVEN** observations of review cases and of planning cases
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report has the per-role section and a `plan.write`
  section, and the roles' figures are those of the review cases alone

#### Scenario: Only review cases

- **GIVEN** observations of review cases only
- **WHEN** the report is written
- **THEN** it has no `plan.write` section

#### Scenario: Keywords that slip back to English

- **GIVEN** a baseline whose plans keep their keywords in their language
  in every run, and a candidate where one of two such cases writes
  English headings in every run
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the verdict fails, naming the keyword share
