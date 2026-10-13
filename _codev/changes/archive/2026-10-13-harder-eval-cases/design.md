# Design: Harder evaluation cases, matched by category

## Context

The suite (`evals/`, crate `codev-evals`, archived change
`2026-10-13-role-evaluation-suite`) loads each case in `case::load` — a
pure function over `case.yaml`, the fixture's bytes and the roles' steps
— and refuses a malformed case; `suite::load_cases` does the reading.
`score::case` pairs expected findings with the findings of the review
record greedily — expected findings in case order, each taking the first
free finding of its role whose location matches — then classifies the
rest. `category` is already a field of every expected finding, but free
text that nothing reads. The observation keeps the whole `ReviewV1`
document, so each finding's `title` and `recommendation` are available
to the scoring without a new run. The review record has no category
field, and this change does not add one: the record is codev's public
format (decision 0002 keeps the JSON contract the boundary), and the
reviewer must not be told what category is expected.

The first baseline (26 cases, 9.62 USD, Claude Code 2.1.289, default
model) scored:

| Role | Expected | TP | Misses | FP | Unreported | Recall | Evidence | Severity |
|---|---|---|---|---|---|---|---|---|
| accessibility | 3 | 3 | 0 | 3 | 1 | 1.00 | 1.00 | 2/3 |
| migration | 3 | 3 | 0 | 1 | 1 | 1.00 | 1.00 | 3/3 |
| newcomer | 2 | 2 | 0 | 2 | 0 | 1.00 | 1.00 | 0/2 |
| ops | 4 | 4 | 0 | 5 | 4 | 1.00 | 1.00 | 3/4 |
| performance | 2 | 2 | 0 | 1 | 1 | 1.00 | 1.00 | 1/2 |
| privacy | 4 | 4 | 0 | 6 | 4 | 1.00 | 1.00 | 1/4 |
| security | 3 | 3 | 0 | 2 | 1 | 1.00 | 1.00 | 3/3 |
| skeptic | 2 | 2 | 0 | 8 | 3 | 1.00 | 1.00 | 2/2 |
| tracer | 1 | 1 | 0 | 0 | 0 | 1.00 | — | 1/1 |

Fixture gaps behind its false positives, read finding by finding:
`skeptic-control-small-flag` cites `src/cli/progress.rs`,
`src/cli/import.rs`, `src/cli/errors.rs`, `docs/cli.md` and the test
`docs_mention_every_flag` on a base that holds none of them;
`skeptic-plan-scope-creep` cites `web/src/settings/`;
`newcomer-code-clever-one-liner` is verified by `npm test -- pricing`
with no test file. In `accessibility-code-icon-button`, the image's
expectation was taken by the button's finding two lines away (R-skeptic-6).

## Goals / Non-Goals

**Goals:**

- A recall that can fall: defects a careful reviewer finds and a
  superficial one misses.
- A false positive that is the role's, not the fixture's.
- A true positive that names the seeded defect, not a neighbour.

**Non-Goals:**

