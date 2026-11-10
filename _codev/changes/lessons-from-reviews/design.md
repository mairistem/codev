# Design: Lessons from review findings and corrections

## Context

This is the second half of the lessons loop outlined in the design of
the archived `2026-10-19-capture-corrections`, section "The second
change". That outline is followed here, and checked against the current
code:

- `codev review --change <name>` reads one active change's `review.md`
  and `corrections.md` (`commands::review`, `engine::review::check`,
  `engine::corrections::check`), checks the evidence against the tree,
  and prints a `ReviewV1` document. Nothing reads an archived record:
  `codev validate` never checks archived changes, and `codev changelog`
  reads only their proposals.
- The archive holds 63 changes on 2026-10-26; 21 have a `review.md`
  (format 1, 241 findings), none a `corrections.md` yet, since
  `capture-corrections` was archived before any change used it.
- The known recurrences are in that archive: compatibility with older
  codevs (`review-record`, `expert-triggers-and-stack-filter`,
  `isolated-review`, `capture-corrections`, `role-models`), symbolic
  links leaving the project (`review-record` R-security-3,
  `isolated-review` R-security-2), workflow input reaching the shell
  (`isolated-review`, `role-evaluation-suite`, `measure-consumption`,
  `implementation-evaluation`), long functions and too many parameters
  (several `newcomer` findings, e.g. "`compare::report` is a ~190-line
  function with seven parameters").
- `/codev-configure` is the only skill that writes `context:` and
  `rules:`. Its `allowed-tools` are `Bash(codev:*), Read, Write, Edit,
  Glob, Grep`, which already cover everything the mode needs.
- `codev validate` (without an item, or `--all`) checks changes, specs,
  decisions and the configuration; the configuration appears as an item
  only when it has a finding. An unknown file directly under `_codev/`
  is ignored by today's binary (checked by laying a
  `_codev/lessons.md` on a copy of this repository).
- `codev archive --json` prints `ArchiveReportV1`, read by
  `/codev-archive`, which parses no other JSON.
- The evaluation suite on `main` covers `plan.write`, `plan.update`,
  `code.implement` and the review steps; no case runs `/codev-archive` or
  `/codev-configure`. The planned change `evaluate-archive-sync-configure`
  covers them: it adds the steps `change.sync`, `change.archive` and
  `configure.enrich` (the default mode of `/codev-configure`), a scoring
  shared by the check-scored steps (named checks per run, pass share,
  runs straying, the per-case verdict of `compare`), the `overlay/` of
  the history cases, and their recorded baseline. This change depends on
  it and is applied only once it is archived.
- `forward-compatibility-policy` is archived (on `feat/forward-compat`,
  pull request #58) and its decision 0013 is accepted: a file written for
  a newer codev warns, unless ignoring it would weaken a check.

Decisions in force that constrain this design: 0001 (the core is pure:
the ledger parser, the selection of served findings and the waiting
signal are computed in `codev-core`), 0002 (effects through the
`FileSystem` port), 0003 (the ledger lives under `_codev/`), 0004 (one
skill per workflow: a mode of an existing workflow adds no artifact),
0006 (YAML through `serde_norway`), 0007 (judgement in the
skill text, data and checks in the binary), 0008 and 0010 (the
measure), 0009 (the mode runs in the session; no model is set for it),
0013 (the ledger carries a format marker, its unknown fields warn).

## Goals / Non-Goals

**Goals**: the history of settled findings and corrections is served by
the binary; a mode of `/codev-configure` turns what recurs into one
durable piece of knowledge per lesson, with the author's agreement and a
traceable origin; a declined lesson is not proposed again; the mode is
measured per call; this repository catches up once.

**Non-goals**: a new skill or a new tool for `/codev-archive`; a "whole
history" option or a first-run behaviour (the catch-up passes the size of
the archive as the window, like any user could); writing code for a
`check` lesson (it is handed to a change); reading Claude Code's memory
from the mode (only the catch-up does it, once, with agreement);
computing the downstream metric over time (second change,
`lessons-effect`).

## Decisions

### Decision: The binary serves the history; grouping and classifying stay in the skill

