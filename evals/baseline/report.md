# Role evaluation report

- Date: 2026-10-14
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 40
- Repetitions: 5
- Cost: 12.80 USD for one run of the suite (sum of the cases' medians), 64.10 USD in all
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

5 runs per case: each measure is the median over the runs, its range in parentheses; compared with a baseline on the same model, not a proof.

## The suite

- Recall: 1.00 (39 of 39 seeded defects found)
- Evidence validity: 1.00 (69 of 69 checked quotes)
- False positives: 10 (7–11), 0.25 (0.17–0.28) per case

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 5 (3/2) | 5 | 5 | 0 | 0 | 1 | 3 | 0 | 1.00 | 1.00 (9/0/0/0) | 5/5 | 1.59 |
| migration | 5 (3/2) | 5 | 5 | 0 | 0 (0–1) | 0 | 1 | 0 | 1.00 | 1.00 (6/0/0/0) | 4/5 | 1.59 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 1.00 | 1.00 (2/0/0/0) | 2/2 | 0.79 |
| ops | 5 (3/2) | 6 | 6 | 0 | 1 (1–2) | 1 | 1 | 0 | 1.00 | 1.00 (9/0/0/0) | 3/6 | 1.63 |
| performance | 5 (3/2) | 4 | 4 | 0 | 0 | 1 | 0 | 0 | 1.00 | 1.00 (5/0/0/0) | 1/4 | 1.50 |
| privacy | 6 (4/2) | 7 | 7 | 0 | 2 (1–3) | 7 | 2 | 0 | 1.00 | 1.00 (19/0/0/0) | 4/7 | 2.11 |
| security | 6 (4/2) | 6 | 6 | 0 | 0 (0–1) | 1 | 0 | 0 | 1.00 | 1.00 (8/0/0/0) | 6/6 | 1.80 |
| skeptic | 3 (2/1) | 3 | 3 | 0 | 5 (4–7) | 1 | 1 | 0 | 1.00 | 1.00 (11/0/0/0) | 2/3 | 1.20 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.57 |

## Consumption

Per case, the median over its runs not in error; per role and for the suite, the sum of those medians. Tokens are summed over every model a run used.

| Role | Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| accessibility | 1.59 | 139 | 137 | 34 | 3 | 5840 | 137658 | 1107000 | 12903 | — | — | — |
| migration | 1.59 | 127 | 124 | 39 | 4 | 5848 | 134844 | 1405799 | 10895 | — | — | — |
| newcomer | 0.79 | 55 | 54 | 16 | 0 | 3506 | 73990 | 558680 | 4476 | — | — | — |
| ops | 1.63 | 156 | 155 | 36 | 1 | 5845 | 137894 | 1111433 | 14290 | — | — | — |
| performance | 1.50 | 114 | 112 | 36 | 3 | 5863 | 129564 | 1274546 | 9761 | — | — | — |
| privacy | 2.11 | 195 | 193 | 50 | 6 | 7067 | 171359 | 1784957 | 18343 | — | — | — |
| security | 1.80 | 140 | 137 | 41 | 1 | 7034 | 160148 | 1342306 | 11949 | — | — | — |
| skeptic | 1.20 | 126 | 125 | 30 | 6 | 3492 | 92667 | 1164124 | 11656 | — | — | — |
| tracer | 0.57 | 43 | 43 | 22 | 4 | 2180 | 51181 | 419731 | 4052 | — | — | — |
| **Suite** | 12.80 | 1094 | 1081 | 304 | 28 | 46675 | 1089305 | 10168576 | 98325 | — | — | — |

## Failures

- skeptic: 5 false positive(s) exceed 2

## Per case

### accessibility-code-dropdown-keyboard-trap

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 25 · 25 · 6 · 0 · 1172 · 28149 · 196251 · 2328 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab is swallowed and Escape does nothing: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 24 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Keyboard trap: Tab cycles inside the open sort menu and Escape does not close it: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` The menu button always announces "collapsed": true positive

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 27 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps Tab and ignores Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces "collapsed": true positive

#### Run 4

- Outcome: scored
- Cost: 0.34 USD, 29 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps the keyboard: Tab is captured and Escape does nothing: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces `aria-expanded="false"`: true positive

#### Run 5

- Outcome: scored
- Cost: 0.30 USD, 22 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:23-25` Open sort menu traps Tab and cannot be closed with Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.31 · 27 · 27 · 6 · 0 · 1162 · 26255 · 189214 · 2693 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 25 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar has no alt attribute: true positive

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner's avatar image has no text alternative: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Check focus visibility and target size of the 16px icon button: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 33 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target size and focus style not verifiable: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Delete target size and focus style not verifiable in the diff: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 27 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Delete target size and focus style not verifiable in the change: duplicate

### accessibility-control-custom-listbox

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 38 · 37 · 8 · 0 · 1174 · 29050 · 243451 · 3402 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.39 USD, 48 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Focus and highlighted option rely on styles the repository does not contain: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:70-73` Focus indicator of the combobox depends on a stylesheet outside the change: unreported (low confidence)
- Finding R-accessibility-2 [low / low] `src/components/SizePicker.tsx:92` Highlighted option is shown only through a class whose style is not in the change: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Focus and highlighted-option styles are not in the change: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 36 s
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Visible focus and highlighted option rely on styles the change does not ship: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 30 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Highlighted option is marked only by a class with no style in the repository: unreported (low confidence)

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.26 · 15 · 15 · 6 · 0 · 1170 · 25799 · 144484 · 1353 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.26 USD, 15 s

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s

#### Run 3

- Outcome: scored
- Cost: 0.25 USD, 15 s

#### Run 4

- Outcome: scored
- Cost: 0.26 USD, 13 s

#### Run 5

- Outcome: scored
- Cost: 0.26 USD, 15 s

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.36 · 34 · 34 · 8 · 3 · 1162 · 28405 · 333600 · 3127 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 28 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Removing the error summary leaves submit with no announced errors or focus target: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 28 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only: true positive
- Finding R-accessibility-2 [medium / low] `design.md#Decision: Invalid fields turn red` Nothing says where focus goes when the form is submitted with errors: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 33 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour only, with no text message: true positive
- Finding R-accessibility-2 [medium / high] `design.md#Decision: Invalid fields turn red` Submitting with invalid or untouched fields gives no feedback once the error list is removed: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 37 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour alone, with no text and no programmatic state: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Submitting with invalid fields gives no feedback once the error list is removed: duplicate
- Finding R-accessibility-3 [low / low] `design.md#Decision: Invalid fields turn red` Contrast of the red and grey borders against their background is not stated: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.38 USD, 39 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text: true positive
- Finding R-accessibility-2 [medium / high] `design.md#Decision: Invalid fields turn red` Removing the error summary leaves a failed submit unannounced and without focus: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 24 · 24 · 7 · 1 · 1176 · 25462 · 275555 · 2232 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 20 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account field renamed in the public API without a BREAKING mark or a transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup step: true positive

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Public JSON field renamed with no BREAKING mark nor compatibility path: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration nor backup: true positive

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account JSON field renamed with no BREAKING mark and no path for consumers: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration and no backup step: true positive

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 28 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account JSON field renamed without a BREAKING mark: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup: true positive

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 32 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Public JSON field renamed without a BREAKING mark or transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible DROP COLUMN with no down migration or backup step: true positive

### migration-code-serde-rename-all

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 25 · 25 · 7 · 0 · 1176 · 26843 · 278440 · 2421 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` camelCase rename silently renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config files without `default_currency` no longer load: true positive

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 40 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` `rename_all = "camelCase"` silently renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:5-10` New required `default_currency` key makes existing `config.toml` files fail to load: true positive

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` `rename_all = "camelCase"` renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` New required `default_currency` key makes existing `config.toml` files fail to load: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6` `rename_all = "camelCase"` renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing `config.toml` files without `default_currency` no longer load: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-12` rename_all = "camelCase" renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config.toml files without default_currency no longer load: true positive

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 21 · 21 · 8 · 2 · 1160 · 27963 · 282641 · 1719 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 21 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 16 s

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 20 s

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 22 s

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 23 s

### migration-control-serde-alias

- Role: migration at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 26 · 26 · 8 · 0 · 1174 · 26518 · 237646 · 1896 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 20 s

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 17 s

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 25 s

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 32 s
- Finding R-migration-1 [low / high] `src/config.rs:10-11` The `listen` alias has no automated test: **false positive**
- Finding R-migration-2 [low / low] `src/config.rs:11` Public Rust fields renamed without deprecation: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 34 s
- Finding R-migration-1 [low / high] `src/config.rs:10-11` No automated test pins the `listen` alias and the `customer_id` JSON name: **false positive**
- Finding R-migration-2 [medium / low] `src/config.rs:11` Public Rust fields renamed without deprecation for crate consumers: unreported (low confidence)

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 30 · 29 · 9 · 1 · 1162 · 28058 · 331517 · 2627 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 33 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in the same release, with no deprecation window: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING, no action for consumers: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 29 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in one step, with no transition for partners: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING, no action for consumers: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 27 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public `status` field removed in the same release, with no compatibility window: true positive
- Finding R-migration-2 [medium / high] `proposal.md#Release note` Release note does not mark the rename as BREAKING nor say what partners must do: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `proposal.md#Release note` The public field rename is not marked BREAKING and the release note gives users nothing to do: true positive
- Finding R-migration-2 [high / high] `design.md#Decision: The field is renamed in place` The rename removes `status` in one step, with no compatibility period for partner integrations: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` The public `status` field is removed with no deprecation period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` The release note does not mark the rename as BREAKING nor say what partners must do: duplicate

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.29 · 24 · 24 · 6 · 0 · 1164 · 24880 · 231202 · 2019 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 25 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs its premium rule into a bitwise one-liner: true positive

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 19 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs its whole rule into an unreadable one-liner: true positive

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 24 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bit-trick one-liner: true positive

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 24 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bitwise one-liner: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 22 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable one-liner that hides its threshold: true positive

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 19 · 19 · 5 · 0 · 1162 · 25111 · 187615 · 1601 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 18 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep and hides its rules in unexplained conditions: true positive

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` hides its rules in five levels of nesting and a duplicated seat loop: true positive

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep and hides its two unexplained filters: true positive

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 20 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:12-42` `import` nests five levels deep and hides its rules in unexplained conditions: true positive

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests five levels deep and hides its seat rules: true positive

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.24 · 12 · 11 · 5 · 0 · 1180 · 23999 · 139863 · 856 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.25 USD, 11 s

#### Run 2

- Outcome: scored
- Cost: 0.25 USD, 11 s

#### Run 3

- Outcome: scored
- Cost: 0.24 USD, 10 s

#### Run 4

- Outcome: scored
- Cost: 0.24 USD, 10 s

#### Run 5

- Outcome: scored
- Cost: 0.23 USD, 16 s

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 32 · 31 · 7 · 0 · 1162 · 26874 · 189043 · 3172 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 23 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Payment errors are swallowed without a log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL hard-coded: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Expected `no-timeout` (timeout, high): found by R-ops-2
- Expected `unbounded-retry` (retry, high): found by R-ops-1
- Finding R-ops-1 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can bill an order several times: true positive
- Finding R-ops-2 [high / high] `app/payments.py:14` Charge call has no timeout and can hang checkout forever: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Charge failures are swallowed with no log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL is hard-coded instead of read from configuration: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 26 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` The provider call has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate charges: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Failed charge attempts are swallowed without a log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL is hard-coded: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.34 USD, 34 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Provider call has no timeout and can hang checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Charge is retried forever, at a fixed 1 s pace, even on permanent errors: true positive
- Finding R-ops-3 [high / high] `app/payments.py:14` Non-idempotent charge is retried without an idempotency key, risking duplicate charges: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:17-18` Payment failures are swallowed with no log: duplicate
- Finding R-ops-5 [medium / high] `app/payments.py:7` Provider URL is hard-coded instead of read from configuration: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 31 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Charge call to the payment provider has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Charge is retried forever, without backoff, on a non-idempotent POST: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Provider errors are swallowed without a log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Payment provider URL is hard-coded: **false positive**

### ops-code-retry-without-backoff

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 39 · 38 · 9 · 0 · 1181 · 28141 · 285147 · 3582 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.41 USD, 46 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Failed deliveries are retried immediately, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:24` Failed and dropped webhooks are logged at debug level, outside the structured logger: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:28-36` Webhook delivery runs inline in the payment event publish path: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 38 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-2, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-1, another severity
- Finding R-ops-1 [high / high] `src/webhooks.ts:17-24` A dropped webhook is only logged at debug level, and a non-2xx response is not logged at all: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:9-16` Failed deliveries are retried immediately, with no backoff: true positive
- Finding R-ops-3 [medium / high] `src/webhooks.ts:33-35` Webhook delivery runs inline in the payment event listener and blocks `publish`: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 35 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:17-24` Failed and dropped deliveries are logged with console.debug in free text, and HTTP errors not at all: true positive

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 38 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:17-24` A dropped webhook is only logged at debug level, in free text, without the HTTP status: true positive

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 48 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-2, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-1, another severity
- Finding R-ops-1 [high / high] `src/webhooks.ts:24-25` A dropped webhook is logged only at debug level, in free text: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:9-23` Failed deliveries are retried immediately, with no backoff: true positive

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 19 · 19 · 6 · 0 · 1170 · 25425 · 143969 · 1537 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 18 s

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 23 s

#### Run 3

- Outcome: scored
- Cost: 0.26 USD, 17 s

#### Run 4

- Outcome: scored
- Cost: 0.25 USD, 16 s

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 19 s

### ops-control-graceful-shutdown

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.37 · 39 · 39 · 7 · 0 · 1174 · 30083 · 249060 · 3758 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 24 s

#### Run 2

- Outcome: scored
- Cost: 0.40 USD, 48 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that times out after the relay accepted it is sent again: unreported (low confidence)
- Finding R-ops-2 [medium / low] `src/worker.rs:30-34` No metric shows the backlog or the send latency: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 39 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-50` A send that times out after the relay accepted it may be retried and duplicated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 42 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that timed out is retried and may deliver the e-mail twice: unreported (low confidence)
- Finding R-ops-2 [low / low] `src/worker.rs:60-61` Registering the stop flag disables the default kill on a second signal: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 33 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-50` A send that times out after the relay accepted it is retried and may be delivered twice: unreported (low confidence)

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 27 · 28 · 7 · 1 · 1158 · 27371 · 244214 · 2241 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 22 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The settlement worker can fail without anyone seeing it: true positive

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 27 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody sees the settlement worker fail or not run: true positive

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 27 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody would see the settlement worker fail: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 32 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody would see the settlement worker fail or not run: true positive

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 36 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The settlement worker can fail without anyone seeing it: true positive

### performance-code-hidden-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.33 · 28 · 28 · 8 · 0 · 1176 · 27414 · 283327 · 2604 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 23 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:23-26` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` The purchase-order regex is compiled once per invoice: true positive

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 35 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2
- Finding R-performance-1 [medium / high] `src/report.rs:39` One customer query per invoice (N+1) in the monthly report: true positive
- Finding R-performance-2 [low / high] `src/report.rs:44-46` The purchase-order regex is compiled once per invoice: true positive

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:38-39` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` The purchase-order regex is compiled once per invoice: true positive

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:39` One customer query per invoice (N+1): true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45` The purchase-order regex is compiled once per invoice: true positive

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 28 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:23-26` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` Regex compiled again for every invoice: true positive

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 17 · 17 · 7 · 0 · 1179 · 24642 · 236309 · 1539 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 20 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on the customer page: true positive

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order_lines query per order (N+1) on the customer page: true positive

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 14 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on the customer page: true positive

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 17 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on every customer page view: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 28 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One query per order to load its lines (N+1): true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.25 · 16 · 16 · 6 · 0 · 1172 · 23798 · 184403 · 1290 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 17 s

#### Run 2

- Outcome: scored
- Cost: 0.23 USD, 11 s

#### Run 3

- Outcome: scored
- Cost: 0.25 USD, 13 s

#### Run 4

- Outcome: scored
- Cost: 0.25 USD, 16 s

#### Run 5

- Outcome: scored
- Cost: 0.26 USD, 16 s

### performance-control-preloaded-helper

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 21 · 21 · 6 · 0 · 1172 · 25958 · 190000 · 1477 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 23 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 20 s

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 18 s

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 17 s

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 32 s
- Finding R-performance-1 [low / low] `src/report.rs:25` Index on invoices.month not verifiable: unreported (low confidence)

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.36 · 32 · 31 · 9 · 3 · 1164 · 27752 · 380507 · 2851 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 32 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export buffers the whole audit log and builds the CSV synchronously on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is planned for the export's filter and sort: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 31 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log and builds the CSV in memory on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is shown for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 30 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log and the whole CSV into memory on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 31 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log into memory and builds one CSV string: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is known to serve the export's filter and sort: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log and builds the CSV in memory on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 26 · 26 · 7 · 0 · 1164 · 27291 · 241212 · 2520 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-4
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: duplicate
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` Beacon is a new processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `app/signup.py:13` No retention period or erasure path for the collected sign-up data: true positive

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` Beacon is a new processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [low / low] `app/signup.py:13` Purpose, lawful basis and retention of sign-up data not stated: unreported (low confidence) (at the location of `email-in-logs`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` New analytics processor Beacon not identified as a processor or transfer: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#Domains` Purpose, lawful basis and retention of the sign-up data not stated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 23 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the third-party analytics processor Beacon: true positive
- Finding R-privacy-3 [medium / low] `app/signup.py:13` Purpose, lawful basis and retention of the stored member data not stated: unreported (low confidence) (at the location of `email-in-logs`, another category)

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 26 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive

### privacy-code-error-message-pii

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.35 · 30 · 29 · 8 · 0 · 1176 · 28912 · 286629 · 2867 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 32 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:19-21` Customer e-mail address written to the error log on a failed lookup: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:38-49` Phone, birth date, address and e-mail sent to the Helply chat vendor when only the name is needed: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:15-18` Support lookup returns birth date, phone and postal address the identification purpose may not need: unreported (low confidence) (at the location of `email-in-error-message`, another category)
- Finding R-privacy-4 [medium / low] `src/support.ts:37-41` Helply not identified as a processor, nor its hosting location: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:40` Helply processor and possible transfer outside the EEA not identified: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Lookup returns birth date, phone and address to agents: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 43 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the logs through the error message: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor Helply receives phone, birth date, address and e-mail when the purpose needs the name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:37` Helply is a new processor with no stated legal basis, contract or transfer location: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns birth date, address and phone without a stated need: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Customer e-mail address written to the error log on a failed lookup: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor Helply receives phone, birth date, address and e-mail when only the name is needed: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:37-40` Helply is a new processor with no stated agreement or transfer location: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns birth date, phone and address beyond identifying the customer: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 30 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address beyond the greeting purpose: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Lookup returns birth date, phone and address to support agents: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#Domains` Helply as processor and its transfer location not identified: unreported (low confidence)

### privacy-code-python-soft-delete

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.38 · 36 · 36 · 11 · 3 · 1180 · 29761 · 384864 · 3231 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.38 USD, 33 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in `users`: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:8-13` Pseudonymous analytics id survives erasure and has no use in the change: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 47 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in the users row: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Erasure does not reach analytics events keyed by analytics_id: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:16-23` No retention or purge rule for deleted accounts, backups included: duplicate
- Finding R-privacy-3 [medium / low] `app/account.py:8-13` analytics_id is added with no use, and deletion does not reach the events it keys: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:16-23` Erasure of backups and analytics copies is not defined: duplicate
- Finding R-privacy-3 [low / low] `app/account.py:8-13` analytics_id is added with no use in the change and is only pseudonymous: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.43 USD, 59 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:13` Erasure does not reach analytics data keyed by `analytics_id`: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:16-17` No retention or purge defined for deleted accounts, backups included: duplicate

### privacy-control-ids-in-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 18 · 18 · 6 · 0 · 1187 · 25684 · 188706 · 1523 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 17 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 20 s

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 16 s

#### Run 4

- Outcome: scored
- Cost: 0.25 USD, 15 s

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 29 s

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.38 · 43 · 42 · 7 · 1 · 1186 · 28327 · 287032 · 4033 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.45 USD, 60 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` The address pattern misses valid and encoded addresses, which reach the logs in full: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:12-13` Field keys are written to the log without masking: unreported (low confidence)
- Finding R-privacy-3 [low / low] `src/logger.ts:7` The mask keeps the first letter and the full domain: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 39 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:3` Percent-encoded addresses reach the logs unmasked: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:3` Addresses with a non-ASCII local part are not masked: **false positive**
- Finding R-privacy-3 [low / high] `src/logger.ts:12-13` Field names are written to the log without masking: **false positive**
- Finding R-privacy-4 [low / low] `src/logger.ts:7` The kept domain can identify the person: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.38 USD, 42 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:3` Addresses with non-ASCII characters are logged unmasked: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:7` URL-encoded addresses escape the mask: unreported (low confidence)
- Finding R-privacy-3 [low / low] `src/logger.ts:5` The kept first letter and full domain may still identify the person: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 47 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with a non-ASCII character escape the mask and are logged in full: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:6-8` URL-encoded addresses (`%40`) are logged in full: **false positive**
- Finding R-privacy-3 [medium / high] `src/logger.ts:5-7` A one-letter local part is logged whole: **false positive**
- Finding R-privacy-4 [low / low] `src/logger.ts:7` The masked form still identifies people on personal domains: unreported (low confidence)
- Finding R-privacy-5 [low / low] `src/logger.ts:12-13` Field names are not masked: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 30 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:3` Addresses with a non-ASCII domain are logged in full: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:6-8` URL-encoded addresses pass through unmasked: **false positive**

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.41 · 42 · 42 · 11 · 2 · 1174 · 31384 · 396514 · 4169 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.41 USD, 50 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or erasure rule for the copies in nightly backups: duplicate
- Finding R-privacy-3 [medium / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly though the greeting does not need it: true positive
- Finding R-privacy-4 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers to a US processor has no stated safeguard: duplicate
- Finding R-privacy-5 [medium / low] `proposal.md#Why` Purpose and lawful basis of the new data are not stated: unreported (low confidence)
- Finding R-privacy-6 [low / low] `design.md#Decision: Birth dates live on the profile` Access, portability and erasure are not shown to reach the new columns: unreported (low confidence) (at the location of `no-retention`, another category)

#### Run 2

- Outcome: scored
- Cost: 0.41 USD, 47 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept after the member opts out: true positive
- Finding R-privacy-2 [medium / low] `design.md#Decision: Birth dates live on the profile` Retention of birthday and phone in the nightly backups is undefined: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Textly, a US-hosted processor, has no stated processing agreement or transfer safeguard: true positive
- Finding R-privacy-4 [low / low] `proposal.md#Why` Lawful basis and purpose of the new data are not stated: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.42 USD, 42 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or erasure rule for the copies in the nightly backups: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers to Textly in the United States has no stated safeguard or processor terms: true positive
- Finding R-privacy-4 [low / low] `design.md#Decision: Greetings are sent by SMS through Textly` The display name is sent to Textly without the purpose requiring it: duplicate
- Finding R-privacy-5 [low / low] `proposal.md#What Changes` Lawful basis for the new data is not stated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.40 USD, 37 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after opt-out and in backups with no expiry: true positive
- Finding R-privacy-2 [medium / low] `tasks.md#1. Greetings` Access, portability and erasure not shown to reach the new columns: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers to Textly in the United States has no stated safeguard: true positive
- Finding R-privacy-4 [low / low] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly may exceed what the greeting needs: unreported (low confidence) (at the location of `processor-transfer`, another category)

#### Run 5

- Outcome: scored
- Cost: 0.44 USD, 39 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or purge rule for the copies in nightly backups: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers and names to Textly in the United States has no stated safeguard: true positive
- Finding R-privacy-4 [low / low] `proposal.md#Why` Lawful basis of the processing is not stated: unreported (low confidence)

### security-code-detail-endpoint-authz

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.32 · 25 · 24 · 7 · 0 · 1179 · 28273 · 243373 · 2230 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` Invoice detail endpoint returns any organization's invoice (IDOR): true positive
- Finding R-security-2 [medium / high] `src/server.ts:17` Error handler sends the exception message and stack trace to the client: true positive

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 24 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-40` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The error handler sends the error message and stack trace to the client: true positive
- Finding R-security-3 [low / high] `src/invoices.ts:27-31` An empty or zero id passes the id validation: **false positive** (at the location of `detail-without-tenant-check`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends error messages and stack traces to the client: true positive

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The error handler sends the error message and stack trace to the client: true positive

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 23 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` Any signed-in user reads any organization's invoice by id: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler returns the error message and stack trace to the client: true positive

### security-code-python-path-traversal

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.29 · 23 · 23 · 6 · 0 · 1177 · 25770 · 187642 · 2226 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 23 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment writes to any ticket without an authorization check: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 22 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-31` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11` save_attachment enforces no authorization on the ticket: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 22 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-31` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11` save_attachment enforces no authorization on the ticket written to: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 24 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-13` save_attachment performs no authorization check: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 26 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment bypasses the ticket authorization: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment takes no agent identity and cannot enforce authorization: unreported (low confidence)

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.29 · 23 · 23 · 6 · 0 · 1174 · 25389 · 234493 · 1764 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 22 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term in searchUsers: true positive

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 23 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term: true positive

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 23 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term in searchUsers: true positive

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 29 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the `q` search term: true positive

### security-control-docs-only

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.28 · 18 · 18 · 7 · 0 · 1174 · 24952 · 235070 · 1377 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.28 USD, 18 s

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 26 s

#### Run 3

- Outcome: scored
- Cost: 0.24 USD, 12 s

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 29 s

#### Run 5

- Outcome: scored
- Cost: 0.26 USD, 15 s

### security-control-dynamic-query

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.26 · 16 · 15 · 6 · 0 · 1170 · 24996 · 143317 · 1369 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.26 USD, 15 s

#### Run 2

- Outcome: scored
- Cost: 0.28 USD, 22 s

#### Run 3

- Outcome: scored
- Cost: 0.24 USD, 12 s

#### Run 4

- Outcome: scored
- Cost: 0.26 USD, 15 s

#### Run 5

- Outcome: scored
- Cost: 0.26 USD, 16 s

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.37 · 35 · 35 · 9 · 1 · 1160 · 30768 · 298411 · 2983 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.37 USD, 40 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, with the bearer token, is logged at info level: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be revoked and never expires: **false positive**
- Finding R-security-4 [medium / low] `specs/api-tokens/spec.md#Requirement: A user creates an API token` Authorization of token creation is not specified: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.36 USD, 44 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header is written to the logs: true positive

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 34 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, and so every token, is written to the logs: true positive

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 33 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, holding the bearer token, is logged: true positive
- Finding R-security-3 [medium / low] `design.md#Risks / Trade-offs` A leaked token cannot be invalidated until a later change: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 28 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-2
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-1
- Finding R-security-1 [high / high] `design.md#Decision: Every request is logged with its headers` Bearer tokens are logged at info level with every request: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.42 · 47 · 46 · 12 · 2 · 1166 · 32218 · 443160 · 4319 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.42 USD, 44 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` `--quiet` after the file is a usage error, and no scenario says so: **false positive**
- Finding R-skeptic-2 [low / high] `tasks.md#1. Flag` Task 1.1 breaks `docs_mention_every_flag` until task 1.2 is done: **false positive**
- Finding R-skeptic-3 [low / high] `specs/cli-import/spec.md#Requirement: `--quiet` silences progress` No scenario keeps the progress lines when `--quiet` is absent: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.42 USD, 46 s
- Finding R-skeptic-1 [medium / high] `design.md#Decision: The progress reporter takes a `quiet` switch` The spec leaves `--quiet` after the file undefined, and the planned parser rejects it: **false positive**
- Finding R-skeptic-2 [low / high] `src/main.rs:8` The usage line keeps omitting `--quiet`: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.45 USD, 48 s
- Finding R-skeptic-1 [low / high] `specs/cli-import/spec.md#Requirement: `--quiet` silences progress` The spec never states that progress still prints without `--quiet`: **false positive**
- Finding R-skeptic-2 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` Where `--quiet` goes on the command line is decided only in the design, and the usage line is not updated: **false positive**
- Finding R-skeptic-3 [low / low] `proposal.md#Domains` `operations` left unticked with no reason although the change targets nightly jobs' output: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.48 USD, 53 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` The position of `--quiet` on the command line is left unspecified: **false positive**
- Finding R-skeptic-2 [low / low] `proposal.md#Domains` No domain is ticked or justified in the proposal: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.40 USD, 45 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` Flag position is unspecified; the design rejects `--quiet` after the file: **false positive**
- Finding R-skeptic-2 [low / high] `tasks.md#1. Flag` The usage line is not updated to mention `--quiet`: **false positive**
- Finding R-skeptic-3 [low / low] `proposal.md#Domains` Domains all unticked with no reason: unreported (low confidence)

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.38 · 39 · 39 · 8 · 2 · 1162 · 29994 · 334142 · 3533 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.39 USD, 39 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `proposal.md#What Changes` Settings page for a preferred export format nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains all unticked while the plan exposes invoices and changes the UI: **false positive**
- Finding R-skeptic-4 [low / low] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` Which date places an invoice in a month is not stated: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.39 USD, 39 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` A plugin system for four future formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` A per-user export-format settings page nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` The export route exposes customer data with every domain unticked: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 30 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats is scope nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `proposal.md#What Changes` Settings page for a preferred export format is scope nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` New endpoint exposing invoice and customer data with security and personal-data unticked: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 56 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` A plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` A settings page for a preferred export format nobody asked for, with no effect: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` The security domain is unticked although the change adds a route that exposes invoices: **false positive**
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The personal-data domain is unticked although the export contains customer names: **false positive**
- Finding R-skeptic-5 [low / high] `proposal.md#Domains` The user-interface domain is unticked although the plan changes the settings page: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains the plan touches are all unticked with no reason: **false positive**

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.40 · 40 · 40 · 10 · 2 · 1164 · 30455 · 386822 · 3804 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.43 USD, 46 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The design's own context contradicts the claim that one e-mail address means one person: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` The job deletes customer records and gives no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: no rule for e-mail case or for conflicting fields: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The plan deletes personal data, but its expert is disabled for this change: **false positive**
- Finding R-skeptic-5 [low / high] `proposal.md#Why` The plan cleans up duplicates nightly but leaves the checkout that creates them unchanged: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.39 USD, 34 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The plan assumes one e-mail is one person, which its own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Merged records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` Operations is unticked although the plan adds a scheduled job: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.39 USD, 47 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context refutes: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Customer records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and leaves the matching rule unstated: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A new scheduled job with the operations domain unticked and no reason: **false positive**
- Finding R-skeptic-5 [medium / low] `proposal.md#Domains` The ticked personal-data domain has no active expert: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 35 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context refutes: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Deleting merged records is irreversible, with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and leaves the surviving data undefined: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A new scheduled job with the operations domain unticked: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.41 USD, 40 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Duplicate records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: duplicate
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A scheduled job is added with operations unticked and no reason: **false positive**

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.27 · 20 · 20 · 11 · 2 · 1090 · 25039 · 195584 · 1838 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 22 s

#### Run 2

- Outcome: scored
- Cost: 0.26 USD, 14 s

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 20 s

#### Run 4

- Outcome: scored
- Cost: 0.22 USD, 19 s

#### Run 5

- Outcome: scored
- Cost: 0.31 USD, 24 s

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output | Calls | Input/call | Output/call
  0.30 · 23 · 23 · 11 · 2 · 1090 · 26142 · 224147 · 2214 · — · — · —

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 27 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 22 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 4

- Outcome: scored
- Cost: 0.27 USD, 19 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 20 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