- Tuning any role or skill (`role-precision-from-baseline` does it,
  against this change's baseline).
- A category field in the review record, or asking reviewers to
  classify their findings.
- Several runs per case and variance: one run per case, as before.
- Changing the runner, its prompts, its permissions or the workflow.

## Decisions

### Decision: The category is recognised by terms, from a vocabulary kept with the suite

`evals/categories.yaml`:

```yaml
categories:
  injection:
    description: Untrusted input reaches a query, a command or an interpreter.
    terms: [injection, sql, parameter, bind, bound, concatenat, escap, cwe-89]
  text-alternative:
    description: An image or icon has no text alternative.
    terms: [alt, alternative, image, img, avatar]
```

Read strictly (`deny_unknown_fields`, decision 0006's `serde_norway`);
a category without term is refused. `suite::load_cases` reads it and
passes it to `case::load`, which refuses an unknown category and copies
the category's terms, lowercased, into the `Expected` it builds: the
scoring stays a pure function of the case and the observation (decision
0001). A finding names a category when its `title` or `recommendation`,
lowercased, contains one of the terms at the start of a word — preceded
by the start of the text or by a character that is not a letter or a
digit — so that `alt` names "alt text" and "alternative" but not
"default" or "health". The `evidence` is left out: it
quotes the code, where `password` or `log` appear whatever the finding
says.

The vocabulary is tuned once, on the first baseline's 26 recorded
observations: with the old cases scored under the new rules, every
finding that named its defect must still be a true positive, and the
icon button's two findings must pair the right way round. Terms are
word prefixes (`concatenat` covers every form of the verb), short
enough to absorb wording, specific enough that two defects of the same
case do not share one. The vocabulary is then frozen for the new run:
an edit made after it — to absorb a wording the run produced — is listed
under **Baseline before and after** with every finding it re-classifies,
each read by a person and naming its seeded defect; otherwise adding
terms until misses vanish would raise recall by construction.

**Alternatives considered**: a `category` field in the review record
filled by the reviewer (changes codev's public format for an evaluation
need, and tells the reviewer what to look for if the categories are
listed); per-case keywords (the same synonyms copied into every case
that seeds the same kind of defect, drifting apart); a model as judge
(non-deterministic, rejected by the first design); a one-to-one
assignment on location alone (fixes the count but still lets a finding
about the button stand for the missing image).

### Decision: Pairs are a maximum matching, deterministic

`score::case` builds, for the evaluated case, the bipartite graph
"expected finding — finding of the same role whose location (or an
alternative) and category match", then finds a maximum matching with
augmenting paths (Kuhn's algorithm): expected findings in case order,
each trying findings in record order. The result is maximal, and the
same inputs give the same pairs. A case holds a handful of findings,
so the quadratic cost is irrelevant.

Unpaired findings are then classified in this order: duplicate (role,
location and category match an expected finding of their role),
cross-role (location and category match another role's expected
finding), unreported (low confidence), false positive. A false positive
or unreported finding whose role and location match an expected finding
but not its category keeps that expected finding's id in a new field,
`near`, of `FindingScore` (omitted from `report.json` when absent), and
the markdown report writes "at the location of `<id>`, another category"
next to it.

**Alternatives considered**: keeping the greedy pairing with the
category added (a finding naming both defects could take the first and
starve the second); counting an off-category finding as a duplicate
(would hide real false positives behind a seeded defect, the very flaw
R-skeptic-5 reported).

### Decision: `alternatives` and `planned` in `case.yaml`

`alternatives` (optional list) is matched exactly like `locations` and
checked the same way at load; `locations` holds where the defect is,
`alternatives` where a reviewer may rightly point at it too — the plan's
release note for a breaking rename, the error handler that logs a
message built elsewhere. The existing cases move their secondary
locations to `alternatives`; the matching is unchanged by that move.

`planned` (optional list) names files and tests a plan introduces.
Refused in a `code.review` case (its tasks are done: what they name must
exist), refused when an entry is already in the fixture (stale), or when
no artifact cites it (dead).

**Alternatives considered**: inferring what a task creates from its
verb ("Add", "Create") — a heuristic that would let a typo in a path
through as "planned".

### Decision: The fixture lint is part of loading a case

A new pure module, `lint`, receives the change's directory, the merged
fixture and the `planned` list, and returns every problem as
`` `<artifact>` cites `<reference>`, which the fixture does not have ``.
`case::load` refuses the case with all of them joined, so a case author
fixes them in one pass; `run`, `score` and `list` load cases, so no case
with a gap can be run or scored. The test `every_case_of_the_suite_loads`
makes it part of `cargo test`.

What is a reference, decided on backquoted spans of the change's
markdown files (`` `…` ``; fenced code blocks are skipped):

- **path**: the span (or, in a span holding spaces — a command — each
  word) after removing a `:12` / `:12-14` suffix and a `#…` heading,
  with `\` read as `/`, is a path when it holds no `*`, `?`, `{`, `<`,
  `$`, `(`, `=`, `@` or `://`, does not start with `/` or `-`, and
  either ends with `/`, or its last segment has an extension of
  [`rs`, `ts`, `tsx`, `js`, `jsx`, `py`, `sql`, `html`, `css`, `scss`,
  `md`, `toml`, `json`, `yaml`, `yml`, `lock`, `vue`, `svelte`, `go`,
  `java`, `kt`, `rb`, `sh`, `txt`, `csv`] and holds a `/` — or is a
  whole span without spaces (a bare file name). A bare name inside a
  command (`ledger import data.csv`) is an argument, not a citation,
  and is skipped. It resolves when it is a file of the fixture relative
  to the project or to the change's directory, a directory prefix of one
  (with a trailing `/`), or, bare, the name of any file;
- **test** (in `tasks.md` only): a span matching
  `^[a-z][a-z0-9]*(_[a-z0-9]+)+$` right after the word "test" or "tests",
  or listed after such a span with only `,`, `and` or `or` between them —
  a function named on the same line (`fetch_rates`, verified by …) is not
  a test; or the filter of a command span: the first word after
  `cargo test` that is neither a flag nor the value of `-p`, `--test`,
  `--features` and the like, the first non-flag word after
  `npm test --`, the word after `-k` in `pytest`. It resolves when a test file of the fixture contains it: outside
  `_codev/`, a path containing `test` or `spec`, or a `.rs` file
  containing `#[test]`.

The rule is deliberately narrow: it catches the gaps the first baseline
showed (paths, directories and test names) without parsing prose.
Anything it cannot recognise is not checked, rather than guessed.

**Alternatives considered**: a separate `codev-evals lint` command (a
second gate a contributor can forget; loading already refuses invalid
cases); checking every word that looks like a path in the prose (MIME
types, routes and URLs would need their own exceptions); using
`codev validate` on the fixture (it checks a plan's structure, not that
the files it names exist).

### Decision: Coverage per expert: three seeded, two controls, one case of two severities

`case::COVERAGE` becomes per expert (3, 2) and newcomer, skeptic, tracer
unchanged; `coverage_gaps` adds, per expert, "no seeded case expects two
findings of different severity", and for `security` and `privacy`, "no
`python` case". The table below lists the cases added; each new seeded
defect sits next to code doing the same thing correctly, so a reviewer
has to read rather than pattern-match.

| Case | Stack | Kind | Expected (severity, category) | Plausible-but-fine code next to it |
|---|---|---|---|---|
| `security-code-detail-endpoint-authz` | TypeScript | seeded | detail route without the tenant check the list route has (high, access-control); error handler returns `err.stack` to the client (medium, information-disclosure) | the list route's tenant filter, a parameterized query |
| `security-code-python-path-traversal` | Python | seeded | download joins the requested name to the upload directory (high, path-traversal) | `secrets.compare_digest` on the token, a bound SQL parameter |
| `security-control-dynamic-query` | Python | control | — | WHERE and ORDER BY built from an allow-list of columns, values bound |
| `privacy-code-error-message-pii` | TypeScript | seeded | full profile with birth date and phone sent to the support widget (high, personal-data-to-processor); e-mail in a thrown error message that the base's error handler logs (medium, personal-data-in-logs; alternative: the handler's log line) | the widget initialised with an opaque id elsewhere |
| `privacy-code-python-soft-delete` | Python | seeded | "delete my account" only stamps `deleted_at`, keeping e-mail, phone and address while the spec promises erasure (high, erasure) | analytics keyed by a salted hash of the id |
| `privacy-control-ids-in-logs` | Python | control | — | logs of order ids, counts and durations |
| `accessibility-code-dropdown-keyboard-trap` | TypeScript (TSX) | seeded | Tab is prevented while the menu is open and nothing but a choice closes it (high, keyboard-trap); `aria-expanded` hard-coded to `false` (medium, state-not-exposed) | arrow keys, menu roles, focus returned to the button after a choice |
| `accessibility-control-custom-listbox` | TypeScript (TSX) | control | — | a listbox with `aria-activedescendant`, arrows, Home/End, Escape, Tab leaving |
| `performance-code-hidden-n-plus-one` | Rust | seeded | `customer_label` queries the database once per invoice of the loop (medium, query-per-item); a `Regex` compiled for every line (low, repeated-work) | totals computed in one aggregate query |
| `performance-control-preloaded-helper` | Rust | control | — | a helper called in a loop that reads a map loaded by one `ANY($1)` query |
| `ops-code-retry-without-backoff` | TypeScript | seeded | webhook retries fire back to back, no delay (medium, retry); delivery failures logged with `console.debug` only (low, observability) | a request timeout through `AbortSignal.timeout` |
| `ops-control-graceful-shutdown` | Rust | control | — | a worker that stops on SIGTERM, bounded waits, structured logs |
| `migration-code-serde-rename-all` | Rust | seeded | `rename_all = "camelCase"` renames the public JSON fields (high, breaking-api); a new configuration field without a default breaks existing files (medium, config-compat) | an internal struct renamed freely |
| `migration-control-serde-alias` | Rust | control | — | a field renamed with `#[serde(alias)]`, output name unchanged |

The suite grows from 26 to 40 cases.

**Alternatives considered**: replacing the easy cases (they still catch a
role that regresses badly, and keep the comparison with the first
baseline readable); more cases per role for statistics (cost; the
variance work is a non-goal).

### Decision: The baseline is recorded again, on the default model

After the lint and the matching are tested, the existing cases are
completed (their gaps fixed, secondary locations moved to
`alternatives`, categories from the vocabulary), then the whole suite
runs once — `cargo run -p codev-evals -- run --out evals/baseline` after
removing the old directory, Claude Code's default model, the default
3 USD budget and 600 s timeout per case. The roles' scores before and
after are recorded below, under **Baseline before and after**, once the
run is done; the first baseline's observations are also re-scored under the new
rules, against the completed cases whose cited lines did not move, to separate the effect of the matching
from the effect of the cases. The workflow's 240-minute job timeout
covers 40 cases at most ten minutes each, two at a time (200 minutes).

**Alternatives considered**: keeping the old observations for the old
cases and running only the new ones (the completed fixtures change what
the reviewer sees: their old observations no longer describe them).

### Every case, planned

| Case | Behavior |
|---|---|
| No git repository | Unchanged: the runner creates each fixture's repository; the lint and the matching read files and strings only. |
| A change with no diff yet | Every `plan.challenge` case; the lint applies to plan cases too, with `planned` for what the plan creates (scenario "A file the plan creates"). |
| A docs-only change | `security-control-docs-only` stays; its README is not a planning artifact, so the lint does not read it. |
| Generated, vendored, binary or very large files | Refused in fixtures as before (64 KiB, UTF-8); the lint reads planning artifacts only. Measuring roles on such files: out of scope. |
| Huge diffs | Out of scope for the measure: fixtures stay small; runs stay bounded by budget and timeout. |
| Monorepos with several stacks | `performance-code-n-plus-one` (Rust and TypeScript) stays; the lint resolves paths in any part of the fixture. |
| Windows paths and CRLF | The lint reads `\` as `/` in a cited path and splits lines with CRLF or LF (test `the_lint_reads_windows_paths_and_crlf`); matching keeps its path normalization (scenario "A Windows path"). The runner remains untested on Windows (out of scope, as before). |
| Offline | `score` and the lint need no network; `run` records Claude Code's error per case, as before. |
| A role disabled by configuration | Unchanged: a disabled evaluated role is `role-not-run`, never a clean control. |
| A step re-run (idempotence) | Matching is deterministic (scenario "Two defects close together, cited in reverse order"); scoring the same directory twice gives the same bytes (existing test). |
| An interrupted run resumed | Unchanged: `run --out <dir>` skips recorded cases. The new baseline is recorded into an empty directory, so no old observation mixes in. |
| Inherited sources | Fixtures inherit nothing: out of scope. |
| Older binaries reading newer files | `report.json` keeps `format: 1` and gains an optional `near` field per finding (additive, its readers ignore unknown fields); `case.yaml` and `categories.yaml` are strict and internal, loaded by the runner of the same commit; `observed.json` is unchanged. |

## Risks / Trade-offs

- [A correct finding worded outside the vocabulary becomes a miss and a
  false positive] → the report names the expected finding next to such
  a finding (`near`), so the human reading the scores sees it; the
  vocabulary is tuned on the first baseline's real wording and each fix
  is one line in `evals/categories.yaml`, followed by `score` on the
  recorded baseline.
- [Terms too broad let a finding pair with the wrong defect anyway] →
  the two defects of a case are chosen in different categories with
  disjoint terms; a unit test checks that the vocabulary's categories
  used together in a case share no term.
- [Harder cases make recall drop for the wrong reason: a defect that is
  not one] → each seeded defect breaks a requirement or a criterion the
  role names, stated in the case's `summary`; the controls hold the same
  pattern done right.
- [The lint misses a gap it cannot recognise, like a function name in
  prose] → the rule is stated in the spec; gaps it cannot see stay
  visible in the report as false positives citing a missing file, to be
  fixed in the case.
- [A full run costs more: 40 cases instead of 26] → about 0.4 USD per
  case as before, about 15 USD in all, still never automatic.

## Migration Plan

Internal to the repository: existing cases are edited in the same
commit as the loader that requires their new fields. Rollback: revert
the commit, which restores the old cases, scoring and baseline together.

## Baseline before and after

Three readings, all on Claude Code 2.1.289 with its default model, one
run per case: the **first** baseline as committed (26 cases, greedy
matching by location); the same observations **re-scored** under this
change's rules, against the completed cases (task 5.2); the **new**
baseline (40 cases, 15.18 USD, recorded 2026-10-13).

| Role | Recall first → new | False positives first → re-scored → new | Severity agreement first → new | Cases new (seeded/control) |
|---|---|---|---|---|
| accessibility | 1.00 → 1.00 | 3 → 2 → 4 | 2/3 → 5/5 | 5 (3/2) |
| migration | 1.00 → 1.00 | 1 → 0 → 5 | 3/3 → 4/5 | 5 (3/2) |
| newcomer | 1.00 → 1.00 | 2 → 5 → 6 | 0/2 → 0/2 | 3 (2/1) |
| ops | 1.00 → 1.00 | 5 → 5 → 11 | 3/4 → 3/6 | 5 (3/2) |
| performance | 1.00 → 1.00 | 1 → 1 → 5 | 1/2 → 1/4 | 5 (3/2) |
| privacy | 1.00 → 1.00 | 6 → 7 → 8 | 1/4 → 4/7 | 6 (4/2) |
| security | 1.00 → 1.00 | 2 → 2 → 4 | 3/3 → 6/6 | 6 (4/2) |
| skeptic | 1.00 → 1.00 | 8 → 11 → 9 | 2/2 → 2/3 | 3 (2/1) |
| tracer | 1.00 → 1.00 | 0 → 0 → 0 | 1/1 → 1/1 | 2 (1/1) |

Evidence validity stays 1.00 for every role. What the readings say:

- **Recall still does not discriminate.** Every seeded defect of the
  fourteen new cases was found, the subtle ones included (the detail
  endpoint without tenant check, the e-mail reaching the logs through an
  error message, the N+1 behind a helper, the Tab trap, the
  `rename_all` attribute, the back-to-back retries). On this model, the
  measure moves through false positives and severity agreement; harder
  defects — or a weaker model — are needed before recall can fall.
- **Matching by category re-classifies, it does not inflate.** Re-scored,
  the first baseline keeps every true positive; the icon button's two
  findings now pair with their own defects; findings that the location
  alone had counted as duplicates while naming another issue become false
  positives (newcomer 2 → 5: duplicated seat filtering, silent skips;
  skeptic 8 → 11: a hidden migration and an untestable preference next to
  the scope creep), and the report names the defect they sat next to.
- **No false positive comes from a missing file any more.** The first
  baseline's fixture gaps (`src/cli/progress.rs`, `web/src/settings/`, a
  verification test that did not exist) are gone; in the new run, no
  finding cites a file or test the fixture lacks. Two remain debatable
  and are the cases' to improve, not the roles': in
  `skeptic-control-small-flag`, `docs_mention_every_flag` iterates an
  empty flag list, so the skeptic rightly says the task's check passes
  without `--quiet`; in `privacy-code-python-soft-delete`, the tests'
  missing erasure check is the seeded defect seen from the test file.
- **The new controls carry most new false positives**, all of them
  plausible but out of the case's scope: `ops-control-graceful-shutdown`
  (4: no health check, hard-coded settings, retries without
  dead-lettering, a warning without cause), `performance-control-preloaded-helper`
  (4 low: cloned numbers, a second scan for the total),
  `accessibility-control-custom-listbox` (4: a native select would do,
  no type-ahead), `migration-control-serde-alias` (3: a file holding both
  keys, no deprecation warning). The roles above the threshold —
  accessibility, migration, newcomer, ops, performance, privacy, security,
  skeptic — are `role-precision-from-baseline`'s material.

Vocabulary edits after the new run (frozen beforehand, R-skeptic-1): one.
`fall back` was added to `rollback`, re-classifying one finding, read by
a person: `ops-plan-no-rollback` R-ops-1 "No way to turn Paylane off or
fall back to Stripe", at the seeded defect's heading, names the missing
way back (false positive → true positive, R-ops-2 becomes its duplicate;
ops false positives 12 → 11). Before the run and with no observation to
fit, `escape` was added to `keyboard-trap` for the new dropdown case.

## Open Questions

None.