`codev review --archived [--last <n>]` reads the last `n` archived
changes (10 by default) and serves, per change, the findings whose last
status is `fixed` or `accepted-risk` and the corrections, each marked
with the lesson that already considered it, plus the counts the
downstream metric needs. It reuses the parsers of the review record and
of the corrections file; it never checks evidence (the tree has moved
on) and never serves it, nor the history beyond the last entry's note:
the title and the recommendation state the lesson, and the rest only
costs tokens. A `rejected` finding was judged wrong, a `disputed` or
`open` one was never settled, and a `deferred` one is still someone's
work: none is a lesson.

Deciding that two findings are "the same lesson", and where a lesson
belongs, is judgement: it lives in the skill text (decisions 0001,
0007). The binary contributes only what is deterministic: selection,
counts, the `considered` marks and the waiting signal.

`--archived` is a flag of `codev review`, not a new command: the data is
the same record and the same corrections, and one more command would be
one more permission and one more page to document. `--archived` and
`--change` are exclusive; `--last` requires `--archived`.

The human output, which the skill reads as printed (the skills never
filter JSON through another program), gives per change one count line,
then one line per finding (`id [role, severity/confidence] title —
recommendation (status) [considered L-n]`) and per correction. The JSON
is a new document, `ArchivedReviewV1`, in `contract.rs`: a different
shape from `ReviewV1`, which describes one active change with its gate.

**Alternatives considered**: a new command `codev lessons` — rejected:
it would serve the same records under a second name, and the history of
reviews belongs to `codev review`. Serving every finding with a status
filter left to the skill — rejected: the skill would pay for reading
`rejected` and `open` findings on every pass. Serving evidence — rejected
for its cost and because it is no longer checked.

### Decision: A lessons ledger in `_codev/lessons.md`, in the house format

What the mode considered must be remembered, or a declined lesson comes
back at every pass. The ledger is a new file at the planning root,
`_codev/lessons.md`, in the format of `review.md` and `corrections.md`:
a required format marker `<!-- codev-lessons: 1 -->`, level-3 headings,
one `yaml` block per entry. Two kinds of entries:

- `### P-<n>` — a pass: `read`, the list of the archived change folders
  it read. A list, not "the newest folder read": folders are named
  `<archive date>-<name>`, so changes archived the same day sort by name,
  not by the order they were archived, and "newer than" would skip some;
  a list also keeps the changes a pass with a short window did not read
  counted as unread;
- `### L-<n>` — a lesson: `outcome` (`written`, `handed-off`,
  `covered`, `declined`, `set-aside`), `destination`, `where`, `origin`
  (a list of `<archived folder>/<R-…|C-…>`), `reason`, `pass`.

Every lesson shown is recorded, whatever its outcome, so that a single
place answers "was this already considered?", which the binary needs for
the `considered` marks and the waiting signal. The origin is also
written where the lesson lands, for the reader of that file: a YAML
comment above a rule or a context line, the `## Context` of a decision.
The two copies cannot drift in a harmful way: the ledger is the mode's
memory, the comment is a note for humans.

The ledger is checked by `codev validate` without an item or with
`--all`, as the decisions and the configuration are; it appears as an
item only when it has a finding, so a clean project's report does not
change. Unknown fields are warnings and a newer marker makes it
unchecked with a warning, the criterion `capture-corrections` applied:
an older codev reading a newer ledger warns, never fails without
`--strict`, never misreads.

