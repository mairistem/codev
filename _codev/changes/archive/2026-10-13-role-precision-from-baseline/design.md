# Design: Role precision, measured against the baseline

## Context

A reviewing role is a prompt: `assets/roles/<id>.md`, a frontmatter
(criteria in `done_when`) and knowledge in the sections Scope, Signals,
By stack, References, Reporting. The files are embedded in the binary
(`codev-core::roles`, `include_str!`) and reach the agent through
`codev roles --json`, never through the skill files. What every role
shares — how to write a finding, the severity and confidence scales —
is `assets/workflows/_review-record.md`, rendered into the `propose` and
`apply` skills (`codev-agents::workflows::review_record`); it does reach
the skills, so they are regenerated with `codev update --force`.

The evaluation suite (`evals/`) runs each case with only its role active
(`runner::roles_to_disable`) and scores the record (`evals/src/score.rs`):
a finding pairs with an expected finding when role, location and
category agree; an unpaired finding is a duplicate or cross-role when it
names an expected defect, otherwise **unreported when its confidence is
`low`, a false positive when `high`**, whatever its severity. That rule
mirrors codev's: only a finding with confidence `high` is presented to
the human (`ReviewFinding::reported`). It is not a scoring flaw; the
scorer stays unchanged.

The committed baseline (2026-10-13, 40 cases, 15.18 USD) finds every
seeded defect and counts 52 false positives. Re-scored against this
change's fixed cases (two alternatives added, below), 50.

## Goals / Non-Goals

**Goals:**

- Fewer false positives per role, toward the threshold of two, with the
  same recall.
- Remedies a human reviewer would follow on any project: none names a
  case, its files or its code.
- Cases that state what a careful reviewer finds: a control holding a
  defect its role's own criteria name is not a control.

**Non-Goals:**

- Changing the scorer, the vocabulary, the runner or the thresholds.
- New expectations in seeded cases: a real defect the case does not
  state (class d, left) is noted below, not added, so that cases are not
  fitted to this run.
- Several runs per case: one run per case, as before, so a score stays a
  noisy point.

## False positives of the baseline, classified

Classes: **(a)** out of the role's scope — another role owns it, code or
data the change does not touch, what the plan excludes, a missing test,
a matter of taste; **(b)** a speculative concern with no evidence of
impact; **(c)** a severity or confidence inflated — the remark may hold,
not as a confident finding; **(d)** a real defect the case author
missed. A fifth group came out of the reading: **(e)** a consequence of
the seeded defect, reported as a finding of its own next to the one that
paired.

