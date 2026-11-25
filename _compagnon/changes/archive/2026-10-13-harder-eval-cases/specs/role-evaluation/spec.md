## RENAMED Requirements

- FROM: `### Requirement: A finding matches an expected finding by role and location`
- TO: `### Requirement: A finding matches an expected finding by role, location and category`

## MODIFIED Requirements

### Requirement: A case declares its fixture, its step, its role and what it expects

The repository SHALL hold an evaluation suite under `evals/cases/`, one
directory per case, named by the case's id. A case holds:

- `case.yaml` — `role` (the role evaluated), `step` (`plan.challenge`,
  `code.review` or `plan.trace`), `change` (the change's name), `stacks`
  (the fixture's languages), `kind` (`seeded` or `control`), a one-line
  `summary`, either `expected` (for `plan.challenge` and `code.review`)
  or `checks` (for `plan.trace`), and optionally `planned` — the files
  and tests the plan introduces, which the fixture therefore does not
  contain yet;
- `base/` — the fixture project as it is on its default branch;
- `change/` — the files the change adds or modifies, its planning
  artifacts under `_codev/changes/<change>/` included.

An expected finding states its `id`, `severity`, `category` and one or
more `locations`, in the review record's location syntax: `path:line` or
`path:start-end` relative to the project for code, `<file>#<heading>` or
`<file>` relative to the change's directory for the plan. It MAY list
`alternatives`: other locations, in the same syntax, where a reviewer
may rightly cite the same defect. The `category` SHALL be one of the
suite's category vocabulary, `evals/categories.yaml`, where each
category has a one-line description and the terms that name it. A
`plan.trace` case states `checks`: `fixed` entries (a file of the
fixture and a text it must contain after the run) and `unchanged` paths
(files the run must leave as they were).

The planning artifacts of a case — every markdown file under
`_codev/changes/<change>/` in the fixture — SHALL cite only what the
fixture contains:

- a file path between backquotes — a path with a `/` whose last part has
  a file extension, a path ending with `/`, or a bare file name with an
  extension — SHALL name a file (or, ending with `/`, a directory) of
  the fixture as the change leaves it, relative to the project or to the
  change's directory; a bare file name matches a file of that name
  anywhere in the fixture; a location suffix (`:12`, `:12-14`) or a
  heading (`#…`) is ignored; inside a backquoted command, each word
  holding a `/` is checked the same way, while a bare name there is an
  argument and is not checked;
- a test cited in `tasks.md` — a backquoted `snake_case` name introduced
  by the word "test" or "tests" (with those listed after it: "tests `a`
  and `b`"), or the filter a backquoted `cargo test`, `npm test --` or
  `pytest -k` command selects — SHALL appear in a test
  file of the fixture: a file outside `_codev/` whose path contains
  `test` or `spec`, or a Rust file holding a `#[test]`;
- unless the case lists it under `planned`. A `code.review` case, whose
  tasks are all done, SHALL list nothing under `planned`; a `planned`
  entry the fixture already contains, or that no artifact cites, is
  refused.

A case SHALL be refused, naming the case and the reason, when: a key is
unknown; the role does not serve the step; a `seeded` case expects
nothing or a `control` expects something; a category is not in the
vocabulary; an expected location or alternative names a file absent
from the fixture, a line beyond its end or a heading it does not have;
a planning artifact cites a file or a test the fixture does not contain
and the case does not plan; a fixture file is not UTF-8 text or is
larger than 64 KiB. The vocabulary SHALL be refused, naming the
category, when a category has no term or a key is unknown.

#### Scenario: The repository's suite is well formed

- **GIVEN** the cases under `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** every case loads, every expected location exists in its
  fixture, and no planning artifact cites a file or a test the fixture
  lacks

#### Scenario: An unknown key

- **GIVEN** a `case.yaml` with the key `expect` instead of `expected`
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names `expect`

#### Scenario: A location beyond the end of the file

- **GIVEN** a seeded case expecting `src/users.ts:120` and a fixture
  whose `src/users.ts` has 40 lines
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names the file and the line

#### Scenario: A binary fixture file

- **GIVEN** a case whose `change/` holds a file that is not UTF-8 text
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names the file

#### Scenario: A plan that cites a file the fixture lacks

- **GIVEN** a case whose `proposal.md` lists `src/cli/progress.rs` under
  Impact, and a fixture without that file
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names `src/cli/progress.rs`
  and `proposal.md`

#### Scenario: A file the plan creates

- **GIVEN** a `plan.challenge` case whose `tasks.md` cites
  `src/export/csv.rs`, absent from the fixture, and lists it under
  `planned`
- **WHEN** the case is loaded
- **THEN** it loads

#### Scenario: A task verified by a test the fixture lacks

- **GIVEN** a `code.review` case whose `tasks.md` reads "verified by
  `cargo test docs_mention_every_flag`", and no test file of the fixture
  holds `docs_mention_every_flag`
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names
  `docs_mention_every_flag`

#### Scenario: A code case that plans

- **GIVEN** a `code.review` case with a `planned` entry
- **WHEN** the case is loaded
- **THEN** it is refused, and the message says a code case plans nothing

#### Scenario: A category outside the vocabulary

- **GIVEN** a seeded case whose expected finding has the category
  `sqli`, absent from `evals/categories.yaml`
- **WHEN** the case is loaded
- **THEN** it is refused, and the message names `sqli`

### Requirement: The suite covers every reviewing role

The suite SHALL hold, for each expert (`security`, `privacy`,
`accessibility`, `performance`, `ops`, `migration`), at least three
seeded cases, two controls, and one seeded case expecting two findings
of different severity; for the `newcomer` and the `skeptic`, at least
two seeded cases and one control; for the `tracer`, at least one seeded
case and one control. Among the cases, the stacks SHALL include Rust,
TypeScript and Python, and `security` and `privacy` SHALL each have a
Python case.

#### Scenario: The repository's suite meets the minimum

- **GIVEN** the cases under `evals/cases/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it passes

#### Scenario: A role without a control

- **GIVEN** a suite where the `ops` role has three seeded cases and one
  control
- **WHEN** its coverage is checked
- **THEN** the check fails and names `ops` and the missing control

#### Scenario: An expert without a case of two severities

- **GIVEN** a suite where every `migration` case expects findings of a
  single severity
- **WHEN** its coverage is checked
- **THEN** the check fails and names `migration`

#### Scenario: Security without Python

- **GIVEN** a suite where no `security` case lists the `python` stack
- **WHEN** its coverage is checked
- **THEN** the check fails and names `security` and `python`

### Requirement: A finding matches an expected finding by role, location and category

A finding of the review record SHALL match an expected finding when it
was written by the expected finding's role (its `### <role-id>` section),
its location matches one of the expected locations or alternatives, and
it names the expected category:

- code: the same path — `\` read as `/`, a leading `./` ignored — and
  line ranges that overlap once the expected range is widened by two
  lines on each side;
- plan: the same file and, when the expected location names a heading,
  the same heading, compared ignoring case and runs of whitespace; an
  expected location without a heading matches any location in that file;
- category: the finding's title or recommendation contains one of the
  category's terms at the start of a word, ignoring case (`alt` names
  "alt text" and "alternative", not "default").

Expected findings and matching findings SHALL be paired one to one so
that the number of pairs is the largest possible, and deterministically:
expected findings are taken in case order, each trying findings in record
order, and an earlier pair gives way only when that lets one more
expected finding be paired. A paired expected finding is a **true positive**; one
left unpaired is a **miss**. The severity of a true positive is compared
with the expected one and counted as **severity agreement**; it does not
decide the match.

#### Scenario: Overlapping lines

- **GIVEN** an expected `security` finding at `src/users.ts:12-14` of
  category `injection`, and a `security` finding at `src/users.ts:15`
  titled "SQL injection through the search term"
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

#### Scenario: Lines too far away

- **GIVEN** an expected `security` finding at `src/users.ts:12-14` and a
  `security` finding of its category at `src/users.ts:30`
- **WHEN** the case is scored
- **THEN** the expected finding is a miss, and the finding is a false
  positive

#### Scenario: The right place, another issue

- **GIVEN** an expected `accessibility` finding `image-without-alt` at
  `src/Toolbar.tsx:12`, of category `text-alternative`, and an
  `accessibility` finding at `src/Toolbar.tsx:13` titled "Clickable div
  unreachable by keyboard"
- **WHEN** the case is scored
- **THEN** `image-without-alt` is a miss, and the finding does not count
  as its true positive

#### Scenario: Two defects close together, cited in reverse order

- **GIVEN** expected findings `div-button` (keyboard, lines 13-15) and
  `image-without-alt` (text alternative, line 12), and a record whose
  first finding names the missing alternative text at line 13 and whose
  second names the keyboard at line 14
- **WHEN** the case is scored
- **THEN** each expected finding is paired with the finding that names
  its category, and both are true positives

#### Scenario: An alternative location

- **GIVEN** an expected `privacy` finding at `src/customers.ts:21` with
  the alternative `src/errors.ts:8`, and a `privacy` finding of its
  category at `src/errors.ts:8`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

#### Scenario: A plan heading written in another case and spacing

- **GIVEN** an expected finding at
  `design.md#Decision: Tokens are stored as given` and a finding of its
  category at `design.md#decision:  tokens are stored as given`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

#### Scenario: A Windows path

- **GIVEN** an expected finding at `src/db/orders.rs:20-24` and a finding
  of its category at `src\db\orders.rs:22`
- **WHEN** the case is scored
- **THEN** the expected finding is a true positive

### Requirement: What a role invents is counted, and only that

A finding that is not paired with an expected finding SHALL be:

- a **duplicate**, not a false positive, when it matches — role,
  location and category — an expected finding of its role already
  paired;
- **cross-role**, not a false positive, when it matches — location and
  category — an expected finding of another role;
- otherwise **unreported** when its confidence is `low` — the human
  never sees it — counted apart, not as a false positive;
- otherwise a **false positive** of its role.

A false positive or an unreported finding whose role and location match
an expected finding but whose category does not SHALL name that expected
finding in the report, so that a reader can tell another issue at the
same place from a wording the vocabulary lacks.

In a `control` case, every finding with a high confidence is a false
positive.

#### Scenario: A clean control

- **GIVEN** a `control` case whose record holds one `privacy` finding of
  high confidence and one of low confidence
- **WHEN** the case is scored
- **THEN** `privacy` has one false positive and one unreported finding

#### Scenario: The same defect raised twice

- **GIVEN** an expected finding and two findings of its role that both
  match it
- **WHEN** the case is scored
- **THEN** there is one true positive, one duplicate, and no false
  positive

#### Scenario: Another issue at the defect's place

- **GIVEN** a seeded `ops` case expecting `unbounded-retry` (category
  `retry`) at `app/payments.py:12-18`, and a high-confidence `ops`
  finding at `app/payments.py:17` titled "Provider URL is hard-coded"
- **WHEN** the case is scored
- **THEN** the finding is a false positive, and the report names
  `unbounded-retry` next to it
