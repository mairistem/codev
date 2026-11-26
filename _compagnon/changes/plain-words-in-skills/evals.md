# Evaluation: plain-words-in-skills

For pull request mairistem/compagnon#65 (tasks 4.1 and 4.2).

## 4.1 — The suite against the baseline

**Result: against the baseline, quality outside its range. Against `main`, run the same way, one regression was this change's own (`configure-answer-no`), now fixed; the others are the model's drift since the baseline. `code.implement` fails on the `format` check, which the case's own reference solution fails in this environment without any model. Task 4.1 is not checked: the user decides.**

Run on Claude Code 2.1.289, the version `.github/workflows/evals.yml` pins
and the baseline was recorded with. The installed 2.1.332 refuses the
runner's sandbox settings (`sandbox.credentials.envVars.0: Expected
object, but received string`), so every case stopped before starting;
the pinned version was passed with `--claude`.

```bash
cargo run -p compagnon-evals -- run --claude <2.1.289> --step plan.update --step change.sync --step change.archive --step configure.enrich --repeat 3 --jobs 4 --out target/evals/plain-small
cargo run -p compagnon-evals -- compare evals/baseline target/evals/plain-small --quality-only
```

### `code.implement`

Run on the build with the configure fix, Claude Code 2.1.289, as
`CONTRIBUTING.md` gives it (`--repeat 3 --jobs 2`; the baseline ran with
`--jobs 4`, which `compare` flags):

```bash
cargo run -p compagnon-evals -- run --claude <2.1.289> --compagnon <this build> --step code.implement --repeat 3 --jobs 2 --out target/evals/plain-implement
cargo run -p compagnon-evals -- compare evals/baseline target/evals/plain-implement --quality-only
```

`compare` fails: 0 of 12 runs pass, against 1.00 in the baseline. The
acceptance assertions (1.00) and the refusals (1.00) hold. Every failing
run fails on one of two checks, never on the code the change asked for:

- **`format`** (7 runs): `cargo fmt --all --check` reports hunks in code
  no run touched. `cargo run -p compagnon-evals -- calibrate
  implement-decision-accept-command`, which runs no model and no skill,
  fails it the same way on the case's merged reference commit
  ("`format` fails on the merged commit"), while the same commit passes
  `cargo fmt --all --check` in a plain checkout here (rustfmt 1.9.0). The
  check fails in the runner's environment whatever the skill does.
- **the last verification task left unchecked** (7 runs, 4.1 or 5.1):
  that task asks for `cargo fmt --all --check` and `validate --strict`;
  the agents reported the format hunks on untouched code and the
  committed decisions 0001–0006 failing their seals
  (`decision_seal_mismatch`), outside the change, and stopped rather
  than reformat or reseal them. Two runs of
  `implement-forward-compatible-config` ran `cargo fmt --all` over
  untouched files to pass `format`, then stopped on the seals.

The closing lines follow the rule, e.g. « Say “finish the tasks of
forward-compatible-config” (shortcut: `/compagnon-apply`) ».

Not run on `main`: that would cost about as much again (about 40 USD),
over the budget agreed. Cost: median sum 13.58 USD against 12.09 at the
baseline's floor (+4.3 % on its median 13.02); reported, not deciding.

```
# Comparison with the baseline

- Verdict: **fail** (quality only: the consumption is reported, it does not decide)
- Model: Claude Code's default
- Review mode: inline
- Appended configuration: none
- Claude Code: baseline 2.1.289 (Claude Code), candidate 2.1.289 (Claude Code)
- Runs at the same time: baseline 4, candidate 2 — **they differ**: the runs did not share the prompt cache alike
- Cases compared: 4; repetitions: baseline 5, candidate 3

## Quality

The candidate's median must stay within the range of the baseline's repetitions.

| Role | Recall: baseline (min–max) | candidate | False positives: baseline (min–max) | candidate | Evidence validity: baseline (min–max) | candidate |
|---|---|---|---|---|---|---|

## `code.implement`

