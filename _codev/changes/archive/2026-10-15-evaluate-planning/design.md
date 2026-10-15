# Design: Evaluate the planning step

## Context

The suite (`evals/`) runs review cases: a fixture committed under
`evals/cases/<id>/{base,change}/`, one role reviewing, findings scored
against expectations, consumption recorded, `codev-evals compare` judging
a candidate against the 5-run baseline. Planning is not covered. A study
of this repository (44 headless runs) gives the protocol this change
adopts: replay a request against the state of the repository before an
archived change, take what that change really modified as the truth, and
score without a model. It also measured what such a run costs: a median
of 1.65 USD, about 30 turns and 30 tool calls, 14 of them before the first
artifact is written.

The cases, the requests and the truth come from that study's dataset: 25
archived changes with their merge commit, 20 of them with specs in
English, 17 of which modified at least one primary requirement; a request
written in French as a user would ask; each expected requirement labelled
primary (what the request is about) or collateral (modified in passing);
and 8 cases written on purpose: 5 against accepted decisions (0007 to
supersede, 0001, 0005, 0006, 0003) and 3 on requirements (the `init`
integration, a Spanish keyword catalog, an edited skill). That makes 25
planning cases.

## Goals / Non-Goals

**Goals**: a planning step in the suite whose quality and consumption
`compare` can judge, with the same runner, repetitions, baseline and
verdict; the contribution rule extended to every change of a skill.

**Non-goals**: evaluating `/codev-explore`, the tasks, `plan.trace` or
`plan.challenge` of a planning run (the review cases already cover the
last two); `/codev-apply`'s implementation; a model as judge; an absolute
score — only comparisons are meaningful (see Risks); cutting the output
tokens for their own sake.

## Decisions

### Decision: A planning case pins a commit, and the runner rebuilds its fixture

