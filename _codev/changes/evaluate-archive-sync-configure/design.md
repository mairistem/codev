# Design: Evaluate `/codev-sync`, `/codev-archive` and `/codev-configure`

## Context

`codev-evals` replays four kinds of cases. Review cases lay a small
`base/` and a `change/` in a new repository. Planning, update and
implementation cases rebuild this repository at a past commit
(`git archive <ref>`), the update and implementation ones with an active
change laid from `change/`, since `main` squash-merges each change with
its archive and no commit holds an active change. Every case installs
the skills of the codev under test (`codev update --force`), runs Claude
Code headlessly in the sandbox with `CODEV_NO_UPDATE_CHECK=1`, and is
scored without a model. The update step (`evals/src/update.rs`) is the
closest model: a list of named checks per run, a run passing when all
pass, and three measures per repetition (share of runs that pass, share
of checks that pass, runs modifying a file outside the step), judged by
`compare` against the baseline's range.

`CODEV_NO_UPDATE_CHECK=1` hides `update` in `codev status --json`, but
not `versionRequirement`, which codev computes from the project's
`codev_version`: a fixture that sets an unsatisfiable `codev_version`
makes the skills' update check ask its question even in the suite.

The three skills do little reasoning: sync and archive run one codev
command and report it, stopping on a refusal; configure reads a bounded
sample of a project and proposes two blocks of YAML. What they get wrong
is conduct — a merge written by hand, a step not stopped, a field or
comment dropped, a write without agreement — which file comparisons can
see.

## Goals / Non-Goals

**Goals**: cover `change.sync`, `change.archive` and `configure.enrich`
as decision 0008 requires, before the three changes that wait on them
(`version-check-stops-verdict-actions`, the archive's relay of
unchecked tasks, `lessons-from-reviews`); record their baseline; plan
the multi-role review cases as a second change.

**Non-goals**: changing any skill or codev command; judging the prose of
a configure proposal with a model; measuring the `lessons` mode of
`/codev-configure` (its own step, `configure.lessons`, comes with
`lessons-from-reviews`); measuring the review meeting (second change,
below).

## Decisions

### Decision: Three steps, one family of check-scored cases, built on the update step

`change.sync`, `change.archive` and `configure.enrich` are scored as the
update step is: named checks per run, the same three measures, the same
verdict in `compare` (pass share not under the baseline's minimum, runs
straying not above its maximum). The update step's scoring and measures
become a shared module that the four steps call with their own checks;
`report.json` keeps its `update` section unchanged and gains one section
per new step with the same shape, so a committed baseline still reads.
Following decision 0008, the steps are named after what they run, so a
change selects exactly what it touches with `--step`: the
version-check follow-up runs `change.sync` and `change.archive`, the
task-progress relay `change.archive`, `lessons-from-reviews`
`change.archive` and `configure.enrich` next to its `configure.lessons`.

**Alternatives considered**: one step for the three skills — a change to
`/codev-archive` would pay for configure runs, against 0008's
per-step selection. Folding sync and archive into one step — the
version-check follow-up touches both, but task-progress and lessons
touch only the archive. A separate scoring module per step — three
copies of the same measures and verdict.

### Decision: Sync and archive cases replay history, like update cases

A case carries `change/`, an archived change's files, laid on `ref`, the
parent of the commit that brought it to `main`, as update cases do; an
optional `overlay/` lays other files on the tree — a configuration with
`codev_version: ">=99"` for the version cases. A refusal case's
`change/` is the archived one edited to fail as wanted: a requirement
stripped of its normative keywords (a requirement renamed so it matches
none fails the merge with the code `invalid`, not validation), a
`tasks.md` removed, or a
`high` finding of `change/review.md` reopened, chosen among those whose
location exists at `ref` so that validation passes and the review gate
is what refuses. `overlay/` mirrors the project root: each file it
holds replaces, at the same relative path, the one the tree at `ref`
has, or is added; a symbolic link, a path under the case's own change,
or a path leaving the case's directory is refused when the case loads.
Another step that needs files of its own laid on a past state can reuse
this shape.

The candidate changes, confirmed by the reference run when the cases
are written:

| Case | Step | Change | Expected |
|---|---|---|---|
| `archive-merges-and-moves` | archive | `2026-10-23-eval-codev-binary-name` | `done`, reply names `Moved to` |
| `archive-unchecked-tasks` | archive | `2026-10-14-fewer-review-turns` (one box unchecked) | `done` |
| `archive-validation-failed` | archive | `2026-10-12-forward-compatible-config`, a requirement without `SHALL` | `refused`, `code: validation_failed`, reply names `codev validate` |
| `archive-review-blocked` | archive | `2026-10-20-role-models`, a `high` finding reopened | `refused`, `code: review_blocked`, reply names `codev review` |
| `archive-planning-incomplete` | archive | `2026-10-23-eval-codev-binary-name` without `tasks.md` | `refused`, reply names `/codev-propose` |
| `archive-version-later` | archive | `2026-10-20-role-models`, `codev_version: ">=99"` | `done`, reply names `>=99` |
| `sync-merges` | sync | `2026-10-12-forward-compatible-config` | `done`, reply names `ready to be archived` |
| `sync-validation-failed` | sync | as `archive-validation-failed` | `refused`, `code: validation_failed`, reply names `codev validate` |
| `sync-planning-incomplete` | sync | as `archive-planning-incomplete` | `refused`, reply does not hold `Synced` |
| `sync-version-later` | sync | `2026-10-23-eval-codev-binary-name`, `codev_version: ">=99"` | `done`, reply names `>=99` |

`sync-planning-incomplete` checks no suggested skill: `/codev-sync`
suggests "`/codev-propose` or `/codev-continue`", and `/codev-continue`
is not a skill codev installs, so a run following the skill could name
either. The case records the stop, not the suggestion, until that line
of the skill is settled by a change of its own.

These record today's behavior. The waiting changes then change the
cases they are about, in their own pull requests: the version-check
follow-up turns both `*-version-later` cases to `refused`, and the
task-progress relay adds `contains: ["unchecked"]` to
`archive-unchecked-tasks`. Since scoring reads only the observations
(messages, digests, the reference), they score the baseline again
without a model and commit its report: the edited case then fails in
the baseline, so it decides nothing, while the per-case verdict holds
every other case to its baseline share; the case must pass in all its
candidate runs, which the report lists, and their tasks say so. The
per-case verdict exists because a step holds four to six cases: one
case falling from 1.00 to 0.33 moves the step's share by less than the
spread of five runs.

**Alternatives considered**: synthetic `base/` projects for sync and
archive — they would not exercise the real specs' merge, and the
history cases already exist as a pattern. An `overlay` key appending
YAML to the configuration — it cannot replace a `codev_version` the
configuration at `ref` already sets.

### Decision: The reference is codev's own command, run by the runner on a copy

Before the agent runs, the runner copies the fixture outside the case's
directory and runs `codev sync --change <change> --json` or `codev
archive --change <change> --json` on it with the
codev under test; it records the exit code, the report and a digest per
file under `_codev/`. A `done` run must leave `_codev/` as the reference
did, the archive folder's date aside; a `refused` run as the fixture
was. This catches a merge written by hand, a partial one and a missing
move, without the case listing every spec the merge touches, and keeps
working when codev's merge changes. A `done` case whose reference fails
ends in error rather than scoring: the case has gone stale, and the
error says so. Scoring stays pure: it reads the recorded digests.

**Alternatives considered**: listing in `case.yaml` the specs created
and updated — it misses a hand merge that touches the right files, and
goes stale with every change to the merge. Checking only that the agent
called `codev archive` in its tool calls — it passes a run that calls it
and then edits the result.

### Decision: Configure cases are small synthetic projects, with the author's answer set by the case

`/codev-configure` reads a project, not a change, and its scenarios are
about stacks (Jest in `__tests__/`, Go without tests, pytest). Five small
`base/` projects, like the review cases', keep the runs short and the
expected texts sure. The prompt plays the author: ask the skill's
confirmation, then continue with the case's `answer`. With `yes` the
written file shows what the skill preserves; with `no`, that it writes
nothing. A project without `_codev/config.yaml` gets the skills, then
loses `_codev/`, so the refusal is measured as a user meets it.

| Case | Answer | Expected |
|---|---|---|
| `configure-ts-jest` | yes | `written`; `tasks` names `Jest`, `__tests__/`, not `Vitest` |
| `configure-go-no-tests` | yes | `written`; `tasks` names `go test ./...`, `go vet`, `_test.go` |
| `configure-python-preserves` | yes | `written`; `language: fr`, `mcp`, `codev_version`, comments kept; `tasks` names `pytest` |
| `configure-answer-no` | no | `unchanged`; reply names `No changes made` |
| `configure-no-config` | — (yes) | `refused`; reply names `codev init` |

**Alternatives considered**: this repository at a past commit — a large
project, several dollars a run, and expected texts that drift with the
code. Stopping at the confirmation, as the `lessons` mode's evaluation
does — it cannot see whether the write preserves the other fields, the
skill's main guardrail. A model judging the proposal — decision 0008
wants checks a reader can redo, and the update and planning steps use
none.