The candidate's shares of runs, acceptance assertions and refusals passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 (1.00–1.00) | 0.00 |
| Checks passing | 1.00 (1.00–1.00) | 0.92 |
| Acceptance | 1.00 (1.00–1.00) | 1.00 |
| Refusals passing | 1.00 (1.00–1.00) | 1.00 |
| Runs modifying a file outside the step | 0 (0–0) | 0 |

## Consumption

Sums, over the compared cases, of each case's median over its repetitions. The candidate's cost must be lower than the baseline's floor, 12.0912 USD: each case at its cheapest run. The duration and the output are reported and never decide: the output carries the model's reasoning, which the quality measures guard.

| Measure | Baseline | Candidate | Change |
|---|---:|---:|---:|
| Cost (USD) | 13.0224 | 13.5836 | +4.3 % |
| Duration (s) | 1317 | 1138 | -13.7 % |
| API duration (s) | 1570 | 1417 | -9.7 % |
| Turns | 173 | 155 | -10.4 % |
| Permission denials | 0 | 1 | — |
| Input tokens, uncached | 4424 | 4470 | +1.0 % |
| Tokens written to the cache | 647272 | 632615 | -2.3 % |
| Tokens read from the cache | 23805148 | 27179371 | +14.2 % |
| Output tokens | 165762 | 171950 | +3.7 % |
| Calls to the model | 182 | 199 | +9.3 % |
| Input per call | 137355 | 139467 | +1.5 % |
| Output per call | 839 | 820 | -2.3 % |

## Where the tokens go

Tokens written to the cache, by source: per case the median over its repetitions, summed over the compared cases. Reported, never deciding: the cost decides.

| Source | Baseline | Candidate | Change |
|---|---:|---:|---:|
| file | 203767 | 211863 | +4.0 % |
| shell | 131286 | 158895 | +21.0 % |
| skill | 62131 | 48938 | -21.2 % |
| search | 33879 | 35555 | +4.9 % |
| edit | 29210 | 20485 | -29.9 % |
| prompt | 26618 | 23751 | -10.8 % |
| output | 16731 | 20853 | +24.6 % |
| compagnon status | 12816 | 2957 | -76.9 % |
| agent | 9587 | 6352 | -33.7 % |
| compagnon roles | 6547 | 18669 | +185.2 % |
| compagnon review | 4305 | 1921 | -55.4 % |
| compagnon validate | 577 | 449 | -22.2 % |

## Per case

| Case | Cost: baseline | candidate | Turns: baseline | candidate | Tokens read from the cache: baseline | candidate |
|---|---:|---:|---:|---:|---:|---:|
| implement-decision-accept-command | 3.7309 | 3.7162 | 82 | 62 | 7618619 | 7987858 |
| implement-fix-merge-and-skill-refresh | 3.8572 | 4.4964 | 8 | 10 | 5176862 | 7848034 |
| implement-forward-compatible-config | 5.2307 | 5.2152 | 80 | 80 | 10911140 | 11241331 |
| implement-refuse-missing-tasks | 0.2036 | 0.1558 | 3 | 3 | 98527 | 102148 |

## Not compared

Cases only one run holds: the cases of the other steps, not run here.

## Failures

