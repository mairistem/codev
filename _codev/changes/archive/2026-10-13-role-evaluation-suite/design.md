# Design: Measure what each role finds and what it invents

## Context

The reviewing roles are built-in markdown files (`assets/roles/*.md`)
served by `codev roles`; the skills apply them at `plan.trace` (the
tracer, which fixes gaps directly), `plan.challenge` (the skeptic and the
active experts) and `code.review` (the newcomer and the active experts).
At the two review steps, findings go to `_codev/changes/<name>/review.md`,
which `codev review --json` parses into `ReviewV1` (`findings[]` with
role, step, severity, confidence, location; `issues[]` whose messages
start with the finding's id; decision 0002 makes that JSON the public
contract). Experts are activated by `## Domains`, and a change's
`change.yaml` can set any disableable role to `never`.

The workspace is `crates/*`; CI runs fmt, clippy and tests on the whole
workspace, on Linux, macOS and Windows; `release.yml` builds
`--bin codev` only. Claude Code runs headlessly with `claude -p`, which
takes `--model`, `--max-budget-usd`, `--allowedTools`,
`--permission-mode`, `--output-format json` (whose result carries
`total_cost_usd`, `duration_ms` and `is_error`) and
`--no-session-persistence`.

## Goals / Non-Goals

**Goals:**

- A number per role — recall, false positives, evidence validity — that a
  role change moves visibly.
- Every judgement on the model's output made by deterministic code, unit
  tested on recorded outputs.
- A run that a contributor can afford: small fixtures, one run per case,
  a budget and a timeout per case.

**Non-Goals:**

- Running the suite in the regular CI, or making it a merge gate.
- Statistical confidence: one run per case gives a noisy point; repeated
  runs and variance are left for later (`--repeat` is not part of this
  change).
- Measuring the artifact roles (`product`, `qa`, `architect`, `lead`),
  the `analyst`, the `investigator` and the `developer`: they write, they
  do not review.
- Measuring interactions between reviewers (who steals whose finding):
  each case isolates one role.
- Tuning any role. A role that scores badly is recorded in the baseline
  and gets its own change.

## Decisions

### Decision: A separate workspace crate, `evals/`, never shipped

`evals/Cargo.toml` defines `codev-evals` (`publish = false`, a `lib`
and a `bin`), added to `members` next to `crates/*`. It depends on no
codev crate: it reads codev's JSON output as any consumer of the contract
does (decision 0002 keeps the contract the boundary), and runs the
`codev` binary built from the same tree. Inside it, decision 0001's split
is kept: `case`, `score` and `report` are pure functions over strings and
values; `runner` and `main` do the I/O and the processes. Dependencies are
the workspace's (`serde`, `serde_json`, `serde_norway` per decision 0006,
`clap`): `Cargo.lock` gains the crate, no package.

**Alternatives considered**: `evals/run.sh` with `jq` (the scoring rules
are the hard part, and shell has no unit tests on Windows); an `xtask`
(the same crate under another name, plus an alias); a subcommand of
`codev` (would ship a model caller in a binary that must never call one);
linking `codev-core` to parse `review.md` directly (bypasses the evidence
check codev runs, and couples the suite to internals).

### Decision: A case is a fixture split in two trees, and a `case.yaml`

```yaml
# evals/cases/security-code-sql-injection/case.yaml
role: security
step: code.review
change: add-user-search
stacks: [typescript]
kind: seeded
summary: A search endpoint concatenates the query string into SQL.
expected:
  - id: sql-injection
    severity: high
    category: injection
    locations: ["src/users.ts:9-11"]
```

```yaml
# a plan.trace case
role: tracer
step: plan.trace
change: add-magic-links
stacks: [rust]
kind: seeded
summary: The spec's expired-link scenario has no task.
checks:
  fixed:
    - path: _codev/changes/add-magic-links/tasks.md
      contains: expired link
  unchanged: []
```

