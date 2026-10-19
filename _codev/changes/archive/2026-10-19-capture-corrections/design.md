# Design: Capture the author's corrections in the change

## Context

A change folder holds the planning artifacts, and `review.md` when roles
reviewed it. The review record already has a layout that both humans and
codev read: a format marker, level-headed entries, and one YAML block per
entry, parsed with `serde-norway` (decision 0006). It is checked only
while the change is active, and moved by `codev archive`. Validation reads
only the files it knows in a change folder. An unknown file is neither
reported nor read. The skills share text through fragments rendered at
install time (`_review-record.md`, `_update-notice.md`). `codev-evals`
replays planning and review cases headlessly. In an evaluation run, no
author is there to correct the agent.

This change is the first of two (see "The second change" below).

## Goals / Non-Goals

**Goals**: a correction given during `propose`, `apply` or `update` is
written once, in the change, in a format a later codev step can read. The
capture adds no reading, and only a short text to three skills. Quality
and tokens are measured per decision 0008.

**Non-goals**: turning corrections into rules, decisions or checks (the
second change); reading Claude Code's personal memory; capturing
corrections outside these three skills; a category field (the review
record has none, and the second change groups entries by judgement).

## Decisions

### Decision: A dedicated `corrections.md`, not a section of `review.md`

Corrections live in their own file in the change folder. The review
record is the reviewers' record: its steps are `plan.challenge` and
`code.review`, its sections are roles, and it exists only when roles are
active. A correction comes from the author, at any step, in any change. A
new step in `review.md` would also break older codevs. They report an
unknown step as the warning `review_unknown_section`, and this project's
CI runs `codev validate --strict`, which fails on any warning. A new file
is ignored by every codev that predates it, because validation reads only
the files it knows.

**Alternatives considered**: a `## corrections` step in `review.md` —
rejected for the strict-mode failure on older codevs and the mismatch
with the record's role structure. A YAML file (`corrections.yaml`) —
rejected: entries are prose read by people in a pull request, and the
review record already sets the house format of markdown headings with
YAML blocks.

### Decision: The review record's format, with four fields and tolerant unknown fields

Each entry is a `### C-<n> <title>` heading and a YAML block with `step`,
`where`, `did` and `asked`, behind a format marker
`<!-- codev-corrections: 1 -->`. The marker is required: a project may
already keep a file of that name in a change, and codev never existed to
read it. A file without the marker is not a corrections file, and codev
neither checks nor lists it. The parser mirrors the review record's,
in `codev-core`, pure (decision 0001). An unknown field is a warning, not
an error. This follows the criterion of `forward-compatible-config`: a
section is tolerant when ignoring an unknown key only means "this feature
is absent". A newer format marker makes the file unchecked, with a
warning, as for the review record. A `did` or `asked` over 300 characters
is a warning, `corrections_too_long`. It is the one guard the binary can
give that an entry is an instruction, not a transcript.

An older codev that meets a newer field or format never fails with an
error and never misreads the file. It reports a warning, so a run under
`--strict` fails until that codev is updated, as for an unknown key of
`_codev/config.yaml` or of the review record. The project's
`codev_version` and the update notice of the skills already tell such a
contributor to update.

**Alternatives considered**: a free-text list — rejected: the second change
must read entries without guessing their shape. Strict unknown fields —
rejected: an older codev would report an error, failing even without
`--strict`. Exempting these warnings from `--strict` — rejected: a
newer file the running codev does not fully check should not pass a gate
silently, and the review record and the configuration already set this
rule. A `category` field — rejected (see Non-goals). `category:`
also raises `review_unknown_field` in the review record today, a sign the
project chose not to categorize.

### Decision: The capture is skill text in one shared fragment

Deciding that something is a correction, and whether it belongs to the
project, is judgement. Judgement lives in the skill text, and the binary
serves and checks data (decisions 0001 and 0007). One fragment,
`fragments/memory-capture.md`, is rendered into `propose`, `apply` and `update`, so
the three skills say the same thing. No new skill is added (decision
0004). The fragment holds:

- the trigger;
- the four fields;
- three examples that count and five kinds that do not;
- the split between project and personal memory;
- the rule that an entry names no person and quotes no personal data or
  secret.

The agent writes the entry with the tools these skills already have
(`Write`, `Edit`). It reads the file only to find the next free
identifier. Then it quotes the entry it wrote to the author: `where`,
`did` and `asked`. The author sees what goes into git before it is
committed, and can have it changed or removed.

**Alternatives considered**: capturing in the binary, for example through
a `codev correction add` command — rejected: the binary cannot tell a
correction from a request, and a command costs a permission prompt where
a file write does not. A summary of corrections at the end of the
session — rejected: it needs the agent to re-read the conversation, which
costs tokens and loses precision.

### Decision: Project knowledge in codev, personal preferences in the personal memory

An instruction about the project — how its code, specs, tests or
documents are written — belongs to the team and goes to `corrections.md`,
which travels through git and the pull request. A preference about how
the agent works with one author, such as tone, language of the
conversation or personal tooling, stays in Claude Code's personal memory.
The skill says so in both directions, so that one instruction is never
written in both places. The second change will offer to bring the
project instructions already in the personal memory into codev. This one
reads no memory.

**Alternatives considered**: writing to both — rejected: two copies drift
apart. Letting Claude Code's memory be the only store — rejected: it is
personal, not versioned with the project, and invisible to the team and
to the second change.

### Decision: Corrections are not review material

The reviewers of `plan.trace` and `plan.challenge` read the change's
planning files. Since a correction can be written before the challenge,
`corrections.md` is left out of that material, next to `change.yaml` and
`review.md`. It records how the author wants things done, not what the
change plans. Briefing every reviewer with it would cost tokens on every
challenge, and invite findings on the author's instructions.

**Alternatives considered**: keeping it as material, so that reviewers
check the plan follows the corrections — rejected: the plan already
follows them, since the agent applied each correction when it was given.
A reviewer would pay to read them again.

### Decision: Measured against the current planning and reviews

Following decision 0008, the change adds text to three skills read on
every call, so it must show no quality loss and a bounded cost. The
fragment is the same text in the three skills. So its cost per call is
measured exactly, without a model: the token size of each rendered skill
before and after, with a ceiling of 400 tokens per skill. A total cost
taken over three repetitions moves by more than that from noise alone:
the baseline's repetitions of one planning case range from 1.46 to
2.53 USD. Quality is measured with the suite:

- `codev-evals run --step plan.write --repeat 3 --jobs 4` with this
  branch's frozen binary, compared with `codev-evals compare` against
  `evals/baseline` and against the latest main runs (`spec-search`'s
  second arm), quality-only, since the baseline's cost floor already
  sits below the current main;
- `--step review`, compared quality-only, which reads `codev-apply` at
  `code.review`;
- no run writes a `corrections.md`, which the runner now records as a
  file modified outside the step, so a capture the agent invents shows up
  as a defect in the report;
- the total cost and the `skill` source reported per case, as a median,
  against the spread of the baseline's repetitions, never credited as a
  gain or a loss inside that spread.

No step of the suite runs `codev-update`, and `codev-apply` runs only at
`code.review`. Decision 0008 asks that a step the suite does not cover be
covered first, by its own change. Whether this change waits for that, or
ships with those steps measured by token size alone, is left to the
author (see the review record).

The benefit, fewer repeated instructions, cannot be measured on the suite:
no author corrects the agent there. The second change measures it on the
archived history.

**Alternatives considered**: simulating a correction in the evaluation
prompt — rejected for now: a headless run has no second turn, and a
correction written into the request is a request, not a correction.

## The second change: lessons from findings and corrections

Planned separately, once this one is archived. In outline:

- **Serving the history**: `codev review --archived --json` serves the
  findings and corrections of archived changes. It covers findings
  `fixed` or `accepted-risk` only, never `rejected` or still disputed,
  and does not check evidence, since the tree has changed. The binary
  serves the data. Grouping and classifying are judgement, in the skill.
- **A `/codev-configure` mode, on demand**: it reads the last N archived
  changes (proposed default: 10), groups what recurs, and proposes each
  lesson for one destination, in this order: an automatable check (a task
  or a change); a decision (`codev decision new`, never an edit of an
  accepted ADR, and a superseding decision when a sealed one proved too
  vague); a rule in `rules:` (specs, design or tasks only; a lesson about
  code is rephrased as a design or tasks rule, or classed as
  automatable) or a line of `context:`; or nothing, when it is general
  expertise that belongs to codev's built-in roles. A rule never restates
  a decision in effect (`codev decision list`). The mode reuses
  configure's diff and confirmation, and nothing is written without
  explicit agreement. The user can change the destination.
- **Thresholds, proposed**: a review finding becomes a lesson when it
  recurs in at least two changes. One correction is enough, because the
  author already judged it, but it is still confirmed.
- **Origin**: each written rule or context line carries a YAML comment
  naming the findings and corrections it comes from. An ADR names them in
  its context. A declined lesson is recorded with its origin, so the mode
  does not propose it again. Where it is recorded is decided in that
  change.
- **Budget**: past ten rules on one artifact, the mode proposes merging
  rather than adding, since each rule weighs on every planning and every
  reviewer brief.
- **`/codev-archive`**: no new step and no new tool. At most one line of
  invitation in its report when lessons are waiting.
- **One-time catch-up, on this repository only**: a task of that change,
  not a feature. It runs the mode once over all of `_codev/changes/archive/`
  and records, for each lesson, whether it is proposed (with its
  destination), already covered (by which decision or rule), or set
  aside, with the reason. It never recreates what exists (decisions,
  including proposed ones; `rules:`; `context:`; active changes). At
  least one outcome is expected for each known recurrence: compatibility
  with older codevs, symbolic links leaving the project, workflow input
  reaching the shell, and long functions. With the author's explicit
  agreement, it reads the feedback entries of this project's Claude Code
  memory and proposes to bring the project instructions into codev,
  deleting nothing. Each write is confirmed by the author.
- **Measure**: the number of findings and corrections per change over the
  following changes, computed from `codev review --archived --json`. The
  cost of the mode is paid only when it runs.

## Risks / Trade-offs

- [The agent notes too much, such as one-off preferences or requests] →
  The fragment's examples, the one-line notice the author can answer, the
  length warning, and the pull request where the file is read; the second
  change requires recurrence for review findings and confirmation for
  every lesson.
- [The agent notes too little and misses a correction] → The loss is the
  current situation. The second change also reads review findings, which
  catch what recurs.
- [An entry carries personal data or a secret from the conversation into
  git] → The fragment forbids naming people and quoting personal data or
  secrets, the entry states an instruction in one or two lines, and the
  agent quotes the entry to the author before it is committed. An entry
  is kept as long as the archive. One that slipped through is edited or
  deleted in its change, archived or not, since codev never checks
  archived changes again. History is rewritten if it was pushed. The
  reference page says so.
- [Three skills grow on every call] → One short fragment, measured
  exactly: at most 400 tokens per rendered skill.
- [Mixed codev versions in a team] → An older codev ignores the file. A
  newer format is reported, not misread.

## Migration Plan

Nothing to migrate: the file is new and optional, and a file of the same
name without the format marker is left alone. An older codev ignores
`corrections.md`: validation reads only the files it knows in a change
folder, and `codev archive` moves the folder whole. A future format
raises `corrections_format_unsupported` instead of being misread. `codev
update` brings the new skill text. It never rewrites the project's files.
The `ReviewV1` document gains `corrections` and `correctionsIssues`
arrays, additions installed skills ignore. The meaning of `issues`, which
the review skills fix in `review.md`, is unchanged, and so is the exit
code of `codev review`.