- code.implement: pass share 0.00 is under the baseline's minimum 1.00
- (not deciding, `--quality-only`) cost 13.5836 USD is not lower than the baseline's floor, 12.0912 USD (each case at its cheapest run; the baseline's median is 13.0224 USD)
fail: code.implement: pass share 0.00 is under the baseline's minimum 1.00
```

### What regressed

1. **`archive-unchecked-tasks`, 0 of 3 (baseline 3 of 5).** The case
   expects the archive to go through on a change whose last task (6.1)
   is unchecked, as `compagnon archive` allows. In all three runs the
   agent refused and ended on a request to finish the task (« finish the
   tasks of fewer-review-turns », shortcut `/compagnon-apply`). The
   archive body's "When to use it" line (every task checked) predates
   this change; what is new is the `next-steps` fragment, which tells the
   agent to name "the way out" of a problem as one request. It plausibly
   makes stopping on the unchecked task, and offering the way out, the
   easier reply. This is the regression this change likely causes.
2. **`configure-go-no-tests` 0 of 3 (baseline 3 of 5),
   `configure-python-preserves` and `configure-ts-jest` 0 of 3
   (baseline 1 of 5 each).** Every failing run fails the same check,
   "`context` spans two to five lines" (9 of 15 runs; 10 of 25 in the
   baseline). The configure body changed only in its messages and the
   fragment, not in what it says about `context`; with three runs against
   five, noise cannot be ruled out, but the drop on `configure-go-no-tests`
   is large.
3. **`archive-version-later` and `sync-version-later`: stopped, not
   scored.** `compagnon archive` and `compagnon sync` now refuse with
   `compagnon_version_unsatisfied` on the fixture, where the case expects
   them to go on: the CLI changed after the baseline (0.6.1, the cases
   were written for it), not the skills. The cases are stale, independent
   of this change.

### Against `main`: what this change causes

The archive and configure cases were run again on `main`'s build
(`2b945b0`, this branch's base), same runner, same Claude Code 2.1.289,
`--repeat 3 --jobs 4`, passed with `--compagnon`
(`target/evals/main-small`). `main` fails exactly like this branch on
`archive-unchecked-tasks` (0 of 3), `configure-go-no-tests`,
`configure-python-preserves` and `configure-ts-jest` (0 of 3 each), and on
the stale `archive-version-later`. Those drops come from the model behind
Claude Code's default having moved since the baseline (2026-10-15), not
from this change: points 1 and 2 above are withdrawn as this change's
regressions.

One difference remains, and it is this change's:

- **`configure-answer-no`: 1 of 3 here, 3 of 3 on `main`.** The case
  expects the reply to say "No changes made" when the author declines.
  `main`'s body says: "No changes made. Run `/compagnon-configure` again
  whenever you want to retry." — the three `main` runs say it word for
  word. This branch's body says: "No changes made. Say “configure
  Compagnon” again whenever you want to retry (shortcut:
  `/compagnon-configure`)." Two of three runs paraphrase it: « I made no
  changes. `_compagnon/config.yaml` still has its current content. »,
  « I didn't make any changes. ». The `next-steps` fragment tells the
  agent to say the quoted requests in the user's own words, which loosens
  the fixed sentence around them. The fix is in this change's scope (task
  2.2, the configure messages): keep "No changes made." as a fixed first
  sentence the fragment does not reword, as the archive and sync refusals
  already are ("reply with this sentence … and nothing more"). Applied below, on the user's request.

**Fixed.** The configure body now says: begin the reply with "No changes
made." word for word, then, alone on the last line, the request to retry
(test `configure_declined_says_no_changes_made_word_for_word`, which fails
on the earlier body). The configure cases run again on the fixed build
(`target/evals/plain-configure`, same settings): `configure-answer-no`
passes 3 of 3, and the comparison against `main` passes:

```
# Comparison with the baseline

- Verdict: **pass** (quality only: the consumption is reported, it does not decide)
- Model: Claude Code's default
- Review mode: inline
- Appended configuration: none
- Claude Code: baseline 2.1.289 (Claude Code), candidate 2.1.289 (Claude Code)
- Runs at the same time: baseline 4, candidate 4
- Cases compared: 5; repetitions: baseline 3, candidate 3

## Quality

The candidate's median must stay within the range of the baseline's repetitions.

| Role | Recall: baseline (min–max) | candidate | False positives: baseline (min–max) | candidate | Evidence validity: baseline (min–max) | candidate |
|---|---|---|---|---|---|---|

## `configure.enrich`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 0.40 | 0.60 |
| Checks passing | 0.91 | 0.94 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `configure-python-preserves` (1 of 3), `configure-ts-jest` (0 of 3).

## Quality improved

The candidate's median does better than the baseline's best repetition:

- configure.enrich: pass rate, from 0.40 at best to 0.60

## Consumption

Sums, over the compared cases, of each case's median over its repetitions. The candidate's cost must be lower than the baseline's floor, 0.8927 USD: each case at its cheapest run. The duration and the output are reported and never decide: the output carries the model's reasoning, which the quality measures guard.

| Measure | Baseline | Candidate | Change |
|---|---:|---:|---:|
| Cost (USD) | 0.9271 | 0.8781 | -5.3 % |
| Duration (s) | 109 | 94 | -13.8 % |
| API duration (s) | 105 | 90 | -14.2 % |
| Turns | 34 | 32 | -5.9 % |
| Permission denials | 7 | 6 | -14.3 % |
| Input tokens, uncached | 4999 | 4989 | -0.2 % |
| Tokens written to the cache | 60146 | 62749 | +4.3 % |
| Tokens read from the cache | 1081713 | 908056 | -16.1 % |
| Output tokens | 11178 | 9636 | -13.8 % |
| Calls to the model | 32 | 27 | -15.6 % |
| Input per call | 35843 | 36255 | +1.1 % |
| Output per call | 347 | 344 | -0.9 % |

## Where the tokens go

Tokens written to the cache, by source: per case the median over its repetitions, summed over the compared cases. Reported, never deciding: the cost decides.

| Source | Baseline | Candidate | Change |
|---|---:|---:|---:|
| prompt | 25816 | 27516 | +6.6 % |
| skill | 17227 | 19610 | +13.8 % |
| file | 5765 | 2620 | -54.6 % |
| compagnon status | 4183 | 5090 | +21.7 % |
| output | 2885 | 2618 | -9.3 % |
| edit | 2404 | 1933 | -19.6 % |
| shell | 1420 | 1989 | +40.1 % |

## Per case

| Case | Cost: baseline | candidate | Turns: baseline | candidate | Tokens read from the cache: baseline | candidate |
|---|---:|---:|---:|---:|---:|---:|
| configure-answer-no | 0.1282 | 0.1325 | 4 | 4 | 126246 | 128172 |
| configure-go-no-tests | 0.2237 | 0.1946 | 9 | 8 | 275247 | 168261 |
| configure-no-config | 0.1102 | 0.1163 | 4 | 4 | 125694 | 127641 |
| configure-python-preserves | 0.2418 | 0.2208 | 9 | 8 | 277606 | 241055 |
| configure-ts-jest | 0.2232 | 0.2139 | 8 | 8 | 276920 | 242927 |

## Not compared

Cases only one run holds: the cases of the other steps, not run here.

## Failures

None.
```

Against the committed baseline, the only case still under its share is
`configure-ts-jest` (0 of 3, baseline 1 of 5), which `main` fails alike.

`compare target/evals/main-small target/evals/plain-small --quality-only`:

```
# Comparison with the baseline

- Verdict: **fail** (quality only: the consumption is reported, it does not decide)
- Model: Claude Code's default
- Review mode: inline
- Appended configuration: none
- Claude Code: baseline 2.1.289 (Claude Code), candidate 2.1.289 (Claude Code)
- Runs at the same time: baseline 4, candidate 4
- Cases compared: 11; repetitions: baseline 3, candidate 3

## Quality

The candidate's median must stay within the range of the baseline's repetitions.

| Role | Recall: baseline (min–max) | candidate | False positives: baseline (min–max) | candidate | Evidence validity: baseline (min–max) | candidate |
|---|---|---|---|---|---|---|

## `change.archive`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 0.80 | 0.80 |
| Checks passing | 0.89 | 0.89 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `archive-unchecked-tasks` (0 of 3).

## `configure.enrich`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 0.40 | 0.20 |
| Checks passing | 0.91 | 0.88 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `configure-answer-no` (1 of 3), `configure-go-no-tests` (0 of 3), `configure-python-preserves` (0 of 3), `configure-ts-jest` (0 of 3).

## Consumption