**What an older codev does with it**: a codev that predates the ledger
ignores `_codev/lessons.md`, because it reads no file of `_codev/` it
does not know (checked on today's binary); its `codev validate --strict`
stays green. A codev that knows format 1 and meets format 2 warns
(`lessons_format_unsupported`), marks nothing as considered, and gives
`lessons: null` in its archive report.

**Alternatives considered**: a `lessons:` key in `_codev/config.yaml` —
rejected: an older codev reports an unknown key as a warning, which fails
this project's `--strict` CI, and the configuration would grow with
every pass. Comments in `_codev/config.yaml` only — rejected: a declined
lesson has nowhere to go, and comments cannot be parsed reliably.
Entries in the archived changes — rejected: archived changes are never
modified, and a lesson spans several of them. Recording only declined
lessons — rejected: the binary could not tell which entries a written
lesson already used.

### Decision: An on-demand mode of `/codev-configure`, with a fixed order of destinations

The mode is `/codev-configure lessons [<n>]`: a mode of the existing
workflow, so that configure stays the only writer of `context:` and
`rules:` (decision 0004 keeps one skill per workflow; it does not by
itself forbid a new one). Its procedure, in the skill text:

1. Refuse without `_codev/config.yaml`, as the default mode does; run
   the update check the other skills run (`{{fragment:version-check}}`,
   which `configure.md` does not include today); and when
   `codev review --archived` fails with a usage error, say that the mode
   needs a newer codev, point to `codev self-update`, and stop, never
   reading the archive by other means.
2. Read `codev review --archived --last <n>`, `codev decision list`,
   `_codev/config.yaml` with its comments (inherited blocks included, as
   `codev instructions` serves them), `codev list`, and the ledger.
3. Group by judgement. Thresholds: findings of at least two different
   changes; a single correction is enough (the author already judged
   it), still confirmed. Skip what is `considered`, and what the ledger
   declined or set aside.
4. Classify each lesson to the first destination that fits: `check`
   (anything a lint, a test or a CI job can enforce: it is handed to a
   change, the mode writes no code); `decision` (an architectural choice:
   `codev decision new`, left `proposed`; never an edit of an accepted
   one; a sealed one proved too vague is superseded with
   `codev decision supersede`); `rule:specs`, `rule:design`,
   `rule:tasks` or `context` (a lesson about code is written as a design
   or tasks rule, or classed as a check); `none` (general expertise that
   belongs to codev's built-in roles: nothing in the project, recorded
   `set-aside` so that codev's maintainers can read it). A lesson already
   held — a decision in effect or proposed, a rule, a context line, an
   active change — is `covered`, naming what holds it. A rule never
   restates a decision in effect. Past ten rules on an artifact, propose
   a merge instead of an addition.
5. Show one line per lesson,
   `L-<n> [<destination>] <lesson> — from <origins>` (or
   `[covered by <what>]`), then the diff of each file to write: the
   configuration with the comment of origin, the decision's draft, the
   ledger entries.
6. Ask for explicit agreement; the author may decline a lesson, give a
   reason, or move it. Write only what was agreed, then run
   `codev validate`.

The order puts first what a machine enforces without spending tokens on
every planning (a check), then what binds the architecture with a
lifecycle and a seal (a decision), then what weighs on every planning
and every reviewer brief (a rule), and last nothing at all. The fixed
line format is what makes the mode measurable without a model.

**Alternatives considered**: a new skill `/codev-lessons` — rejected
because two skills would write `rules:`, and a second skill would
duplicate configure's diff and confirmation. Running the
mode inside `/codev-archive` — rejected: it would add a step and a cost
to every archive. Writing a `check` lesson's code from configure —
rejected: configure modifies no code; a check is a change of its own,
planned and reviewed.

### Decision: `/codev-archive` adds one line, driven by an additive field

`codev archive --json` gains an additive object `lessons` (`changes`,
`corrections`, `window`, `waiting`) in `ArchiveReportV1`. `changes`
counts the archived changes no pass has read, whatever they hold;
`window` is the `n` that makes `--last <n>` include all of them;
`waiting` is `true` when ten unread changes have piled up, or when one of
them holds a correction no lesson considered. The skill ends its success
report with one invitation line naming
`/codev-configure lessons <window>` when `waiting` is `true`; no new
step, no new command, no new tool. Passing the window, rather than
relying on the mode's default of ten, makes the pass read every unread
change, even when some changes were archived between the passes'
windows.

Its cost: listing the archive, and parsing the `corrections.md` of the
unread changes only; no review record is read. On a project that never
runs a pass, that is every archived change's corrections file at each
archive, a linear read of small files on a command the user runs by
hand, not a hot path; task 4.1 times it on this repository's archive.

Per decision 0008, the archive skill is measured on the `change.archive`
step that `evaluate-archive-sync-configure` adds, following that
change's rule for a waiting change: the cases it is about are edited in
this change's pull request, the baseline is scored again without a new
recording, and the edited cases must pass in every candidate run. Every
`done` case replays a tree of this repository with dozens of archived
changes and no ledger, so with this branch's binary `waiting` is `true`:
those cases gain `contains: ["/codev-configure lessons"]`. One case is
added, `archive-lessons-not-waiting`: the change of
`archive-merges-and-moves` with an overlay ledger whose pass read every
archived folder of the tree, expecting `done` and
`absent: ["/codev-configure lessons"]` (the scenario "Nothing waiting");
its five baseline runs are recorded with `main`'s binary, which gives
no `lessons` field and so no line either. A refused archive gives no
success report, hence no line. The other cases must not regress against
the step's baseline, case by case. "An older binary" cannot be played,
the suite running the codev under test: it stays checked by hand.

**What an older codev does with it**: an older `/codev-archive` ignores
the field; a newer skill run with an older binary finds no field and
prints nothing. An archive never fails because of the computation: an
unreadable ledger gives `lessons: null`.

**Alternatives considered**: computing "waiting" in the skill from
`codev review --archived` — rejected: one more command in the archive
skill, which the brief excludes. Inviting after every archive — rejected
as noise: most changes have settled findings, which alone are not a
lesson.

### Decision: The mode is measured by a new evaluation step, `configure.lessons`

Decision 0008 asks that a change to a skill be evaluated on the steps it
touches, a step not covered being covered first by its own change. The
existing behaviour this change touches — the default mode of
`/codev-configure` and `/codev-archive` — is covered first by
`evaluate-archive-sync-configure`. The lessons mode is new, so it has no
earlier behaviour to cover: its step comes with it, built on the
infrastructure that change introduces, and its first baseline is
recorded by this change against a bar set now: median recall at least
0.8, median share of runs that pass at least 0.6, no run modifying a
file. The step:

- a case pins a `ref` of this repository (its archive is the material),
  a window `last`, an optional `overlay/` (a ledger, a proposed
  decision, an archived `corrections.md`) loaded and laid as the
  `overlay/` of a sync or archive case, and `expected.lessons`
  (origins, acceptable destinations) and `expected.absent`;
- the runner replays `/codev-configure lessons <last>` headlessly; with
  no one to agree, the mode must stop after its diff, so any write is a
  defect;
- the checks are deterministic, on the fixed lesson lines, and go
  through the shared scoring of check-scored steps, so pass share, runs
  modifying a file and the per-case verdict of `compare` are computed as
  for `configure.enrich`;
- the step adds its own measures, recall and unexpected lessons, and the
  cost per run, which the suite already traces to its sources.

Six cases, from this repository's archive: symbolic links (decision or
design rule), long functions and too many parameters (check: a Clippy
lint), compatibility with older codevs with a proposed decision in the
overlay (covered), workflow input reaching the shell declined in the
overlay's ledger (absent), a finding seen once (absent), and a single
correction in an overlay's archived change (a tasks rule). Five runs
each in the baseline, about 30 runs, at a cost of the order of a
planning case each.

The default mode of `/codev-configure` reads the same `SKILL.md`, which
grows by the mode's text. It is measured on the `configure.enrich` step:
a candidate of five runs per case with this branch's skills must not
regress against that step's baseline (pass share, runs straying, and
the per-case verdict), and its cost per run is compared with the
baseline's. The rendered skill's token size before and after is also
given, with a ceiling of 1,500 tokens added, judged per decision 0010
(text length is not a token lever; the cost must only not rise beyond
that bound).

