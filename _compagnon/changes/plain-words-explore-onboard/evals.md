# Evaluation: plain-words-explore-onboard (task 3.1)

Candidate recorded with Claude Code 2.1.289, the compagnon built from this
branch (`target/debug/compagnon`), five repetitions of the nine
`idea.explore` and `project.onboard` cases, four at a time, as the baseline.
The baseline, `evals/baseline/`, was not re-recorded.

## Commands

```bash
npm install --prefix /tmp/cc289 @anthropic-ai/claude-code@2.1.289
cargo build -p compagnon-cli
git fetch --unshallow origin   # the runner needs the full history
cargo run -p compagnon-evals -- run --claude /tmp/cc289/node_modules/.bin/claude \
  --step idea.explore --step project.onboard --repeat 5 --jobs 4 --out target/evals/pweo
cargo run -p compagnon-evals -- compare evals/baseline target/evals/pweo --quality-only
cargo run -p compagnon-evals -- compare evals/baseline target/evals/pweo
```

45 runs, none in error. The whole run cost **7.19 USD** (the baseline's
45 runs: 6.85 USD).

## Quality (`--quality-only`): pass

```
- Verdict: **pass** (quality only: the consumption is reported, it does not decide)
- Claude Code: baseline 2.1.289 (Claude Code), candidate 2.1.289 (Claude Code)
- Runs at the same time: baseline 4, candidate 4
- Cases compared: 9; repetitions: baseline 5, candidate 5

## `idea.explore`

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 | 1.00 |
| Checks passing | 1.00 | 1.00 |
| Runs modifying a file outside the step | 0 | 0 |

Every case passes in all its candidate runs.

## `project.onboard`

| Measure | Baseline (min–max) | Candidate |
|---|---|---|
| Runs passing | 1.00 | 1.00 |
| Checks passing | 1.00 | 1.00 |
| Runs modifying a file outside the step | 0 | 0 |

Every case passes in all its candidate runs.

## Failures

None.
- (not deciding, `--quality-only`) cost 1.4688 USD is not lower than the baseline's floor, 1.2906 USD (each case at its cheapest run; the baseline's median is 1.3680 USD)
```

## Cost (without `--quality-only`): fail, not deciding for this change

This change is not meant to make the skills consume less, so the cost does
not decide (CONTRIBUTING: `--quality-only` for a change of what a skill
does). It is reported: +7.4 % on the sum of the medians, mostly the longer
skill texts (+32 % of the tokens the skill writes to the cache).

```
- Verdict: **fail**

| Measure | Baseline | Candidate | Change |
|---|---:|---:|---:|
| Cost (USD) | 1.3680 | 1.4688 | +7.4 % |
| Duration (s) | 139 | 139 | +0.4 % |
| API duration (s) | 130 | 132 | +1.5 % |
| Turns | 70 | 72 | +2.9 % |
| Permission denials | 7 | 8 | +14.3 % |
| Input tokens, uncached | 8817 | 8819 | +0.0 % |
| Tokens written to the cache | 105535 | 114603 | +8.6 % |
| Tokens read from the cache | 1316985 | 1381878 | +4.9 % |
| Output tokens | 13047 | 13519 | +3.6 % |
| Calls to the model | 41 | 42 | +2.4 % |
| Input per call | 34916 | 35845 | +2.7 % |
| Output per call | 312 | 331 | +6.1 % |

| Source | Baseline | Candidate | Change |
|---|---:|---:|---:|
| prompt | 58130 | 59201 | +1.8 % |
| skill | 24446 | 32287 | +32.1 % |
| compagnon doctor | 8184 | 8308 | +1.5 % |
| file | 8058 | 5140 | -36.2 % |
| compagnon list | 3903 | 5676 | +45.4 % |
| compagnon status | 1069 | 1168 | +9.3 % |
| search | 536 | 621 | +15.9 % |
| shell | 448 | 1300 | +190.2 % |
| compagnon decision | 179 | 182 | +1.7 % |

| Case | Cost: baseline | candidate | Turns: baseline | candidate |
|---|---:|---:|---:|---:|
| explore-crystallizes-overdue-csv | 0.1490 | 0.1546 | 4 | 4 |
| explore-missing-sms-reminders | 0.1568 | 0.1641 | 5 | 5 |
| explore-not-worth-gateway-retry | 0.1447 | 0.1545 | 4 | 5 |
| onboard-configured | 0.1616 | 0.1707 | 10 | 10 |
| onboard-no-compagnon | 0.1128 | 0.1233 | 4 | 4 |
| onboard-planning-complete | 0.1652 | 0.1695 | 11 | 11 |
| onboard-planning-incomplete | 0.1595 | 0.1761 | 10 | 11 |
| onboard-several-changes | 0.1652 | 0.1830 | 12 | 12 |
| onboard-thin-config | 0.1532 | 0.1730 | 10 | 10 |

fail: cost 1.4688 USD is not lower than the baseline's floor, 1.2906 USD (each case at its cheapest run; the baseline's median is 1.3680 USD)
```

## The last lines read

`onboard-several-changes`, the last line of each of the five replies:

| Run | Last line |
|---|---|
| 1 | Which one do you want to continue? |
| 2 | Which one do you want to continue? |
| 3 | Which one do you want to continue? |
| 4 | Which one do you want to continue? |
| 5 | Which one do you want to continue? |

A plain question, with no shortcut on it. Above it, run 1 lists each change
with a plain request first and the command as its shortcut:

> - `add-bookmark-tags`, plan not finished (1/4 artifacts): say “finish the plan of add-bookmark-tags” (shortcut: `/compagnon-propose add-bookmark-tags`)
> - `import-browser-bookmarks`, plan not finished (1/4 artifacts): say “finish the plan of import-browser-bookmarks” (shortcut: `/compagnon-propose import-browser-bookmarks`)
>
> Which one do you want to continue?

In the baseline, the same replies ended on a bare list of
`/compagnon-propose <name>` commands, or on "Pick the one you want to work
on first. Once a change's planning is complete, implement it with
`/compagnon-apply <name>`."

Other cases, run 1:

- `onboard-planning-complete`: “Say “implement add-ingredient-search” (shortcut: `/compagnon-apply add-ingredient-search`)” — plain request first, command as the shortcut.
- `onboard-thin-config`: “For now, say “configure Compagnon for this project” (shortcut: `/compagnon-configure`).” — the same. Its relayed doctor fixes keep their commands after a plain gloss (“your project's rules are not written yet (`config.rules`) — fix: `/compagnon-configure`”), as the exemption for doctor's fix lines allows.
- `explore-crystallizes-overdue-csv`: “I can plan this change — say “plan this change” (shortcut: `/compagnon-propose`).” — the same.

## Failures

None: every run of every case passes its checks.
