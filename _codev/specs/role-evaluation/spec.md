# Role Evaluation Specification

## Purpose

Measures what each reviewing role finds, misses and invents, on small
projects with seeded defects and clean controls, so that a change to a
role is judged on scores rather than on a reading of its text.

## Requirements

### Requirement: A case declares its fixture, its step, its role and what it expects

This requirement describes a review case; a planning case, whose step is
`plan.write`, is declared as the `planning-evaluation` capability says,
and the suite loads both kinds from `evals/cases/`.

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

#### Scenario: Review and planning cases side by side

- **GIVEN** `evals/cases/` holding review cases with `base/` and
  `change/`, and planning cases holding only a `case.yaml`
- **WHEN** the suite loads its cases
- **THEN** both kinds load, each checked by the rules of its kind

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

### Requirement: The runner runs each case alone, headlessly, with Claude Code

`codev-evals run` SHALL, for each selected case, in a new directory
outside the repository:

- for a review case, create a git repository whose branch `main` holds
  `base/` with codev initialized (configuration and skills,
  non-interactive), and a branch holding `change/` on top of it, both
  committed; a planning case builds its fixture as the
  `planning-evaluation` capability says;
- for a review case, write the change's `change.yaml` so that every reviewing role serving
  the case's step other than the evaluated one is set to `never` — the
  evaluated role reviews alone;
- run Claude Code non-interactively in that directory, reading the stream
  of events it prints rather than its final document alone, with a
  prompt that names the skill step to run and the evaluation mode: at
  `plan.challenge` and `code.review`, write the findings to the review
  record and leave every finding `open`, without the author's answer and
  without modifying any other file; at `plan.trace`, fix the gaps in the
  plan as the step says;
- run it as a user who works with Claude Code's sandbox would: the
  shell commands Claude Code approves in its sandbox, and `codev`, which
  the skills grant, run without a prompt, in a sandbox that lets them
  write only in the case's directory and the temporary directory and
  reach no host but the Anthropic API; file edits inside the case's
  directory are accepted; any other call — a write elsewhere, a network
  tool, a program neither approves — is denied and counted;
  the runner SHALL check, before any case runs, that the sandbox can
  run on the machine — on Linux, `bubblewrap` and `socat` — and exit with
  code 2 naming what is missing, rather than run without it;