Sums, over the compared cases, of each case's median over its repetitions. The candidate's cost must be lower than the baseline's floor, 1.4934 USD: each case at its cheapest run. The duration and the output are reported and never decide: the output carries the model's reasoning, which the quality measures guard.

| Measure | Baseline | Candidate | Change |
|---|---:|---:|---:|
| Cost (USD) | 1.5314 | 1.8563 | +21.2 % |
| Duration (s) | 150 | 147 | -2.4 % |
| API duration (s) | 141 | 135 | -4.6 % |
| Turns | 56 | 55 | -1.8 % |
| Permission denials | 10 | 9 | -10.0 % |
| Input tokens, uncached | 9944 | 9946 | +0.0 % |
| Tokens written to the cache | 109750 | 151097 | +37.7 % |
| Tokens read from the cache | 1771093 | 1894346 | +7.0 % |
| Output tokens | 14436 | 13544 | -6.2 % |
| Calls to the model | 54 | 55 | +1.9 % |
| Input per call | 35017 | 37370 | +6.7 % |
| Output per call | 266 | 253 | -4.9 % |

## Where the tokens go

Tokens written to the cache, by source: per case the median over its repetitions, summed over the compared cases. Reported, never deciding: the cost decides.

| Source | Baseline | Candidate | Change |
|---|---:|---:|---:|
| prompt | 56895 | 60238 | +5.9 % |
| skill | 27907 | 29185 | +4.6 % |
| file | 8297 | 45461 | +447.9 % |
| compagnon status | 7518 | 7852 | +4.4 % |
| output | 3020 | 2898 | -4.0 % |
| edit | 2404 | 2116 | -12.0 % |
| compagnon archive | 1864 | 1807 | -3.1 % |
| shell | 1420 | 1288 | -9.3 % |

## Per case

| Case | Cost: baseline | candidate | Turns: baseline | candidate | Tokens read from the cache: baseline | candidate |
|---|---:|---:|---:|---:|---:|---:|
| archive-merges-and-moves | 0.1249 | 0.2937 | 5 | 6 | 158605 | 246329 |
| archive-planning-incomplete | 0.1017 | 0.2576 | 3 | 4 | 90008 | 141261 |
| archive-review-blocked | 0.1204 | 0.1285 | 5 | 5 | 158512 | 161315 |
| archive-unchecked-tasks | 0.1385 | 0.1450 | 4 | 4 | 124211 | 126189 |
| archive-validation-failed | 0.1188 | 0.1267 | 5 | 5 | 158044 | 160623 |
| archive-version-later | — | — | — | — | — | — |
| configure-answer-no | 0.1282 | 0.1326 | 4 | 4 | 126246 | 128139 |
| configure-go-no-tests | 0.2237 | 0.2007 | 9 | 7 | 275247 | 240645 |
| configure-no-config | 0.1102 | 0.1177 | 4 | 4 | 125694 | 127615 |
| configure-python-preserves | 0.2418 | 0.2243 | 9 | 8 | 277606 | 280349 |
| configure-ts-jest | 0.2232 | 0.2295 | 8 | 8 | 276920 | 281881 |

## Not compared

Cases only one run holds: the cases of the other steps, not run here.

## Failures

- baseline: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- baseline: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- baseline: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- configure.enrich: pass rate 0.20 is under the baseline's minimum 0.40
- configure.enrich: case `configure-answer-no` passes in 0.33 of its runs, under its baseline share 1.00
- (not deciding, `--quality-only`) the consumption cannot be compared: case `archive-version-later` has no cost in the baseline; case `archive-version-later` has no cost in the candidate
fail: baseline: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: baseline: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: baseline: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: configure.enrich: pass rate 0.20 is under the baseline's minimum 0.40
fail: configure.enrich: case `configure-answer-no` passes in 0.33 of its runs, under its baseline share 1.00
```

The other steps hold: `plan.update` 1.00 (baseline 1.00), `change.sync`
1.00 (baseline 1.00, range 0.75–1.00). The cost falls by 21 % over the
compared cases.

### `compare` output

```
# Comparison with the baseline