`base/` is the project on `main`, `change/` the files the change adds or
overwrites, planning artifacts included; the runner lays `change/` over
`base/` on a branch, so the diff is exactly `change/`. A plan case puts
only planning files in `change/`; a code case puts its code there and the
plan with every task checked. A code case may omit `design.md` and set
`skip_specs: true`: its plan only gives the reviewer context.
`change.yaml` is written by the runner, never by the case.

`case.yaml` is read with `deny_unknown_fields`: a typo is refused, not
ignored. The role-to-step table comes from the `at:` of
`assets/roles/<role>.md`, read from the repository, so a role moved to
another step makes its cases fail to load.

**Alternatives considered**: a single YAML per case with files inlined
(unreadable for code, no syntax highlighting); one shared fixture per
stack (a seeded defect in one case would be a false positive in the
others).

### Decision: The runner isolates the evaluated role and runs one skill step

For each case, in `<work>/<case-id>/` (`--work`, default the system's
temporary directory, removed unless `--keep`):

1. copy `base/`; `git init -b main`; `codev init --yes --no-detect
   --no-integration --preset full --language en`; commit
   (`git -c user.name=codev-evals -c user.email=evals@codev.invalid`,
   `commit.gpgsign=false`);
2. `git switch -c change`; copy `change/` over; write
   `_codev/changes/<change>/change.yaml` — `schema`, `created`,
   `skip_specs: true` when the change has no `specs/`, and `roles:` with
   every role returned by `codev roles --at <step> --change <change>
   --json` other than the evaluated one set to `never`; commit;
3. run `claude -p <prompt> --output-format json --no-session-persistence
   --permission-mode acceptEdits --allowedTools "Bash(codev:*)
   Bash(git diff:*) Bash(git log:*) Bash(git show:*) Bash(git status:*)
   Read Write Edit Glob Grep Skill" --max-budget-usd <b> [--model <m>]`,
   with the directory of the `codev` binary first on `PATH`. git is
   allowed by read-only subcommand only: `Bash(git:*)` would let the agent
   run any program through git's configuration (`-c core.pager=…`,
   aliases). On Unix the process starts in its own process group, and the
   whole group is terminated (`kill -TERM -<pgid>`, then `-KILL` after
   five seconds) when the timeout expires, so the shells and tools it
   spawned stop too; on Windows only the process itself is killed (the
   runner is not supported there, see the table below);
4. run `codev review --change <change> --json` (review steps) and keep its
   JSON whatever the exit code — exit 1 means the record has an error,
   which the scoring turns into `record-invalid` or an invalid evidence;
   only an output that is not JSON is a run error. Read the checked files
   (`plan.trace`). List the files the run modified (`git status
   --porcelain`) outside what the step allows — `review.md` at the review
   steps, the change's directory at `plan.trace` — as `strayed`; write
   `observed.json`.

The prompt (one template per step, in the runner) names the skill file,
the step's heading, the change and the evaluation mode. At the review
steps: "write every finding to the review record as the step says
(a, b, c, g), skip the author's answer and the re-check (d, e, f), leave
every finding `open`, modify no file but `review.md`, ask no question".
At `plan.trace`: "fix the gaps directly in the artifacts as the step says,
modify only files of the change, write no review record". The answer loop
is left out because it measures the author, not the reviewer, and could
edit the code under review.

`observed.json`:

```json
{
  "format": 1,
  "case": "security-code-sql-injection",
  "model": null,
  "error": null,
  "costUsd": 0.42,
  "durationMs": 81234,
  "strayed": [],
  "review": { "...": "the ReviewV1 document, as codev printed it" },
  "files": { "_codev/changes/add-magic-links/tasks.md": "..." }
}
```

`strayed` lists the files modified outside what the step allows, and the
report shows them per case next to the findings whose status is no longer
`open`: the prompt asks the agent to stay inside one step, and the report
shows when it did not, instead of trusting it. `model` is `null` for
Claude Code's default; `error` is the reason a run
stopped (`timeout after 900 s`, `claude exited with 1: …`,
`claude reported an error: …`); `review` is `null` when the record is
missing (`exists: false`).

