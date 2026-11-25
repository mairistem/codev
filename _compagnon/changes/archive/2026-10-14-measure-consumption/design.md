# Design: Measure what a run consumes, and compare two runs with a verdict

## Context

The evaluation suite lives in the `codev-evals` crate (`evals/`), outside
codev's four crates. It already separates pure parts from the shell:
`case.rs`, `score.rs`, `report.rs` and `reliability.rs` are pure; the parts
of `runner.rs` below the "Shell" comment, `suite.rs` and `main.rs` touch the
disk and start processes. Decision 0001 governs codev's crates, not this
one, but the suite follows the same split and this change keeps to it.

Today, a run writes `<out>/<case>/observed.json` (format 1) and keeps
Claude Code's raw output next to it as `claude.json`. That raw output
already holds what this change needs: `usage` and `modelUsage` (tokens per
model), `num_turns`, `duration_api_ms` and `permission_denials`. In the
committed baseline, the 40 cases add up to 373 turns, 51 denials, 1.18 M
uncached and cache-written input tokens, 12.3 M tokens read from the
cache and 0.11 M output tokens: what a run costs is mostly its context
re-read on every turn.

The runner calls `claude -p … --permission-mode acceptEdits
--allowedTools "Bash(codev:*) Bash(git diff:*) …"`. Almost every denial in
the baseline is a compound command (`cd … && codev …`, `cat … | …`) that
the patterns do not match.

## Goals / Non-Goals

**Goals**: record consumption per run; repeat each case and score the
repetitions; compare two runs with a verdict that is strict on quality and
decides on cost; run the agent the way a user working with Claude Code's
sandbox would.

**Non-goals**: cases for writing artifacts or code (`/codev-propose` and
`/codev-apply` authoring), which come in a separate change; any statistical
test beyond the baseline's observed range; changing how a finding is
matched or classified; any change to codev itself.

## Decisions

### Decision: Approve every shell command inside Claude Code's sandbox

The runner keeps `--permission-mode acceptEdits`, narrows `--allowedTools`
to `Bash(codev:*)` — exactly what the skills grant in their
`allowed-tools`, which a user's session applies when the skill is invoked
and which the evaluation's agent, reading the skill file, would otherwise
lack — and passes `--settings` with Claude Code's sandbox:

```json
{"sandbox": {"enabled": true, "autoAllowBashIfSandboxed": true,
  "allowUnsandboxedCommands": false, "failIfUnavailable": true,
  "network": {"allowedDomains": ["api.anthropic.com"]},
  "credentials": {"envVars": ["ANTHROPIC_API_KEY"]}}}
```

and `--disallowedTools WebFetch WebSearch`. The shell commands Claude
Code knows (`ls`, `git`, `cat`, `cd … &&`…), compound or not, are approved
without a prompt in the sandbox, `codev` through the grant; another
program (`python3`, `curl`) still asks, which a headless run denies and
counts. Every command runs in the sandbox: it can write only in the working directory (the case's) and the
temporary directory, reach no host but the API, and not read the key.
The file tools are outside the sandbox but under `acceptEdits`: an edit
inside the case's directory is accepted, one elsewhere would need a
prompt, which a headless run denies. This is a configuration a real user
runs with, so the measure is in real conditions.

A run on this machine (Claude Code 2.1.289, macOS) checked it before the
plan was settled, and a second one, during implementation, showed that
the sandbox's auto-approval does not cover `codev` itself — hence the
grant: `cd src && cat a.txt | head -1` and
`X=1; echo $X > out.txt && cat out.txt` ran; `echo … > $HOME/…`, `curl
https://example.com`, `printenv` and a Write to `$HOME` were each denied
and listed in `permission_denials`, and nothing was written outside the
directory. Denials are reported, never a failure: a non-zero count says
the agent tried something the sandbox refused, which the reader
investigates.

On Linux the sandbox needs `bubblewrap` and `socat`, and on Ubuntu 24.04
an AppArmor profile that lets `bwrap` create user namespaces; the
workflow installs them. The runner's preflight checks them before any case
runs and exits with code 2 naming what is missing; `failIfUnavailable`
stays as a second guard, a case failing rather than running unconfined.

**Alternatives considered**: `--permission-mode bypassPermissions` lets a
prompt injection in a fixture or a role file run anything with the
machine's credentials and the API key in reach (review finding
R-security-1); Claude Code itself recommends it only for sandboxes
without internet access. `--permission-mode auto` is realistic too, but
its classifier is a model call per tool call, which adds consumption
that has nothing to do with codev to the very measure this change
introduces. Widening the allowlist (`Bash(*)`) has the power of bypass
without its name. Keeping today's allowlist keeps a bias unrelated to
codev: an optimization could "save" tokens merely by triggering fewer
denials.