### Decision: The update check's question is answered **Later** by the prompt

Every new step's prompt says that, if the skill asks whether to update
codev, the author answers **Later**: the answer that leads to the dead
end of `R-migration-2`, and the only one a sandboxed run can take
(`self-update` needs the network). The rest of the prompt stays neutral
— it never says what to write or refuse — so the checks measure the
skill, not the prompt.

**Alternatives considered**: a per-case answer — **Update now** cannot
succeed in the sandbox, so the choice has one value.

### Decision: The cost guidance of decision 0008 holds

Each run gets the review cases' budget (3 USD) and timeout (600 s): a
run calls one or two codev commands, or reads a small project. The
suite stays on demand: only `--step` selects these cases, the CI
workflow is unchanged, and the baseline is recorded locally with the
pinned Claude Code, the new steps' cases alone, as `CONTRIBUTING`
already allows for a new step. Estimated from the update step's
baseline (0.31 USD a run) and the review cases' (0.29 to 0.40): about
0.25 USD a sync or archive run and 0.50 a configure run, so 50 runs at
about 12.50 USD and 25 at about 12.50 USD — about 25 USD for the
baseline, and a third of that for a `--repeat 3` candidate of one step.

**Alternatives considered**: three repetitions in the baseline — the
suite asks five, and the spread decides `compare`'s verdict.

### Decision: The review meeting is a second change, `evaluate-review-meeting`

Multi-role cases touch the review case format, the scoring by role, and
need a baseline in each review mode; together with this change, the
plan would double, and `opposed-findings` is the only change waiting on
it. Planned next, in short:

- a review case may give `roles: [<id>, …]` instead of `role`, each
  expected finding then naming its `role`; the runner leaves those roles
  active and sets the others to `never`, as today for the one role;
- `expected` gains `merged` (expected findings of several roles about
  one defect, which the meeting must merge into one finding whose
  `raised_by` names them) and `opposed` (pairs of expected findings
  asking for fixes that cannot both be applied, which must stay two
  findings, each by its role);
- each role keeps today's measures; the meeting gets its own: the share
  of `merged` groups merged and of `opposed` pairs kept apart, per
  repetition, judged by `compare` as the other steps;
- at least three cases — two roles with one shared defect, two roles
  with opposed fixes (performance asking for a cache, the skeptic
  refusing stale data), and a control — at `plan.challenge` and
  `code.review`, recorded five times in the inline mode and five times
  in the isolated mode, the isolated runs in a baseline of their own,
  since `compare` refuses two review modes.

`opposed-findings` then adds the link check (`conflicts_with` between
the two findings of an `opposed` pair) to the cases it measures.

**Alternatives considered**: one change for both — about twice the
scope and a baseline in two modes before any of the waiting changes can
start. Scoring the link now — `conflicts_with` does not exist on `main`
and its format is still under review.

## Risks / Trade-offs

- [The pinned Claude Code (2.1.289) is no longer installable, so the new
  steps cannot be recorded alone] → Record them with the version the
  workflow then pins and record the rest of the baseline again, as
  `CONTRIBUTING` requires for a change of version.
- [A past change no longer archives with today's codev (a rule added
  since)] → The reference run says so when the case is written; pick
  another change, or fix it in `change/`.
- [An archive at midnight moves the change to another date than the
  reference's] → The date of the archive folder is left out of the
  comparison.
- [A configure check too strict on the YAML shape fails a good
  proposal (a block scalar, a folded rule)] → The keys and the rules are
  read as YAML; the comments and the lines of `context` are counted in
  the text, where a folded block keeps its lines; a case's baseline below a
  pass share of 0.8 is reviewed before it is committed.
- [The runner's reference command writes outside its copy] → It runs in
  a copy under the run's temporary folder, with the environment the
  runner already gives codev; the agent's directory is checked
  unchanged before the agent starts.
- [Five more runs per waiting change] → Each waiting change selects
  only its steps with `--step`, at a few dollars per candidate.

## Migration Plan

None for users: `evals/` is not shipped. `report.json` gains three
sections; an older `codev-evals` reading a newer baseline ignores them.

## Open Questions

- Should the sync and archive cases also check that the skill reports
  each created and updated spec path? The reference's report gives them;
  the skill's step 3 lists them. Left out to keep the cases stable, it
  can be added as a `contains` per case.
- Should `configure.enrich` count the files the agent reads against the
  skill's reading budget (5 docs, 8 sources, 3 tests)? The tool calls are
  recorded; a check would need a rule for what is a source file.