- Verdict: **fail** (quality only: the consumption is reported, it does not decide)
- Model: Claude Code's default
- Review mode: inline
- Appended configuration: none
- Claude Code: baseline 2.1.289 (Claude Code), candidate 2.1.289 (Claude Code)
- Runs at the same time: baseline 4, candidate 4
- Cases compared: 21; repetitions: baseline 5, candidate 3

## Quality

The candidate's median must stay within the range of the baseline's repetitions.

| Role | Recall: baseline (min–max) | candidate | False positives: baseline (min–max) | candidate | Evidence validity: baseline (min–max) | candidate |
|---|---|---|---|---|---|---|

## `plan.update`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 | 1.00 |
| Checks passing | 1.00 | 1.00 |
| Runs modifying a file outside the step | 0 | 0 |

## `change.sync`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 (0.75–1.00) | 1.00 |
| Checks passing | 1.00 (0.92–1.00) | 1.00 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `sync-merges` (2 of 3).

## `change.archive`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 (0.83–1.00) | 0.80 |
| Checks passing | 1.00 (0.91–1.00) | 0.89 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `archive-unchecked-tasks` (0 of 3).

## `configure.enrich`

The candidate's share of runs passing must not fall under the baseline's minimum, nor its runs modifying a file outside the step rise above its maximum, nor any case's share of runs passing fall under its share in the baseline; the share of checks passing is reported and never decides.

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 0.40 (0.20–0.60) | 0.20 |
| Checks passing | 0.91 (0.71–0.94) | 0.88 |
| Runs modifying a file outside the step | 0 | 0 |

Cases not passing in all their candidate runs: `configure-answer-no` (1 of 3), `configure-go-no-tests` (0 of 3), `configure-python-preserves` (0 of 3), `configure-ts-jest` (0 of 3).

## Consumption

Sums, over the compared cases, of each case's median over its repetitions. The candidate's cost must be lower than the baseline's floor, 4.7906 USD: each case at its cheapest run. The duration and the output are reported and never decide: the output carries the model's reasoning, which the quality measures guard.

| Measure | Baseline | Candidate | Change |
|---|---:|---:|---:|
| Cost (USD) | 5.1150 | 4.0338 | -21.1 % |
| Duration (s) | 428 | 307 | -28.2 % |
| API duration (s) | 440 | 284 | -35.4 % |
| Turns | 135 | 118 | -12.6 % |
| Permission denials | 20 | 19 | -5.0 % |
| Input tokens, uncached | 21477 | 19596 | -8.8 % |
| Tokens written to the cache | 445019 | 334987 | -24.7 % |
| Tokens read from the cache | 3990307 | 3843362 | -3.7 % |
| Output tokens | 35139 | 29666 | -15.6 % |
| Calls to the model | 113 | 107 | -5.3 % |
| Input per call | 39535 | 39240 | -0.7 % |
| Output per call | 315 | 291 | -7.6 % |

## Where the tokens go

Tokens written to the cache, by source: per case the median over its repetitions, summed over the compared cases. Reported, never deciding: the cost decides.

| Source | Baseline | Candidate | Change |
|---|---:|---:|---:|
| prompt | 169076 | 119635 | -29.2 % |
| skill | 150222 | 73063 | -51.4 % |
| compagnon status | 31412 | 9814 | -68.8 % |
| file | 26248 | 93887 | +257.7 % |
| compagnon list | 26001 | 18077 | -30.5 % |
| edit | 6389 | 4862 | -23.9 % |
| search | 4741 | 1678 | -64.6 % |
| output | 3206 | 3735 | +16.5 % |
| compagnon archive | 3084 | 1807 | -41.4 % |
| compagnon validate | 3021 | 3569 | +18.1 % |
| shell | 2718 | 1809 | -33.4 % |
| compagnon sync | 1943 | 1073 | -44.8 % |

## Per case