`case.yaml` holds `ref`; the runner extracts `git archive <ref>` into the
case's directory, makes it a fresh repository, then runs `codev update
--force` with the binary under test and commits. Copying a whole
repository per case into `evals/cases/` would add megabytes per case and
duplicate history git already keeps. The cost is that the suite needs the
full history: loading a case checks that `ref` exists and names it when it
does not (a shallow clone), and both workflows that load cases — the
evaluation workflow and the CI's test jobs, whose `cargo test` loads the
suite — check out with `fetch-depth: 0`. Loading also refuses a `ref` whose tree already holds
the case itself, which would leak the expected answer.

**Alternatives considered**: committing the fixtures (as the review cases
do) — too large and redundant; a separate repository of fixtures — a
second thing to keep in step for no gain.

### Decision: The truth is what the archived change did, primary and collateral apart

`expected.modified` is the primary requirements, `collateral` the ones
modified in passing, `decisions` the decisions to follow or supersede. The
study found misses concentrated on collateral requirements (a check list,
a JSON contract): counting them in the main recall would make it measure
thoroughness on chores rather than understanding of the request, so they
are reported apart. The labels come from the study (an LLM labelled them
without seeing any run); a task reviews them by hand before the baseline.

### Decision: The plan is scored with codev's own parser, in a pure module

`evals` gains a dependency on `codev-core` and scores with
`parser::parse_delta` and `parse_spec`, so French keywords and every
delta form count exactly as `codev sync` reads them. This respects
decision 0002: `codev-core` stays at the bottom of the graph, and
`evals`, a crate that is not shipped, depends on it, never the reverse;
decision 0007 is untouched, the runner already starts Claude Code and is
not part of the binary. Scoring stays pure (`evals/src/plan.rs`): the
shell part (`suite.rs`) reads the specs and decisions at `ref` with
`git show` and hands their names and titles to it, as it hands
observations today.

**Alternatives considered**: matching headings with regular expressions —
a second parser that would drift from codev's.

### Decision: The runner reads Claude Code's event stream for every case

`--output-format stream-json --verbose` prints the same final `result`
event as `json` does, plus every message: the runner keeps the raw stream
(`claude.jsonl`, ignored by git like `claude.json`), reads the result as
before, counts the tool calls, and finds the orientation — the calls
before the first `Write`, `Edit` or `MultiEdit` of a file under
`_codev/changes/`. Both are optional fields of the observation's
`consumption`, so earlier observations stay readable and the format stays
2. The stream holds the whole transcript, and its first event
(`system/init`) the session's settings, whose paths name the operator's
home directory: the runner strips that event once read. The transcript
itself may still name that directory — Claude Code saves a large tool
output there and the agent reads it back —, so the stream as a whole
stays local and git-ignored, and the evaluation workflow keeps its
artifact 14 days. The observations record no path of the machine — the `root` of
each `codev validate` report is dropped. A write through a shell redirection is not seen as a write; the
skills write their artifacts with the file tools, which the study's 44
runs confirm.

### Decision: Every call to the model is accounted for, and its tokens traced to their source

The event stream gives each call's usage (cache writes, cache reads,
uncached input, output) and the messages around it. The runner records
one entry per call. A call's cache writes are what entered the context
since the previous call: the tool results returned in between and the
previous call's output. They are shared among those in proportion to
their length, each under its source (`skill`, `codev <subcommand>`,
`file`, `search`, `shell`, `edit`, `output`; the first call's writes go
to `prompt`, Claude Code's own and the request, outside codev's reach).
Cache writes are what to trace: in this environment they are 67 % of a
run's cost (8 USD per million against 0.20 for a read, fitted on the
runs of three recordings with R² ≈ 1), and a token written once is re-read by every later
call, so the source that writes it pays twice. The report ranks sources
by cost, for the suite and per step, and `compare` shows each source's
change: the table names the next waste to cut, and the verdict proves
the cut.

**Alternatives considered**: counting characters of tool results alone
misses what Claude Code adds around them and ignores the price; tracing
re-reads per source would charge every source with the whole history,
which the cache-write share already orders correctly.

### Decision: The verdict guards quality, rewards improvement, and leaves the output free

`compare` keeps quality blocking (no measure outside the baseline's
range) and the cost under the floor for a change meant to consume less.
It also names every measure where the candidate beats the baseline's best
repetition as "quality improved", so a change can aim higher than
holding. Input per call and output per call are reported for both runs;
the output never decides, because it carries the model's reasoning: a
change that makes the model think less shows in the quality measures,
which refuse it.

### Decision: Every change to the workflow is evaluated, per call, for quality and tokens

> Promoted to ADR **0008** — see `_codev/decisions/0008-every-change-to-the-workflow-is-evaluated-per-call-for-quality-and-tokens.md`.

### Decision: Planning measures sit next to the roles, by step

`SuiteScore` gains a `planning` section — per run the outcome and the five
measures, per repetition the step's measures, then medians and ranges —
computed only from `plan.write` runs, while the role tallies take only
review runs. `compare` adds the planning criteria to the quality verdict
and leaves consumption as it is: the floor already spans every compared
case. A planning run has no threshold of its own: what is "good" is the
baseline's range.

### Decision: Cases are selected by step, and CI runs planning only on demand

`run --step <step>` selects cases by step (`review` for the three review
steps, `plan.write` for planning), so that a change is measured on the
steps it touches and a review-only run does not pay for planning. The
evaluation workflow gains a `step` input, `review` by default; with
`plan.write` it bounds `repeat` to 1, since 25 planning cases at their
1,800-second worst case and four jobs take up to 190 minutes, and the
hosted job stops at 360. Larger planning runs are recorded locally, like
the baseline.

### Decision: The baseline is recorded again, every case, with every call

The review runs of the committed baseline were recorded from Claude
Code's final document, without the event stream: they hold no call, so
the sources of their tokens are unknown. The baseline is therefore
recorded again in full — the 40 review cases and the 25 planning cases,
five runs each, with the Claude Code version the workflow pins — rather
than completed. That costs about 270 USD and several hours, once; every
later comparison then has each call of each step to compare with.

**Alternatives considered**: resuming the baseline with the planning
cases only — cheaper, but the review steps, the ones measured so far,
would stay without per-call accounting, against the goal that every call
be evaluated.

## Risks / Trade-offs

- [The model knows codev's public history and plans from memory] → The
  same contamination holds for baseline and candidate; the suite compares,
  it never claims an absolute planning score, and the reliability chapter
  shows the planning figures with that limit stated next to them.
- [A history rewrite removes a pinned commit] → Loading fails on the case,
  naming the commit; the cases are rebased together with the history.
- [An old state's configuration does not load with today's codev] → The
  fixture build fails as a case error, named; the dataset's 25 states were
  all prepared this way by the study, and a task runs every case once
  before the baseline.
- [Planning runs are slow and costly: about 1.65 USD and several minutes
  each] → Planning cases run under their own default budget and timeout;
  a candidate selects its steps with `--step`, and `compare` judges the
  cases both runs hold.
- [LLM-made labels and requests] → Reviewed by hand in a task; a test
  refuses a request that copies the archived proposal.

## Migration Plan

Observations keep format 2 with new optional fields; existing output
directories stay readable. The committed baseline is recorded again in
full, every observation with its calls.