- record what it observed: the review record as `codev review --json`
  reports it, the files the case checks, the files the run modified
  outside what the step allows, what the run consumed as Claude Code
  reports it — the cost, the duration and the API duration, the number
  of turns, the input tokens read without the cache, written to the
  cache and read from the cache, and the output tokens, each summed over
  every model the run used, the models it used, the number of
  permission denials, the number of tool calls, and how many of them
  came before the run first wrote a file under `_codev/changes/` (its
  orientation; absent when it wrote none), and every call to the model —
  or the error that stopped it. For each call, the observation SHALL hold
  the tokens it wrote to the cache, read from it, read without it and
  produced, the tool calls it asked for, and where the tokens it wrote
  came from: each tool result returned since the previous call, and the
  previous call's own output, gets a share of the call's cache writes in
  proportion to its length, under its source — `skill` (a file under
  `.claude/skills/`), `codev <subcommand>` (a shell command that runs
  codev), `file` (another file read), `search` (a search over files),
  `shell` (another shell command), `edit` (the result of a write or an
  edit), `output` (the model's previous output) — and the first call's
  writes go to `prompt` (Claude Code's own prompt and the request). A measure Claude
  Code does not report is recorded as absent, never as zero.

With `--repeat N` (default 1, at least 1), the runner SHALL run each
selected case N times, each repetition in a directory of its own, and
record each repetition's observation separately. A value of `--repeat`
that is not a whole number of at least 1 SHALL exit with code 2 before
any case runs, naming the flag.

The model SHALL be the one named by the environment variable
`CODEV_EVAL_MODEL`, else Claude Code's default. Each case SHALL run under
a budget (`CODEV_EVAL_BUDGET_USD`, default 3 for a review case) and a
timeout (`CODEV_EVAL_TIMEOUT_SECS`, default 600 for a review case); a
planning case's defaults are given by the `planning-evaluation`
capability. Before any case runs, the
runner SHALL check that `claude`, `git` and the `codev` binary can be
executed, and exit with code 2 naming the missing one otherwise; and that
the `codev` binary's file is named `codev` (`codev.exe` on Windows),
exiting with code 2 naming the file otherwise: the agent calls codev by
name, through the search path whose first folder is the binary's, so a
binary under another name would leave the agent running whatever other
`codev` the search path holds. A case
that fails or times out is recorded as an error; the others still run —
except when Claude Code reports that the account's usage limit is reached:
the runner then records that run as an error, starts no other run, lets
the runs already started end, and exits with 1 saying the limit was
reached, so that running it again on the same directory resumes where it
stopped.

#### Scenario: Only the evaluated role reviews

- **GIVEN** a `code.review` case evaluating `security`, at a step where
  `newcomer` and `security` serve
- **WHEN** the runner prepares the change
- **THEN** its `change.yaml` sets `newcomer` to `never` and leaves
  `security` as configured

#### Scenario: The prompt names the step and the evaluation mode

- **GIVEN** a `plan.challenge` case on the change `add-sessions`
- **WHEN** the runner builds its prompt
- **THEN** the prompt names the `codev-propose` skill, its
  `plan.challenge` step, the change `add-sessions`, and says to leave
  every finding `open` and to modify no file but the review record

#### Scenario: The model and the budget come from the environment

- **GIVEN** `CODEV_EVAL_MODEL=claude-sonnet-4-5` and
  `CODEV_EVAL_BUDGET_USD=1.5`
- **WHEN** the runner builds the Claude Code command
- **THEN** it passes that model and that budget; without the variables,
  it passes no model and a budget of 3

#### Scenario: A run that strays outside the step

- **GIVEN** a `code.review` case whose run modified `src/users.ts` and
  wrote `review.md`
- **WHEN** its observation is recorded
- **THEN** it lists `src/users.ts` as modified outside the step, not
  `review.md`, and the report shows it for that case

#### Scenario: Claude Code is not installed

- **GIVEN** a `claude` command that cannot be executed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `claude`

#### Scenario: A compound command is not denied

- **GIVEN** a case whose agent runs `cd src && cat users.ts | head -20`
- **WHEN** the runner runs the case
- **THEN** the command runs, and the observation records 0 permission
  denials

#### Scenario: A write outside the case and a network call are denied

- **GIVEN** a case whose agent runs `echo x > $HOME/probe.txt`, then
  `curl https://example.com`, then writes `$HOME/probe2.txt` with its
  file tool
- **WHEN** the runner runs the case
- **THEN** none of the three takes effect, and the observation records
  3 permission denials

#### Scenario: A machine without the sandbox

- **GIVEN** a Linux machine where `socat` is not installed
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before setting up any case, and the
  message names the sandbox and `socat`

#### Scenario: What a run consumed is recorded

- **GIVEN** a run for which Claude Code reports 9 turns, and per model:
  `claude-opus-5-5` with 14 uncached input tokens, 29 349 written to the
  cache, 304 094 read from it and 2 713 output tokens, and
  `claude-haiku-4-5-20251001` with 1 173 uncached input tokens and 15
  output tokens
- **WHEN** its observation is recorded
- **THEN** it holds 9 turns, 1 187 uncached input tokens, 29 349 written
  to the cache, 304 094 read from it and 2 728 output tokens, and both
  models, next to the cost and the durations

#### Scenario: A run whose output reports no usage

- **GIVEN** a run that times out before Claude Code prints its result
- **WHEN** its observation is recorded
- **THEN** its tokens and turns are absent, not zero, and the case is in
  error

#### Scenario: The account's usage limit is reached

- **GIVEN** a run of 40 cases, one job at a time, whose third run ends
  with Claude Code reporting "You've hit your session limit"
- **WHEN** the runner records it
- **THEN** that run is recorded as an error, no fourth run starts, and the
  runner exits with 1 saying the usage limit was reached
- **AND** running the same command again runs that case and the 37 others

#### Scenario: Each case repeated

- **GIVEN** two selected cases
- **WHEN** the user runs `codev-evals run --repeat 3`
- **THEN** six runs are made, and each case holds three observations

#### Scenario: An invalid repetition count

- **WHEN** the user runs `codev-evals run --repeat 0`
- **THEN** it exits with code 2 before setting up any case, and the
  message names `--repeat`

#### Scenario: Tool calls and orientation are recorded

- **GIVEN** a run whose events hold 27 tool calls, the twelfth of which
  writes `_codev/changes/add-search/proposal.md`
- **WHEN** its observation is recorded
- **THEN** it holds 27 tool calls and an orientation of 11

#### Scenario: Where a call's tokens come from

- **GIVEN** a call that wrote 6,000 tokens to the cache after two tool
  results: `codev roles --at code.review --json`, 9,000 characters long,
  and a read of `src/users.ts`, 3,000 characters long, the previous call
  having produced no text
- **WHEN** its observation is recorded
- **THEN** the call attributes 4,500 tokens to `codev roles` and 1,500 to
  `file`

#### Scenario: A run that writes nothing under the changes

- **GIVEN** a run whose agent writes no file under `_codev/changes/`
- **WHEN** its observation is recorded
- **THEN** its orientation is absent, not zero

#### Scenario: A codev binary under another name

- **GIVEN** `--codev /tmp/frozen/codev-candidate`
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before any case runs, naming
  `codev-candidate` and saying the file must be named `codev`

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

### Requirement: Evidence validity comes from codev's own check

For each finding, the evidence SHALL be counted as `found` when
`codev review` reports no evidence or location issue for it, `moved` when
it reports `review_evidence_moved`, `invalid` when it reports
`review_evidence_not_found` or `review_location_missing`, and `unchecked`
when it reports `review_evidence_unchecked`. A role's evidence validity
is (`found` + `moved`) divided by every finding but the `unchecked` ones;
a role with no checked finding has no validity, which no threshold
fails.

#### Scenario: A quote that is not in the code

- **GIVEN** a finding for which `codev review` reports
  `review_evidence_not_found`
- **WHEN** the case is scored
- **THEN** its evidence is `invalid`, and its role's validity counts it

#### Scenario: A quote a few lines away

- **GIVEN** a finding for which `codev review` reports
  `review_evidence_moved`
- **WHEN** the case is scored
- **THEN** its evidence is `moved` and counts as valid

### Requirement: A case whose role did not review is a failed case, not a clean one

A case SHALL be marked with its reason, and every expected finding
counted as a miss, when the run stopped with an error, when the record is
missing, when the record has an error that keeps codev from reading it,
or when the evaluated role's section is missing or reads `Disabled by
configuration.` Such a case is never counted as a clean control.

#### Scenario: The evaluated role was disabled

- **GIVEN** a seeded `security` case whose record's `security` section
  reads `Disabled by configuration.`
- **WHEN** the case is scored
- **THEN** the case is marked `role-not-run`, and its expected finding
  is a miss

#### Scenario: The run timed out

- **GIVEN** a case whose observation records a timeout error
- **WHEN** the case is scored
- **THEN** the case is marked `error` with the message, and the suite
  fails

### Requirement: The tracer is scored on the plan it leaves

At `plan.trace` the tracer fixes the gaps it finds instead of recording
them. A `fixed` check SHALL be a true positive when the file contains its
text after the run, ignoring case, and a miss otherwise; an `unchanged`
path SHALL be a false positive of the tracer when its content changed,
ignoring line endings.

#### Scenario: The missing task was added

- **GIVEN** a seeded `tracer` case whose check expects `tasks.md` to
  contain `expired link`, and a `tasks.md` that names "the expired link
  scenario" after the run
- **WHEN** the case is scored
- **THEN** the check is a true positive

#### Scenario: A complete plan rewritten

- **GIVEN** a `control` `tracer` case whose `tasks.md` must stay
  unchanged, and a run that edited it
- **WHEN** the case is scored
- **THEN** the tracer has one false positive

### Requirement: The report is per role, in markdown and JSON, and the exit code follows thresholds

`codev-evals run` and `codev-evals score` SHALL write `report.md` and
`report.json` in their output directory, with, per role: the cases, the
expected findings, true positives, misses, false positives, unreported,
duplicates, cross-role findings, recall (true positives over expected),
evidence validity and severity agreement; and per case: its outcome,
each finding's classification, the findings whose status is no longer
`open` and the files modified outside the step. The model, date and
versions of the run are those recorded when the run started, and the
report SHALL say how many repetitions it holds. The order is fixed — roles and cases by
id, repetitions by number — so that the same observations give the same bytes.

Each role's measures SHALL be computed for each repetition over that
repetition's cases, and the report SHALL give, for each measure, its
median over the repetitions and, when there are several, its minimum and
maximum. The suite-level figures — recall over every seeded defect,
evidence validity over every checked finding, false positives in all
and per case — SHALL likewise be computed for each repetition over all
its cases, then given as their median with its minimum and maximum.
Each case's outcome and findings SHALL be given per repetition.

The report SHALL also give what the runs consumed: per case, the median
over its repetitions of the cost, the duration, the API duration, the
turns, each kind of token, the permission denials, the input per call
(the tokens written to and read from the cache and read without it,
divided by the calls) and the output per call; per role and for
the whole suite, the sum of those medians, except the input and the
output per call, which do not add up: there, the cases' medians weighted
by their calls. A measure absent from a
repetition is left out of its median; a measure absent from every
repetition is shown as `—`. It SHALL give, for the whole suite and per
step, where the tokens go: each source with the sum of its per-case
median tokens written to the cache and its share of the cost they make,
ranked from the most to the least costly.

A role SHALL fail when the median of its recall is under `--min-recall`
(default 0.5), the median of its false positives exceeds
`--max-false-positives` (default 2), or the median of its evidence
validity is under `--min-evidence` (default 0.8); each threshold
can also be set by `CODEV_EVAL_MIN_RECALL`,
`CODEV_EVAL_MAX_FALSE_POSITIVES` and `CODEV_EVAL_MIN_EVIDENCE`, the flag
winning. The command SHALL exit with 0 when every role passes and no
repetition of a case is in error — a planning run that created no change
(`no-plan`) counting as in error — 1 otherwise, naming each failing role
and measure and each case in error with its repetition, and 2 on
an invalid threshold, an invalid case or a missing tool. With a single
repetition, the measures and the verdict SHALL be those of that run.

#### Scenario: Every role passes

- **GIVEN** observations where each role's recall, false positives and
  evidence validity meet the thresholds
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 0 and writes both reports

#### Scenario: A role under the recall threshold

- **GIVEN** observations where `ops` finds one of four expected findings
- **WHEN** the user runs `codev-evals score <dir> --min-recall 0.5`
- **THEN** it exits with 1, and the message names `ops`, its recall
  0.25 and the threshold

#### Scenario: An invalid threshold

- **GIVEN** `CODEV_EVAL_MIN_RECALL=lots`
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 2 and names the variable

#### Scenario: A role measured over three repetitions

- **GIVEN** three repetitions where `ops` has a recall of 1.00, 0.50 and
  1.00
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the report gives `ops` a recall of 1.00, from 0.50 to 1.00,
  and `ops` passes a recall threshold of 0.5

#### Scenario: One repetition in error

- **GIVEN** three repetitions of `ops-code-no-timeout`, the second of
  which timed out
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 1, naming the case and its second repetition,
  and that repetition's cost is left out of the case's median

#### Scenario: The suite's recall over repetitions

- **GIVEN** three repetitions finding 36, 38 and 39 of 39 seeded defects
- **WHEN** the report is written
- **THEN** it gives the suite's recall as 0.974, from 0.923 to 1.00

#### Scenario: Consumption per case and for the suite

- **GIVEN** a case whose three repetitions read 300 000, 320 000 and
  900 000 tokens from the cache
- **WHEN** the report is written
- **THEN** the case shows 320 000 tokens read from the cache, and the
  suite's total counts 320 000 for it

#### Scenario: Where the tokens go

- **GIVEN** runs whose calls attribute, per case and as medians, 60,000
  tokens to `skill`, 25,000 to `codev roles` and 15,000 to `file`
- **WHEN** the report is written
- **THEN** its sources table lists `skill`, then `codev roles`, then
  `file`, with their tokens and their share of the cost

#### Scenario: A planning run that planned nothing

- **GIVEN** observations where one planning run created no change
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 1, naming the case and its repetition

### Requirement: Recorded runs are scored again without a model, and resumed

Each repetition's observation SHALL be written as
`<out>/<case>/<n>/observed.json`, `<n>` the repetition's number from 1,
carrying a format marker. An observation written directly as
`<out>/<case>/observed.json`, as earlier runs did, SHALL be read as the
case's only repetition. `codev-evals score <dir>` SHALL score the
observations of a directory again, calling no model, and give the same
report as the run did. `codev-evals run --out <dir>` SHALL skip a
repetition whose observation already exists in `<dir>` and is not in
error, unless `--rerun` is given — a repetition recorded in error is run
again — so that an interrupted run is resumed and a run asked for more
repetitions than recorded runs only the missing ones; it SHALL refuse,
with code 2 and naming both models, to resume a directory recorded with
another model than the one asked, and, naming both versions, one recorded
with another version of Claude Code than the one installed, so that runs
added to a directory are never credited to another version. An observation whose format marker
is above the one the runner knows SHALL be refused with a message naming
the case. An observation of an earlier format SHALL be read, the measures
it does not hold being absent.

#### Scenario: Scoring twice gives the same report

- **GIVEN** a directory of recorded observations
- **WHEN** the user runs `codev-evals score <dir>` twice
- **THEN** both runs write identical `report.json` and `report.md`

#### Scenario: A resumed run

- **GIVEN** an output directory holding the observations of three cases
  out of five
- **WHEN** the user runs `codev-evals run --out <dir>`
- **THEN** only the two other cases are run

#### Scenario: More repetitions than recorded

- **GIVEN** an output directory where each case has its repetitions 1
  and 2
- **WHEN** the user runs `codev-evals run --out <dir> --repeat 5`
- **THEN** only the repetitions 3, 4 and 5 of each case are run

#### Scenario: A repetition recorded in error is run again

- **GIVEN** an output directory where the second repetition of
  `ops-code-no-timeout` timed out and every other repetition succeeded
- **WHEN** the user runs `codev-evals run --out <dir>` with the same
  `--repeat`
- **THEN** only that repetition is run, and its new observation replaces
  the one in error

#### Scenario: An observation in the earlier layout and format

- **GIVEN** a directory holding `<case>/observed.json` with format 1 and
  no token counts
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** the case is scored as one repetition, its quality measures
  as before and its tokens and turns shown as `—`

#### Scenario: Resuming with another model

- **GIVEN** an output directory recorded with Claude Code's default model
- **WHEN** the user runs `codev-evals run --out <dir>` with
  `CODEV_EVAL_MODEL=claude-sonnet-4-5`
- **THEN** it exits with code 2 and names both models

#### Scenario: A newer observation

- **GIVEN** an `observed.json` with format 3
- **WHEN** the user runs `codev-evals score <dir>`
- **THEN** it exits with 2, naming the case and the format

#### Scenario: Resuming with another Claude Code

- **GIVEN** an output directory whose `run.json` records Claude Code
  2.1.289, and Claude Code 2.1.301 installed
- **WHEN** the user runs `codev-evals run --out <dir>`
- **THEN** it exits with code 2 before running any case, naming both
  versions

### Requirement: A committed baseline is the reference for role changes

The repository SHALL hold a baseline under `evals/baseline/`: the
observations of a run of the whole suite against the current roles, with
at least five repetitions of every case and no repetition in error, and
the report scored from them, with the model and the date of the run.
Scoring the baseline's observations again SHALL give its committed
report. Claude Code's raw output of each run SHALL never be committed
with it.

#### Scenario: The baseline agrees with the scoring

- **GIVEN** `evals/baseline/`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** scoring its observations gives its committed `report.json`

#### Scenario: A case missing from the baseline

- **GIVEN** a case added to the suite after the baseline was recorded
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case, so that the baseline is recorded
  again

#### Scenario: A case with too few repetitions

- **GIVEN** a baseline where one case has three repetitions
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case and its number of repetitions

#### Scenario: A baseline repetition in error

- **GIVEN** a baseline where the fourth repetition of one case timed out
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails and names the case and the repetition

#### Scenario: A raw log in the baseline

- **GIVEN** a baseline holding `<case>/2/claude.json`
- **WHEN** the files to commit are listed
- **THEN** that file is ignored by version control

### Requirement: The suite runs in CI only on demand, and only with a key

The repository SHALL hold a workflow `.github/workflows/evals.yml`
started by hand only (`workflow_dispatch`), which runs the suite with the
`ANTHROPIC_API_KEY` secret and keeps its output as a workflow artifact.
When the secret is absent, the workflow SHALL end successfully after
printing that the evaluations were skipped and why.

#### Scenario: Never on a pull request

- **GIVEN** `.github/workflows/evals.yml`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if the workflow has a trigger other than
  `workflow_dispatch`

#### Scenario: No secret

- **GIVEN** the workflow and a repository without `ANTHROPIC_API_KEY`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails unless the workflow tests the secret before running
  the suite and prints a skip message

### Requirement: A role change comes with its scores

`CONTRIBUTING.md` and `CONTRIBUTING.fr.md` SHALL say that a pull request
modifying a file of `assets/roles/` runs the evaluations and pastes the
output of `codev-evals compare --quality-only` against the baseline;
that a pull request meant to reduce what the skills consume pastes the
output of `codev-evals compare` against the baseline — the same quality
of answer, fewer tokens; that any other pull request that changes what a
skill does — a file of `assets/workflows/`, or the JSON a skill reads,
`crates/codev-cli/src/contract.rs` — pastes the output of
`codev-evals compare --quality-only`, which still reports what its runs
consume; and that both run on the cases of the steps the pull request
touches (`--step`); that the candidate
of a comparison runs with at least three repetitions (`--repeat 3`); that
a run confines the agent in Claude Code's sandbox, and what the sandbox
needs on Linux; and that recording the baseline
again is followed by `codev-evals docs`, which regenerates the
reliability chapter. The Roles chapter of each language SHALL say how
the built-in roles are measured, where the baseline is, and that a role
change is judged with `codev-evals compare`.

#### Scenario: Both languages say it

- **GIVEN** the two contributing guides and the two `roles.md` chapters
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails if one of them does not name `evals/baseline` or
  `codev-evals compare`, if a contributing guide does not name
  `assets/roles/`, `assets/workflows/`, `codev-evals -- docs` or
  `--repeat 3`

#### Scenario: A guide that forgets the rule

- **GIVEN** a French contributing guide that does not name
  `assets/roles/`
- **WHEN** the documentation check runs
- **THEN** it fails and names `CONTRIBUTING.fr.md`

#### Scenario: A guide that forgets the skills

- **GIVEN** an English contributing guide that does not name
  `assets/workflows/`
- **WHEN** the documentation check runs
- **THEN** it fails and names `CONTRIBUTING.md` and `assets/workflows/`

### Requirement: A reliability chapter is generated from the committed baseline

`codev-evals docs` SHALL write the chapter `reliability.md` of the English
and the French books from the committed baseline's `report.json` and
`run.json`, the same scores in both languages. Each page SHALL hold:

- what the suite measures and what it does not;
- the run: its date, the model, the Claude Code and codev versions, the
  number of cases (seeded and controls), the number of repetitions of
  each case, and the cost;
- headline figures: recall over every seeded defect, evidence validity
  over every checked finding, the false positives and their number per
  case, and how many roles meet every threshold;
- a chart of false positives per role, the threshold drawn as a labeled
  line, and each role over it marked by an icon and a text label;
- a chart of recall and severity agreement per role, the recall
  threshold drawn, and each role under it marked by an icon and a text
  label;
- a table of every role: cases (seeded/control), defects found over
  expected, misses, false positives, evidence validity, severity
  agreement, and whether it meets the thresholds, in words and with an
  icon;
- the thresholds, and, when a role fails one, a statement naming how
  many roles fail and on which measure;
- when the baseline holds planning cases, the planning figures: the
  median, minimum and maximum of the primary recall, the collateral
  recall, the probable duplicates, the decisions ignored and the invalid
  plans;
- what a run of the suite consumes: the cost, the input and output per
  call, and where the tokens go — each source with its tokens and its
  share of the cost, ranked, as a table and a chart drawn as the other
  charts are;
- the limits of the measure — among them that each score is the median
  of the repetitions, whose number it names — how to run the suite, and
  a link to `evals/baseline` in the repository.

The scores the page shows SHALL be the medians the report gives.

The charts SHALL be drawn without a script, readable in the light and
the dark themes of the website and of `codev docs`, each with a title
and a text description for assistive technologies, and each mark naming
its role and value on hover. A measure that has no value — recall for a
role without a seeded case — SHALL show as `—` and draw no mark.

The command SHALL exit with 0 after writing both pages, and with 2,
naming the file, when `report.json` or `run.json` is missing, is not
valid, or carries a format newer than the command reads, and naming both
files when the run recorded in `report.json` is not the one `run.json`
describes. The committed
pages SHALL be exactly what the command produces from the committed
baseline.

#### Scenario: The committed pages follow the baseline

- **GIVEN** the committed `evals/baseline/` and both `reliability.md`
- **WHEN** the test suite of `codev-evals` runs
- **THEN** it fails, naming the page, if a committed page differs from
  what `codev-evals docs` would write

#### Scenario: A role over the false-positive threshold

- **GIVEN** a report where `ops` has 11 false positives and the
  threshold is 2
- **WHEN** the page is generated
- **THEN** the false-positive chart marks `ops` with an icon and the
  label saying it is over the threshold, and the table says `ops` fails
  on false positives

#### Scenario: A role under the recall threshold

- **GIVEN** a report where `ops` finds one of four seeded defects and
  the recall threshold is 0.50
- **WHEN** the page is generated
- **THEN** its recall mark is a quarter of the scale, it is marked under
  the threshold with an icon and a label, and the headline recall counts
  its misses

#### Scenario: A role without a seeded case

- **GIVEN** a report where `tracer` has no expected finding
- **WHEN** the page is generated
- **THEN** its recall reads `—` in the chart and the table, and no
  recall mark is drawn for it

#### Scenario: Every role within the thresholds

- **GIVEN** a report where every role meets every threshold
- **WHEN** the page is generated
- **THEN** no role is marked, and the page says every role meets the
  thresholds

#### Scenario: A report the command cannot read

- **GIVEN** a baseline directory whose `report.json` carries format 3
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and the format, and
  writes no page

#### Scenario: A report of another run

- **GIVEN** a baseline directory whose `report.json` records the date
  2026-10-13 and whose `run.json` records 2026-11-02
- **WHEN** the user runs `codev-evals docs`
- **THEN** it exits with 2, names `report.json` and `run.json`, and
  writes no page

#### Scenario: The planning figures and the sources on the page

- **GIVEN** a baseline with planning cases whose calls attribute most of
  the cost to `skill`
- **WHEN** the page is generated
- **THEN** it gives the planning figures, and its sources table and chart
  list `skill` first, in both languages
### Requirement: Two runs are compared, with a verdict on quality and consumption

`codev-evals compare <baseline> <candidate>` SHALL read two output
directories, calling no model, and compare them on the cases both hold,
each run's role measures computed on those cases alone. It SHALL print a
markdown report on its standard output with, per role, each quality
measure of both runs (the baseline's median, minimum and maximum, the
candidate's median) and, for the whole of the common cases, the sum of
the per-case medians of the cost, the API duration, the turns, each kind
of token and the permission denials, in both runs and as a relative
change. It SHALL list the cases only one run holds, which are not
compared.

The verdict SHALL fail on quality when, for a role, the candidate's
median recall is under the baseline's minimum recall, its median false
positives are above the baseline's maximum, or its median evidence
validity is under the baseline's minimum; and, for the planning step
`plan.write`, when the candidate's median recall of the primary
requirements is under the baseline's minimum, or its median of probable
duplicates, of decisions ignored or of plans that do not validate is
above the baseline's maximum. A measure without a value in
either run decides nothing. The report SHALL give the planning measures
in a section of their own, next to the roles. A run's total cost is the sum, over the
compared cases, of each case's median cost. It SHALL fail on consumption
when the candidate's total cost is not lower than the baseline's floor —
the sum, over the compared cases, of each case's lowest cost over its
repetitions not in error — so that a candidate must be cheaper than
anything the baseline's own spread produced, and the same code compared
with itself never passes; or when a compared case has no cost in either
run; `--quality-only` SHALL leave
consumption out of the verdict while still reporting it. The duration
SHALL be reported and SHALL never decide. A repetition in error in either
run SHALL fail the verdict, naming the run, the case and the repetition.

The report SHALL also compare the input per call and the output per call
of both runs, and where the tokens go, source by source, with each
relative change. The output never decides: it carries the model's
reasoning, which the quality measures guard. The report SHALL name every
quality measure on which the candidate's median does better than the
best repetition of the baseline — a recall or an evidence validity above
its maximum, false positives, duplicates, decisions ignored or invalid
plans under its minimum — as quality improved.

The command SHALL exit with 0 when the verdict passes, 1 when it fails,
naming each failing role and measure with both values, and each cause on
consumption; and 2, writing no report, when a directory cannot be read,
when the two runs were recorded with different models, naming both, unless
`--across-models` is given, or, when both hold review cases, in
different review modes, naming both, or when they have no case in
common. The report SHALL name both runs' appended configurations and say
when they differ. Two runs are recorded with different
models when the models they asked for differ, or when the models their
observations record as used differ; observations that record none, of
the earlier format, compare by the model asked alone. `--across-models`
SHALL compare such runs anyway, for a change that is itself a change of
model, and the report SHALL then name both runs' models at its top. The report SHALL
name both runs' Claude Code versions and how many runs each made at the
same time, and say so when either differs.

#### Scenario: A cheaper run of the same quality

- **GIVEN** a baseline where `security` has a recall from 0.83 to 1.00,
  a total cost of 2.04 USD and a floor of 1.95 USD, and a candidate on
  the same cases with a median recall of 1.00 and a total cost of
  1.70 USD
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0, and the report shows the cost down by about
  17 %

#### Scenario: A gain within the baseline's spread

- **GIVEN** a baseline with a total cost of 2.04 USD and a floor of
  1.95 USD, and a candidate of the same quality at 1.99 USD
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming 1.99 USD and the baseline's floor,
  1.95 USD

#### Scenario: The same code compared with itself

- **GIVEN** a baseline
- **WHEN** the user runs `codev-evals compare <baseline> <baseline>`
- **THEN** it fails on consumption and passes on quality

#### Scenario: A cheaper run that misses more

- **GIVEN** a baseline where `ops` has a recall from 0.83 to 1.00, and a
  cheaper candidate where its median recall is 0.67
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming `ops`, the recall 0.67 and the
  baseline's minimum 0.83

#### Scenario: A run of the same quality that costs as much

- **GIVEN** a candidate whose role measures stay within the baseline's
  ranges and whose total cost is above the baseline's
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1 and names the cost of both runs; with
  `--quality-only`, it exits with 0 and still reports both costs

#### Scenario: Only the duration grew

- **GIVEN** a candidate cheaper than the baseline, of the same quality,
  whose API duration is twice the baseline's
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0 and reports the duration's increase

#### Scenario: A candidate on a subset of the cases

- **GIVEN** a baseline of 40 cases and a candidate that ran 10 of them
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** both runs are measured on those 10 cases, and the report
  lists the 30 others as not compared

#### Scenario: A baseline in the earlier format

- **GIVEN** a baseline of format 1, which records each case's cost but
  no tokens, and a cheaper candidate of format 2 of the same quality
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 0, the costs compared and the baseline's tokens
  and turns shown as `—`

#### Scenario: A baseline without costs

- **GIVEN** a baseline whose observations hold no cost
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1 and says the consumption cannot be compared;
  with `--quality-only`, the verdict rests on quality alone

#### Scenario: Two models

- **GIVEN** a baseline recorded with Claude Code's default model and a
  candidate recorded with `claude-sonnet-4-5`
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both models, and prints no report

#### Scenario: The same default, another model used

- **GIVEN** a baseline and a candidate both recorded with Claude Code's
  default model, whose observations record `claude-opus-5-5` and
  `claude-sonnet-5-5` as used
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both models, and prints no report

#### Scenario: Comparing across models on purpose

- **GIVEN** a baseline whose observations record `claude-opus-5-5` and a
  candidate whose observations record `claude-opus-5-5` and
  `claude-sonnet-5-5`
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>
  --across-models`
- **THEN** it compares them, and the report names both runs' models at
  its top

#### Scenario: Different appended configurations

- **GIVEN** a baseline and a candidate recorded with different
  `--fixture-config` texts
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report names both texts and says they differ

#### Scenario: Two review modes

- **GIVEN** a baseline recorded in the `inline` review mode and a
  candidate in the `isolated` mode
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2, names both modes, and prints no report

#### Scenario: Another Claude Code version

- **GIVEN** a baseline recorded with Claude Code 2.1.289 and a candidate
  with 2.1.301, on the same model
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report names both versions and says they differ

#### Scenario: No case in common

- **GIVEN** a baseline and a candidate that share no case
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 2 and says no case is in common

#### Scenario: Quality improved

- **GIVEN** a baseline whose `skeptic` has from 3 to 6 false positives,
  and a candidate whose median is 2
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report says the quality of `skeptic` improved on false
  positives, from 3 at best to 2

#### Scenario: Lighter calls

- **GIVEN** a candidate whose input per call falls from 38,000 to 31,000
  tokens and whose `skill` source falls by half
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** the report shows both changes, the input per call at −18 %
  and the `skill` source at −50 %

#### Scenario: A plan that misses more requirements

- **GIVEN** a baseline whose planning cases have a primary recall from
  0.60 to 0.75, and a cheaper candidate whose median is 0.50
- **WHEN** the user runs `codev-evals compare <baseline> <candidate>`
- **THEN** it exits with 1, naming `plan.write`, the recall 0.50 and the
  baseline's minimum 0.60

### Requirement: The runner can play the review cases in the isolated mode

`codev-evals run --review-mode isolated` SHALL write `review: { mode:
isolated }` into each review case's `_codev/config.yaml` before the
skills of the codev under test are rendered, so that the skill runs the
evaluated role as its own sub-agent, as a project in that mode does; the
sub-agent tool is then granted like `codev`. `--fixture-config <file>`
SHALL append the YAML of `<file>` to each review case's
`_codev/config.yaml` at the same moment, for a setting a measure needs.
Planning and update cases ignore both options. `--review-mode` accepts
`inline` (the default) and `isolated`; an unknown mode, a
`--fixture-config` file that is missing or not a YAML mapping, or one
holding `review:` — the mode comes from `--review-mode` alone, so that
`run.json` records the mode the cases ran in —, SHALL exit with code 2
before any case runs. `run.json` SHALL record the review mode and the
appended configuration; a run resumed with other values SHALL exit with
code 2, as for another model; a `run.json` without them, of the earlier
format, reads as `inline` with nothing appended.

After an isolated case's run, the runner SHALL check that the role
actually ran alone: the review record's step says `Mode: isolated.`,
holds no `Fallback:` line, and the session started at least one
sub-agent. A case that fails the check ends in error, naming what was
missing, since the skill falls back to the inline mode by design
(decision 0007).

The run's cost and token totals SHALL be those Claude Code reports for
the whole session, its sub-agents included. The per-call measures SHALL
count the session's own calls; the tokens of the totals that the
session's calls do not account for SHALL be reported as the source
`sub-agents`, so that each token of an isolated run is traced to a source
(decision 0008). A run whose totals fall below the sum of its own calls
ends in error.

#### Scenario: A review case in the isolated mode

- **GIVEN** a review case of the `skeptic` at `plan.challenge`
- **WHEN** the user runs `codev-evals run --case <it> --review-mode
  isolated`
- **THEN** the case's configuration holds `review: { mode: isolated }`,
  and `run.json` records `isolated`

#### Scenario: A case that fell back to the inline mode

- **GIVEN** an isolated case whose review record holds `Mode: isolated.`
  and a `Fallback:` line, or whose session started no sub-agent
- **WHEN** its run is checked
- **THEN** it ends in error, naming what was missing

#### Scenario: The sub-agents' tokens

- **GIVEN** an isolated run whose session totals hold 120,000 input tokens
  and whose own calls hold 70,000
- **WHEN** its consumption is recorded
- **THEN** the source `sub-agents` holds the 50,000 others

#### Scenario: A configuration appended to every case

- **GIVEN** a file holding `roles: { skeptic: { done_when: ["The rollback
  plan is stated"] } }`
- **WHEN** the user runs `codev-evals run --step review --fixture-config
  <file>`
- **THEN** each review case's configuration ends with that text, and
  `run.json` records it

#### Scenario: Options that cannot be applied

- **GIVEN** `--review-mode sometimes`, or a `--fixture-config` file that
  does not exist, or one holding `review:`
- **WHEN** the user runs `codev-evals run`
- **THEN** it exits with code 2 before any case runs, naming the cause

#### Scenario: Resuming in another mode

- **GIVEN** an output directory recorded in the `inline` mode
- **WHEN** the user runs `codev-evals run --out <it> --review-mode
  isolated`
- **THEN** it exits with code 2, naming both modes

#### Scenario: Resuming with another appended configuration

- **GIVEN** an output directory recorded with one `--fixture-config`
- **WHEN** the user resumes it with another file's text
- **THEN** it exits with code 2

#### Scenario: A run recorded before these options

- **GIVEN** a `run.json` without a review mode nor an appended
  configuration
- **WHEN** it is read
- **THEN** it reads as `inline` with nothing appended, and resuming it
  without the options continues