### Decision: One directory per repetition, `<out>/<case>/<n>/`

Each repetition writes its `observed.json`, `claude.json`,
`claude.stderr` and `review.md` under `<out>/<case>/<n>/`, `n` from 1, and
sets up its fixture in `<work>/<case>-<n>`. Reading a directory, a case
whose `observed.json` sits directly under `<case>/` is read as repetition
1. Resuming skips a `(case, n)` whose observation exists, so asking for
`--repeat 5` on a directory with 2 repetitions runs 3, 4 and 5. The job
queue holds `(case, n)` pairs, so repetitions run in parallel like cases.

**Alternatives considered**: flat directories `<case>-r<n>` mix the
repetition into the case id, which every reader would then have to parse.
Overwriting a single `observed.json` per case loses the spread the
verdict needs.

### Decision: Observation format 2 carries a `consumption` object

`Observed` gains `consumption`: `durationApiMs`, `turns`,
`permissionDenials`, `models` (the keys of `modelUsage`, sorted), and
`tokens` with `input`, `cacheCreation`, `cacheRead`, `output`. Each field is optional: absent when Claude Code did
not print it (a timeout, a crash), never 0. The tokens are summed over
`modelUsage`, every model the run used — Claude Code also calls a small
model for housekeeping, and the cost already includes it — and fall back
to `usage` when `modelUsage` is absent. `FORMAT` becomes 2 for
`observed.json`, `run.json` and `report.json`; format 1 is still read,
with `consumption` absent. `RunInfo` gains `repeat`, the number of
repetitions asked.

**Alternatives considered**: reading `claude.json` at scoring time
instead of copying into `observed.json` would make the raw log, whose
shape Claude Code owns, part of the scoring contract. Only the top-level
`usage` would miss the housekeeping model's tokens, which the cost counts.

### Decision: Score each repetition as today, then take medians

`score::case` is unchanged and runs on every observation. Role measures
are computed per repetition with the existing aggregation, restricted to
that repetition's cases; the role's reported value is the median over the
repetitions, with its minimum and maximum. The median of an even count is
the mean of the two middle values, rounded to three decimals like the
other ratios. Thresholds apply to the medians. With one repetition the
median is the single value, so the committed baseline's verdict and the
reliability chapter's figures do not move because of the method — only
because the baseline is recorded again.

The suite-level figures the reliability chapter shows — recall over
every seeded defect, evidence validity, false positives — are computed
per repetition over all its cases, then given as a median with its
range, so the page copies a figure the report states rather than
summing per-role medians, which would be the median of nothing.

Consumption is a median per case over the repetitions that have the
measure, then summed per role and for the suite: a median per case keeps
one runaway repetition (a retry loop, a stall) from dominating the total.

**Alternatives considered**: pooling every repetition's counts (true
positives over expected across all of them) hides the spread and turns
the false-positive threshold, which counts per run, into a sum. The mean
instead of the median lets one bad repetition move the result.

### Decision: `compare` re-scores both directories and uses the baseline's range as the noise band

`codev-evals compare <baseline> <candidate> [--quality-only]` loads both
directories' observations, keeps the cases both hold, scores each run on
those cases only — so a candidate run on 10 cases is compared with the
baseline's same 10 — and hands the two scores to a pure `compare` module
that returns the verdict and the markdown report. Re-scoring instead of
reading `report.json` means both runs are judged by the same, current
scoring rules.