The downstream metric is the number of reported findings and of
corrections per archived change, read from the `counts` of
`codev review --archived`. It is computed by the second change, once
ten changes are archived after the catch-up.

**Alternatives considered**: a model-graded check of the lessons —
rejected: the suite scores without a model, and the fixed line format
makes it unnecessary. Measuring the mode only on this repository's
catch-up — rejected: one run, no repetitions, no baseline.

### Decision: The catch-up is a task of this change, run once with the shipped mode

Once the mode is implemented, the catch-up runs
`/codev-configure lessons <size of the archive>` on this repository:
the window is the number of archived changes, an ordinary argument, so
no "whole history" option exists. Every lesson is recorded in the ledger
under one pass, `P-1`: written (destination), handed off, covered (by
which decision, rule or active change), declined, or set aside with the
reason. It never recreates what exists — decisions including proposed
ones, `rules:`, `context:`, active changes. Each known recurrence gets an
outcome:

- compatibility with older codevs: `covered` by decision 0013, accepted
  by the archived change `forward-compatibility-policy`; never a second
  decision or a rule on the same subject;
- symbolic links leaving the project: proposed as a decision (paths read
  from project content are resolved and refused outside the root),
  else a design rule;
- workflow input reaching the shell: a `check` (a workflow linter in
  CI), handed off;
