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
not a mark of the crystallizes outcome; the offer the skill prescribes for
it is ("I can create the change"). The `absent` texts became:

- `explore-crystallizes-overdue-csv`: `["information is missing", "not worth pursuing", "wasn't worth pursuing"]`
  — "wasn't worth pursuing" added: the not-worth trial reply says it that
  way, and without it that reply passed every `absent` check of this case
  (see the cross-scoring below).
- `explore-missing-sms-reminders`: `["I can create the change", "not worth pursuing", "wasn't worth pursuing"]`
  (was `["/compagnon-propose", "not worth pursuing"]`).
- `explore-not-worth-gateway-retry`: `["I can create the change", "information is missing"]`
  (was `["/compagnon-propose"]`).

`contains` unchanged everywhere; the onboard cases unchanged. Scored again
(`cargo run -p compagnon-evals -- score target/evals/eo-trial`): the nine
runs pass.

### Cross-scoring

Each trial observation scored against every other case of its step (its
`case` field rewritten, `compagnon-evals score` on the copies,
`target/evals/eo-cross`), to check that a reply concluding on another
outcome or state fails: 6 explore pairs, 30 onboard pairs.

| Case scored | Trial observation of | Checks | Failed checks |
|---|---|---|---|
| `explore-crystallizes-overdue-csv` | `explore-missing-sms-reminders` | 5 of 6 | does not say "information is missing" |
| `explore-crystallizes-overdue-csv` | `explore-not-worth-gateway-retry` | 5 of 6 | does not say "wasn't worth pursuing" |
| `explore-missing-sms-reminders` | `explore-crystallizes-overdue-csv` | 4 of 6 | says "provider"; does not say "I can create the change" |
| `explore-missing-sms-reminders` | `explore-not-worth-gateway-retry` | 4 of 6 | says "provider"; does not say "wasn't worth pursuing" |
| `explore-not-worth-gateway-retry` | `explore-crystallizes-overdue-csv` | 3 of 5 | says "retry.go"; does not say "I can create the change" |
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
| `onboard-planning-incomplete` | `onboard-configured` | 5 of 6 | says "add-due-dates" |
| `onboard-planning-incomplete` | `onboard-no-compagnon` | 4 of 6 | says "/compagnon-propose"; says "add-due-dates" |
| `onboard-planning-incomplete` | `onboard-planning-complete` | 3 of 6 | says "/compagnon-propose"; says "add-due-dates"; does not say "/compagnon-apply" |
| `onboard-planning-incomplete` | `onboard-several-changes` | 5 of 6 | says "add-due-dates" |
| `onboard-planning-incomplete` | `onboard-thin-config` | 4 of 6 | says "add-due-dates"; does not say "/compagnon-configure" |
| `onboard-several-changes` | `onboard-configured` | 3 of 5 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-no-compagnon` | 3 of 5 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-planning-complete` | 2 of 5 | says "add-bookmark-tags"; says "import-browser-bookmarks"; does not say "/compagnon-apply" |
| `onboard-several-changes` | `onboard-planning-incomplete` | 3 of 5 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-several-changes` | `onboard-thin-config` | 3 of 5 | says "add-bookmark-tags"; says "import-browser-bookmarks" |
| `onboard-thin-config` | `onboard-configured` | 4 of 5 | says "/compagnon-configure" |
| `onboard-thin-config` | `onboard-no-compagnon` | 3 of 5 | says "/compagnon-configure"; does not say "compagnon init" |
| `onboard-thin-config` | `onboard-planning-complete` | 3 of 5 | says "/compagnon-configure"; does not say "/compagnon-apply" |
| `onboard-thin-config` | `onboard-planning-incomplete` | 4 of 5 | says "/compagnon-configure" |
| `onboard-thin-config` | `onboard-several-changes` | 4 of 5 | says "/compagnon-configure" |

36 of 36 cross-scored runs fail.

The crystallizes case is held only by its `absent` texts: the two other
outcomes' replies also name `/compagnon-propose`, so its `contains`
passes on them; "information is missing" fails the first, "wasn't worth
pursuing" the second.