Cases run `--jobs` at a time (default 2), each in its own directory. A
preflight runs `claude --version`, `git --version` and `codev --version`
(`--codev`, default `target/debug/codev` of the repository) before any
case.

**Alternatives considered**: running the whole `/codev-propose` or
`/codev-apply` (it writes the plan or the code itself: nothing seeded
would remain to find); running the answer loop (an author agreeing with
an invented finding would hide it); `--dangerously-skip-permissions`
(`acceptEdits` with a closed tool list is enough, and safer on a
contributor's machine); a fixed model (the suite must compare roles on
the model contributors use).

### Decision: Scoring rules, all in `score`

`score::case(&Case, &Observed) -> CaseScore` and
`score::suite(&[CaseScore], &Thresholds) -> SuiteScore`, pure, follow the
spec: matching by role and location (two lines of tolerance around the
expected range, headings compared case- and space-insensitively, a
location without heading or line matching the whole file); expected
findings matched in case order by the first finding in record order;
then each other finding classified false positive, unreported,
duplicate or cross-role; evidence from `issues[]` by the id prefix of
their message (`R-security-1: …`), the codes of `review-record`.

A case's outcome is `scored`, `error` (the run stopped), `no-record`,
`record-invalid` (an error issue other than evidence or location, such as
`review_invalid`, with no finding parsed for the role) or `role-not-run`
(the role's state is `disabled` or `missing`). Every outcome but `scored`
counts its expectations as misses; `error` also fails the suite.

Two lines of tolerance: a reviewer citing the statement after the one the
author marked still found the defect; ten would let a finding on the next
function match.

**Alternatives considered**: matching on the finding's title or category
by keywords (a model's wording varies, and keyword lists are a second
model); a model as judge (non-deterministic, and judged by the same kind
of system it judges); counting every unmatched finding as false positive
(a low-confidence finding is never shown to the human: it costs nothing
to them).

### Decision: Thresholds per role, flags over environment

`--min-recall` 0.5, `--max-false-positives` 2 (per role, over all its
cases), `--min-evidence` 0.8, each also read from `CODEV_EVAL_*`. The
defaults are deliberately loose: one run per case is noisy, and the
suite's first purpose is comparison with the baseline, not a gate. Exit
codes: 0 pass, 1 a role fails or a case errored, 2 usage (bad threshold,
invalid case, missing tool, unsupported observation format).

**Alternatives considered**: thresholds in a committed file (one more
format to version; flags are enough for a workflow input); comparing with
the baseline automatically (one noisy run against another would flag
noise; the human comparison in the pull request is the intended use).

### Decision: The baseline is a recorded run, checked by the tests

`evals/baseline/` is the output directory of a full run: `run.json`,
`report.md`, `report.json` and `<case>/observed.json`. `run.json` —
`{"format": 1, "model": null, "date": "2026-10-13", "claudeVersion":
"…", "codevVersion": "…"}` — is written once when a run starts in an
empty directory and never by `score`, which copies it into the report:
the date and versions belong to the run, so scoring the same directory
twice gives the same bytes. A test scores the observations again and compares
with `report.json`, and checks that every case of the suite has an
observation. The baseline therefore doubles as the recorded fixtures of
the scoring core: real model outputs, not hand-written ones; hand-written
observations under `evals/src/testdata/` cover the rules one by one.

**Alternatives considered**: only a markdown table (it drifts from the
scoring rules silently); no committed observations (a change of the
scoring rules could not be told from a change of the roles).

### Decision: The workflow is dispatch-only and checks its secret first

`evals.yml`: `on: workflow_dispatch` with optional inputs (model,
thresholds), passed to the run step through `env:` and quoted in the
script, never interpolated into `run:` (a dispatcher could otherwise
inject commands in a job that holds the key); one job on `ubuntu-latest`; a first step exposes whether
`secrets.ANTHROPIC_API_KEY` is set through an environment variable and,
when it is not, prints "Evaluations skipped: the ANTHROPIC_API_KEY secret
is not set in this repository" and ends successfully; otherwise it builds
codev, installs Claude Code with `npm install -g @anthropic-ai/claude-code`,
runs the suite with a job timeout of 240 minutes — twenty-six cases at
most ten minutes each, two at a time, take at most 130 — and uploads the
output directory with `if: always()`, so a stopped or failing job keeps
what it measured. No secret is added by this change. Not on `assets/roles/**`
pushes: an automatic run would spend money on every role edit, including
drafts; the contributor rule asks for the scores instead.

**Alternatives considered**: `pull_request` with a path filter (cost on
every push to a pull request, and secrets are not available to forks
anyway).

### Every case, planned

| Case | Behavior |
|---|---|
| No git repository | The runner creates the fixture's repository itself; without `git`, the preflight exits 2 (scenario "Claude Code is not installed" covers the check, test `preflight_names_a_missing_tool`). How a role behaves outside git is not measured: out of scope. |
| A change with no diff yet | Every `plan.challenge` and `plan.trace` case is one: its diff holds only planning files. |
| A docs-only change | The `security` control `security-control-docs-only` changes only a README that talks about tokens; a finding there is a false positive. |
| Generated, vendored, binary or very large files | Refused in fixtures: not UTF-8 or over 64 KiB (scenario "A binary fixture file"). Measuring roles on such files is out of scope. |
| Huge diffs | Out of scope for the measure; fixtures are small by the 64 KiB rule; a run is bounded by its budget and timeout (scenario "The run timed out"). |
| Monorepos with several stacks | `performance-code-n-plus-one` has a Rust service and a TypeScript front-end, both detected by `codev roles`. |
| Windows paths and CRLF | Locations read `\` as `/` (scenario "A Windows path"); `unchanged` compares without `\r`; files are compared and counted without line-ending assumptions. The runner is exercised on macOS and Linux; on Windows it is untested (out of scope: the workflow runs on Linux). |
| Offline | `run` fails per case with the error Claude Code reports, recorded as `error` (scenario "The run timed out" covers the outcome); `score` needs no network. |
| A role disabled by configuration | The runner disables every other role on purpose; if the evaluated role ends up disabled, the case is `role-not-run`, never a clean control (scenario "The evaluated role was disabled"). |
| A step re-run (idempotence) | Each case runs in a fresh copy; scoring is a pure function, sorted output (scenario "Scoring twice gives the same report"). |
| An interrupted run resumed | `run --out <dir>` skips recorded cases (scenario "A resumed run"). |
| Inherited sources | Fixtures inherit nothing: out of scope. |
| Older binaries reading newer files | `observed.json`, `run.json` and `report.json` carry `format: 1`; a higher format is refused by name (scenario "A newer observation"); `ReviewV1` is read leniently, unknown fields ignored, as the contract is additive; `case.yaml` is strict, internal to the repository, and loads with the runner of the same commit. |

## Risks / Trade-offs

- [One run per case is noisy: a role's score moves without any change]
  → the report says so; the contributor rule asks for before and after on
  the same model; `--case` and `--role` let a contributor re-run the
  cases of the role they touched.
- [Seeded defects are too easy, so recall says little] → each seeded
  defect is the kind of issue the role's criteria name, placed in
  realistic code next to correct code; controls hold the same domain
  without the defect.
- [An agent with edit rights in a fixture runs commands] → a fresh
  temporary copy, `acceptEdits` with a closed tool list (`codev` and
  `git` only for Bash), a budget and a timeout; the fixture holds no
  secret. The workflow passes the key as an environment variable to the
  run step only.
- [The suite costs money] → never automatic; per-case budget (3 USD
  default), small fixtures, one run per case; the report shows the cost.
- [The model's default changes under the baseline] → the report records
  the model asked for (or "default") and Claude Code's version.
- [The baseline goes stale when a case is added] → a test fails until it
  is recorded again.

## Migration Plan

Nothing to migrate: a new crate and new files. Rollback: remove `evals/`
from `members` and delete it, with the workflow.

## Open Questions

None.