| Case | Cost: baseline | candidate | Turns: baseline | candidate | Tokens read from the cache: baseline | candidate |
|---|---:|---:|---:|---:|---:|---:|
| archive-merges-and-moves | 0.1980 | 0.2937 | 5 | 6 | 174136 | 246329 |
| archive-planning-incomplete | 0.1712 | 0.2576 | 3 | 4 | 89571 | 141261 |
| archive-review-blocked | 0.1891 | 0.1285 | 5 | 5 | 164187 | 161315 |
| archive-unchecked-tasks | 0.2082 | 0.1450 | 5 | 4 | 164471 | 126189 |
| archive-validation-failed | 0.1846 | 0.1267 | 5 | 5 | 167116 | 160623 |
| archive-version-later | 0.1924 | — | 5 | — | 163955 | — |
| configure-answer-no | 0.2082 | 0.1326 | 4 | 4 | 125725 | 128139 |
| configure-go-no-tests | 0.2878 | 0.2007 | 8 | 7 | 258828 | 240645 |
| configure-no-config | 0.1898 | 0.1177 | 5 | 4 | 129158 | 127615 |
| configure-python-preserves | 0.2990 | 0.2243 | 8 | 8 | 256342 | 280349 |
| configure-ts-jest | 0.3253 | 0.2295 | 10 | 8 | 335934 | 281881 |
| sync-merges | 0.2205 | 0.1260 | 6 | 5 | 175996 | 160154 |
| sync-planning-incomplete | 0.1836 | 0.2562 | 4 | 4 | 90778 | 141067 |
| sync-validation-failed | 0.1896 | 0.1255 | 5 | 5 | 167032 | 160229 |
| sync-version-later | 0.1974 | — | 5 | — | 163898 | — |
| update-design-decision | 0.3300 | 0.2932 | 10 | 11 | 252767 | 296770 |
| update-proposal-ripple | 0.4326 | 0.4651 | 13 | 12 | 278859 | 379888 |
| update-refuse-missing | 0.1658 | 0.1743 | 3 | 3 | 89399 | 99036 |
| update-specs-ripple-tasks | 0.4066 | 0.3011 | 11 | 9 | 376897 | 322596 |
| update-specs-scenario | 0.2599 | 0.1973 | 8 | 6 | 183562 | 174493 |
| update-tasks-add | 0.2754 | 0.2388 | 7 | 8 | 181696 | 214783 |

## Not compared

Cases only one run holds: the review, plan.write and code.implement cases, not run here.

## Failures

