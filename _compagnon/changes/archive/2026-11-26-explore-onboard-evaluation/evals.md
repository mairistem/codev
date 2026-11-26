# Evaluation: explore-onboard-evaluation

Tasks 3.1 and 3.2. The user agreed beforehand to the cost: a trial run of
the nine new cases once (about 3 USD), then the baseline of five
repetitions (15 to 25 USD), with a hard cap of 30 USD.

Run on Claude Code 2.1.289, the version `.github/workflows/evals.yml` pins
and the baseline was recorded with (the installed 2.1.332 refuses the
runner's sandbox settings), passed with `--claude`; Compagnon built from
this branch (`compagnon 0.7.0`, `target/debug/compagnon`).

## 3.1 — Trial run

```bash
cargo run -p compagnon-evals -- run --claude <2.1.289> --step idea.explore --step project.onboard --repeat 1 --jobs 4 --out target/evals/eo-trial
```

Cost: 1.77 USD (explore 0.67, onboard 1.10). Every run touched no file
and created no change.

| Case | First scoring | Failed check | Final message |
|---|---|---|---|
| `explore-crystallizes-overdue-csv` | 5 of 5, passes | — | "Outcome: **it crystallizes**", three points, then "I can create the change — ask me for `/compagnon-propose`." |
| `explore-missing-sms-reminders` | 4 of 5, fails | absent "/compagnon-propose" | "outcome 2: information is missing": the provider (Claire Dubois, operations), the hour, consent; ends "Once points 1 and 2 are answered, this should be ready for `/compagnon-propose`". |
| `explore-not-worth-gateway-retry` | 3 of 4, fails | absent "/compagnon-propose" | "This one wasn't worth pursuing": names the spec and `internal/gateway/retry.go`; notes a separate idempotency gap that "could be its own exploration, and a change through `/compagnon-propose` if needed". |
| `onboard-configured` | 7 of 7, passes | — | read `README.md`, then `/compagnon-propose <an-idea>` (or `/compagnon-explore`). |
| `onboard-no-compagnon` | 5 of 5, passes | — | run `compagnon init`, then `/compagnon-onboard` again. |
| `onboard-planning-complete` | 6 of 6, passes | — | `/compagnon-apply add-ingredient-search`. |
| `onboard-planning-incomplete` | 6 of 6, passes | — | `/compagnon-propose add-due-dates` to finish planning. |
| `onboard-several-changes` | 5 of 5, passes | — | lists both changes, "Pick the one to work on", each with `/compagnon-propose <name>`. |
| `onboard-thin-config` | 5 of 5, passes | — | `/compagnon-configure` first, then `/compagnon-propose`. |

### Adjustments

Both failing explore runs are good replies: they conclude on their own
outcome, and name `/compagnon-propose` only as the step that comes after
(once the information is given, or for a separate topic). The command is
not a mark of the crystallizes outcome.

After the trial, the `absent` texts became "I can create the change" (the
offer the skill prescribes) in place of "/compagnon-propose". The
baseline (3.2) then showed six more good replies failing an `absent`
text, all naming the step after the recommended one; their texts were
adjusted again and the baseline scored again, not recorded again
(`CONTRIBUTING.md`: a changed expectation of an explore or onboard case
is scored again):

- `explore-missing-sms-reminders` run 2 ends "Once the provider is
  confirmed, come back with the answers and I can create the change with
  `/compagnon-propose`." The crystallizes mark is now the outcome's name,
  "crystallizes": every crystallizes reply (trial and baseline, 6 of 6)
  says it, no missing or not-worth reply (12) does.
- `onboard-planning-incomplete` runs 2 and 3 end "When all 4 artifacts
  are written, `/compagnon-apply add-due-dates` will implement it", after
  recommending `/compagnon-propose add-due-dates`. `/compagnon-apply` left
  `absent`; in exchange `contains` holds the skill's recommendation for
  this state whole, "/compagnon-propose add-due-dates" (in all 6 replies),
  so a reply recommending `/compagnon-apply add-due-dates` instead still
  fails; `absent` takes the other states' marks, "/compagnon-configure"
  and "compagnon init".
- `onboard-several-changes` runs 2, 4 and 5 end "Once a change's planning
  is complete, implement it with `/compagnon-apply <name>`." The wrong
  conclusion here is picking one change to implement: `absent` is now
  "/compagnon-apply add-bookmark-tags" and "/compagnon-apply
  import-browser-bookmarks".

Final texts (`contains` unchanged but for `onboard-planning-incomplete`):

| Case | `contains` | `absent` |
|---|---|---|
| `explore-crystallizes-overdue-csv` | "/compagnon-propose" | "information is missing", "not worth pursuing", "wasn't worth pursuing" |
| `explore-missing-sms-reminders` | "provider" | "crystallizes", "not worth pursuing", "wasn't worth pursuing" |
| `explore-not-worth-gateway-retry` | "retry.go" | "crystallizes", "information is missing" |
| `onboard-planning-incomplete` | "/compagnon-propose add-due-dates" | "/compagnon-configure", "compagnon init" |
| `onboard-several-changes` | "add-bookmark-tags", "import-browser-bookmarks" | "/compagnon-apply add-bookmark-tags", "/compagnon-apply import-browser-bookmarks" |

"wasn't worth pursuing" was added to the crystallizes case because the
not-worth trial reply says it that way, and without it that reply passed
every `absent` check of the crystallizes case. The other onboard cases are
unchanged. Scored again with the final texts
(`cargo run -p compagnon-evals -- score target/evals/eo-trial`): the nine
trial runs pass.

### Cross-scoring

Each trial observation scored against every other case of its step (its
`case` field rewritten, `compagnon-evals score` on the copies,
`target/evals/eo-cross`), with the final texts, to check that a reply
concluding on another outcome or state fails: 6 explore pairs, 30 onboard
pairs.

| Case scored | Trial observation of | Checks | Failed checks |
|---|---|---|---|
| `explore-crystallizes-overdue-csv` | `explore-missing-sms-reminders` | 5 of 6 | does not say "information is missing" |
| `explore-crystallizes-overdue-csv` | `explore-not-worth-gateway-retry` | 5 of 6 | does not say "wasn't worth pursuing" |
| `explore-missing-sms-reminders` | `explore-crystallizes-overdue-csv` | 4 of 6 | says "provider"; does not say "crystallizes" |
| `explore-missing-sms-reminders` | `explore-not-worth-gateway-retry` | 4 of 6 | says "provider"; does not say "wasn't worth pursuing" |
| `explore-not-worth-gateway-retry` | `explore-crystallizes-overdue-csv` | 3 of 5 | says "retry.go"; does not say "crystallizes" |
| `explore-not-worth-gateway-retry` | `explore-missing-sms-reminders` | 3 of 5 | says "retry.go"; does not say "information is missing" |
| `onboard-configured` | `onboard-no-compagnon` | 4 of 7 | says "README.md"; says "/compagnon-propose"; does not say "compagnon init" |
| `onboard-configured` | `onboard-planning-complete` | 4 of 7 | says "README.md"; says "/compagnon-propose"; does not say "/compagnon-apply" |
| `onboard-configured` | `onboard-planning-incomplete` | 6 of 7 | says "README.md" |
| `onboard-configured` | `onboard-several-changes` | 6 of 7 | says "README.md" |
| `onboard-configured` | `onboard-thin-config` | 5 of 7 | says "README.md"; does not say "/compagnon-configure" |
| `onboard-no-compagnon` | `onboard-configured` | 4 of 5 | says "compagnon init" |
| `onboard-no-compagnon` | `onboard-planning-complete` | 3 of 5 | says "compagnon init"; does not say "/compagnon-apply" |
| `onboard-no-compagnon` | `onboard-planning-incomplete` | 4 of 5 | says "compagnon init" |
| `onboard-no-compagnon` | `onboard-several-changes` | 4 of 5 | says "compagnon init" |
| `onboard-no-compagnon` | `onboard-thin-config` | 3 of 5 | says "compagnon init"; does not say "/compagnon-configure" |
| `onboard-planning-complete` | `onboard-configured` | 3 of 6 | says "/compagnon-apply"; says "add-ingredient-search"; does not say "/compagnon-propose" |
| `onboard-planning-complete` | `onboard-no-compagnon` | 4 of 6 | says "/compagnon-apply"; says "add-ingredient-search" |
| `onboard-planning-complete` | `onboard-planning-incomplete` | 3 of 6 | says "/compagnon-apply"; says "add-ingredient-search"; does not say "/compagnon-propose" |
| `onboard-planning-complete` | `onboard-several-changes` | 3 of 6 | says "/compagnon-apply"; says "add-ingredient-search"; does not say "/compagnon-propose" |
| `onboard-planning-complete` | `onboard-thin-config` | 2 of 6 | says "/compagnon-apply"; says "add-ingredient-search"; does not say "/compagnon-propose"; does not say "/compagnon-configure" |
| `onboard-planning-incomplete` | `onboard-configured` | 4 of 5 | says "/compagnon-propose add-due-dates" |
| `onboard-planning-incomplete` | `onboard-no-compagnon` | 3 of 5 | says "/compagnon-propose add-due-dates"; does not say "compagnon init" |
| `onboard-planning-incomplete` | `onboard-planning-complete` | 4 of 5 | says "/compagnon-propose add-due-dates" |
| `onboard-planning-incomplete` | `onboard-several-changes` | 4 of 5 | says "/compagnon-propose add-due-dates" |
| `onboard-planning-incomplete` | `onboard-thin-config` | 3 of 5 | says "/compagnon-propose add-due-dates"; does not say "/compagnon-configure" |
| `onboard-several-changes` | `onboard-configured` | 4 of 6 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-no-compagnon` | 4 of 6 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-planning-complete` | 4 of 6 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-planning-incomplete` | 4 of 6 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-thin-config` | 4 of 6 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-thin-config` | `onboard-configured` | 4 of 5 | says "/compagnon-configure" |
| `onboard-thin-config` | `onboard-no-compagnon` | 3 of 5 | says "/compagnon-configure"; does not say "compagnon init" |
| `onboard-thin-config` | `onboard-planning-complete` | 3 of 5 | says "/compagnon-configure"; does not say "/compagnon-apply" |
| `onboard-thin-config` | `onboard-planning-incomplete` | 4 of 5 | says "/compagnon-configure" |
| `onboard-thin-config` | `onboard-several-changes` | 4 of 5 | says "/compagnon-configure" |

36 of 36 cross-scored runs fail.

The crystallizes case is held only by its `absent` texts: the two other
outcomes' replies also name `/compagnon-propose`, so its `contains`
passes on them; "information is missing" fails the first, "wasn't worth
pursuing" the second. Between onboard states, the fixtures differ, so the
change names already fail most pairs; within a fixture, a wrong
recommendation is held by `contains` (the state's recommendation) and by
`absent` (the other states' commands).

## 3.2 — Baseline

```bash
cargo run -p compagnon-evals -- run --claude <2.1.289> --step idea.explore --step project.onboard --repeat 5 --jobs 4 --out evals/baseline
cargo run -p compagnon-evals -- score evals/baseline     # after the adjustments above
cargo run -p compagnon-evals -- docs
```

45 runs (9 cases × 5), none in error, none modifying a file or creating a
change; resumed into the existing baseline (its `run.json` unchanged:
Claude Code 2.1.289). Cost: 6.85 USD.

| Step | Cases | Runs passing | Checks passing | Runs modifying a file | Cost per suite run (USD) |
|---|---|---|---|---|---|
| `idea.explore` | 3 | 1.00 | 1.00 | 0 | 0.45 |
| `project.onboard` | 6 | 1.00 | 1.00 | 0 | 0.92 |

Before the second adjustment the same observations scored
`idea.explore` 1.00 (0.67–1.00) runs passing and `project.onboard` 0.83
(0.67–1.00): the six failures listed above, all good replies.

Median cost per run: explore 0.14 to 0.16 USD, onboard 0.11 to 0.17 USD —
under the 0.30 to 0.50 USD `CONTRIBUTING.md` estimates.

The whole baseline's verdict stays **fail**, as before this change (the
`skeptic` false positives and `plan-decision-yaml-lib`, untouched).

Total spent for 3.1 and 3.2: 1.77 + 6.85 = 8.62 USD.
