# Role evaluation report

- Date: 2026-10-15
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 65
- Repetitions: 5
- Cost: 55.07 USD for one run of the suite (sum of the cases' medians), 276.46 USD in all
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

5 runs per case: each measure is the median over the runs, its range in parentheses; compared with a baseline on the same model, not a proof.

## The suite

- Recall: 1.00 (39 of 39 seeded defects found)
- Evidence validity: 1.00 (69 of 69 checked quotes)
- False positives: 9 (8–11), 0.23 (0.20–0.28) per case

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 5 (3/2) | 5 | 5 | 0 | 0 | 2 | 3 | 0 | 1.00 | 1.00 (10/0/0/0) | 5/5 | 1.61 |
| migration | 5 (3/2) | 5 | 5 | 0 | 1 (0–2) | 0 | 1 | 0 | 1.00 | 1.00 (7/0/0/0) | 4/5 | 1.61 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 1.00 | 1.00 (2/0/0/0) | 2/2 | 0.79 |
| ops | 5 (3/2) | 6 | 6 | 0 | 1 | 2 | 1 | 0 | 1.00 | 1.00 (10/0/0/0) | 3/6 | 1.62 |
| performance | 5 (3/2) | 4 | 4 | 0 | 0 | 0 | 1 | 0 | 1.00 | 1.00 (5/0/0/0) | 2/4 | 1.49 |
| privacy | 6 (4/2) | 7 | 7 | 0 | 1 (0–2) | 8 | 2 | 0 | 1.00 | 1.00 (18/0/0/0) | 4/7 | 2.07 |
| security | 6 (4/2) | 6 | 6 | 0 | 2 (1–3) | 1 | 0 | 0 | 1.00 | 1.00 (10/0/0/0) | 6/6 | 1.92 |
| skeptic | 3 (2/1) | 3 | 3 | 0 | 4 (3–6) | 1 | 1 | 0 | 1.00 | 1.00 (9/0/0/0) | 2/3 | 1.20 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 (0–1) | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.58 |

## `plan.write`

Per repetition, every planning run pooled; then the median over the repetitions, with its range. No threshold: `codev-evals compare` judges these against the baseline.

| Cases | Primary recall | Collateral recall | Probable duplicates | Decisions ignored | Plans that do not validate | Cost (USD) |
|---|---|---|---|---|---|---|
| 25 | 0.69 (0.58–0.78) | 0.39 (0.28–0.44) | 14 (6–15) | 0 (0–1) | 1 (0–2) | 42.18 |

## Consumption

Per case, the median over its runs not in error; per role and for the suite, the sum of those medians. Tokens are summed over every model a run used.

| Role | Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| accessibility | 1.61 | 135 | 134 | 34 | 1 | 5842 | 139778 | 1157688 | 12963 | 29 | 44719 | 452 |
| migration | 1.61 | 126 | 126 | 37 | 5 | 5848 | 136692 | 1414018 | 11688 | 35 | 44356 | 323 |
| newcomer | 0.79 | 53 | 53 | 15 | 0 | 3504 | 74403 | 514409 | 4337 | 14 | 42297 | 310 |
| ops | 1.62 | 159 | 159 | 35 | 2 | 5845 | 138793 | 1099670 | 14221 | 28 | 44429 | 474 |
| performance | 1.49 | 116 | 116 | 35 | 3 | 5861 | 130893 | 1224144 | 10068 | 31 | 43866 | 329 |
| privacy | 2.07 | 200 | 199 | 42 | 3 | 7055 | 172353 | 1486494 | 18403 | 37 | 44882 | 489 |
| security | 1.92 | 154 | 153 | 46 | 2 | 7042 | 167082 | 1479147 | 13753 | 38 | 44545 | 365 |
| skeptic | 1.20 | 109 | 109 | 31 | 6 | 3488 | 95501 | 1069377 | 10733 | 25 | 46659 | 418 |
| tracer | 0.58 | 39 | 39 | 19 | 4 | 2180 | 53041 | 444499 | 3896 | 12 | 41555 | 344 |
| **Suite** | 55.07 | 7023 | 7631 | 1020 | 69 | 75658 | 3576753 | 62712329 | 723264 | 1025 | 63465 | 696 |

## Where the tokens go

The tokens written to the cache, by source: per case the median over its runs, summed over the cases; their share is their share of what those writes cost.

| Source | Tokens | Share |
|---|---:|---:|
| skill | 843692 | 23 % |
| file | 791139 | 21 % |
| prompt | 490778 | 13 % |
| edit | 372821 | 10 % |
| search | 338245 | 9 % |
| shell | 205076 | 6 % |
| output | 163212 | 4 % |
| codev instructions | 158702 | 4 % |
| codev roles | 100208 | 3 % |
| codev review | 95800 | 3 % |
| codev validate | 64896 | 2 % |
| codev new | 54593 | 2 % |
| agent | 22404 | 1 % |
| codev --help | 2347 | 0 % |
| codev list | 507 | 0 % |
| codev status | 295 | 0 % |

### `code.review`

| Source | Tokens | Share |
|---|---:|---:|
| skill | 354581 | 48 % |
| prompt | 207013 | 28 % |
| codev review | 83841 | 11 % |
| codev roles | 39027 | 5 % |
| edit | 34656 | 5 % |
| shell | 12732 | 2 % |
| file | 8560 | 1 % |
| output | 608 | 0 % |

### `plan.challenge`

| Source | Tokens | Share |
|---|---:|---:|
| skill | 125240 | 44 % |
| prompt | 73874 | 26 % |
| file | 41103 | 14 % |
| edit | 16369 | 6 % |
| codev review | 11959 | 4 % |
| shell | 8856 | 3 % |
| codev roles | 2604 | 1 % |
| search | 1962 | 1 % |
| output | 1176 | 0 % |

### `plan.trace`

| Source | Tokens | Share |
|---|---:|---:|
| skill | 27187 | 54 % |
| prompt | 14589 | 29 % |
| file | 3499 | 7 % |
| codev roles | 2480 | 5 % |
| edit | 1138 | 2 % |
| shell | 1014 | 2 % |
| output | 59 | 0 % |

### `plan.write`

| Source | Tokens | Share |
|---|---:|---:|
| file | 737977 | 28 % |
| skill | 336684 | 13 % |
| search | 336283 | 13 % |
| edit | 320658 | 12 % |
| prompt | 195302 | 7 % |
| shell | 182474 | 7 % |
| output | 161369 | 6 % |
| codev instructions | 158702 | 6 % |
| codev validate | 64896 | 2 % |
| codev roles | 56097 | 2 % |
| codev new | 54593 | 2 % |
| agent | 22404 | 1 % |
| codev --help | 2347 | 0 % |
| codev list | 507 | 0 % |
| codev status | 295 | 0 % |

## Failures

- skeptic: 4 false positive(s) exceed 2
- case `plan-decision-yaml-lib` (repetition 1) planned nothing: no change was created

## Per case

### accessibility-code-dropdown-keyboard-trap

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.34 · 28 · 28 · 7 · 0 · 1174 · 29007 · 242047 · 2688 · 6 · 45425 · 498

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 28 s
- Tool calls: 6, 4 before the first artifact
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps Tab and cannot be closed with Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces "collapsed": true positive
- Finding R-accessibility-3 [low / low] `src/components/SortMenu.tsx:33` Visible focus on the menu items is unverified: unreported (low confidence) (at the location of `expanded-state-hard-coded`, another category)

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab is captured and Escape does nothing: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces "collapsed", even when the menu is open: true positive

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Tool calls: 8, 3 before the first artifact
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab cycles inside it and Escape does not close it: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps Tab and cannot be dismissed with Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 28 s
- Tool calls: 7, 2 before the first artifact
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps Tab and cannot be closed with Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces "collapsed": true positive

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 26 · 26 · 6 · 0 · 1164 · 25640 · 232948 · 2631 · 6 · 43292 · 439

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Tool calls: 5, 3 before the first artifact
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt text: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Delete target size and focus style not verifiable: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 23 s
- Tool calls: 5, 3 before the first artifact
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable and inoperable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Delete target size and focus style not verifiable: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Tool calls: 5, 3 before the first artifact
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt text: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Focus visibility and target size of the icon button not verifiable: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 27 s
- Tool calls: 6, 4 before the first artifact
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-2, another severity
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: true positive
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: duplicate
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target size and focus style not visible in the change: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 34 s
- Tool calls: 6, 4 before the first artifact
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target may be smaller than 24 by 24 CSS pixels: duplicate

### accessibility-control-custom-listbox

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.34 · 34 · 33 · 7 · 0 · 1174 · 28376 · 242714 · 3035 · 6 · 45402 · 506

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 31 s
- Tool calls: 6, 4 before the first artifact
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Highlighted option relies on a stylesheet the change does not include: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 37 s
- Tool calls: 8, 3 before the first artifact
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:70-73` Focus indicator of the combobox cannot be checked from the change: unreported (low confidence)
- Finding R-accessibility-2 [low / low] `src/components/SizePicker.tsx:92` Highlighted option is shown only through a class whose style is not in the change: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Tool calls: 6, 4 before the first artifact
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option's visible indicator depends on an unshipped style: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.34 USD, 35 s
- Tool calls: 7, 4 before the first artifact
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option and focused combobox have no style in the change: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 33 s
- Tool calls: 5, 3 before the first artifact
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option and combobox focus rely on styles the change does not include: unreported (low confidence)
- Finding R-accessibility-2 [low / low] `src/components/SizePicker.tsx:87-95` Option touch target size is not set by the change: unreported (low confidence)

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 15 · 15 · 6 · 0 · 1170 · 26095 · 144661 · 1419 · 4 · 42982 · 319

#### Run 1

- Outcome: scored
- Cost: 0.25 USD, 13 s
- Tool calls: 4, 2 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Tool calls: 5, 3 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 15 s
- Tool calls: 5, 3 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 17 s
- Tool calls: 6, 4 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.25 USD, 14 s
- Tool calls: 4, 2 before the first artifact

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.37 · 32 · 32 · 8 · 1 · 1160 · 30660 · 295318 · 3190 · 7 · 45746 · 456

#### Run 1

- Outcome: scored
- Cost: 0.37 USD, 30 s
- Tool calls: 8, 5 before the first artifact
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are signalled by a red border only: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Removing the error summary leaves submit errors unannounced: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 36 s
- Tool calls: 8, 5 before the first artifact
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text: true positive
- Finding R-accessibility-2 [medium / high] `design.md#Decision: Invalid fields turn red` A failed submit is no longer announced and focus is not moved: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 31 s
- Tool calls: 6, 4 before the first artifact
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Invalid fields are identified by colour only, with no text message: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Removing the error summary leaves no feedback or focus target on submit: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 32 s
- Tool calls: 7, 5 before the first artifact
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text message: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Submitting an invalid form is no longer announced and focus is not moved: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 32 s
- Tool calls: 7, 4 before the first artifact
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Removing the error list leaves submit with no error identification or focus move: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 24 · 23 · 6 · 0 · 1174 · 25543 · 232619 · 2323 · 6 · 43172 · 387

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8-9` Public JSON field renamed user_name to userName with no BREAKING mark or compatibility: true positive

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Tool calls: 6, 4 before the first artifact
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8` JSON field user_name renamed userName with no BREAKING mark or transition: true positive

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Tool calls: 5, 3 before the first artifact
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account field renamed in the public API without a BREAKING mark or transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Tool calls: 6, 4 before the first artifact
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Public API field `user_name` renamed `userName` without a BREAKING mark or compatibility path: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` `DROP COLUMN legacy_email` is irreversible, with no down migration and no backup step: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 22 s
- Tool calls: 5, 3 before the first artifact
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account JSON field renamed with no BREAKING mark or transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup step: true positive

### migration-code-serde-rename-all

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 26 · 26 · 7 · 1 · 1176 · 27365 · 277216 · 2504 · 7 · 44370 · 326

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 27 s
- Tool calls: 6, 4 before the first artifact
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` `rename_all = "camelCase"` renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` New required `default_currency` key makes existing `config.toml` files fail to load: true positive

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Tool calls: 10, 5 before the first artifact
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-9` camelCase rename silently breaks every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config.toml files fail to load without `default_currency`: true positive

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 20 s
- Tool calls: 5, 3 before the first artifact
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6` rename_all = "camelCase" renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config.toml files without default_currency stop the service from starting: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Tool calls: 6, 4 before the first artifact
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6` camelCase renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:10` Existing config.toml without default_currency no longer loads: true positive

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 25 s
- Tool calls: 8, 5 before the first artifact
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` `rename_all = "camelCase"` renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Required `default_currency` makes existing `config.toml` files fail to load: true positive

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 19 · 20 · 8 · 2 · 1160 · 27823 · 283744 · 1722 · 7 · 44553 · 217

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 16 s
- Tool calls: 6, 4 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 18 s
- Tool calls: 7, 5 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 25 s
- Tool calls: 9, 7 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 19 s
- Tool calls: 7, 5 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 22 s
- Tool calls: 9, 7 before the first artifact

### migration-control-serde-alias

- Role: migration at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.33 · 29 · 29 · 8 · 0 · 1178 · 27406 · 327270 · 2399 · 8 · 44400 · 310

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 30 s
- Tool calls: 7, 5 before the first artifact
- Finding R-migration-1 [low / high] `src/config.rs:10-11` No automated test that an old config file with `listen` still loads: **false positive**
- Finding R-migration-2 [low / high] `src/api.rs:10-11` No automated test that the order JSON still says `customer_id`: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Tool calls: 7, 5 before the first artifact
- Finding R-migration-1 [medium / low] `src/config.rs:10-11` Public Rust fields renamed with no deprecation or BREAKING note: unreported (low confidence)
- Finding R-migration-2 [low / high] `tasks.md#1. Implementation` Compatibility paths verified only by a manual staging check, no test: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 28 s
- Tool calls: 7, 5 before the first artifact
- Finding R-migration-1 [medium / low] `src/config.rs:10-11` The compatibility paths for `listen` and `customer_id` have no automated test: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Tool calls: 5, 3 before the first artifact
- Finding R-migration-1 [low / high] `src/config.rs:10-11` No automated test pins the `listen` alias and the `customer_id` JSON name: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 16 s
- Tool calls: 5, 3 before the first artifact

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 29 · 29 · 8 · 2 · 1160 · 28555 · 293169 · 2740 · 7 · 45111 · 385

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Tool calls: 7, 5 before the first artifact
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in place, with no deprecation or compatibility period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING in the release note, with no action for consumers: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 28 s
- Tool calls: 8, 4 before the first artifact
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed with no deprecation or transition period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING in the release note: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 27 s
- Tool calls: 7, 5 before the first artifact
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed with no compatibility period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING in the release note: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 24 s
- Tool calls: 7, 4 before the first artifact
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in place with no deprecation or side-by-side period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING and no action given to users: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 30 s
- Tool calls: 9, 6 before the first artifact
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` The field is removed in place, with no deprecation or side-by-side period for partners: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` The breaking rename is not marked BREAKING and the release note says nothing of what partners must do: duplicate

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 21 · 21 · 5 · 0 · 1162 · 25098 · 186597 · 1883 · 5 · 42539 · 377

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 19 s
- Tool calls: 4, 2 before the first artifact
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs a bit-trick threshold, an assignment and a push into one unreadable line: true positive

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 22 s
- Tool calls: 5, 3 before the first artifact
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bit-twiddling one-liner: true positive

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bit-twiddling one-liner: true positive

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 19 s
- Tool calls: 4, 2 before the first artifact
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` hides its premium threshold in a bit-twiddling one-liner: true positive

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 21 s
- Tool calls: 4, 2 before the first artifact
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bit-twiddling one-liner: true positive

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 20 · 19 · 5 · 0 · 1162 · 25265 · 187898 · 1589 · 5 · 42865 · 318

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 18 s
- Tool calls: 4, 2 before the first artifact
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:12-42` `import` nests six levels deep and hides its rules in unexplained conditions: true positive

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 22 s
- Tool calls: 5, 3 before the first artifact
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep and hides its filters: true positive

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 27 s
- Tool calls: 8, 4 before the first artifact
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests five levels deep and hides its rules in unexplained filters: true positive

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 19 s
- Tool calls: 4, 2 before the first artifact
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests five levels deep, duplicates the seat loop and hides its filters: true positive

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 16 s
- Tool calls: 4, 2 before the first artifact
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep and hides its filtering rules: true positive

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.24 · 12 · 12 · 5 · 0 · 1180 · 24040 · 139914 · 865 · 4 · 41284 · 216

#### Run 1

- Outcome: scored
- Cost: 0.26 USD, 12 s
- Tool calls: 5, 3 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.23 USD, 12 s
- Tool calls: 4, 2 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.24 USD, 12 s
- Tool calls: 4, 2 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.24 USD, 9 s
- Tool calls: 4, 2 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.24 USD, 10 s
- Tool calls: 4, 2 before the first artifact

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 33 · 33 · 8 · 0 · 1162 · 27253 · 190018 · 3240 · 5 · 43687 · 540

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 25 s
- Tool calls: 5, 2 before the first artifact
- Expected `no-timeout` (timeout, high): found by R-ops-2
- Expected `unbounded-retry` (retry, high): found by R-ops-1
- Finding R-ops-1 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge: true positive
- Finding R-ops-2 [high / high] `app/payments.py:14` Payment call has no timeout: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Charge failures are swallowed without a log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL hard-coded: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 32 s
- Tool calls: 7, 3 before the first artifact
- Expected `no-timeout` (timeout, high): found by R-ops-2
- Expected `unbounded-retry` (retry, high): found by R-ops-1
- Finding R-ops-1 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can bill an order several times: true positive
- Finding R-ops-2 [high / high] `app/payments.py:14` The payment call has no timeout and can hang the checkout path: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Payment failures are swallowed without any log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` The provider URL is hard-coded instead of read from configuration: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 34 s
- Tool calls: 8, 3 before the first artifact
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Charge call to the payment provider has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge, on every error: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Charge failures are swallowed without any log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL hard-coded instead of read from configuration: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 34 s
- Tool calls: 8, 4 before the first artifact
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Charge call to the payment provider has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate payments: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Charge failures are swallowed without any log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Payment provider URL is hard-coded: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.30 USD, 27 s
- Tool calls: 4, 2 before the first artifact
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate charges and loop forever: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Charge failures are swallowed without any log or actionable error: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Payment provider URL is hard-coded to production: **false positive**

### ops-code-retry-without-backoff

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.33 · 41 · 40 · 7 · 0 · 1179 · 27117 · 235586 · 3403 · 6 · 43941 · 529

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 40 s
- Tool calls: 8, 2 before the first artifact
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:24-25` A dropped webhook is invisible to operators: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:28-36` Errors in the payment listener are not handled: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 32 s
- Tool calls: 5, 3 before the first artifact
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:17-24` Failed and dropped deliveries are logged only at debug level, as free text, without the HTTP status: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:33-35` Delivery is awaited inside the payment event listener, so a slow merchant can delay the publisher for up to 25 s: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 25 s
- Tool calls: 4, 2 before the first artifact
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back-to-back with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:20-25` A dropped webhook is only logged at debug level, as free text: true positive

#### Run 4

- Outcome: scored
- Cost: 0.41 USD, 49 s
- Tool calls: 11, 5 before the first artifact
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:17-24` A dropped webhook is logged only at debug level, in free text, and failed HTTP statuses are never logged: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:33-35` Webhook delivery runs inline in the payment event chain, for up to about 25 s per event: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.39 USD, 46 s
- Tool calls: 6, 4 before the first artifact
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Failed deliveries are retried back-to-back, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:20-25` Dropped webhooks are logged only at debug level, in free text: true positive
- Finding R-ops-3 [medium / high] `src/webhooks.ts:17-19` Non-2xx responses are retried without recording their status: duplicate

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 20 · 20 · 6 · 0 · 1170 · 26619 · 144080 · 1739 · 4 · 42978 · 427

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 20 s
- Tool calls: 5, 3 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.25 USD, 17 s
- Tool calls: 4, 2 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Tool calls: 5, 3 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 21 s
- Tool calls: 6, 4 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 32 s
- Tool calls: 8, 2 before the first artifact
- Finding R-ops-1 [medium / low] `src/rates.rs:28-37` Final failure of the rates fetch emits no log or signal of its own: unreported (low confidence)

### ops-control-graceful-shutdown

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.37 · 39 · 39 · 7 · 0 · 1174 · 30245 · 246393 · 3659 · 6 · 46335 · 523

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Tool calls: 6, 4 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.40 USD, 45 s
- Tool calls: 8, 3 before the first artifact
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that times out after the relay accepted it is retried, duplicating the e-mail: unreported (low confidence)
- Finding R-ops-2 [medium / low] `src/worker.rs:26-29` The health beat does not depend on the queue being reachable: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 38 s
- Tool calls: 7, 5 before the first artifact
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that times out after the relay accepted it is retried and may duplicate the e-mail: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 33 s
- Tool calls: 5, 3 before the first artifact
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that times out after the relay accepted it is retried and may be delivered twice: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 40 s
- Tool calls: 6, 4 before the first artifact
- Finding R-ops-1 [low / low] `src/worker.rs:46-50` A send that times out after the relay took the message is sent again: unreported (low confidence)

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 26 · 26 · 7 · 2 · 1160 · 27559 · 283593 · 2180 · 7 · 44573 · 363

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 22 s
- Tool calls: 6, 4 before the first artifact
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The settlement worker can fail without anyone seeing it: true positive

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Tool calls: 7, 4 before the first artifact
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The nightly settlement worker fails unseen: true positive

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Tool calls: 7, 5 before the first artifact
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The nightly settlement worker can fail without anyone seeing it: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Tool calls: 5, 3 before the first artifact
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back once the Stripe account is closed: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The nightly settlement worker can fail or not run without anyone seeing it: true positive

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 28 s
- Tool calls: 5, 3 before the first artifact
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` Switch to Paylane has no way back once the Stripe account is closed: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` settlement-worker has no failure signal, so a missed nightly run goes unnoticed: true positive

### performance-code-hidden-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 30 · 30 · 7 · 0 · 1174 · 26993 · 235663 · 2423 · 6 · 43922 · 404

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 29 s
- Tool calls: 8, 3 before the first artifact
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2
- Finding R-performance-1 [medium / high] `src/report.rs:39` One customer query per invoice (N+1): true positive
- Finding R-performance-2 [low / high] `src/report.rs:44-46` The purchase-order regex is compiled for every invoice: true positive

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 29 s
- Tool calls: 6, 4 before the first artifact
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:23-26` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45` The purchase-order regex is compiled again for every invoice: true positive

#### Run 3

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Tool calls: 6, 3 before the first artifact
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:23-26` One customer query per invoice (N+1): true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` Regex compiled once per invoice: true positive

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 27 s
- Tool calls: 5, 3 before the first artifact
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:39` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45` Purchase-order regex compiled once per invoice: true positive

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 33 s
- Tool calls: 8, 3 before the first artifact
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:23-26` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45-46` Purchase-order regex compiled once per invoice: true positive

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 17 · 17 · 6 · 0 · 1179 · 24472 · 231138 · 1417 · 6 · 42776 · 259

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Tool calls: 5, 3 before the first artifact
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on the customer page: true positive

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 20 s
- Tool calls: 5, 3 before the first artifact
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order_lines query per order (N+1) in order_summaries: true positive

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 16 s
- Tool calls: 5, 3 before the first artifact
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on every customer page view: true positive

#### Run 4

- Outcome: scored
- Cost: 0.26 USD, 15 s
- Tool calls: 5, 3 before the first artifact
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order on every customer page view: true positive

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 29 s
- Tool calls: 7, 3 before the first artifact
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1): true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 17 · 17 · 7 · 0 · 1172 · 25003 · 188513 · 1396 · 5 · 42962 · 252

#### Run 1

- Outcome: scored
- Cost: 0.25 USD, 15 s
- Tool calls: 5, 3 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Tool calls: 6, 4 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.25 USD, 23 s
- Tool calls: 5, 3 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 17 s
- Tool calls: 6, 4 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 15 s
- Tool calls: 6, 4 before the first artifact

### performance-control-preloaded-helper

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 18 · 17 · 6 · 0 · 1172 · 26070 · 188229 · 1680 · 5 · 43087 · 336

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Tool calls: 6, 4 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 17 s
- Tool calls: 6, 4 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 14 s
- Tool calls: 5, 3 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 15 s
- Tool calls: 5, 3 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 21 s
- Tool calls: 5, 3 before the first artifact

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 34 · 34 · 9 · 3 · 1164 · 28355 · 380601 · 3152 · 9 · 45491 · 363

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 31 s
- Tool calls: 6, 4 before the first artifact
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log of an organization into memory: true positive
- Finding R-performance-2 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Building the whole CSV with map/join blocks the event loop: duplicate
- Finding R-performance-3 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the org_id filter sorted by created_at: duplicate
- Finding R-performance-4 [low / low] `design.md#Decision: The export loads every event, then writes the CSV` The choice of the simplest approach is not backed by a measurement: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 35 s
- Tool calls: 8, 6 before the first artifact
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log into memory and builds one CSV string: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is shown for the export's filter and sort: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 30 s
- Tool calls: 8, 6 before the first artifact
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export buffers every event of the organization in memory and builds the CSV in one synchronous pass: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the export's filter and sort: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 34 s
- Tool calls: 8, 6 before the first artifact
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads an unbounded result set into memory: true positive
- Finding R-performance-2 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Building the whole CSV synchronously blocks the event loop: duplicate
- Finding R-performance-3 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index stated for the export query's filter and sort: duplicate
- Finding R-performance-4 [low / high] `design.md#Decision: The export loads every event, then writes the CSV` "Simplest" is the only rationale, with no measurement: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 34 s
- Tool calls: 7, 5 before the first artifact
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export buffers the whole audit log in memory and builds the CSV in one synchronous call: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` The export's filter and sort may lack a supporting index: duplicate

### plan-built-in-experts

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.91 · 320 · 318 · 31 · 1 · 1170 · 101563 · 1957313 · 34587 · 25 · 84055 · 1329

#### Run 1

- Outcome: scored
- Cost: 1.98 USD, 308 s
- Tool calls: 37, 13 before the first artifact
- Requirement `roles::codev knows a catalog of roles bound to workflow steps`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `roles::The project configures roles under `roles:``: modified
- Collateral `roles::Invalid role settings are reported with stable codes`: modified
- Collateral `roles::`codev roles` serves the roles of a step`: modified
- Collateral `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `docs::The Roles chapter lists exactly the catalog's roles`: missed
- Added `skills::The `apply` skill reviews the change's code as each active expert`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.87 USD, 314 s
- Tool calls: 30, 16 before the first artifact
- Requirement `roles::codev knows a catalog of roles bound to workflow steps`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `roles::The project configures roles under `roles:``: modified
- Collateral `roles::Invalid role settings are reported with stable codes`: modified
- Collateral `roles::`codev roles` serves the roles of a step`: modified
- Collateral `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `docs::The Roles chapter lists exactly the catalog's roles`: missed
- Added `skills::The `apply` skill has the change's code reviewed by its experts`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.91 USD, 320 s
- Tool calls: 28, 11 before the first artifact
- Requirement `roles::codev knows a catalog of roles bound to workflow steps`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified
- Collateral `docs::The Roles chapter lists exactly the catalog's roles`: modified
- Collateral `roles::The project configures roles under `roles:``: modified
- Collateral `roles::Invalid role settings are reported with stable codes`: modified
- Collateral `roles::`codev roles` serves the roles of a step`: modified
- Collateral `skills::The default schema gives each artifact a role with "Done when" criteria`: modified

#### Run 4

- Outcome: scored
- Cost: 1.85 USD, 328 s
- Tool calls: 29, 13 before the first artifact
- Requirement `roles::codev knows a catalog of roles bound to workflow steps`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `roles::The project configures roles under `roles:``: modified
- Collateral `roles::Invalid role settings are reported with stable codes`: modified
- Collateral `roles::`codev roles` serves the roles of a step`: modified
- Collateral `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `docs::The Roles chapter lists exactly the catalog's roles`: missed
- Added `skills::The `propose` skill ticks the proposal's domains`: **probable duplicate**
- Added `skills::The `apply` skill reviews the code with the active experts`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 2.17 USD, 382 s
- Tool calls: 39, 18 before the first artifact
- Requirement `roles::codev knows a catalog of roles bound to workflow steps`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified
- Collateral `roles::The project configures roles under `roles:``: modified
- Collateral `roles::Invalid role settings are reported with stable codes`: modified
- Collateral `roles::`codev roles` serves the roles of a step`: modified
- Collateral `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `docs::The Roles chapter lists exactly the catalog's roles`: missed

### plan-decision-accept-command

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.54 · 209 · 207 · 25 · 0 · 1189 · 94604 · 1855054 · 20415 · 23 · 82580 · 924

#### Run 1

- Outcome: scored
- Cost: 1.54 USD, 209 s
- Tool calls: 25, 16 before the first artifact
- Requirement `decisions::Creating a decision through the CLI`: modified
- Requirement `decisions::Body seal recorded by the CLI`: modified
- Requirement `decisions::`decision new` writes the ADR and the seal in the same plan`: modified
- Collateral `decisions::Stable JSON contract for all commands`: modified

#### Run 2

- Outcome: scored
- Cost: 1.27 USD, 174 s
- Tool calls: 21, 10 before the first artifact
- Requirement `decisions::Creating a decision through the CLI`: modified
- Requirement `decisions::Body seal recorded by the CLI`: modified
- Requirement `decisions::`decision new` writes the ADR and the seal in the same plan`: modified
- Collateral `decisions::Stable JSON contract for all commands`: modified

#### Run 3

- Outcome: scored
- Cost: 1.30 USD, 165 s
- Tool calls: 21, 12 before the first artifact
- Requirement `decisions::Creating a decision through the CLI`: modified
- Requirement `decisions::Body seal recorded by the CLI`: modified
- Requirement `decisions::`decision new` writes the ADR and the seal in the same plan`: modified
- Collateral `decisions::Stable JSON contract for all commands`: modified

#### Run 4

- Outcome: scored
- Cost: 1.57 USD, 213 s
- Tool calls: 25, 14 before the first artifact
- Requirement `decisions::Creating a decision through the CLI`: modified
- Requirement `decisions::Body seal recorded by the CLI`: modified
- Requirement `decisions::`decision new` writes the ADR and the seal in the same plan`: modified
- Collateral `decisions::Stable JSON contract for all commands`: modified

#### Run 5

- Outcome: scored
- Cost: 1.62 USD, 227 s
- Tool calls: 24, 14 before the first artifact
- Requirement `decisions::Creating a decision through the CLI`: modified
- Requirement `decisions::Body seal recorded by the CLI`: modified
- Requirement `decisions::`decision new` writes the ADR and the seal in the same plan`: modified
- Collateral `decisions::Stable JSON contract for all commands`: modified

### plan-decision-disk-cache

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.04 · 159 · 157 · 23 · 1 · 1102 · 66421 · 1239870 · 14435 · 20 · 64903 · 753

#### Run 1

- Outcome: scored
- Cost: 1.44 USD, 179 s
- Tool calls: 27, 18 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 1.11 USD, 166 s
- Tool calls: 23, 13 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 1.03 USD, 159 s
- Tool calls: 22, 11 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 1.05 USD, 156 s
- Tool calls: 21, 12 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.96 USD, 132 s
- Tool calls: 22, 14 before the first artifact

### plan-decision-edit-inherited

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.33 · 160 · 159 · 27 · 1 · 1101 · 80607 · 1762797 · 14845 · 24 · 76084 · 645

#### Run 1

- Outcome: scored
- Cost: 1.33 USD, 159 s
- Tool calls: 24, 13 before the first artifact
- Requirement `decisions::Inherited decisions taken into account`: **missed**
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: **missed**
- Added `decisions::`codev decision deviate --copy` starts the deviation from the inherited text`: **probable duplicate**
- The plan does not validate

#### Run 2

- Outcome: scored
- Cost: 1.24 USD, 156 s
- Tool calls: 26, 12 before the first artifact
- Requirement `decisions::Inherited decisions taken into account`: **missed**
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: **missed**
- Added `decisions::`codev decision deviate --copy` starts the deviation from the inherited text`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.52 USD, 198 s
- Tool calls: 31, 18 before the first artifact
- Requirement `decisions::Inherited decisions taken into account`: **missed**
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: **missed**
- Added `decisions::`codev decision deviate --copy` starts the deviation from the inherited content`: **probable duplicate**

#### Run 4

- Outcome: scored
- Cost: 1.47 USD, 230 s
- Tool calls: 31, 13 before the first artifact
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::Inherited decisions taken into account`: **missed**
- Added `decisions::`codev decision deviate --copy` starts the deviation from the inherited decision's content`: **probable duplicate**
- Added `decisions::`decision deviate --json` reports whether the body was copied`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.11 USD, 148 s
- Tool calls: 21, 11 before the first artifact
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::Inherited decisions taken into account`: **missed**
- Added `decisions::A copied deviation reproduces the inherited decision`: **probable duplicate**

### plan-decision-hidden-folder

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.61 · 254 · 251 · 34 · 3 · 1110 · 83242 · 2265289 · 23305 · 31 · 75824 · 762

#### Run 1

- Outcome: scored
- Cost: 1.36 USD, 209 s
- Tool calls: 30, 15 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 1.65 USD, 254 s
- Tool calls: 41, 19 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 1.61 USD, 255 s
- Tool calls: 33, 18 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 1.89 USD, 297 s
- Tool calls: 49, 17 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 1.37 USD, 212 s
- Tool calls: 29, 11 before the first artifact

### plan-decision-lifecycle-via-accept

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.64 · 219 · 217 · 26 · 0 · 1185 · 100043 · 1916284 · 21962 · 24 · 87284 · 940

#### Run 1

- Outcome: scored
- Cost: 1.56 USD, 207 s
- Tool calls: 26, 15 before the first artifact
- Requirement `decisions::Supersession resolved as a chain`: modified
- Requirement `decisions::Superseding a decision through the CLI`: modified
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::The `codev decision promote` command extracts a design block into an ADR`: modified
- Requirement `decisions::`decision accept` accepts and seals a proposed decision`: modified

#### Run 2

- Outcome: scored
- Cost: 1.77 USD, 224 s
- Tool calls: 25, 15 before the first artifact
- Requirement `decisions::Superseding a decision through the CLI`: modified
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::The `codev decision promote` command extracts a design block into an ADR`: modified
- Requirement `decisions::Supersession resolved as a chain`: **missed**
- Requirement `decisions::`decision accept` accepts and seals a proposed decision`: **missed**
- Added `decisions::A proposed decision supersedes nothing`: **probable duplicate**
- Added `decisions::Accepting a decision applies its supersessions`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.64 USD, 218 s
- Tool calls: 25, 16 before the first artifact
- Requirement `decisions::Supersession resolved as a chain`: modified
- Requirement `decisions::Superseding a decision through the CLI`: modified
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::The `codev decision promote` command extracts a design block into an ADR`: modified
- Requirement `decisions::`decision accept` accepts and seals a proposed decision`: modified

#### Run 4

- Outcome: scored
- Cost: 1.59 USD, 208 s
- Tool calls: 23, 14 before the first artifact
- Requirement `decisions::Supersession resolved as a chain`: modified
- Requirement `decisions::Superseding a decision through the CLI`: modified
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::The `codev decision promote` command extracts a design block into an ADR`: modified
- Requirement `decisions::`decision accept` accepts and seals a proposed decision`: modified

#### Run 5

- Outcome: scored
- Cost: 1.67 USD, 224 s
- Tool calls: 24, 13 before the first artifact
- Requirement `decisions::Superseding a decision through the CLI`: modified
- Requirement `decisions::The `codev decision deviate` command creates a deviation ADR`: modified
- Requirement `decisions::The `codev decision promote` command extracts a design block into an ADR`: modified
- Requirement `decisions::`decision accept` accepts and seals a proposed decision`: modified
- Requirement `decisions::Supersession resolved as a chain`: **missed**
- Added `decisions::Accepting a supersession retires the superseded decisions`: **probable duplicate**
- Added `decisions::A draft does not take effect before it is accepted`: **probable duplicate**

### plan-decision-llm-in-binary

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.32 · 199 · 198 · 26 · 2 · 1103 · 77870 · 1388653 · 19022 · 20 · 73051 · 906

#### Run 1

- Outcome: scored
- Cost: 1.40 USD, 257 s
- Tool calls: 24, 10 before the first artifact
- Requirement `review-mode::The project and a change set the review mode`: modified
- Requirement `skills::In isolated mode, each reviewing role runs alone, then the skill consolidates`: **missed**
- Added `skills::In api mode, codev runs the roles and the skill consolidates`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.18 USD, 179 s
- Tool calls: 25, 10 before the first artifact
- Requirement `skills::In isolated mode, each reviewing role runs alone, then the skill consolidates`: modified
- Requirement `review-mode::The project and a change set the review mode`: **missed**

#### Run 3

- Outcome: scored
- Cost: 1.57 USD, 270 s
- Tool calls: 29, 11 before the first artifact
- Requirement `review-mode::The project and a change set the review mode`: modified
- Requirement `skills::In isolated mode, each reviewing role runs alone, then the skill consolidates`: **missed**
- Added `skills::In api mode, codev dispatches the roles and the skill consolidates`: **probable duplicate**

#### Run 4

- Outcome: scored
- Cost: 1.32 USD, 199 s
- Tool calls: 26, 11 before the first artifact
- Requirement `skills::In isolated mode, each reviewing role runs alone, then the skill consolidates`: modified
- Requirement `review-mode::The project and a change set the review mode`: **missed**
- Added `review-mode::`codev roles --briefs` serves each reviewer's brief whole`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.13 USD, 166 s
- Tool calls: 20, 10 before the first artifact
- Requirement `skills::In isolated mode, each reviewing role runs alone, then the skill consolidates`: modified
- Requirement `review-mode::The project and a change set the review mode`: **missed**

### plan-decision-yaml-lib

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.80 · 109 · 108 · 20 · 3 · 1082 · 51230 · 897229 · 9713 · 15 · 59526 · 608

#### Run 1

- Outcome: no-plan
- Cost: 0.35 USD, 28 s
- Tool calls: 5, no artifact written
- Decision 0006: **ignored**

#### Run 2

- Outcome: scored
- Cost: 0.80 USD, 118 s
- Tool calls: 19, 14 before the first artifact
- The plan does not validate

#### Run 3

- Outcome: scored
- Cost: 0.80 USD, 127 s
- Tool calls: 21, 9 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.77 USD, 108 s
- Tool calls: 17, 10 before the first artifact
- The plan does not validate

#### Run 5

- Outcome: scored
- Cost: 0.83 USD, 105 s
- Tool calls: 26, 19 before the first artifact

### plan-declarative-roles

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.95 · 310 · 308 · 38 · 4 · 1181 · 101134 · 2562747 · 30726 · 32 · 84672 · 1017

#### Run 1

- Outcome: scored
- Cost: 1.91 USD, 309 s
- Tool calls: 34, 16 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: **missed**
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: **missed**
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `instructions::`codev instructions` describes one artifact of a change`: modified
- Collateral `instructions::The human output of `codev instructions` reads top to bottom`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::The `propose` and `apply` skills take each step's criteria from `codev roles``: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.95 USD, 307 s
- Tool calls: 37, 22 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: **missed**
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: **missed**
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `instructions::`codev instructions` describes one artifact of a change`: missed
- Collateral `instructions::The human output of `codev instructions` reads top to bottom`: missed
- Added `instructions::`codev instructions` carries the role owning the artifact`: **probable duplicate**
- Added `skills::Skills take role criteria from `codev roles` and skip disabled steps`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 2.20 USD, 370 s
- Tool calls: 38, 14 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified
- Collateral `instructions::`codev instructions` describes one artifact of a change`: modified
- Collateral `instructions::The human output of `codev instructions` reads top to bottom`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed

#### Run 4

- Outcome: scored
- Cost: 1.69 USD, 282 s
- Tool calls: 33, 15 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: **missed**
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: **missed**
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `instructions::`codev instructions` describes one artifact of a change`: missed
- Collateral `instructions::The human output of `codev instructions` reads top to bottom`: missed
- Added `instructions::`codev instructions` carries the artifact's effective role`: **probable duplicate**
- Added `skills::Skills apply the roles codev serves at each step`: **probable duplicate**
- Added `skills::A step whose roles are all disabled is skipped`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 2.05 USD, 364 s
- Tool calls: 43, 14 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified
- Collateral `instructions::`codev instructions` describes one artifact of a change`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `instructions::The human output of `codev instructions` reads top to bottom`: missed

### plan-docs-and-tests-in-done

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.26 · 171 · 170 · 30 · 1 · 1164 · 68857 · 1608320 · 16103 · 25 · 67462 · 596

#### Run 1

- Outcome: scored
- Cost: 1.26 USD, 162 s
- Tool calls: 24, 18 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified

#### Run 2

- Outcome: scored
- Cost: 1.10 USD, 142 s
- Tool calls: 23, 13 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: **missed**
- Added `skills::The `apply` skill runs the full test suite before declaring a change done`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.18 USD, 171 s
- Tool calls: 29, 21 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified

#### Run 4

- Outcome: scored
- Cost: 1.37 USD, 210 s
- Tool calls: 37, 18 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified

#### Run 5

- Outcome: scored
- Cost: 1.28 USD, 189 s
- Tool calls: 29, 22 before the first artifact
- Requirement `skills::The `propose` skill checks traceability before presenting the plan`: modified
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: **missed**
- Added `skills::The tech lead's criteria require documentation and automated tests`: **probable duplicate**
- Added `skills::The `apply` skill runs the full test suite before declaring a change done`: **probable duplicate**

### plan-expert-triggers-and-stack-filter

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.88 · 320 · 318 · 31 · 1 · 1173 · 98129 · 2197573 · 31521 · 28 · 82031 · 1142

#### Run 1

- Outcome: scored
- Cost: 1.80 USD, 329 s
- Tool calls: 30, 16 before the first artifact
- Requirement `roles::`codev roles` serves the roles of a step`: modified

#### Run 2

- Outcome: scored
- Cost: 1.88 USD, 328 s
- Tool calls: 39, 17 before the first artifact
- Requirement `roles::`codev roles` serves the roles of a step`: modified

#### Run 3

- Outcome: scored
- Cost: 1.90 USD, 316 s
- Tool calls: 29, 16 before the first artifact
- Requirement `roles::`codev roles` serves the roles of a step`: modified

#### Run 4

- Outcome: scored
- Cost: 1.90 USD, 320 s
- Tool calls: 33, 20 before the first artifact
- Requirement `roles::`codev roles` serves the roles of a step`: modified

#### Run 5

- Outcome: scored
- Cost: 1.74 USD, 308 s
- Tool calls: 29, 15 before the first artifact
- Requirement `roles::`codev roles` serves the roles of a step`: **missed**
- Added `roles::Each expert declares the triggers of its domain`: **probable duplicate**
- Added `roles::`codev roles` reads the change's diff`: **probable duplicate**
- Added `roles::`codev roles` suggests the domains the diff points to`: **probable duplicate**
- Added `roles::`codev roles` serves the knowledge of the project's stacks`: **probable duplicate**
- Added `roles::The project sets the stack filter under `stacks:``: **probable duplicate**
- Added `roles::The human output of `codev roles` shows the stacks and the suggestions`: **probable duplicate**

### plan-fix-core-command-edge-cases

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  2.24 · 290 · 392 · 29 · 3 · 1220 · 145240 · 3245923 · 33914 · 49 · 65978 · 636

#### Run 1

- Outcome: scored
- Cost: 2.10 USD, 289 s
- Tool calls: 49, 29 before the first artifact
- Requirement `changes::A change name is kebab-case and unique`: modified
- Requirement `changes::`codev list` lists the active changes and, with `--specs`, the capabilities`: modified
- Requirement `schemas::A schema is resolved from the project before the built-in one`: modified
- Requirement `schemas::`codev schemas` lists the usable schemas`: **missed**
- Collateral `changes::The commands taking `--change` pick the change predictably`: missed
- Collateral `instructions::`codev instructions` fails with stable codes`: missed
- Collateral `schemas::The schema of a change is chosen once, at creation`: missed

#### Run 2

- Outcome: scored
- Cost: 2.76 USD, 465 s
- Tool calls: 63, 48 before the first artifact
- Requirement `changes::A change name is kebab-case and unique`: modified
- Requirement `changes::`codev list` lists the active changes and, with `--specs`, the capabilities`: modified
- Requirement `schemas::`codev schemas` lists the usable schemas`: **missed**
- Requirement `schemas::A schema is resolved from the project before the built-in one`: **missed**
- Collateral `changes::The commands taking `--change` pick the change predictably`: modified
- Collateral `schemas::The schema of a change is chosen once, at creation`: modified
- Collateral `instructions::`codev instructions` fails with stable codes`: missed

#### Run 3

- Outcome: scored
- Cost: 1.93 USD, 262 s
- Tool calls: 52, 31 before the first artifact
- Requirement `changes::A change name is kebab-case and unique`: modified
- Requirement `changes::`codev list` lists the active changes and, with `--specs`, the capabilities`: modified
- Requirement `schemas::`codev schemas` lists the usable schemas`: **missed**
- Requirement `schemas::A schema is resolved from the project before the built-in one`: **missed**
- Collateral `changes::The commands taking `--change` pick the change predictably`: missed
- Collateral `instructions::`codev instructions` fails with stable codes`: missed
- Collateral `schemas::The schema of a change is chosen once, at creation`: missed

#### Run 4

- Outcome: scored
- Cost: 2.24 USD, 215 s
- Tool calls: 60, 45 before the first artifact
- Requirement `changes::A change name is kebab-case and unique`: modified
- Requirement `changes::`codev list` lists the active changes and, with `--specs`, the capabilities`: modified
- Requirement `schemas::`codev schemas` lists the usable schemas`: **missed**
- Requirement `schemas::A schema is resolved from the project before the built-in one`: **missed**
- Collateral `changes::The commands taking `--change` pick the change predictably`: missed
- Collateral `instructions::`codev instructions` fails with stable codes`: missed
- Collateral `schemas::The schema of a change is chosen once, at creation`: missed

#### Run 5

- Outcome: scored
- Cost: 2.26 USD, 422 s
- Tool calls: 59, 46 before the first artifact
- Requirement `changes::A change name is kebab-case and unique`: modified
- Requirement `changes::`codev list` lists the active changes and, with `--specs`, the capabilities`: modified
- Requirement `schemas::`codev schemas` lists the usable schemas`: **missed**
- Requirement `schemas::A schema is resolved from the project before the built-in one`: **missed**
- Collateral `changes::The commands taking `--change` pick the change predictably`: missed
- Collateral `instructions::`codev instructions` fails with stable codes`: missed
- Collateral `schemas::The schema of a change is chosen once, at creation`: missed

### plan-forward-compatible-config

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.55 · 269 · 267 · 29 · 2 · 1168 · 85192 · 1969214 · 23571 · 25 · 77146 · 903

#### Run 1

- Outcome: scored
- Cost: 1.73 USD, 280 s
- Tool calls: 36, 14 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: modified

#### Run 2

- Outcome: scored
- Cost: 1.55 USD, 285 s
- Tool calls: 28, 17 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: modified

#### Run 3

- Outcome: scored
- Cost: 1.44 USD, 229 s
- Tool calls: 28, 11 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: modified

#### Run 4

- Outcome: scored
- Cost: 1.69 USD, 267 s
- Tool calls: 30, 12 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: modified

#### Run 5

- Outcome: scored
- Cost: 1.45 USD, 269 s
- Tool calls: 27, 13 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: modified

### plan-generated-changelog

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.55 · 281 · 278 · 28 · 1 · 1163 · 80710 · 1828247 · 24525 · 27 · 72556 · 931

#### Run 1

- Outcome: scored
- Cost: 1.80 USD, 295 s
- Tool calls: 48, 37 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `skills::The `propose` skill checks traceability before presenting the plan`: missed

#### Run 2

- Outcome: scored
- Cost: 1.58 USD, 280 s
- Tool calls: 34, 22 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `skills::The `propose` skill checks traceability before presenting the plan`: missed
- Added `skills::The default proposal asks for a categorized release note`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.55 USD, 256 s
- Tool calls: 32, 12 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `skills::The `propose` skill checks traceability before presenting the plan`: missed

#### Run 4

- Outcome: scored
- Cost: 1.40 USD, 251 s
- Tool calls: 25, 13 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `skills::The `propose` skill checks traceability before presenting the plan`: missed

#### Run 5

- Outcome: scored
- Cost: 1.34 USD, 281 s
- Tool calls: 27, 17 before the first artifact
- Requirement `skills::The default schema gives each artifact a role with "Done when" criteria`: modified
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Collateral `skills::The `propose` skill checks traceability before presenting the plan`: missed

### plan-harder-eval-cases

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.51 · 278 · 276 · 22 · 1 · 1165 · 84045 · 1352195 · 24844 · 19 · 77362 · 1173

#### Run 1

- Outcome: scored
- Cost: 1.69 USD, 318 s
- Tool calls: 30, 14 before the first artifact
- Requirement `role-evaluation::A case declares its fixture, its step, its role and what it expects`: modified
- Requirement `role-evaluation::The suite covers every reviewing role`: modified
- Requirement `role-evaluation::What a role invents is counted, and only that`: **missed**
- Added `role-evaluation::A case's plan agrees with its fixture`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.51 USD, 277 s
- Tool calls: 20, 10 before the first artifact
- Requirement `role-evaluation::A case declares its fixture, its step, its role and what it expects`: modified
- Requirement `role-evaluation::The suite covers every reviewing role`: modified
- Requirement `role-evaluation::What a role invents is counted, and only that`: modified

#### Run 3

- Outcome: scored
- Cost: 1.63 USD, 346 s
- Tool calls: 33, 15 before the first artifact
- Requirement `role-evaluation::A case declares its fixture, its step, its role and what it expects`: modified
- Requirement `role-evaluation::The suite covers every reviewing role`: modified
- Requirement `role-evaluation::What a role invents is counted, and only that`: modified

#### Run 4

- Outcome: scored
- Cost: 1.38 USD, 252 s
- Tool calls: 21, 12 before the first artifact
- Requirement `role-evaluation::A case declares its fixture, its step, its role and what it expects`: modified
- Requirement `role-evaluation::The suite covers every reviewing role`: modified
- Requirement `role-evaluation::What a role invents is counted, and only that`: modified

#### Run 5

- Outcome: scored
- Cost: 1.29 USD, 255 s
- Tool calls: 16, 10 before the first artifact
- Requirement `role-evaluation::A case declares its fixture, its step, its role and what it expects`: modified
- Requirement `role-evaluation::The suite covers every reviewing role`: modified
- Requirement `role-evaluation::What a role invents is counted, and only that`: modified

### plan-isolated-review

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.77 · 272 · 270 · 35 · 1 · 1171 · 99595 · 2447950 · 24158 · 29 · 88043 · 961

#### Run 1

- Outcome: scored
- Cost: 2.53 USD, 432 s
- Tool calls: 52, 22 before the first artifact
- Requirement `skills::The `propose` and `apply` skills take their roles from `codev roles``: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::The `propose` and `apply` skills run each reviewer in isolation when the project asks for it`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.46 USD, 272 s
- Tool calls: 28, 14 before the first artifact
- Requirement `skills::The `propose` and `apply` skills take their roles from `codev roles``: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::In isolated mode, each reviewer runs alone in a sub-agent`: **probable duplicate**
- Added `skills::The skill holds a meeting before writing the isolated findings`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.77 USD, 253 s
- Tool calls: 37, 14 before the first artifact
- Requirement `skills::The `propose` and `apply` skills take their roles from `codev roles``: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::In isolated mode, each reviewing role reviews alone as a sub-agent`: **probable duplicate**
- Added `skills::In isolated mode, the skill consolidates the findings in a meeting`: **probable duplicate**

#### Run 4

- Outcome: scored
- Cost: 1.92 USD, 294 s
- Tool calls: 34, 19 before the first artifact
- Requirement `skills::The `propose` and `apply` skills take their roles from `codev roles``: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::The `propose` and `apply` skills run each reviewer alone in isolated mode`: **probable duplicate**
- Added `skills::The `propose` and `apply` skills consolidate isolated reviews in a meeting`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.62 USD, 227 s
- Tool calls: 26, 19 before the first artifact
- Requirement `skills::The `propose` and `apply` skills take their roles from `codev roles``: **missed**
- Collateral `doctor::`codev doctor` runs a fixed list of read-only checks`: missed
- Added `skills::In isolated mode, each reviewing role reviews alone, then the skill holds the meeting`: **probable duplicate**

### plan-read-french-keywords

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.94 · 292 · 316 · 35 · 4 · 1182 · 107764 · 2421352 · 29830 · 29 · 77058 · 978

#### Run 1

- Outcome: scored
- Cost: 1.82 USD, 291 s
- Tool calls: 33, 20 before the first artifact
- Requirement `spec-parsing::Localized structural defects`: modified
- Requirement `validation::Structural rules beyond the parser`: modified

#### Run 2

- Outcome: scored
- Cost: 1.59 USD, 275 s
- Tool calls: 34, 13 before the first artifact
- Requirement `spec-parsing::Localized structural defects`: modified
- Requirement `validation::Structural rules beyond the parser`: modified

#### Run 3

- Outcome: scored
- Cost: 2.94 USD, 270 s
- Tool calls: 73, 40 before the first artifact
- Requirement `spec-parsing::Localized structural defects`: modified
- Requirement `validation::Structural rules beyond the parser`: modified

#### Run 4

- Outcome: scored
- Cost: 2.16 USD, 385 s
- Tool calls: 51, 37 before the first artifact
- Requirement `spec-parsing::Localized structural defects`: modified
- Requirement `validation::Structural rules beyond the parser`: modified

#### Run 5

- Outcome: scored
- Cost: 1.94 USD, 318 s
- Tool calls: 36, 21 before the first artifact
- Requirement `spec-parsing::Localized structural defects`: modified
- Requirement `validation::Structural rules beyond the parser`: modified

### plan-requirement-session-hook

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.33 · 196 · 194 · 29 · 2 · 1109 · 78522 · 1647436 · 18191 · 23 · 75090 · 791

#### Run 1

- Outcome: scored
- Cost: 1.30 USD, 181 s
- Tool calls: 29, 17 before the first artifact
- Requirement `init::`codev init` integrates codev into the repository`: modified
- Requirement `init::codev's text lives in managed sections that never touch user content`: **missed**
- Added `init::`codev init` installs a Claude Code session start hook without overwriting the user's settings`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.34 USD, 204 s
- Tool calls: 24, 10 before the first artifact
- Requirement `init::`codev init` integrates codev into the repository`: modified
- Requirement `init::codev's text lives in managed sections that never touch user content`: **missed**
- Added `init::`codev init` installs a Claude Code session start hook without overwriting the user's settings`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.24 USD, 183 s
- Tool calls: 20, 12 before the first artifact
- Requirement `init::`codev init` integrates codev into the repository`: modified
- Requirement `init::codev's text lives in managed sections that never touch user content`: **missed**
- Added `init::`codev init` registers a Claude Code session-start hook without overwriting the user's settings`: **probable duplicate**

#### Run 4

- Outcome: scored
- Cost: 1.33 USD, 196 s
- Tool calls: 28, 17 before the first artifact
- Requirement `init::`codev init` integrates codev into the repository`: modified
- Requirement `init::codev's text lives in managed sections that never touch user content`: **missed**
- Added `init::`codev init` installs a Claude Code session start hook without overwriting the user's settings`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.58 USD, 238 s
- Tool calls: 28, 16 before the first artifact
- Requirement `init::`codev init` integrates codev into the repository`: modified
- Requirement `init::codev's text lives in managed sections that never touch user content`: **missed**
- Added `init::`codev init` installs a Claude Code session start hook`: **probable duplicate**
- Added `init::The session start hook never overwrites the user's settings`: **probable duplicate**

### plan-requirement-skill-edited

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.39 · 193 · 190 · 31 · 2 · 1113 · 80416 · 1903808 · 18276 · 27 · 73531 · 661

#### Run 1

- Outcome: scored
- Cost: 1.39 USD, 207 s
- Tool calls: 30, 18 before the first artifact
- Requirement `skills::`codev update` regenerates the skills of the configured workflows`: modified

#### Run 2

- Outcome: scored
- Cost: 1.46 USD, 193 s
- Tool calls: 31, 17 before the first artifact
- Requirement `skills::`codev update` regenerates the skills of the configured workflows`: modified

#### Run 3

- Outcome: scored
- Cost: 1.18 USD, 164 s
- Tool calls: 26, 16 before the first artifact
- Requirement `skills::`codev update` regenerates the skills of the configured workflows`: modified

#### Run 4

- Outcome: scored
- Cost: 1.24 USD, 175 s
- Tool calls: 30, 20 before the first artifact
- Requirement `skills::`codev update` regenerates the skills of the configured workflows`: modified

#### Run 5

- Outcome: scored
- Cost: 1.59 USD, 237 s
- Tool calls: 41, 18 before the first artifact
- Requirement `skills::`codev update` regenerates the skills of the configured workflows`: modified

### plan-requirement-spanish

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.53 · 235 · 232 · 29 · 1 · 1103 · 81193 · 1960958 · 24142 · 25 · 75674 · 904

#### Run 1

- Outcome: scored
- Cost: 1.28 USD, 200 s
- Tool calls: 27, 12 before the first artifact
- Requirement `keyword-languages::Every structural keyword has a key and a text per language`: modified
- Requirement `keyword-languages::A keyword is recognized in every catalog language`: modified

#### Run 2

- Outcome: scored
- Cost: 1.64 USD, 235 s
- Tool calls: 31, 14 before the first artifact
- Requirement `keyword-languages::Every structural keyword has a key and a text per language`: modified
- Requirement `keyword-languages::A keyword is recognized in every catalog language`: modified

#### Run 3

- Outcome: scored
- Cost: 1.70 USD, 247 s
- Tool calls: 28, 13 before the first artifact
- Requirement `keyword-languages::Every structural keyword has a key and a text per language`: modified
- Requirement `keyword-languages::A keyword is recognized in every catalog language`: **missed**

#### Run 4

- Outcome: scored
- Cost: 1.32 USD, 197 s
- Tool calls: 23, 11 before the first artifact
- Requirement `keyword-languages::Every structural keyword has a key and a text per language`: modified
- Requirement `keyword-languages::A keyword is recognized in every catalog language`: modified

#### Run 5

- Outcome: scored
- Cost: 1.53 USD, 234 s
- Tool calls: 29, 14 before the first artifact
- Requirement `keyword-languages::Every structural keyword has a key and a text per language`: modified
- Requirement `keyword-languages::A keyword is recognized in every catalog language`: modified

### plan-review-record

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.89 · 314 · 325 · 35 · 3 · 1185 · 97371 · 2440020 · 32907 · 29 · 78801 · 1135

#### Run 1

- Outcome: scored
- Cost: 2.28 USD, 313 s
- Tool calls: 56, 37 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified

#### Run 2

- Outcome: scored
- Cost: 1.65 USD, 277 s
- Tool calls: 28, 12 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Added `skills::The `apply` skill records its review findings`: **probable duplicate**
- Added `skills::The `archive` skill relays a review that blocks the archive`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 2.13 USD, 381 s
- Tool calls: 34, 19 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified

#### Run 4

- Outcome: scored
- Cost: 1.89 USD, 306 s
- Tool calls: 36, 18 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: **missed**
- Added `skills::The `propose` and `apply` skills log their review findings in the review record`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.81 USD, 327 s
- Tool calls: 31, 16 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified
- Requirement `skills::The `apply` skill reviews the change's code as a newcomer`: modified

### plan-role-precision-from-baseline

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.81 · 238 · 236 · 37 · 2 · 1199 · 105250 · 2566435 · 23242 · 31 · 90081 · 801

#### Run 1

- Outcome: scored
- Cost: 1.79 USD, 224 s
- Tool calls: 36, 20 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified

#### Run 2

- Outcome: scored
- Cost: 2.05 USD, 269 s
- Tool calls: 38, 20 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified

#### Run 3

- Outcome: scored
- Cost: 1.81 USD, 231 s
- Tool calls: 32, 20 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified

#### Run 4

- Outcome: scored
- Cost: 1.56 USD, 237 s
- Tool calls: 34, 23 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified

#### Run 5

- Outcome: scored
- Cost: 1.89 USD, 275 s
- Tool calls: 37, 24 before the first artifact
- Requirement `skills::The `propose` skill challenges the plan and reports points to challenge`: modified

### plan-specify-core-commands

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  4.33 · 157 · 675 · 14 · 1 · 1301 · 331554 · 5278495 · 64765 · 141 · 36917 · 426

#### Run 1

- Outcome: scored
- Cost: 5.47 USD, 127 s
- Tool calls: 173, 164 before the first artifact
- Requirement `decisions::Showing a decision through the CLI`: **missed**
- Added `decisions::`decision show` reports the decision and its raw file`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 3.53 USD, 182 s
- Tool calls: 90, 74 before the first artifact
- Requirement `decisions::Showing a decision through the CLI`: **missed**
- Added `decisions::Output of `codev decision show``: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 3.92 USD, 156 s
- Tool calls: 117, 104 before the first artifact
- Requirement `decisions::Showing a decision through the CLI`: **missed**
- Added `decisions::What `codev decision show` returns`: **probable duplicate**

#### Run 4

- Outcome: scored
- Cost: 4.67 USD, 523 s
- Tool calls: 115, 99 before the first artifact
- Requirement `decisions::Showing a decision through the CLI`: **missed**
- Added `decisions::Outputs of `codev decision show``: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 4.33 USD, 148 s
- Tool calls: 122, 75 before the first artifact
- Requirement `decisions::Showing a decision through the CLI`: modified

### plan-specify-untracked-behaviors

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.40 · 192 · 189 · 32 · 1 · 1192 · 79165 · 2068014 · 16181 · 29 · 72775 · 548

#### Run 1

- Outcome: scored
- Cost: 1.47 USD, 191 s
- Tool calls: 31, 22 before the first artifact
- Requirement `init::`codev init --inherit` declares inherited sources`: **missed**
- Added `init::Inherited paths written by `codev init` use forward slashes`: **probable duplicate**

#### Run 2

- Outcome: scored
- Cost: 1.08 USD, 136 s
- Tool calls: 26, 17 before the first artifact
- Requirement `init::`codev init --inherit` declares inherited sources`: **missed**
- Added `init::Inherited paths made absolute by `codev init` use forward slashes`: **probable duplicate**

#### Run 3

- Outcome: scored
- Cost: 1.48 USD, 186 s
- Tool calls: 40, 25 before the first artifact
- Requirement `init::`codev init --inherit` declares inherited sources`: modified

#### Run 4

- Outcome: scored
- Cost: 1.36 USD, 212 s
- Tool calls: 31, 19 before the first artifact
- Requirement `init::`codev init --inherit` declares inherited sources`: **missed**
- Added `init::Inherited paths written by `codev init` use forward slashes`: **probable duplicate**

#### Run 5

- Outcome: scored
- Cost: 1.40 USD, 191 s
- Tool calls: 34, 21 before the first artifact
- Requirement `init::`codev init --inherit` declares inherited sources`: **missed**
- Added `init::Inherited paths written by `codev init` use forward slashes`: **probable duplicate**

### plan-update-notice-and-self-update

- Role: planner at `plan.write`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  1.68 · 295 · 293 · 30 · 2 · 1162 · 88500 · 2041707 · 28022 · 26 · 76713 · 1028

#### Run 1

- Outcome: scored
- Cost: 1.18 USD, 195 s
- Tool calls: 18, 7 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: **missed**
- Collateral `configure::The `configure` skill enriches `_codev/config.yaml` by analyzing the project`: missed
- Collateral `decisions::A git source is never read from a floating branch`: missed
- Collateral `doctor::`codev status` hints at `codev doctor``: missed

#### Run 2

- Outcome: scored
- Cost: 1.79 USD, 319 s
- Tool calls: 29, 15 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: **missed**
- Collateral `configure::The `configure` skill enriches `_codev/config.yaml` by analyzing the project`: missed
- Collateral `decisions::A git source is never read from a floating branch`: missed
- Collateral `doctor::`codev status` hints at `codev doctor``: missed

#### Run 3

- Outcome: scored
- Cost: 1.72 USD, 294 s
- Tool calls: 29, 17 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: **missed**
- Collateral `configure::The `configure` skill enriches `_codev/config.yaml` by analyzing the project`: missed
- Collateral `decisions::A git source is never read from a floating branch`: missed
- Collateral `doctor::`codev status` hints at `codev doctor``: missed

#### Run 4

- Outcome: scored
- Cost: 1.68 USD, 293 s
- Tool calls: 34, 20 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: **missed**
- Collateral `configure::The `configure` skill enriches `_codev/config.yaml` by analyzing the project`: missed
- Collateral `decisions::A git source is never read from a floating branch`: missed
- Collateral `doctor::`codev status` hints at `codev doctor``: missed

#### Run 5

- Outcome: scored
- Cost: 1.68 USD, 349 s
- Tool calls: 35, 15 before the first artifact
- Requirement `doctor::`codev doctor` runs a fixed list of read-only checks`: **missed**
- Collateral `configure::The `configure` skill enriches `_codev/config.yaml` by analyzing the project`: missed
- Collateral `decisions::A git source is never read from a floating branch`: missed
- Collateral `doctor::`codev status` hints at `codev doctor``: missed

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 25 · 24 · 5 · 0 · 1162 · 26923 · 190025 · 2382 · 5 · 43533 · 470

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 24 s
- Tool calls: 4, 2 before the first artifact
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` New processor Beacon not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `app/signup.py:13` No purpose, lawful basis, retention or erasure stated for the new member data: unreported (low confidence) (at the location of `email-in-logs`, another category)

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Tool calls: 4, 2 before the first artifact
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` New analytics processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `app/signup.py:13` No stated purpose, lawful basis or retention for the collected e-mail and display name: unreported (low confidence) (at the location of `email-in-logs`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 28 s
- Tool calls: 5, 3 before the first artifact
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS to count sign-ups: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` New processor Beacon not identified in the plan, transfer location unknown: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#Domains` Purpose, lawful basis and retention of the sign-up data are not stated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Tool calls: 4, 2 before the first artifact
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` Beacon is a new processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#Domains` Purpose, lawful basis and retention of the sign-up data not stated: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` Sign-up logs the member's e-mail address and display name: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` The analytics event sends the e-mail address to the Beacon SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` Beacon is a new processor that the plan does not identify: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#What Changes` No retention or erasure defined for the collected e-mail and display name: unreported (low confidence)

### privacy-code-error-message-pii

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.34 · 30 · 30 · 6 · 0 · 1174 · 29317 · 241900 · 2828 · 6 · 45350 · 453

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Tool calls: 6, 4 before the first artifact
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:40` Helply processor and transfer not identified: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns more than identifying the customer needs: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 29 s
- Tool calls: 5, 3 before the first artifact
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Customer e-mail address written to the error log on a failed lookup: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:42-47` Phone, birth date, address and e-mail sent to the Helply chat vendor when the purpose is a greeting by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:37-40` Helply is a new processor that the change does not identify: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns birth date and address with no stated need: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 29 s
- Tool calls: 5, 3 before the first artifact
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address it does not need: true positive
- Finding R-privacy-3 [medium / low] `proposal.md#Domains` Helply as processor: contract, location and lawful basis not identified: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns birth date and address to agents by default: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 36 s
- Tool calls: 8, 3 before the first artifact
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Customer e-mail address written to the error log on a failed lookup: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor Helply receives phone, birth date and address to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns the full customer record, birth date and address included: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:37-40` Helply is a new processor with no stated agreement or transfer location: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20-21` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:43-49` Chat vendor receives phone, birth date and address it does not need: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:15-17` Agent lookup returns the full customer record: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:38-41` Helply not identified as a processor, transfer location unknown: duplicate

### privacy-code-python-soft-delete

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 31 · 31 · 7 · 0 · 1174 · 29122 · 245777 · 2816 · 6 · 45486 · 443

#### Run 1

- Outcome: scored
- Cost: 0.38 USD, 31 s
- Tool calls: 9, 7 before the first artifact
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Erasure does not reach analytics events keyed by the stable analytics_id: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:17` No retention or purge defined for deleted accounts, backups included: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 40 s
- Tool calls: 6, 4 before the first artifact
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Erasure does not reach analytics events keyed by analytics_id: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:16-23` No retention or purge for the soft-deleted row and its backups: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Tool calls: 6, 4 before the first artifact
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Analytics events keyed by analytics_id are not reached by the erasure: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 36 s
- Tool calls: 9, 7 before the first artifact
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in `users`: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:16-23` No retention or purge rule for deleted accounts, backups included: duplicate
- Finding R-privacy-3 [medium / low] `app/account.py:8-13` Analytics pseudonym stays linkable after erasure: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Tool calls: 6, 4 before the first artifact
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in the users row: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Analytics pseudonym survives erasure and is not reached by deletion: unreported (low confidence)

### privacy-control-ids-in-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 24 · 24 · 6 · 0 · 1187 · 26362 · 188333 · 1755 · 5 · 43176 · 354

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 23 s
- Tool calls: 6, 4 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Tool calls: 5, 3 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 32 s
- Tool calls: 6, 4 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 30 s
- Tool calls: 5, 3 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Tool calls: 5, 3 before the first artifact
- Finding R-privacy-1 [medium / low] `app/fulfilment.py:26` No retention stated for the order ids the new log lines keep: unreported (low confidence)

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.40 · 50 · 50 · 9 · 2 · 1188 · 29656 · 335799 · 4865 · 8 · 45675 · 638

#### Run 1

- Outcome: scored
- Cost: 0.40 USD, 47 s
- Tool calls: 8, 6 before the first artifact
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with non-ASCII characters are logged in full: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:5-7` Masked form keeps the first letter and the full domain: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `src/logger.ts:3` URL-encoded addresses are not masked: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.45 USD, 59 s
- Tool calls: 10, 8 before the first artifact
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Percent-encoded e-mail addresses reach the logs in full: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:3` Addresses with characters outside the ASCII class escape masking: **false positive**
- Finding R-privacy-3 [medium / low] `src/logger.ts:12-15` Field keys are written to the log without masking: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:7` The kept domain and first letter may re-identify a customer: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.38 USD, 44 s
- Tool calls: 7, 5 before the first artifact
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with a non-ASCII character are logged in full: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:12-13` Field keys are written to the log without masking: **false positive**
- Finding R-privacy-3 [medium / low] `src/logger.ts:3` URL-encoded addresses escape the mask: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:7` The kept domain and first letter can still identify the person: unreported (low confidence)

#### Run 4

- Outcome: no-record
- Cost: 0.43 USD, 60 s
- Tool calls: 7, 3 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.40 USD, 50 s
- Tool calls: 9, 7 before the first artifact
- Finding R-privacy-1 [medium / high] `src/logger.ts:3` Addresses with non-ASCII or special local-part characters are logged in full or partly: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:6-8` URL-encoded addresses escape masking: unreported (low confidence)
- Finding R-privacy-3 [low / low] `src/logger.ts:12-15` Field keys are not masked: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:5-7` Kept first letter and full domain may re-identify on small domains: unreported (low confidence)

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.39 · 39 · 39 · 9 · 1 · 1170 · 30973 · 284660 · 3757 · 7 · 45239 · 497

#### Run 1

- Outcome: scored
- Cost: 0.43 USD, 44 s
- Tool calls: 12, 10 before the first artifact
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-2
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone outlive the opt-in, in the table and in backups: true positive
- Finding R-privacy-2 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Phone numbers and names go to a US processor with no stated safeguard: true positive
- Finding R-privacy-3 [low / low] `proposal.md#Why` Purpose and lawful basis are not stated: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 40 s
- Tool calls: 8, 5 before the first artifact
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-2
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after opt-out, backups with no expiry: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly beyond what the SMS needs: true positive
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer to Textly in the United States has no stated safeguard: duplicate
- Finding R-privacy-4 [low / low] `proposal.md#Why` Lawful basis of the birthday processing not stated: unreported (low confidence)
- Finding R-privacy-5 [low / low] `design.md#Decision: Birth dates live on the profile` Access and portability exports not said to include the new columns: unreported (low confidence) (at the location of `no-retention`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.39 USD, 39 s
- Tool calls: 12, 9 before the first artifact
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or purge rule for the copies in the nightly backups: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Phone numbers and names sent to a US processor with no transfer safeguard or processor terms stated: true positive
- Finding R-privacy-4 [low / low] `proposal.md#What Changes` Lawful basis of the new processing is not stated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 37 s
- Tool calls: 6, 4 before the first artifact
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention period for the copies in the nightly backups: duplicate
- Finding R-privacy-3 [medium / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly when the greeting only needs the phone: true positive
- Finding R-privacy-4 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer to Textly in the United States has no stated safeguard or processor agreement: duplicate
- Finding R-privacy-5 [low / low] `proposal.md#Why` Lawful basis for the birthday data is not stated: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.39 USD, 37 s
- Tool calls: 8, 6 before the first artifact
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-2
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after opt-out and in backups with no expiry: true positive
- Finding R-privacy-2 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers to Textly in the United States has no stated safeguard: true positive
- Finding R-privacy-3 [low / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly though the greeting does not need it: duplicate
- Finding R-privacy-4 [medium / low] `proposal.md#Why` Lawful basis for the birthday and phone processing is not stated: unreported (low confidence)

### security-code-detail-endpoint-authz

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.33 · 26 · 25 · 7 · 0 · 1179 · 28873 · 244056 · 2464 · 6 · 45685 · 411

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 25 s
- Tool calls: 6, 4 before the first artifact
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` Invoice detail endpoint returns any organization's invoice (IDOR): true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends the stack trace to the client: true positive

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 23 s
- Tool calls: 5, 3 before the first artifact
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The JSON error handler sends the exception message and stack trace to the client: true positive

#### Run 3

- Outcome: scored
- Cost: 0.38 USD, 40 s
- Tool calls: 7, 5 before the first artifact
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler returns the error message and stack trace to any client: true positive
- Finding R-security-3 [medium / high] `src/invoices.ts:27-31` Invoice id is not range-checked and a database error escapes the async handler: **false positive** (at the location of `detail-without-tenant-check`, another category)

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 25 s
- Tool calls: 5, 3 before the first artifact
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The error handler sends the error message and stack trace to the client: true positive

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 37 s
- Tool calls: 7, 5 before the first artifact
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler returns the error message and stack trace to any client: true positive
- Finding R-security-3 [medium / low] `src/invoices.ts:27-31` Unbounded numeric id reaches the database and its rejection escapes Express: unreported (low confidence) (at the location of `detail-without-tenant-check`, another category)

### security-code-python-path-traversal

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 26 · 26 · 7 · 0 · 1179 · 27398 · 232624 · 2493 · 6 · 43626 · 416

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Tool calls: 7, 5 before the first artifact
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment writes to any ticket without an authorization check: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Tool calls: 6, 4 before the first artifact
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment writes to any ticket without an authorization check: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Tool calls: 6, 4 before the first artifact
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-16` save_attachment stores into any ticket without an authorization check: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Tool calls: 5, 3 before the first artifact
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:28-31` Path traversal in read_attachment bypasses the ticket check: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-16` save_attachment enforces no authorization on the target ticket: unreported (low confidence)
- Finding R-security-3 [medium / high] `app/attachments.py:13-20` An upload with an existing name silently overwrites the stored attachment: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 29 s
- Tool calls: 6, 3 before the first artifact
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-15` save_attachment enforces no authorization on the target ticket: unreported (low confidence)

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 23 · 22 · 7 · 0 · 1174 · 27129 · 233913 · 1866 · 6 · 43789 · 281

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 31 s
- Tool calls: 9, 3 before the first artifact
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 23 s
- Tool calls: 6, 4 before the first artifact
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term of GET /users/search: true positive

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 16 s
- Tool calls: 5, 3 before the first artifact
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Tool calls: 6, 4 before the first artifact
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the `q` search term: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Tool calls: 6, 3 before the first artifact
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term: true positive

### security-control-docs-only

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 24 · 23 · 8 · 0 · 1176 · 26530 · 279210 · 2039 · 7 · 43827 · 311

#### Run 1

- Outcome: scored
- Cost: 0.23 USD, 14 s
- Tool calls: 4, 2 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 23 s
- Tool calls: 7, 5 before the first artifact
- Finding R-security-1 [low / low] `README.md:19-20` Revocation guarantee documented without code to back it: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Tool calls: 9, 5 before the first artifact
- Finding R-security-1 [low / high] `README.md:18-20` Exposed token stays valid until the new one is deployed: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Tool calls: 6, 4 before the first artifact
- Finding R-security-1 [low / high] `README.md:13-20` Exposed token stays valid until the new one is deployed: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Tool calls: 9, 5 before the first artifact
- Finding R-security-1 [low / high] `README.md:18-19` Exposed token stays valid until the new one is deployed: **false positive**

### security-control-dynamic-query

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 18 · 18 · 7 · 0 · 1172 · 25712 · 190690 · 1572 · 5 · 43556 · 292

#### Run 1

- Outcome: scored
- Cost: 0.26 USD, 15 s
- Tool calls: 5, 3 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Tool calls: 5, 3 before the first artifact

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 18 s
- Tool calls: 6, 4 before the first artifact

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 27 s
- Tool calls: 6, 4 before the first artifact

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Tool calls: 6, 4 before the first artifact

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.39 · 38 · 38 · 10 · 2 · 1162 · 31440 · 298654 · 3319 · 8 · 46192 · 450

#### Run 1

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Tool calls: 11, 9 before the first artifact
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, and so every API token, is written to the logs: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token can never be invalidated: no revocation and no expiry: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.40 USD, 40 s
- Tool calls: 7, 5 before the first artifact
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-2
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-1
- Finding R-security-1 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, bearer tokens included, is written to the logs: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be revoked and never expires: **false positive**
- Finding R-security-4 [medium / low] `specs/api-tokens/spec.md#Requirement: A user creates an API token` The plan does not say whether a bearer token may create new tokens: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 34 s
- Tool calls: 8, 6 before the first artifact
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, and so every bearer token, is written to the logs: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be invalidated: revocation is deferred and tokens never expire: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.40 USD, 52 s
- Tool calls: 13, 10 before the first artifact
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header is logged at info level: true positive
- Finding R-security-3 [medium / high] `design.md#Risks / Trade-offs` A leaked token cannot be invalidated: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.39 USD, 37 s
- Tool calls: 9, 6 before the first artifact
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, hence every bearer token, is written to the logs: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be revoked, so the change ships without a way to cut off a compromised integration: **false positive**

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.41 · 33 · 33 · 13 · 2 · 1164 · 32194 · 387184 · 3196 · 9 · 46727 · 414

#### Run 1

- Outcome: scored
- Cost: 0.36 USD, 27 s
- Tool calls: 8, 6 before the first artifact

#### Run 2

- Outcome: scored
- Cost: 0.43 USD, 60 s
- Tool calls: 12, 10 before the first artifact
- Finding R-skeptic-1 [low / high] `specs/cli-import/spec.md#Requirement: `--quiet` silences progress` The spec does not say where `--quiet` goes, and the parser accepts it only before the file: **false positive**
- Finding R-skeptic-2 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` `Progress::new(quiet)` "does nothing", but the lines are printed by `row()`: **false positive**
- Finding R-skeptic-3 [low / high] `tasks.md#1. Flag` The usage message is not part of the plan: **false positive**
- Finding R-skeptic-4 [low / low] `proposal.md#Domains` Every domain is unticked with no reason: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 33 s
- Tool calls: 11, 9 before the first artifact
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` The flag's position is fixed by the design but no scenario covers it elsewhere: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.46 USD, 50 s
- Tool calls: 20, 18 before the first artifact
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` `--quiet` after the file is a usage error, and no scenario says so: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.41 USD, 32 s
- Tool calls: 12, 10 before the first artifact

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.39 · 35 · 35 · 9 · 2 · 1162 · 31970 · 342792 · 3530 · 8 · 46342 · 432

#### Run 1

- Outcome: scored
- Cost: 0.41 USD, 41 s
- Tool calls: 16, 14 before the first artifact
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` A plugin system for formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` A per-user export-format setting nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` security left unticked for a route exposing invoices: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.44 USD, 39 s
- Tool calls: 12, 10 before the first artifact
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` A plugin system for future formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` A per-user export-format settings page nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains the plan touches are all unticked, with no reason: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.39 USD, 34 s
- Tool calls: 8, 6 before the first artifact
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats is scope nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format is scope nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Invoice export endpoint touches security and personal data with every domain unticked: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 35 s
- Tool calls: 8, 6 before the first artifact
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains touched by the plan are all unticked without a reason: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.38 USD, 34 s
- Tool calls: 7, 5 before the first artifact
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains all unticked while the plan exposes invoice data and changes a page: **false positive**
- Finding R-skeptic-4 [medium / high] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` Export requirement has no scenario for an unauthorized caller: **false positive**

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.39 · 41 · 41 · 9 · 2 · 1162 · 31337 · 339401 · 4007 · 8 · 46899 · 408

#### Run 1

- Outcome: scored
- Cost: 0.45 USD, 43 s
- Tool calls: 11, 9 before the first artifact
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context refutes: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Merged records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` Operations and migration are unticked with no reason: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 39 s
- Tool calls: 8, 6 before the first artifact
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Customer records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` Operations and migration are unticked with no reason: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.39 USD, 38 s
- Tool calls: 7, 5 before the first artifact
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Deleting the merged records is irreversible, with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and leaves conflicting fields undefined: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The plan schedules and runs a data-deleting job with operations and migration unticked: **false positive**
- Finding R-skeptic-5 [low / low] `proposal.md#Why` The plan cleans up duplicates without stopping their source: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.43 USD, 40 s
- Tool calls: 10, 5 before the first artifact
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Deleting merged customers is a one-way door with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and does not pin down what "share an e-mail address" means: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A new scheduled job is introduced with the operations domain unticked and no reason: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 40 s
- Tool calls: 7, 5 before the first artifact
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The operations domain is unticked for a new scheduled job: **false positive**
- Finding R-skeptic-5 [low / low] `proposal.md#Why` The cause of the duplicates is left in place: unreported (low confidence)

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.29 · 19 · 20 · 9 · 2 · 1090 · 26283 · 224664 · 1878 · 6 · 41893 · 327

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 24 s
- Tool calls: 10, 9 before the first artifact
- Changed although it should not: `_codev/changes/add-session-timeout/tasks.md` (**false positive**)

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 19 s
- Tool calls: 6, no artifact written

#### Run 3

- Outcome: scored
- Cost: 0.20 USD, 16 s
- Tool calls: 8, no artifact written

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 24 s
- Tool calls: 11, no artifact written

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 17 s
- Tool calls: 8, no artifact written

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 20 · 20 · 10 · 2 · 1090 · 26758 · 219835 · 2018 · 6 · 41217 · 362

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 20 s
- Tool calls: 11, 9 before the first artifact
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 19 s
- Tool calls: 9, 7 before the first artifact
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 18 s
- Tool calls: 9, 8 before the first artifact
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 20 s
- Tool calls: 9, 7 before the first artifact
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 19 s
- Tool calls: 10, 9 before the first artifact
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

