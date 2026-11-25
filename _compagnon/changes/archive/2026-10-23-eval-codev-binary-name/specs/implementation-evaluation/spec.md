## MODIFIED Requirements

### Requirement: An implementation case pins a past state, a change's plan and a hidden acceptance script

An implementation case SHALL be a directory of `evals/cases/` holding a
`case.yaml` with `step: code.implement`, `kind` (`seeded`), `ref` (a
commit of this repository, the state before the change), `merged` (the
commit that brought the change to `main`), `change` (a change name),
`summary`, and `expected`; and a folder `change/` holding the planning
files of `_codev/changes/<change>/`, as archived at `merged`. They MAY
differ from the archive only where a task's verification needs a tool
the run's sandbox does not hold, such as `mdbook`, or the codev the change
builds — `codev` on the search path is the codev under test, whose
validation and doctor judge an older change by today's rules —: that
verification is reworded into one the sandbox can run, such as `cargo run
-p codev-cli -- validate --strict`, and `summary` says so.
`expected` SHALL hold `refuse`
(whether the skill must refuse to implement), `docs` (whether the run
must touch the documentation of both languages) and `acceptance` (the
names of the acceptance assertions). A case that does not refuse SHALL
hold a `tasks.md` in `change/` whose tasks are all unchecked, no
`review.md`, and an `acceptance.sh` next to `case.yaml`, with at least
one assertion named in `acceptance`. A refusing case SHALL hold no
`tasks.md`, no `acceptance.sh` and no assertion, and expect no
documentation.

Loading a case SHALL fail, naming the case and the cause, when `ref` or
`merged` is not a commit of the repository's history; when the tree at
`ref` already holds `_codev/changes/<change>/` or `evals/cases/<case
id>/`; when `change/` is missing; when a non-refusing case's `tasks.md`
has a checked task or none, or its `change/` holds a `review.md`; when
`acceptance.sh` is missing, or present in a refusing case; when two
assertions have the same name; or when an unknown key is found.

#### Scenario: A case replaying a change

- **GIVEN** a `case.yaml` with `step: code.implement`, `ref`, `merged`,
  `acceptance: [accept-moves-the-status]`, an `acceptance.sh`, and a
  `change/` holding a proposal, a delta spec, a design and a `tasks.md`
  whose three tasks are unchecked
- **WHEN** the suite loads its cases
- **THEN** the case loads, as a `code.implement` case

#### Scenario: A plan already implemented

- **GIVEN** a non-refusing case whose `tasks.md` has a checked task
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case and `tasks.md`

#### Scenario: A refusing case with an acceptance script

- **GIVEN** a case with `refuse: true` and an `acceptance.sh`
- **WHEN** the suite loads its cases
- **THEN** it fails, naming the case

#### Scenario: A task that validates with the change's own codev

- **GIVEN** an archived change whose last task runs `codev validate
  --strict`, which today's codev fails on its proposal written before the
  `## Domains` section existed
- **WHEN** the suite's tests run
- **THEN** they fail unless that verification is `cargo run -q -p
  codev-cli -- validate --strict` and the case's `summary` says so