| Case | Finding | Class | Remedy |
|---|---|---|---|
| accessibility-control-custom-listbox | R-accessibility-1 native select would do (medium) | a — alternative design, the widget works | accessibility Scope: a working custom widget is not a finding |
| accessibility-control-custom-listbox | R-accessibility-4 `aria-selected` on the committed value (low) | d — the select-only pattern selects the highlighted option | fixture fixed |
| accessibility-control-custom-listbox | R-accessibility-5 highlight not reset on reopening (low) | d — stale highlight after Escape | fixture fixed |
| accessibility-control-custom-listbox | R-accessibility-6 no type-ahead (low) | c — optional refinement, required keys work | accessibility Scope: optional refinements |
| migration-code-serde-rename-all | R-migration-4 no test of the migration path | a — missing test | shared: missing test |
| migration-control-serde-alias | R-migration-1 file with both keys fails | b — no existing file holds the new key | migration Scope: inputs no existing file holds |
| migration-control-serde-alias | R-migration-2 no automated test | a — missing test | shared: missing test |
| migration-control-serde-alias | R-migration-3 alias read without warning (low) | c — improvement to a working path | migration Scope + Reporting: name what breaks |
| migration-plan-breaking-rename | R-migration-4 API version not bumped | d — the seeded rename, cited at the version line | `openapi.yaml:4` added to `alternatives`: now a duplicate |
| newcomer-code-clever-one-liner | R-newcomer-4 return type hides its keys | a — type correctness, the developer's | newcomer Scope: correctness is not readability |
| newcomer-code-clever-one-liner | R-newcomer-5 no test describes the split | a — missing test | newcomer Scope + shared |
| newcomer-code-deep-nesting | R-newcomer-2 seat loop duplicated (low) | e — the case's summary names it with the nesting | newcomer Reporting: one finding per function |
| newcomer-code-deep-nesting | R-newcomer-4 unexplained filters (low) | e — same function | newcomer Reporting: one finding per function |
| newcomer-code-deep-nesting | R-newcomer-5 no test describes the import | a — missing test | newcomer Scope + shared |
| newcomer-control-plain-code | R-newcomer-1 docstring repeats the constant (low) | a — taste: the docstring is accurate | newcomer Scope |
| ops-code-no-timeout | R-ops-4 provider URL hard-coded (low) | b — matters only if environments differ, unseen | Reporting: name the failure, else confidence `low` |
| ops-code-retry-without-backoff | R-ops-4 delivery blocks the publisher | d — real, unstated; left | none (no expectation added) |
| ops-code-retry-without-backoff | R-ops-5 no switch to turn webhooks off | b — a redeploy undoes the change | ops Scope: way back |
| ops-control-bounded-retries | R-ops-2 errors flattened to `String` (low) | a — error type design, the developer's | ops Scope + By stack reworded |
| ops-control-bounded-retries | R-ops-3 4xx retried like 5xx (low) | b — three spaced retries of a safe read | ops Scope: bounded, spaced, safe retries |
| ops-control-graceful-shutdown | R-ops-1 no health, metric or start log (high) | d — the role's criterion "exposes health and the metrics" | fixture fixed: heartbeat, counters, start log |
| ops-control-graceful-shutdown | R-ops-2 retried forever, never dead-lettered | d — the role's criterion "retries are bounded" | fixture fixed: dead-lettered after 12 attempts |
| ops-control-graceful-shutdown | R-ops-4 warning carries no cause | a — `SendError` is base code the change does not touch | shared: untouched code; ops Scope |
| ops-control-graceful-shutdown | R-ops-5 timings hard-coded (low) | b — tuning constants, not per-environment values | ops Scope + Signals: configuration that differs per environment |
| ops-plan-no-rollback | R-ops-4 no timeout or retry policy in the spec | b — the code decides it; nothing rules it out | ops Scope: plan level |
| ops-plan-no-rollback | R-ops-5 other provider errors unspecified | b — same | ops Scope: plan level |
| performance-code-hidden-n-plus-one | R-performance-3 `regex` crate added (low) | a — back-end build size, not a runtime cost | performance Scope |
| performance-control-preloaded-helper | R-performance-1 total costs a second scan (low) | b — one aggregate query per report | performance Scope: constant factor |
| performance-control-preloaded-helper | R-performance-2 `regex` crate added (low) | a — same as above | performance Scope |
| performance-control-preloaded-helper | R-performance-3 label formatted per invoice (low) | b — constant factor per item | performance Scope |
| performance-control-preloaded-helper | R-performance-4 numbers cloned (low) | b — constant factor per item | performance Scope |
| privacy-code-email-in-logs | R-privacy-5 purpose and lawful basis not stated (low) | c — a question only the controller answers | privacy Reporting: such a question is confidence `low` |
| privacy-code-python-soft-delete | R-privacy-2 tests do not check erasure | d — the seeded defect seen from the tests | `tests/test_account.py:6-11` added to `alternatives`: now a duplicate |
| privacy-control-masked-logs | R-privacy-1 masked address still identifies | b — depends on domains not seen | privacy Scope: re-identification on unseen data is `low` |
| privacy-control-masked-logs | R-privacy-4 field masking untested | a — missing test | shared: missing test |
| privacy-control-masked-logs | R-privacy-5 addresses already logged stay | a — data the change does not add or keep | privacy Scope |
| privacy-plan-birth-date-retention | R-privacy-3 no opt-out or erasure requirement | d — real, unstated; left | none (no expectation added) |
| privacy-plan-birth-date-retention | R-privacy-4 purpose and lawful basis not stated | c — a question only the controller answers | privacy Reporting |
| privacy-plan-birth-date-retention | R-privacy-5 display name sent to the SMS processor (low) | b — the name serves the greeting the plan describes | privacy Scope: detail that serves the stated purpose |
| security-code-python-path-traversal | R-security-3 upload name not allow-listed | b — no attack beyond the agent's own ticket | security Scope: hardening without an attack |
| security-control-docs-only | R-security-1 exposed token valid while redeploying | c — documentation advice graded as a vulnerability | security Scope: documentation advice |
| security-control-docs-only | R-security-2 no routine rotation (low) | b — hardening practice, no attack | security Scope |
| security-plan-tokens-in-clear | R-security-3 a leaked token cannot be revoked | a — revocation is a stated Non-Goal | shared + security Scope: deferred protection |
| skeptic-control-small-flag | R-skeptic-1 the docs test passes without `--quiet` | d — `FLAGS` stayed empty | fixture fixed |
| skeptic-control-small-flag | R-skeptic-2 the scenario passes on a small file | d — progress prints every 1 000 rows | fixture fixed |
| skeptic-plan-scope-creep | R-skeptic-4 the preference has no observable effect | e — consequence of the scope creep | shared + skeptic: one finding per flaw |
| skeptic-plan-scope-creep | R-skeptic-5 the preference is a hidden migration | a — migration lens, the expert's | skeptic: lenses dropped |
| skeptic-plan-scope-creep | R-skeptic-7 CSV date and total formats (low) | b — detail an implementer settles | skeptic Scope |
| skeptic-plan-unsupported-assumption | R-skeptic-3 the spec covers the happy path only | e — consequence of the e-mail assumption | skeptic: one finding per flaw |
| skeptic-plan-unsupported-assumption | R-skeptic-4 merged name and address unspecified | a — personal data, the privacy expert's (ticked) | skeptic: lenses dropped |
| skeptic-plan-unsupported-assumption | R-skeptic-5 the cause of duplicates stays (low) | d — real, the plan's context shows it; left | none |
| skeptic-plan-unsupported-assumption | R-skeptic-6 one task, happy-path tests (low) | e — consequence of the happy-path spec | skeptic: one finding per flaw |