- candidate: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `sync-version-later` (repetition 1) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `sync-version-later` (repetition 2) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- candidate: case `sync-version-later` (repetition 3) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
- change.archive: pass rate 0.80 is under the baseline's minimum 0.83
- change.archive: case `archive-unchecked-tasks` passes in 0.00 of its runs, under its baseline share 0.60
- configure.enrich: case `configure-go-no-tests` passes in 0.00 of its runs, under its baseline share 0.60
- configure.enrich: case `configure-python-preserves` passes in 0.00 of its runs, under its baseline share 0.20
- configure.enrich: case `configure-ts-jest` passes in 0.00 of its runs, under its baseline share 0.20
- (not deciding, `--quality-only`) the consumption cannot be compared: case `archive-version-later` has no cost in the candidate; case `sync-version-later` has no cost in the candidate
fail: candidate: case `archive-version-later` (repetition 1) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `archive-version-later` (repetition 2) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `archive-version-later` (repetition 3) stopped: case `archive-version-later`: `compagnon archive` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `sync-version-later` (repetition 1) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `sync-version-later` (repetition 2) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: candidate: case `sync-version-later` (repetition 3) stopped: case `sync-version-later`: `compagnon sync` exits with 1 and the code `compagnon_version_unsatisfied` on the fixture, where the case expects it done; the case no longer describes what compagnon does
fail: change.archive: pass rate 0.80 is under the baseline's minimum 0.83
fail: change.archive: case `archive-unchecked-tasks` passes in 0.00 of its runs, under its baseline share 0.60
fail: configure.enrich: case `configure-go-no-tests` passes in 0.00 of its runs, under its baseline share 0.60
fail: configure.enrich: case `configure-python-preserves` passes in 0.00 of its runs, under its baseline share 0.20
fail: configure.enrich: case `configure-ts-jest` passes in 0.00 of its runs, under its baseline share 0.20
```


## 4.2 — Two replies in French

Scratch project: `compagnon init --yes --language fr` from this branch's
build, a shell script `salut.sh`, and two changes with `skip_specs: true`:

- `ajouter-salut`: proposal, design and tasks written, both tasks checked,
  the script committed on a branch, the plan review (`plan.challenge`)
  recorded;
- `ajouter-au-revoir`: only the proposal written.

Each was asked in French with `claude -p` (Claude Code 2.1.332, the
project's skills as `compagnon init` installs them).

### Tasks done — « Réalise la modification ajouter-salut. »

> La modification `ajouter-salut` est terminée et prête à être archivée.
>
> - **Tâches :** les 2 tâches étaient déjà cochées à mon arrivée, implémentées dans le commit `708ed66`. Je n'ai donc rien écrit dans le code. J'ai relancé les deux vérifications prévues et elles passent : `./salut.sh Ana` affiche « Salut, Ana ! » et `./salut.sh` affiche « Salut, monde ! ».
> - **Fichier touché :** `salut.sh` seulement, 2 lignes ajoutées.
> - **Tests :** le projet n'a pas de suite de tests automatisés. Seules les deux commandes ci-dessus ont été exécutées. `compagnon validate` ne signale aucun problème.
> - **Corrections pendant la relecture :** aucune. Aucun domaine n'est activé, donc aucun expert n'a relu.
> - **Points de relecture :** rien à signaler. Le relecteur « nouvel arrivant » trouve le script clair.
> - **À décider :** rien.
>
> L'archivage était bloqué parce que le registre de relecture n'avait pas de section pour la relecture du code. Je l'ai ajoutée dans `_compagnon/changes/ajouter-salut/review.md`, et l'archivage est maintenant possible. Ce fichier est modifié mais pas encore commité.
>
> Une fois que vous aurez relu la modification, dites :
>
> « archive la modification ajouter-salut » (raccourci : `/compagnon-archive`, ou `compagnon archive` dans un terminal)

### Plan incomplete — « Réalise la modification ajouter-au-revoir. »

> Je n'ai rien implémenté : le plan de la modification `ajouter-au-revoir` n'est pas terminé (`isPlanningComplete: false`).
>
> État des documents de planification :
> - **proposal.md** : rédigée
> - **specs** : ignorées (pas de spécification pour cette modification)
> - **design.md** : pas encore rédigée, c'est ce qui bloque
> - **tasks.md** : bloquée tant que la conception n'existe pas. Le fichier n'existe pas encore, donc il n'y a aucune tâche à réaliser.
>
> Il faut d'abord terminer le plan, en rédigeant la conception puis les tâches. Vous pouvez aussi rédiger ces fichiers à la main dans `_compagnon/changes/ajouter-au-revoir/`.
>
> « termine le plan de ajouter-au-revoir » (raccourci : `/compagnon-propose`)

### Checked

| | Tasks done | Plan incomplete |
|---|---|---|
| Reply in French | yes | yes |
| Lexicon: « modification », never « change » | yes | yes |
| One request alone on the last line | yes | yes |
| Shortcut after the request | `/compagnon-archive` | `/compagnon-propose` |

Two remarks:

- The archive line adds a second shortcut, `compagnon archive` in a
  terminal. It is still one request; the rule does not forbid it.
- A first attempt, before the scratch change had its plan review
  recorded, ended on « Fais la revue du plan de ajouter-salut »
  (raccourci : `/compagnon-update`). The request was right for that state,
  but the plan review belongs to the planning step, whose shortcut is
  `/compagnon-propose`. The scratch project was missing a record any real
  change has, so this is noted, not counted.