Quality fails for a role when the candidate's median leaves the range the
baseline's repetitions spanned: recall under the baseline's minimum,
false positives above its maximum, evidence validity under its minimum.
Consumption follows the same logic: the candidate's total cost (sum of
per-case medians) must be lower than the baseline's floor, the sum of
each case's cheapest run. A no-op change therefore does not pass by
luck: every case median is at least that case's minimum, so compared
with itself the baseline always fails on cost. (The plan first set the
bar at the cheapest whole repetition; implementing it showed that a
baseline whose cases are expensive in different repetitions — three
cases each costing 9 in a different one of three repetitions, 0
otherwise — has medians summing to 0 while every repetition costs 9, so
the unchanged code would pass. The per-case floor has no such gap.) The model is held constant: the
command refuses two runs whose asked models differ, or whose
observations record different models as used — `run.json`'s model is
`null` for Claude Code's default, which can change with Claude Code's
version — and it reports the two Claude Code versions. At a constant
model the cost is the tokens weighted by their price, so it is the
single number that answers "did it consume less"; the per-kind tokens and
the turns are reported to explain where the gain comes from. Duration is
reported only: it follows the API's load as much as the work done.

**Alternatives considered**: a statistical test (Mann–Whitney, bootstrap
intervals) is opaque and needs more than five points to say anything. A
fixed tolerance (recall −0.05) is arbitrary and blind to how noisy each
role actually is. Comparing total tokens instead of cost weighs a cache
read like an output token, whereas a cache read costs a small fraction of
an output token.

### Decision: The baseline is recorded again with five repetitions

The baseline becomes the run of the 40 cases with `--repeat 5` in the
sandbox, at the Claude Code version the workflow pins — installed for the
recording as CONTRIBUTING says, and checked by a suite test against the
baseline's `run.json` — four jobs at a time. At the last baseline's cost that is about 70 USD,
within the budget the user granted. A suite test checks that every case
of the baseline has at least five repetitions.

**Alternatives considered**: three repetitions halve the cost but make
the noise band rest on three points; with no budget constraint, five is
the better reference.

## Risks / Trade-offs

- [A fixture or a role file holds a prompt injection that the agent
  follows] → The sandbox bounds what a shell command can do — no write
  outside the case's directory, no network but the API, no access to
  the key — and file edits elsewhere are denied. The `strayed` check is
  no containment, only a report on the case's own working tree. What is
  left is what the agent writes in the case's directory, which is thrown
  away, and what it sends to the API, which is the run itself.
- [A narrow baseline range — recall 1.00 in all five repetitions — fails
  a candidate whose median drops to 0.80 for one unlucky case] → The
  strictness is wanted: the user asked for no loss of quality. The report
  names the role, the measure and both values, and a doubtful verdict is
  settled by running more repetitions of the candidate on that role's
  cases.
- [A candidate is cheaper by chance] → It must beat the baseline's
  floor, each case at its cheapest run, not its median; CONTRIBUTING asks for at least
  three repetitions of the candidate, and the report shows the per-kind
  tokens and turns, so the reader sees where a gain comes from.
- [The prompt cache, shared by runs of one account for up to an hour,
  makes a run read what another wrote] → Cache writes and reads are
  reported separately, and CONTRIBUTING recommends recording the
  candidate with the baseline's `--jobs`. Concurrent repetitions of one
  case can share cache entries in both runs alike.
- [A baseline repetition in error would make every comparison fail] →
  A resume runs errored repetitions again, and the suite test refuses a
  baseline with one.
- [Raw Claude Code logs committed with the baseline] → `evals/.gitignore`
  ignores `baseline/**/claude.json` and `baseline/**/claude.stderr`, the
  per-repetition layout included.
- [A full baseline of 200 runs does not fit a hosted CI job, capped at
  360 minutes] → The workflow's `repeat` input is for targeted runs; the
  baseline is recorded locally, as CONTRIBUTING says, and the job's
  timeout is the hosted maximum.
- [Old output directories written in format 1] → Still read, one
  repetition each. They hold the cost, so `compare` still judges
  consumption on it; only the tokens, turns, API duration and denials
  show as `—`. A directory whose cases hold no cost at all is the one
  case where `compare` fails on consumption unless `--quality-only`,
  saying why.

## Migration Plan

1. Ship the code with the committed baseline still in format 1 and the
   old layout: the tests read it as one repetition and its report is
   re-scored into format 2.
2. Record the new baseline (`run --out evals/baseline --repeat 5`), then
   regenerate the reliability chapter with `codev-evals docs`.
3. Rollback is reverting the change: the old baseline comes back with it.