Totals, over the 52 false positives: (a) 16, (b) 15, (c) 5, (d) 11,
(e) 5. Of the eleven (d), eight are fixed in the cases — two re-scored
away at once by an alternative location, six in fixtures that change —
and three are left as real defects the cases do not state.

## Decisions

### Decision: Evidence and exclusions go in the shared review guidance

`_review-record.md` gains three bullets next to the scales: grade with
evidence (a `medium` or `high` finding with confidence `high` quotes the
line and states the scenario that fails; what depends on something
unseen, or a preference, is confidence `low`); what is not a finding
(another role's concern, untouched code or data, what the plan
excludes, a missing test the role's criteria do not ask for, taste); one
finding per defect (consequences in the recommendation; two fixes, two
findings). The severity and confidence definitions of
`codev-core::review` stay as they are: the bullets apply them, they do
not change the scales the checks and the docs share.

Why it generalizes: the three rules are what code review guidelines ask
of any reviewer — show the failing case, stay in your lane, one comment
per problem — and they hold for every role, including those of a
project's own catalog. They follow the existing definition of confidence
`low` ("depends on something the reviewer could not see or verify")
rather than inventing a threshold. Classes (a), (b) and (e) of every
role are reached by them.

**Alternatives considered**: repeating the rules in every role file
(eight copies drifting apart); changing the scale definitions in
`codev-core` (they are a public vocabulary shared with docs and checks,
and they already say the right thing); counting low-severity findings as
unreported in the scorer (it would hide what codev does present to the
human: a confident `low` finding is noise in the review too).

### Decision: Each expert names what it does not report

Each expert's Scope gains a "Do not report" list, and its Reporting a
sentence on what a finding must name. The items are kinds of remarks,
stated so that they hold on any project; none cites a case, a file or a
fixture's code:

- **ops** — no feature flag when a redeploy undoes the change (its own
  criterion lists "a deploy of the previous version" as a way back);
  tuning constants are not configuration, values that differ between
  environments are; error type design and untouched code are not
  operations; bounded, spaced retries of a safe operation are fine
  whatever they retry; at `plan.challenge`, the handling each call's
  code will provide is not a plan finding unless the design rules it
  out. Reporting: name the dependency that fails and what the operator
  sees. These are the questions an SRE asks in review: what pages me, and
  how do I get back.
- **performance** — constant-factor savings are not findings; a back-end
  dependency's build size is not a runtime cost; an index on a schema
  you cannot see is confidence `low`. Per-item work stays a finding when
  it is expensive compared with the item (a query, a remote call,
  compiling a pattern). Reporting: a cost not stated as a function of
  growth is `low`. This is the profiling rule the role already cites:
  measure what grows.
- **accessibility** — a working custom widget is not a finding for not
  being native; optional refinements of a pattern are not findings when
  its required interaction works; styles the diff does not show are
  confidence `low`. Reporting: the key pressed or the announcement
  heard, and what happens. WCAG is met or not by what the user can do,
  not by the element chosen.
- **migration** — improving a compatibility path that works, inputs no
  existing file can hold, and version numbers in a project that does
  not state semantic versioning are not findings. Reporting: the
  existing file, row, call or consumer, and the error it meets.
- **privacy** — data the change does not add, copy, expose or keep;
  re-identification depending on data not seen (confidence `low`); a
  detail sent to a processor that serves the stated purpose. Reporting:
  the data and who receives it; purpose and lawful basis are questions
  for the controller, confidence `low`, as the role already said.
- **security** — hardening without a nameable attack; a protection the
  plan defers in Non-Goals unless shipping without it is unsafe;
  documentation advice graded as a vulnerability. Reporting: who sends
  what through which entry point; an attack that needs what you could
  not see is confidence `low`. This sharpens the role's existing "never
  report a vulnerability you cannot point to".

The ops signal "Configuration baked in" now names values that differ
between environments, and its Rust pitfall names an error logged without
its cause instead of any `.to_string()`: the old wording invited the
very remarks the classification puts in (a) and (b).

**Alternatives considered**: a stricter severity scale per role (the
false positives are as often `low` as `medium`: severity alone does not
separate them); listing forbidden phrases or patterns (fitted to this
run's wording, it would not generalize).

### Decision: The skeptic challenges the plan, the experts their domains

The skeptic's lens criterion ("security & privacy, compatibility &
migration, operability, performance, accessibility & UX") is replaced by
a **Domains** criterion: every domain the plan touches is ticked, so its
expert reviews it; the domain itself is the expert's. The signal
"Hidden migration" becomes "Domain left to nobody". New Scope: the
plan's logic and evidence; irreversible steps stay the skeptic's (a
property of the plan, whatever its domain); details an implementer
settles, consequences of a flaw already reported, and alternatives the
plan's own context does not support are not findings. New Reporting:
the claim that does not hold and the plan's text that shows it; high
when the plan would build the wrong thing or destroy something with no
way back.

Why: since `built-in-experts`, each domain has an expert activated by
`## Domains` and by the diff's triggers, and the skeptic already records
a finding for every suggested domain left unticked (`skills` spec,
"act on suggested domains"). The lenses made the skeptic a second,
shallower reviewer of each domain: R-skeptic-4 and R-skeptic-5 are
domain remarks that the privacy and migration experts own. Routing
through Domains keeps a domain covered when no expert is active, which
the lenses did only by duplicating the expert. This changes a
requirement of the `skills` spec (the delta in this change).

**Alternatives considered**: keeping the lenses only when no expert is
active (the skeptic cannot tell an expert disabled by configuration from
one not activated, and a disabled expert was disabled on purpose);
keeping them as low-confidence remarks (still a shallow duplicate).

### Decision: The newcomer reads one function at a time

The newcomer gains a Scope (readability of the code the change touches;
correctness and missing tests are not its own; accurate comments and
taste are not findings) and a Reporting section: one finding per
function or block, grouping its nesting, duplicated branches, unexplained
filters and names, located at it; medium when a reader is likely to
misread what the code does. A human reviewer writes one comment on a
tangled function, not four; the grouped finding also locates where the
developer will rewrite.

**Alternatives considered**: capping the number of newcomer findings
(arbitrary, and it would drop a second function's problem).

### Decision: Fix only what the cases misstate

The fixtures change where a finding is right about the fixture:

| Case | Fix | Why it is a fixture weakness |
|---|---|---|
| `skeptic-control-small-flag` | task 1.1 adds `--quiet` to `FLAGS`; the design says `src/main.rs` (new, in the base) parses it and passes it to `import::run(path, quiet)`; the quiet scenario imports 2 500 rows | the check `docs_mention_every_flag` iterated an empty list, and a small file printed no progress line anyway: both verifications passed without the flag; the flag had no path from the command line |
| `privacy-code-python-soft-delete` | `alternatives` gains `tests/test_account.py:6-11` | the test that checks deletion without checking erasure is where a reviewer rightly shows the same defect |
| `migration-plan-breaking-rename` | `alternatives` gains `openapi.yaml:4` | the published version line is where consumers learn of the same breaking rename, like the release note already listed |
| `ops-control-graceful-shutdown` | a base `Health` port (heartbeat for `/healthz`, outcome counters for `/metrics`); the worker logs its start with its settings, beats, counts, and dead-letters a job still failing after 12 attempts | the control broke two of the ops role's own criteria — "exposes health and the metrics an operator needs", "retries are bounded" — so it was not clean |
| `accessibility-control-custom-listbox` | `aria-selected` follows the highlighted option; opening the list highlights the current value | the component claims the select-only combobox pattern, which selects the highlighted option, and a reopened list announced a stale option |

No seeded defect moves or weakens, and no expectation is added. Each fix
is decided from the role's criteria or the pattern the code claims, as
written before this change, not from the role's new wording. CONTRIBUTING
asks not to tune a role and its cases in the same pull request; these
fixes only remove defects the cases did not mean to hold, and the
committed baseline is re-scored against them (column "re-scored"
below). Re-scoring reaches only the two alternatives: the three changed
controls are seen anew by the model, so their six (d) findings are also
subtracted from the re-scored count (column "adjusted": accessibility
−2, ops −2, skeptic −2). The new run is compared with "adjusted", so
that the roles' effect is not credited with the fixtures'.

**Alternatives considered**: adding the three real defects left in
seeded cases as expectations (fitting cases to one run's output); leaving
the controls as they were (a control that breaks its role's criteria
measures nothing).

### Decision: Measured by one full run, recorded as the new baseline

After the tests pass, `evals/baseline/` is removed and the whole suite
runs once on Claude Code's default model (`cargo run -p codev-evals --
run --out evals/baseline`), every role included, then `cargo run -p
codev-evals -- docs` regenerates the reliability chapter. One extra full
run is allowed if recall drops and needs a fix; never more than two.

## Baseline before and after

Readings on Claude Code's default model, one run per case: the
**committed** baseline (2026-10-13); the same observations
**re-scored** against this change's cases; **adjusted**, the re-scored
false positives minus the (d) findings of the three changed controls;
the **new** baseline recorded with this change's roles.

The new baseline: 40 cases, 14.23 USD, Claude Code 2.1.289, default
model, recorded 2026-10-13, one full run.

| Role | Recall committed → new | False positives committed → re-scored → adjusted → new | Severity agreement committed → new | Evidence validity committed → new | Unreported committed → new |
|---|---|---|---|---|---|
| accessibility | 1.00 → 1.00 | 4 → 4 → 2 → 0 | 5/5 → 5/5 | 1.00 → 1.00 | 2 → 2 |
| migration | 1.00 → 1.00 | 5 → 4 → 4 → 0 | 4/5 → 5/5 | 1.00 → 1.00 | 1 → 0 |
| newcomer | 1.00 → 1.00 | 6 → 6 → 6 → 0 | 0/2 → 2/2 | 1.00 → 1.00 | 1 → 0 |
| ops | 1.00 → 1.00 | 11 → 11 → 9 → 1 | 3/6 → 3/6 | 1.00 → 1.00 | 5 → 1 |
| performance | 1.00 → 1.00 | 5 → 5 → 5 → 0 | 1/4 → 2/4 | 1.00 → 1.00 | 3 → 1 |
| privacy | 1.00 → 1.00 | 8 → 7 → 7 → 1 | 4/7 → 4/7 | 1.00 → 1.00 | 8 → 8 |
| security | 1.00 → 1.00 | 4 → 4 → 4 → 0 | 6/6 → 6/6 | 1.00 → 1.00 | 3 → 2 |
| skeptic | 1.00 → 1.00 | 9 → 9 → 7 → 5 | 2/3 → 2/3 | 1.00 → 1.00 | 2 → 0 |
| tracer | 1.00 → 1.00 | 0 → 0 → 0 → 0 | 1/1 → 1/1 | — → — | 0 → 0 |

In all: 52 → 50 → 44 → 7 false positives, recall 1.00 for every role,
so no extra run was needed. Eight roles of nine now meet the threshold.
What the readings say:

- **The shared rules do most of the work.** Missing tests, untouched
  code and taste are gone from every role; the newcomer writes one
  finding per function (two findings in three cases, both true
  positives, both at the expected severity); controls end with no
  confident finding for accessibility, migration, performance, security
  and newcomer.
- **Fewer findings, not hidden ones.** Unreported findings fall too
  (25 → 14): the roles mostly stop raising the remarks rather than
  recording them `low`. Privacy keeps 8, mostly questions for the
  controller, which its Reporting now asks to record `low`.
- **The seven left**, read finding by finding:
  `ops-code-no-timeout` R-ops-4 (the provider URL hard-coded, now
  medium: a value that differs between environments, which the ops rule
  still lets through — a real, minor remark); `privacy-control-masked-logs`
  R-privacy-1 (addresses with non-ASCII characters escape the mask: a
  real limit of the control's pattern, class d); and the skeptic's five.
  Two of them are the new **Domains** criterion working as designed:
  `skeptic-plan-scope-creep` R-skeptic-3 (a new endpoint returning
  customers' invoices, every domain unticked) and
  `skeptic-plan-unsupported-assumption` R-skeptic-4 (a nightly job
  rewriting production data, operations and migration unticked) — true
  of those fixtures, whose boxes were left unticked because the cases
  predate the criterion. The three others are R-skeptic-3 (happy path
  only, a consequence of the e-mail assumption, still split out),
  R-skeptic-5 (the cause of duplicates left in place: the (d) left
  above) and R-skeptic-6 (no task schedules the job: a real, minor
  gap). The skeptic stays above the threshold; tuning it again on this
  run's wording, or ticking those fixtures' Domains after seeing it,
  would fit the measure to one run, so both are left to a follow-up
  with its own baseline.
- **Severity agreement rises** for migration, newcomer and performance,
  and falls nowhere.

## Every case, planned

| Case | Behavior |
|---|---|
| No git repository, no diff yet | Unchanged: the roles' text changes, not how they are run. At `plan.challenge` the plan-level rule of ops applies. |
| A docs-only change | The security rule on documentation advice grades it `low` unless following it exposes data. |
| Generated, vendored, binary or very large files | Untouched code is not a finding (shared rule), as before for most roles, now for all. |
| Monorepos with several stacks | Unchanged; the By stack sections keep their stacks. |
| Windows paths and CRLF | Role files are parsed as before (CRLF test kept); no path rule changes. |
| A role disabled by configuration | The shared rule says another role's concern is not a finding even when that role is not active; the skeptic's Domains check keeps a touched domain from going unreviewed silently. |
| Older binaries reading newer files | No file format changes; installed skills keep the old guidance until `codev update`. |
| Inherited sources, offline, idempotence | Unaffected. |

## Risks / Trade-offs

- [Recall drops: a seeded defect is now judged speculative or another
  role's] → the remedies keep the seeded kinds explicit (expensive
  per-item work, irreversible plan steps, a new component nobody sees
  fail); matching ignores confidence, so a defect recorded `low` still
  counts as found; the full run checks every role, and a drop is
  investigated before finishing.
- [Grouping hides a second defect] → the rule keeps two defects needing
  two fixes as two findings, even at the same place (a seeded case holds
  two defects at one heading).
- [The skeptic stops seeing a domain no expert reviews] → its Domains
  criterion and the existing unticked-suggestion finding route the
  domain to its expert.
- [One run is noisy] → the report says so; the comparison is per role on
  the same model, and the classification above is read finding by
  finding, not inferred from counts.
- [Real defects become `low` and hidden] → a `low` finding stays in the
  record and is answered; `codev review` lists it; only its presentation
  changes.

## Migration Plan

Projects get the new roles with the next binary, and the new review
guidance in their skills after `codev update`. Rollback: revert the
commit; the baseline and the chapter revert with it.