- long functions and too many parameters: a `check` (Clippy's
  `too_many_lines` and `too_many_arguments` in the workspace lints),
  handed off.

With the author's explicit agreement, the catch-up then reads the
feedback entries of this project's Claude Code memory and proposes, for
each project instruction (not a personal preference), a destination in
codev, deleting nothing from the memory. Every write of the catch-up is
confirmed by the author.

**Alternatives considered**: a `--all` option — rejected: a one-time
need does not deserve a permanent flag. Running the catch-up before the
mode exists, by hand — rejected: it would not test the mode, and the
ledger entries would not come from it.

### Decision: The work is split in two changes

This change carries everything a pass needs, its measure and the
catch-up. A second change, `lessons-effect`, comes once ten changes are
archived after the catch-up: it computes the downstream metric — the
median reported findings and corrections per change, for the ten
changes before the catch-up and the ten after, overall and per known
recurrence — in the reliability chapter, from
`codev review --archived --json`, and decides whether the thresholds or
the window need tuning. It cannot be planned in full now: its data does
not exist yet.

It comes after a first change, `evaluate-archive-sync-configure`, which
covers with a baseline the existing behaviour this change touches
(`change.archive`, `configure.enrich`), as decision 0008 requires. That
change must be archived before this one is applied; this change's
evaluation tasks build on its scoring, its `overlay/` and its baselines.

**Alternatives considered**: splitting the `configure.lessons` step into
a change of its own, first — rejected: there is no existing mode to
cover, so the step would wait for this one anyway. Leaving
`/codev-archive` and the default mode of `/codev-configure` measured by
catalog tests and token size only — rejected: a departure from decision
0008 (finding R-skeptic-3), which the author declined. Splitting the binary's data from the
mode — rejected: the data has no reader without the mode, and the
catch-up needs both.

## Risks / Trade-offs

- [The mode groups findings that are not the same lesson] → Recurrence
  across two changes, the origins shown on every line, the author's
  agreement, and the recall and unexpected-lessons measures.
- [Rules pile up and weigh on every planning] → The order of
  destinations puts checks and decisions first, the ten-rule budget
  proposes merges, and a lesson already held is `covered`.
- [A declined lesson comes back] → The ledger, the `considered` marks,
  and the `absent` check of the evaluation.
- [The ledger and the origin comments drift] → The ledger is the only one
  the binary reads; a comment removed by hand loses nothing the mode
  needs.
- [The evaluation costs money] → Run only with `--step
  configure.lessons`, recorded locally (decision 0008).
- [The catch-up imports personal data from the memory] → Read only with
  the author's agreement, only project instructions proposed, and each
  write confirmed; corrections' rule applies: no person, no secret.
- [The default mode of configure gets slower or worse] → Measured
  against the `configure.enrich` baseline, and its rendered size with a
  ceiling.
- [`evaluate-archive-sync-configure` is not archived when this change
  is applied] → Tasks 0.1 checks it first and stops; nothing of this
  change is built on a step that does not exist yet.

## Migration Plan

Nothing to migrate: the ledger is new and optional, and the archive
report's field is additive. `codev update` brings the new `configure` and
`archive` skills; it never writes the ledger. A project whose
`codev_version` allows an older binary keeps working: that binary
ignores the ledger, and its `codev review` rejects `--archived` with a
usage error. The update check alone would not explain it (it speaks only
when `codev_version` is unmet or an update is available), so the lessons
mode recognises that usage error itself, says a newer codev is needed,
points to `codev self-update`, and stops. This repository raises nothing: its CI builds the
binary it validates with.

## Open Questions

- The bar for the first baseline (recall 0.8, pass share 0.6) is set
  before any run; the author may tighten it once the runs are known.
