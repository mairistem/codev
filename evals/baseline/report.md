# Role evaluation report

- Date: 2026-10-14
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 40
- Repetitions: 5
- Cost: 13.69 USD for one run of the suite (sum of the cases' medians), 68.86 USD in all
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

5 runs per case: each measure is the median over the runs, its range in parentheses; compared with a baseline on the same model, not a proof.

## The suite

- Recall: 1.00 (39 of 39 seeded defects found)
- Evidence validity: 1.00 (71 of 71 checked quotes)
- False positives: 10 (8–11), 0.25 (0.20–0.28) per case

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 5 (3/2) | 5 | 5 | 0 | 0 | 1 | 3 | 0 | 1.00 | 1.00 (9/0/0/0) | 5/5 | 1.69 |
| migration | 5 (3/2) | 5 | 5 | 0 | 0 (0–2) | 0 | 1 | 0 | 1.00 | 1.00 (6/0/0/0) | 4/5 | 1.67 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 1.00 | 1.00 (2/0/0/0) | 2/2 | 0.85 |
| ops | 5 (3/2) | 6 | 6 | 0 | 1 (0–1) | 2 | 1 | 0 | 1.00 | 1.00 (10/0/0/0) | 3/6 | 1.75 |
| performance | 5 (3/2) | 4 | 4 | 0 | 0 (0–1) | 1 | 0 | 0 | 1.00 | 1.00 (6/0/0/0) | 1/4 | 1.62 |
| privacy | 6 (4/2) | 7 | 7 | 0 | 2 (1–4) | 7 | 3 | 0 | 1.00 | 1.00 (19/0/0/0) | 4/7 | 2.25 |
| security | 6 (4/2) | 6 | 6 | 0 | 2 (0–2) | 2 | 0 | 0 | 1.00 | 1.00 (10/0/0/0) | 6/6 | 1.93 |
| skeptic | 3 (2/1) | 3 | 3 | 0 | 4 (3–6) | 1 | 1 | 0 | 1.00 | 1.00 (9/0/0/0) | 2/3 | 1.33 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.61 |

## Consumption

Per case, the median over its runs not in error; per role and for the suite, the sum of those medians. Tokens are summed over every model a run used.

| Role | Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| accessibility | 1.69 | 155 | 152 | 36 | 5 | 5907 | 141138 | 1434876 | 13611 |
| migration | 1.67 | 141 | 137 | 41 | 6 | 5907 | 139411 | 1517887 | 11820 |
| newcomer | 0.85 | 55 | 53 | 18 | 0 | 3539 | 77080 | 614478 | 4498 |
| ops | 1.75 | 179 | 173 | 40 | 5 | 5919 | 141410 | 1569410 | 14874 |
| performance | 1.62 | 135 | 131 | 40 | 5 | 5923 | 138649 | 1388478 | 10471 |
| privacy | 2.25 | 232 | 227 | 52 | 8 | 7144 | 179700 | 1943028 | 20085 |
| security | 1.93 | 162 | 159 | 44 | 5 | 7120 | 166237 | 1748536 | 13104 |
| skeptic | 1.33 | 138 | 137 | 32 | 9 | 3527 | 107404 | 1235109 | 12303 |
| tracer | 0.61 | 51 | 50 | 20 | 5 | 2186 | 52146 | 547052 | 4340 |
| **Suite** | 13.69 | 1249 | 1219 | 323 | 48 | 47172 | 1143175 | 11998854 | 105106 |

## Failures

- skeptic: 4 false positive(s) exceed 2

## Per case

### accessibility-code-dropdown-keyboard-trap

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.36 · 31 · 31 · 7 · 1 · 1187 · 29644 · 292297 · 2819

#### Run 1

- Outcome: scored
- Cost: 0.36 USD, 31 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab is captured and Escape does nothing: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces "collapsed": aria-expanded is hard-coded to false: true positive

#### Run 2

- Outcome: scored
- Cost: 0.37 USD, 32 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-3
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:23-25` Tab is captured inside the open sort menu: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:21-30` Escape does not close the sort menu: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/SortMenu.tsx:34` aria-expanded is hard-coded to "false": true positive

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 22 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab cycles inside and Escape does nothing: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 32 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:23-25` Open sort menu traps Tab and ignores Escape: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces collapsed: true positive

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 30 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-3
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:23-25` Tab is captured inside the open sort menu: keyboard trap: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:21-30` The open menu cannot be closed with Escape: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/SortMenu.tsx:34` aria-expanded is hard-coded to "false" on the menu button: true positive

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.34 · 31 · 30 · 8 · 1 · 1177 · 27267 · 332505 · 2756

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 28 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target size and focus style not visible in the change: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 30 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Focus style and target size of the icon button are not visible in the change: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 28 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete icon target size and focus style not verifiable: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 33 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no text alternative: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target size and focus style to check: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 32 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable with the keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Icon button target size and focus style not verifiable: duplicate

### accessibility-control-custom-listbox

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.35 · 38 · 37 · 7 · 1 · 1187 · 29040 · 283162 · 3321

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Highlighted option relies on a class with no style in the change: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 34 s
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option relies on a stylesheet the change does not include: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 39 s
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Focus and highlighted-option styles are not part of the change: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Finding R-accessibility-1 [low / low] `src/components/SizePicker.tsx:92` Highlighted option relies on an unstyled class: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 38 s
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option and combobox focus rely on styles that are not in the repository: unreported (low confidence)

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.29 · 22 · 22 · 7 · 1 · 1185 · 26260 · 237011 · 1729

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 22 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 24 s

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 21 s

#### Run 4

- Outcome: scored
- Cost: 0.25 USD, 17 s

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 37 s

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.35 · 33 · 33 · 7 · 1 · 1171 · 28927 · 289901 · 2986

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 34 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour only, with no text and no programmatic state: true positive
- Finding R-accessibility-2 [medium / high] `design.md#Decision: Invalid fields turn red` Nothing tells a keyboard or screen reader user where the errors are on submit: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 37 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text: true positive
- Finding R-accessibility-2 [medium / low] `design.md#Decision: Invalid fields turn red` Nothing tells the customer about errors on submit once the top error list is removed: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by a red border only, with no text: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Submitting an invalid form no longer tells the user anything: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 27 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour only, with no text message: true positive
- Finding R-accessibility-2 [high / high] `proposal.md#What Changes` Removing the error summary leaves a failed submit silent: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 33 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Invalid fields are identified by colour only, with no text and no programmatic state: true positive
- Finding R-accessibility-2 [medium / low] `design.md#Decision: Invalid fields turn red` Submitting with errors gives no summary and no focus move once the top error list is removed: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.35 · 37 · 36 · 9 · 1 · 1189 · 28531 · 328584 · 2914

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 36 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible column drop with no down migration or backup: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8-9` Account JSON field renamed to userName without a BREAKING mark or compatibility path: true positive

#### Run 2

- Outcome: scored
- Cost: 0.39 USD, 37 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:1-2` Irreversible column drop with no down migration or backup step: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8-9` Public JSON field renamed user_name → userName with no BREAKING mark or transition: true positive

#### Run 3

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Public JSON field `user_name` renamed `userName` without a BREAKING mark or transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible `DROP COLUMN legacy_email` with no down migration, backup step or release note: true positive

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 35 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` JSON field `user_name` renamed `userName` without a BREAKING mark or transition: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` Irreversible `DROP COLUMN legacy_email` with no down migration or backup: true positive

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 49 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8` Renamed public JSON field breaks existing API consumers without warning: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` DROP COLUMN is irreversible, with no down migration and no backup step: true positive
- Finding R-migration-3 [medium / low] `migrations/0005_drop_legacy_email.sql:2` Column dropped in the same release as code, while old instances may still read it: duplicate

### migration-code-serde-rename-all

- Role: migration at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.33 · 28 · 27 · 8 · 1 · 1189 · 26970 · 328118 · 2471

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:5-6` rename_all = "camelCase" renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config.toml files without default_currency stop loading: true positive

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6` rename_all = "camelCase" renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:10` New required default_currency key makes existing config.toml fail to load: true positive

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 28 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6` camelCase renaming silently renames every existing order field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` New required `default_currency` key makes existing config files fail to load: true positive

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 35 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:6-10` camelCase renames every existing order JSON field: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` Existing config.toml files fail to load without default_currency: true positive

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 27 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2
- Finding R-migration-1 [high / high] `src/api.rs:6-10` rename_all camelCase silently renames the existing public JSON fields: true positive
- Finding R-migration-2 [medium / high] `src/config.rs:9-10` Existing config.toml files without default_currency no longer load: true positive

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.34 · 23 · 22 · 9 · 2 · 1173 · 28572 · 334389 · 1865

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 24 s

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 21 s

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 22 s

#### Run 4

- Outcome: scored
- Cost: 0.43 USD, 37 s

#### Run 5

- Outcome: scored
- Cost: 0.32 USD, 20 s

### migration-control-serde-alias

- Role: migration at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.29 · 22 · 22 · 6 · 0 · 1183 · 26342 · 186523 · 1848

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 16 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 22 s

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 19 s

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 30 s
- Finding R-migration-1 [low / high] `src/config.rs:10-11` No automated test that a pre-4.2 file with `listen` still loads: **false positive**
- Finding R-migration-2 [low / high] `src/api.rs:10-11` No automated test that the orders JSON still says `customer_id`: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 34 s
- Finding R-migration-1 [low / high] `src/config.rs:10-11` The `listen` alias and the `customer_id` JSON key have no automated test: **false positive**
- Finding R-migration-2 [medium / low] `config.example.toml:2` Rolling back to 4.1 fails on a config file already moved to `bind_address`: unreported (low confidence)

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.36 · 31 · 30 · 9 · 2 · 1173 · 28996 · 340273 · 2722

#### Run 1

- Outcome: scored
- Cost: 0.36 USD, 28 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in place, with no deprecation or side-by-side period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING, release note says nothing about what partners must do: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.39 USD, 30 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in place, no compatibility period for partners: true positive
- Finding R-migration-2 [medium / high] `proposal.md#Release note` Breaking rename not marked BREAKING, release note gives no action: duplicate

#### Run 3

- Outcome: scored
- Cost: 0.40 USD, 36 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed in place, with no transition for partner integrations: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Release note does not mark the rename BREAKING nor say what partners must do: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 29 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field rename removes `status` with no deprecation or side-by-side period: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` Breaking rename not marked BREAKING and the release note gives partners no action: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 31 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed with no deprecation or transition period: true positive
- Finding R-migration-2 [high / high] `proposal.md#What Changes` The rename is not marked BREAKING and the release note gives partners no action: duplicate

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.30 · 23 · 22 · 6 · 0 · 1173 · 26122 · 236152 · 2004

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs an unexplained bitmask threshold into a dense one-liner: true positive

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 22 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs its rule into an unreadable one-liner with a magic bitmask: true positive

#### Run 3

- Outcome: scored
- Cost: 0.31 USD, 28 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:8-10` `bucket` packs the premium rule into an unreadable bit-twiddling one-liner: true positive

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 22 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs the premium rule into an unreadable bitwise one-liner: true positive

#### Run 5

- Outcome: scored
- Cost: 0.30 USD, 23 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs its threshold into a bitwise one-liner nobody can read: true positive

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.30 · 20 · 19 · 7 · 0 · 1173 · 26527 · 238462 · 1612

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 34 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep, duplicates its seat loop and leaves its filters unexplained: true positive

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 20 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests five levels deep, duplicates its seat loop and leaves its filters unexplained: true positive

#### Run 3

- Outcome: scored
- Cost: 0.27 USD, 17 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:12-42` `import` nests six levels deep and hides its seat rules in unexplained conditions: true positive

#### Run 4

- Outcome: scored
- Cost: 0.28 USD, 18 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-39` `import` nests five levels deep and hides its rules in unexplained conditions: true positive

#### Run 5

- Outcome: scored
- Cost: 0.30 USD, 19 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:14-40` `import` nests six levels deep and hides its seat rules in unexplained filters: true positive

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.24 · 12 · 12 · 5 · 0 · 1193 · 24431 · 139864 · 882

#### Run 1

- Outcome: scored
- Cost: 0.25 USD, 12 s

#### Run 2

- Outcome: scored
- Cost: 0.23 USD, 11 s

#### Run 3

- Outcome: scored
- Cost: 0.24 USD, 12 s

#### Run 4

- Outcome: scored
- Cost: 0.24 USD, 15 s

#### Run 5

- Outcome: scored
- Cost: 0.25 USD, 12 s

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.35 · 35 · 34 · 9 · 1 · 1177 · 27365 · 316553 · 3328

#### Run 1

- Outcome: scored
- Cost: 0.36 USD, 35 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry loop with a fixed 1 s sleep, also on non-retryable errors: true positive
- Finding R-ops-3 [high / high] `app/payments.py:14` Retrying a non-idempotent charge POST without an idempotency key can charge an order twice: duplicate
- Finding R-ops-4 [high / high] `app/payments.py:17-18` Payment failures are swallowed without any log: duplicate
- Finding R-ops-5 [medium / low] `app/payments.py:7` Production payment URL hard-coded instead of read from configuration: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 33 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang the checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate charges: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Failed charge attempts are swallowed without any log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Payment provider URL hard-coded instead of read from configuration: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 31 s
- Expected `no-timeout` (timeout, high): found by R-ops-2
- Expected `unbounded-retry` (retry, high): found by R-ops-1
- Finding R-ops-1 [high / high] `app/payments.py:12-18` Charge is retried forever, without backoff, on a non-idempotent call: true positive
- Finding R-ops-2 [high / high] `app/payments.py:14` Payment call has no timeout: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Payment failures are swallowed without any log: duplicate
- Finding R-ops-4 [medium / low] `app/payments.py:7` Provider URL hard-coded instead of read from configuration: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 36 s
- Expected `no-timeout` (timeout, high): found by R-ops-2
- Expected `unbounded-retry` (retry, high): found by R-ops-1
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang the checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate charges and never ends: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Charge failures are swallowed with no log, so an operator cannot see them: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Production provider URL is hard-coded instead of read from configuration: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 37 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang the checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry loop with fixed one-second delay, including on permanent errors: true positive
- Finding R-ops-3 [high / high] `app/payments.py:14-18` Retrying a non-idempotent charge can charge the customer twice: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:17-18` Payment failures are swallowed without any log: duplicate
- Finding R-ops-5 [medium / high] `app/payments.py:7` Provider URL hard-coded instead of read from configuration: **false positive**

### ops-code-retry-without-backoff

- Role: ops at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.37 · 41 · 40 · 9 · 1 · 1195 · 28488 · 335325 · 3739

#### Run 1

- Outcome: scored
- Cost: 0.34 USD, 33 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook delivery retries immediately, with no backoff: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:17-24` Failed and dropped deliveries are logged as free text at debug level, HTTP failures not at all: true positive

#### Run 2

- Outcome: scored
- Cost: 0.37 USD, 41 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Retries fire back-to-back with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:21-24` A dropped webhook is only logged at debug level, in free text: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:33-35` Delivery retries block the payment event dispatch loop: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.41 USD, 50 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:20-25` A dropped webhook is logged only at debug level, in free text, through console: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:33-35` Delivery runs inside the payment event publish, holding it up to 25 seconds: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 43 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Webhook retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:17-25` Failed and dropped deliveries are invisible to operators: true positive

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 37 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Failed deliveries are retried immediately, with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:24` A dropped webhook is logged only with console.debug, in free text: true positive
- Finding R-ops-3 [medium / high] `src/webhooks.ts:17-22` Non-2xx responses are retried without any log, and attempt failures are free text: duplicate

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.29 · 22 · 21 · 6 · 0 · 1185 · 26114 · 235373 · 1754

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 23 s

#### Run 2

- Outcome: scored
- Cost: 0.29 USD, 21 s

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 19 s

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 35 s
- Finding R-ops-1 [medium / low] `src/rates.rs:28-37` Final fetch failure leaves no log or signal of its own: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.28 USD, 20 s

### ops-control-graceful-shutdown

- Role: ops at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.39 · 42 · 41 · 8 · 1 · 1189 · 30857 · 343133 · 3620

#### Run 1

- Outcome: scored
- Cost: 0.39 USD, 45 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-50` A send that times out after the relay accepted it is retried and duplicates the e-mail: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.42 USD, 42 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-50` A send that times out after the relay accepted it is retried and duplicates the e-mail: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.38 USD, 43 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-48` A send that times out after the relay took the message is retried and may be delivered twice: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 42 s
- Finding R-ops-1 [medium / low] `src/worker.rs:46-50` A send that times out after the relay accepted it is retried and duplicates the e-mail: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 37 s

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.34 · 38 · 37 · 8 · 2 · 1173 · 28586 · 339026 · 2433

#### Run 1

- Outcome: scored
- Cost: 0.37 USD, 29 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back once Stripe is closed: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody would see the settlement worker fail or not run: true positive

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 26 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The Paylane switch-over has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody sees the settlement worker fail: true positive

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 37 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back once Stripe is closed: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody would see the settlement worker fail or not run: true positive

#### Run 4

- Outcome: scored
- Cost: 0.40 USD, 47 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back once the Stripe account is closed: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The nightly settlement-worker can fail or not run without anyone noticing: true positive

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 42 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` Nobody would see the settlement worker fail: true positive

### performance-code-hidden-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.33 · 32 · 31 · 9 · 1 · 1187 · 28922 · 284238 · 2284

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 26 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:23-26` One customer query per invoice (N+1): true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45` Purchase-order regex compiled once per invoice: true positive

#### Run 2

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:38-39` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` The purchase-order regex is compiled once per invoice: true positive

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 34 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:39` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` The purchase-order regex is compiled again for every invoice: true positive

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 32 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:38-40` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:45` The purchase-order pattern is compiled once per invoice: true positive

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 25 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:38-40` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-45` The purchase-order regex is compiled once per invoice: true positive

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.30 · 18 · 18 · 8 · 1 · 1195 · 25428 · 325909 · 1466

#### Run 1

- Outcome: scored
- Cost: 0.30 USD, 18 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on the customer page: true positive

#### Run 2

- Outcome: scored
- Cost: 0.27 USD, 15 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order-lines query per order (N+1) on every customer page view: true positive

#### Run 3

- Outcome: scored
- Cost: 0.30 USD, 26 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One query per order to load its lines (N+1): true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 22 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` N+1 queries: one order_lines query per order: true positive

#### Run 5

- Outcome: scored
- Cost: 0.26 USD, 17 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One order_lines query per order (N+1) in order_summaries: true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.30 · 25 · 24 · 7 · 1 · 1185 · 26554 · 233906 · 2119

#### Run 1

- Outcome: scored
- Cost: 0.33 USD, 41 s
- Finding R-performance-1 [low / low] `src/cart.ts:18-19` The id list's bound rests on a validation not visible in the repository: unreported (low confidence)
- Finding R-performance-2 [low / high] `tasks.md#1. Implementation` The speed-up is not backed by a measurement: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.26 USD, 16 s

#### Run 3

- Outcome: scored
- Cost: 0.34 USD, 36 s
- Finding R-performance-1 [low / high] `tasks.md#1. Implementation` The speed gain is not backed by a measurement: **false positive**
- Finding R-performance-2 [low / low] `src/cart.ts:18-19` The cart-size bound the query relies on is not in the repository: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.30 USD, 25 s

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 19 s

### performance-control-preloaded-helper

- Role: performance at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.31 · 23 · 22 · 7 · 0 · 1183 · 28139 · 192225 · 1805

#### Run 1

- Outcome: scored
- Cost: 0.27 USD, 16 s

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 23 s

#### Run 3

- Outcome: scored
- Cost: 0.28 USD, 18 s

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 23 s

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 28 s
- Finding R-performance-1 [medium / low] `src/report.rs:25` No visible index backs the month filter the report makes hot: unreported (low confidence)

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.38 · 36 · 36 · 9 · 2 · 1173 · 29606 · 352200 · 2797

#### Run 1

- Outcome: scored
- Cost: 0.38 USD, 36 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Export loads the whole audit log into memory and builds the CSV on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index stated for the org_id filter and created_at sort: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.43 USD, 33 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Export loads the whole audit log into memory and builds the CSV synchronously: true positive

#### Run 3

- Outcome: scored
- Cost: 0.36 USD, 35 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log into memory and builds one CSV string on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 43 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log into memory and builds the CSV on the event loop: true positive
- Finding R-performance-2 [medium / high] `specs/audit-export/spec.md#Requirement: Administrators export the audit log` The requirement exports every event with no bound on the result set: duplicate
- Finding R-performance-3 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is stated for the export's filter and sort: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 37 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log into memory and builds it on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is shown for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.36 · 36 · 35 · 9 · 1 · 1177 · 28767 · 329399 · 3090

#### Run 1

- Outcome: scored
- Cost: 0.35 USD, 45 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-4
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: duplicate
- Finding R-privacy-3 [medium / low] `app/analytics.py:5` Beacon is a new processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `app/signup.py:13` No retention period or erasure path for the collected e-mail and display name: true positive
- Finding R-privacy-5 [low / low] `proposal.md#Domains` Purpose and lawful basis of the collected data not stated: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` Beacon is a new processor not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `app/signup.py:11-13` No purpose, lawful basis or retention stated for the collected e-mail and display name: unreported (low confidence) (at the location of `email-in-logs`, another category)

#### Run 3

- Outcome: scored
- Cost: 0.38 USD, 31 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` Beacon not identified as a processor, nor its location: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#What Changes` No lawful basis or retention stated for the collected e-mail address and display name: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.33 USD, 27 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `proposal.md#Domains` No purpose, lawful basis or retention stated for the collected e-mail and display name: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 37 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1` New processor Beacon not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [low / low] `proposal.md#Domains` Lawful basis and retention of sign-up data not stated: unreported (low confidence)

### privacy-code-error-message-pii

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.37 · 36 · 35 · 8 · 1 · 1187 · 29885 · 294743 · 3183

#### Run 1

- Outcome: scored
- Cost: 0.39 USD, 36 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:42-47` Chat vendor receives phone, birth date and address it does not need: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns the whole customer record: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:37-40` Helply as processor: transfer, retention and erasure not identified: duplicate
- Finding R-privacy-5 [low / high] `src/support.ts:32` Customer id written to the error log: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 27 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:40` Helply as processor and its transfer location are not identified: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Agent lookup returns birth date and address with no stated need: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 34 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:42-47` Chat vendor receives phone, birth date, address and e-mail to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns birth date and address with no stated need: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:37-40` Helply as a new processor is not identified: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 52 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Typed e-mail address written to the log platform on a failed lookup: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor Helply receives phone, birth date, address and e-mail to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns birth date, address and phone with no stated need: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `src/support.ts:37` Helply not identified as a processor, nor its data location: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.43 USD, 43 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Typed e-mail address written to the log platform through the error message: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date, address and e-mail when greeting needs a name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns birth date, phone and address with no stated need: unreported (low confidence)
- Finding R-privacy-4 [low / low] `proposal.md#Domains` Helply processor and its location not identified in the plan: unreported (low confidence)
- Finding R-privacy-5 [low / high] `src/support.ts:32` Customer id written to the log platform on a missing record: **false positive**

### privacy-code-python-soft-delete

- Role: privacy at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.37 · 41 · 39 · 8 · 1 · 1187 · 29771 · 295059 · 3334

#### Run 1

- Outcome: scored
- Cost: 0.40 USD, 41 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in the users row: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:8-13` New analytics pseudonym is never cut off when the account is deleted: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:17` No retention or backup rule for deleted accounts: duplicate

#### Run 2

- Outcome: scored
- Cost: 0.34 USD, 30 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Analytics pseudonym survives deletion and is re-identifiable with the salt: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.41 USD, 42 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in the users row: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:8-13` The new analytics id stays linkable after the account is deleted: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:16-23` Erasure from backups and the retention of deleted rows are not defined: duplicate

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 45 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's e-mail, name, phone and postal address: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Analytics events keyed by analytics_id are not erased with the account: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `app/account.py:17` No retention rule for deleted accounts and their backups: duplicate

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 28 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's personal data in `users`: true positive
- Finding R-privacy-2 [medium / low] `app/account.py:11-13` Erasure does not reach analytics events keyed by `analytics_id`: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `proposal.md#What Changes` Retention and erasure in backups are not defined: unreported (low confidence)

### privacy-control-ids-in-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.31 · 27 · 27 · 8 · 1 · 1205 · 27623 · 286107 · 1899

#### Run 1

- Outcome: scored
- Cost: 0.26 USD, 16 s

#### Run 2

- Outcome: scored
- Cost: 0.31 USD, 20 s

#### Run 3

- Outcome: scored
- Cost: 0.33 USD, 27 s

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 28 s

#### Run 5

- Outcome: scored
- Cost: 0.35 USD, 28 s

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.42 · 51 · 50 · 10 · 2 · 1203 · 31427 · 376629 · 4546

#### Run 1

- Outcome: scored
- Cost: 0.38 USD, 40 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with non-ASCII characters are logged in full or partly unmasked: **false positive**
- Finding R-privacy-2 [low / low] `src/logger.ts:5-7` The masked form keeps the domain and first letter: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 50 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:12-15` Field keys are written to the log unmasked: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:3` Addresses with non-ASCII characters escape the mask entirely: **false positive**
- Finding R-privacy-3 [low / low] `src/logger.ts:6-8` Percent-encoded addresses in logged URLs are not masked: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:5-7` The mask keeps the first letter and the full domain: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.42 USD, 49 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with non-ASCII or percent-encoded characters reach the logs unmasked: **false positive**
- Finding R-privacy-2 [low / low] `src/logger.ts:5-7` The masked form keeps the first letter and the full domain: unreported (low confidence)
- Finding R-privacy-3 [low / high] `src/logger.ts:12-15` Field names are logged without masking: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.47 USD, 57 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with a non-ASCII character are logged in full: **false positive**
- Finding R-privacy-2 [high / low] `src/logger.ts:6-8` Percent-encoded addresses in URLs are not masked: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `src/logger.ts:12-13` Field names are written to the log without masking: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:5` The masked form keeps the domain and first letter: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.45 USD, 53 s
- Finding R-privacy-1 [high / high] `src/logger.ts:3` Addresses with non-ASCII characters are logged in clear: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:3-7` URL-encoded addresses are not masked: **false positive**
- Finding R-privacy-3 [medium / high] `src/logger.ts:12-15` Field names are written to the log without masking: **false positive**
- Finding R-privacy-4 [low / low] `src/logger.ts:5-7` The masked form keeps the first letter and the full domain: unreported (low confidence)

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.42 · 41 · 41 · 9 · 2 · 1185 · 32227 · 361091 · 4033

#### Run 1

- Outcome: scored
- Cost: 0.42 USD, 40 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after the member opts out: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention period for the new data in nightly backups: duplicate
- Finding R-privacy-3 [medium / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly beyond what the greeting requires: true positive
- Finding R-privacy-4 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers to Textly in the United States has no stated safeguard: duplicate
- Finding R-privacy-5 [low / low] `proposal.md#Why` Lawful basis and purpose of the new data not stated: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.42 USD, 49 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Opting out neither erases the birthday and phone nor stops the greetings: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept indefinitely in nightly backups, with no expiry or erasure: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Phone numbers and names transferred to a US processor without a stated safeguard: true positive
- Finding R-privacy-4 [low / low] `proposal.md#Why` Purpose and lawful basis of the birthday SMS not stated: unreported (low confidence)
- Finding R-privacy-5 [low / low] `design.md#Risks / Trade-offs` What the job logs when Textly fails is not defined: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.39 USD, 37 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Opting out does not erase the birthday and phone number: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or purge rule for the copies in nightly backups: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer of phone numbers and names to Textly in the United States has no stated safeguard: true positive
- Finding R-privacy-4 [low / low] `proposal.md#Why` Lawful basis for marketing text messages not stated: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.45 USD, 47 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Opting out leaves the birthday and phone number stored: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention or purge for the copies in nightly backups: duplicate
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly beyond what delivery needs: true positive
- Finding R-privacy-4 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Transfer to a US processor without a stated safeguard or processor agreement: duplicate
- Finding R-privacy-5 [medium / low] `proposal.md#Why` Purpose and lawful basis not stated; marketing purpose mixed in: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 35 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-2
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after opt-out and in backups with no expiry: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name sent to Textly though the greeting does not need it: true positive
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` US transfer to Textly with no stated safeguard, processor terms or retention: duplicate
- Finding R-privacy-4 [low / low] `proposal.md#Why` Lawful basis for marketing birthday texts not stated: unreported (low confidence)

### security-code-detail-endpoint-authz

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.33 · 27 · 26 · 8 · 1 · 1195 · 28234 · 330006 · 2107

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 33 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:33-36` Any signed-in user can read any organization's invoice by id: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The error handler sends the error message and stack trace to the client: true positive

#### Run 2

- Outcome: scored
- Cost: 0.35 USD, 26 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends error messages and stack traces to clients: true positive

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 25 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends the error message and stack trace to the client: true positive

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 23 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` Any signed-in user can read any organization's invoice by id: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` The error handler sends the error message and stack trace to the client: true positive

#### Run 5

- Outcome: scored
- Cost: 0.33 USD, 29 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends the error message and stack trace to the client: true positive

### security-code-python-path-traversal

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.34 · 32 · 31 · 7 · 1 · 1193 · 28289 · 283339 · 2725

#### Run 1

- Outcome: scored
- Cost: 0.38 USD, 41 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment bypasses the ticket authorization: true positive
- Finding R-security-2 [medium / high] `app/attachments.py:13-17` An upload silently overwrites an existing attachment of the same name: **false positive**
- Finding R-security-3 [medium / low] `app/attachments.py:11` save_attachment takes no caller identity, so it enforces no authorization: unreported (low confidence)
- Finding R-security-4 [medium / low] `app/attachments.py:16-17` Uploaded data has no size or type limit: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.33 USD, 32 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:26-32` Path traversal in read_attachment bypasses the ticket authorization check: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-15` save_attachment enforces no authorization on the target ticket: unreported (low confidence)

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment bypasses the ticket check: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment checks no authorization: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.35 USD, 32 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment through the client-supplied name: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment takes no caller and checks no authorization: unreported (low confidence)
- Finding R-security-3 [medium / low] `app/attachments.py:16-21` An upload silently overwrites an earlier attachment of the same name: unreported (low confidence)

#### Run 5

- Outcome: scored
- Cost: 0.34 USD, 30 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:26-32` Path traversal in read_attachment bypasses the ticket authorization: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment performs no authorization check: unreported (low confidence)

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.29 · 20 · 20 · 6 · 0 · 1185 · 26697 · 236648 · 1538

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 18 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 3

- Outcome: scored
- Cost: 0.29 USD, 17 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term: true positive

#### Run 4

- Outcome: scored
- Cost: 0.32 USD, 23 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection in user search through the `q` query parameter: true positive

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 19 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term: true positive

### security-control-docs-only

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.32 · 30 · 29 · 9 · 1 · 1191 · 26574 · 371996 · 2224

#### Run 1

- Outcome: scored
- Cost: 0.32 USD, 25 s
- Finding R-security-1 [medium / low] `README.md:19-20` Revocation guarantee documented without enforcing code in the repository: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.32 USD, 26 s
- Finding R-security-1 [low / high] `README.md:18-20` Exposed-token rotation keeps the leaked token valid until redeploy: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.32 USD, 33 s
- Finding R-security-1 [low / low] `README.md:19-20` Revocation guarantee documented without code to back it: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.36 USD, 29 s
- Finding R-security-1 [low / high] `README.md:18-20` Exposed token stays valid until the new one is deployed: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.37 USD, 53 s
- Finding R-security-1 [low / low] `README.md:18-19` Exposed-token rotation keeps the leaked token valid until after redeploy: unreported (low confidence)

### security-control-dynamic-query

- Role: security at `code.review`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.29 · 18 · 17 · 6 · 0 · 1185 · 26002 · 234116 · 1430

#### Run 1

- Outcome: scored
- Cost: 0.29 USD, 20 s

#### Run 2

- Outcome: scored
- Cost: 0.26 USD, 15 s

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 42 s
- Finding R-security-1 [medium / low] `app/search.py:22` Search returns every ticket with no caller scope: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.29 USD, 18 s

#### Run 5

- Outcome: scored
- Cost: 0.27 USD, 16 s

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.36 · 36 · 35 · 8 · 2 · 1171 · 30441 · 292431 · 3080

#### Run 1

- Outcome: scored
- Cost: 0.36 USD, 35 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The middleware logs the Authorization header, writing every token to the logs: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be invalidated by any means: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 47 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header is logged at info level: true positive
- Finding R-security-3 [medium / high] `design.md#Risks / Trade-offs` A leaked token cannot be revoked and never expires: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.35 USD, 31 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, holding the bearer token, is logged: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be invalidated: revocation is deferred: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.38 USD, 38 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, and so every bearer token, is written to the logs: true positive
- Finding R-security-3 [medium / high] `design.md#Risks / Trade-offs` A leaked token can never be invalidated: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.36 USD, 30 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-2
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-1
- Finding R-security-1 [high / high] `design.md#Decision: Every request is logged with its headers` Request logging writes every bearer token to the logs: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-3 [low / low] `design.md#Decision: Tokens are stored as given` Token lookup by database equality may leak timing: duplicate

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.42 · 43 · 43 · 11 · 3 · 1175 · 33316 · 405497 · 3830

#### Run 1

- Outcome: scored
- Cost: 0.42 USD, 50 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` `--quiet` after the file is rejected, and no scenario says so: **false positive**
- Finding R-skeptic-2 [low / high] `tasks.md#1. Flag` The usage line printed by `ledger` is not updated: **false positive**
- Finding R-skeptic-3 [low / low] `proposal.md#Domains` Every domain is unticked with no reason: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.46 USD, 43 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` The flag's position on the command line is left unspecified: **false positive**
- Finding R-skeptic-2 [low / high] `tasks.md#1. Flag` Task 1.1 adds the flag to `FLAGS` before its documentation exists, leaving the docs test red: **false positive**
- Finding R-skeptic-3 [low / high] `src/main.rs:8` The usage line printed by `src/main.rs` is not updated to mention `--quiet`: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.45 USD, 42 s
- Finding R-skeptic-1 [low / high] `design.md#Decision: The progress reporter takes a `quiet` switch` The position of `--quiet` after the file is left unspecified: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.41 USD, 44 s

#### Run 5

- Outcome: scored
- Cost: 0.41 USD, 34 s

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.41 · 39 · 39 · 10 · 3 · 1177 · 31073 · 422752 · 3585

#### Run 1

- Outcome: scored
- Cost: 0.41 USD, 39 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format is out of scope: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains the plan touches are left unticked without a reason: **false positive**
- Finding R-skeptic-4 [low / low] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` Which invoices belong to a month is not defined: unreported (low confidence)

#### Run 2

- Outcome: scored
- Cost: 0.38 USD, 36 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Every domain unticked while the plan exposes invoice data and adds a UI page: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.43 USD, 44 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` A plugin system for export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `proposal.md#What Changes` A per-user settings page nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Every domain unticked with no reason, though the plan exposes customer data and adds a page: **false positive**

#### Run 4

- Outcome: scored
- Cost: 0.39 USD, 39 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats is scope nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `proposal.md#What Changes` Settings page for a preferred export format is scope nobody asked for and has no observable effect: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Every domain is unticked with no reason, though the plan exposes customer invoice data and changes a page: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.44 USD, 58 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin registry for formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Settings page for a preferred export format is out of scope: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` Domains all unticked though the plan exposes invoices and changes a UI page: **false positive**

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.51 · 55 · 55 · 11 · 3 · 1175 · 43015 · 406860 · 4888

#### Run 1

- Outcome: scored
- Cost: 0.39 USD, 39 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and differs from the design on case: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The nightly job's operations domain is unticked with no reason: **false positive**

#### Run 2

- Outcome: scored
- Cost: 0.51 USD, 47 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge rests on "one e-mail = one person", which the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Deleting the merged records is irreversible and the plan has no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The requirement leaves e-mail matching edge cases unspecified: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A new nightly job is deployed but the operations domain is unticked with no reason: **false positive**

#### Run 3

- Outcome: scored
- Cost: 0.48 USD, 56 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context refutes: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` Operations is unticked although the plan adds a nightly job: **false positive**
- Finding R-skeptic-5 [low / low] `proposal.md#Why` The plan leaves the source of duplicates in place: unreported (low confidence)

#### Run 4

- Outcome: scored
- Cost: 0.53 USD, 55 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Deleting the merged records is irreversible, with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` The new nightly job leaves the operations domain unticked: **false positive**

#### Run 5

- Outcome: scored
- Cost: 0.56 USD, 58 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge rests on "one e-mail = one person", which the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Losing records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path and leaves "share an e-mail address" undefined: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` A new nightly job is deployed with operations unticked and no reason: **false positive**
- Finding R-skeptic-5 [medium / high] `_codev/changes/merge-duplicate-customers/change.yaml:3-4` The privacy role is disabled for a change that merges and deletes personal data: **false positive**
- Finding R-skeptic-6 [low / low] `proposal.md#Why` A nightly clean-up leaves the cause of the duplicates in place: unreported (low confidence)

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.30 · 21 · 21 · 10 · 3 · 1092 · 25690 · 271079 · 1935

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 22 s

#### Run 2

- Outcome: scored
- Cost: 0.30 USD, 20 s

#### Run 3

- Outcome: scored
- Cost: 0.21 USD, 28 s

#### Run 4

- Outcome: scored
- Cost: 0.31 USD, 21 s

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 21 s

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Consumption (median): Cost (USD) | Duration (s) | API duration (s) | Turns | Denials | Input, uncached | Cache writes | Cache reads | Output
  0.31 · 30 · 29 · 10 · 2 · 1094 · 26456 · 275973 · 2405

#### Run 1

- Outcome: scored
- Cost: 0.31 USD, 24 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 2

- Outcome: scored
- Cost: 0.23 USD, 30 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 3

- Outcome: scored
- Cost: 0.37 USD, 35 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 4

- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

#### Run 5

- Outcome: scored
- Cost: 0.29 USD, 21 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

