# Role evaluation report

- Date: 2026-10-13
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 26
- Cost: 9.62 USD
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

One run per case: a score is a noisy point, compared with a baseline on the same model, not a proof.

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 3 (2/1) | 3 | 3 | 0 | 3 | 1 | 3 | 0 | 1.00 | 1.00 (10/0/0/0) | 2/3 | 1.10 |
| migration | 3 (2/1) | 3 | 3 | 0 | 1 | 1 | 3 | 0 | 1.00 | 1.00 (8/0/0/0) | 3/3 | 1.10 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 2 | 0 | 5 | 0 | 1.00 | 1.00 (9/0/0/0) | 0/2 | 1.09 |
| ops | 3 (2/1) | 4 | 4 | 0 | 5 | 4 | 1 | 0 | 1.00 | 1.00 (14/0/0/0) | 3/4 | 1.11 |
| performance | 3 (2/1) | 2 | 2 | 0 | 1 | 1 | 4 | 0 | 1.00 | 1.00 (8/0/0/0) | 1/2 | 1.07 |
| privacy | 3 (2/1) | 4 | 4 | 0 | 6 | 4 | 2 | 0 | 1.00 | 1.00 (16/0/0/0) | 1/4 | 1.30 |
| security | 3 (2/1) | 3 | 3 | 0 | 2 | 1 | 0 | 0 | 1.00 | 1.00 (6/0/0/0) | 3/3 | 1.02 |
| skeptic | 3 (2/1) | 2 | 2 | 0 | 8 | 3 | 6 | 0 | 1.00 | 1.00 (19/0/0/0) | 2/2 | 1.33 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.50 |

## Failures

- accessibility: 3 false positive(s) exceed 2
- ops: 5 false positive(s) exceed 2
- privacy: 6 false positive(s) exceed 2
- skeptic: 8 false positive(s) exceed 2

## Per case

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-1, another severity
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:14` Icon-only delete control has no accessible name: true positive
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner's avatar image has no alt attribute: duplicate
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13-14` Delete target may be smaller than 24 by 24 CSS pixels: duplicate

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Outcome: scored
- Cost: 0.41 USD, 43 s
- Finding R-accessibility-1 [medium / high] `templates/newsletter.html:10-15` Hidden error message is always read as the field's description: **false positive**
- Finding R-accessibility-2 [medium / low] `templates/newsletter.html:1` Native validation disabled with nothing in the change to reveal the error: unreported (low confidence)
- Finding R-accessibility-3 [low / high] `templates/newsletter.html:1-2` Form landmark has no accessible name although its heading has an id: **false positive**

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.34 USD, 28 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour only: true positive
- Finding R-accessibility-2 [high / high] `proposal.md#What Changes` Removing the error summary leaves no announced feedback on submit: **false positive**
- Finding R-accessibility-3 [medium / high] `specs/checkout-validation/spec.md#Scenario: An invalid postcode` Spec scenarios only verify the border colour: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Outcome: scored
- Cost: 0.33 USD, 28 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:1-2` Irreversible DROP COLUMN with no down migration or backup: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8-9` Public JSON field renamed with no BREAKING mark: true positive
- Finding R-migration-3 [medium / high] `proposal.md#Release note` Release note announces the rename as a plain change: duplicate
- Finding R-migration-4 [medium / high] `src/api.rs:8` Rename shipped without a deprecation or side-by-side period: duplicate
- Finding R-migration-5 [low / low] `tasks.md#1. Implementation` No test covers the migration path: unreported (low confidence)

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Outcome: scored
- Cost: 0.41 USD, 25 s

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.36 USD, 31 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `proposal.md#Release note` The breaking rename of a public field is not marked BREAKING: true positive
- Finding R-migration-2 [high / high] `design.md#Decision: The field is renamed in place` The field is removed in the same release that introduces its replacement, with no deprecation: duplicate
- Finding R-migration-3 [medium / high] `tasks.md#1. Rename` No migration path for existing consumers is planned or tested: **false positive**

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 34 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1, another severity
- Finding R-newcomer-1 [high / high] `src/pricing.ts:9-10` bucket hides the premium threshold in a bitmask one-liner that does not compute it: true positive
- Finding R-newcomer-2 [low / high] `src/pricing.ts:9-10` Obscure names and untyped string keys in bucket: duplicate
- Finding R-newcomer-3 [medium / high] `tasks.md#1. Implementation` The verification test cited by the task does not exist: **false positive**
- Finding R-newcomer-4 [low / high] `src/pricing.ts:1-6` Module comment reads as a comment on the interface: **false positive**

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.46 USD, 46 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1, another severity
- Finding R-newcomer-1 [low / high] `src/import.rs:14-41` `import` nests six levels deep: true positive
- Finding R-newcomer-2 [low / high] `src/import.rs:21-32` Seat filtering on `'@'` is duplicated and unexplained: duplicate
- Finding R-newcomer-3 [medium / low] `src/import.rs:17-18` Seats are added to an existing account only when its plan differs, with no explanation: duplicate
- Finding R-newcomer-4 [low / high] `src/import.rs:15` Rows with an empty email are skipped silently: duplicate
- Finding R-newcomer-5 [medium / high] `src/import.rs:11-12` No test describes the import behavior: duplicate

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Outcome: scored
- Cost: 0.27 USD, 16 s

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 33 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout and can hang the checkout forever: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge can duplicate payments: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Failures are swallowed silently and never surface to the caller or the logs: duplicate
- Finding R-ops-4 [low / high] `app/payments.py:7` Provider URL is hard-coded: **false positive**

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Outcome: scored
- Cost: 0.37 USD, 38 s
- Finding R-ops-1 [medium / low] `src/rates.rs:37` Final fetch failure is neither logged nor counted, so stale rates can go unnoticed: unreported (low confidence)
- Finding R-ops-2 [low / high] `src/rates.rs:26` Every error is retried, including permanent HTTP 4xx responses: **false positive**
- Finding R-ops-3 [low / high] `src/rates.rs:18` Errors flattened to String lose their source chain: **false positive**
- Finding R-ops-4 [low / low] `src/rates.rs:28` Feed URL copied verbatim into error messages: unreported (low confidence)
- Finding R-ops-5 [low / high] `src/rates.rs:21` Timeout, attempt count and initial pause are hard-coded: **false positive**

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.37 USD, 44 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` No way back from the switch to Paylane: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` A failed or missed nightly settlement goes unnoticed: true positive
- Finding R-ops-3 [medium / low] `specs/payments/spec.md#Requirement: Card payments go through Paylane` No timeout, retry or idempotency policy for Paylane calls: unreported (low confidence)
- Finding R-ops-4 [medium / high] `specs/payments/spec.md#Scenario: A payment without order` Unmatched payments are written to a table nobody watches: **false positive**
- Finding R-ops-5 [low / low] `proposal.md#Impact` Paylane configuration and credentials are not specified: unreported (low confidence)

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Outcome: scored
- Cost: 0.33 USD, 26 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` N+1 query: one `lines_of_order` query per order: true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Outcome: scored
- Cost: 0.32 USD, 33 s
- Finding R-performance-1 [low / high] `proposal.md#Release note` The speed claim is not backed by a measurement: **false positive**
- Finding R-performance-2 [low / low] `src/cart.ts:18-19` The 100-item cart bound the query relies on is not found in the code: unreported (low confidence)

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.42 USD, 39 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the organization's whole audit log into memory: true positive
- Finding R-performance-2 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Building the CSV with map/join blocks the single event loop: duplicate
- Finding R-performance-3 [medium / high] `specs/audit-export/spec.md#Requirement: Administrators export the audit log` The requirement mandates an unbounded result with no range: duplicate
- Finding R-performance-4 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is named for the export's filter and sort: duplicate
- Finding R-performance-5 [low / high] `design.md#Decision: The export loads every event, then writes the CSV` The approach is chosen without weighing alternatives or measuring: duplicate

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.48 USD, 45 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the third-party analytics vendor Beacon: true positive
- Finding R-privacy-3 [medium / low] `app/analytics.py:1-5` New processor Beacon not identified, nor its location or contract: unreported (low confidence)
- Finding R-privacy-4 [medium / high] `app/signup.py:13` No retention period nor erasure path for the stored e-mail and display name: duplicate
- Finding R-privacy-5 [low / high] `proposal.md#Domains` Purpose and lawful basis of the collected data not stated: **false positive**

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Outcome: scored
- Cost: 0.40 USD, 43 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:6-8` Masked address keeps the full domain and first letter: **false positive**
- Finding R-privacy-2 [medium / high] `src/logger.ts:3` Addresses the pattern misses still reach the logs in full or in part: **false positive**
- Finding R-privacy-3 [low / low] `src/logger.ts:12-15` Field keys are written unmasked: unreported (low confidence)
- Finding R-privacy-4 [low / high] `src/logger.test.ts:4-11` No test proves that log fields are masked: **false positive**
- Finding R-privacy-5 [medium / low] `proposal.md#Why` Logs already written keep full addresses for 90 days: unreported (low confidence)

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.42 USD, 40 s
- Expected `no-retention` (retention, medium): found by R-privacy-2
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `design.md#Decision: Greetings are sent by SMS through Textly` Phone and display name sent to a US processor with no transfer basis: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` No retention period or purge for birth date and phone, backups included: true positive
- Finding R-privacy-3 [medium / high] `specs/birthday-greetings/spec.md#Requirement: Members give their birth date and phone number` Birth date and phone made mandatory for a marketing purpose, no lawful basis stated: **false positive**
- Finding R-privacy-4 [low / high] `proposal.md#What Changes` Full birth date collected when only day and month are needed: **false positive**
- Finding R-privacy-5 [medium / high] `design.md#Decision: Birth dates live on the profile` Data subject rights not addressed for the new data and its copies: duplicate
- Finding R-privacy-6 [low / low] `design.md#Risks / Trade-offs` No rule keeping phone numbers out of the job's logs and retries: unreported (low confidence)

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.30 USD, 21 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10` SQL injection through the search term (CWE-89): true positive

### security-control-docs-only

- Role: security at `code.review`, control
- Outcome: scored
- Cost: 0.38 USD, 33 s
- Finding R-security-1 [medium / low] `README.md:19-20` Documented revocation guarantee has no implementation in this repository: unreported (low confidence)
- Finding R-security-2 [low / high] `README.md:13-14` Non-expiring tokens documented without a periodic rotation recommendation: **false positive**

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.34 USD, 25 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, and so every bearer token, is written to the logs: true positive
- Finding R-security-3 [medium / high] `proposal.md#What Changes` Revocation is announced but neither specified nor planned: **false positive**

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Outcome: scored
- Cost: 0.43 USD, 54 s
- Finding R-skeptic-1 [high / high] `proposal.md#Impact` The plan targets CLI files that do not exist in this repository: **false positive**
- Finding R-skeptic-2 [medium / high] `design.md#Context` The design rests on an unverified description of the progress and error paths: **false positive**
- Finding R-skeptic-3 [medium / high] `tasks.md#1. Flag` Task 1.2 is verified by a test that does not exist and that no task creates: **false positive**
- Finding R-skeptic-4 [low / high] `specs/cli-import/spec.md#Scenario: A quiet import` The scenario "nothing is printed" is broader than the requirement: **false positive**
- Finding R-skeptic-5 [low / low] `specs/cli-import/spec.md#Requirement: `--quiet` silences progress` Warnings are classified neither as progress nor as errors: unreported (low confidence)

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.43 USD, 49 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats is scope creep: true positive
- Finding R-skeptic-2 [medium / high] `design.md#Decision: Exports go through a plugin registry` The simpler solution was dismissed only because of speculative extensibility: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#What Changes` Per-user export-format settings page is scope creep: duplicate
- Finding R-skeptic-4 [medium / high] `proposal.md#Impact` The impacted frontend `web/src/settings/` does not exist in the project: **false positive**
- Finding R-skeptic-5 [medium / high] `design.md#Decision: Exports go through a plugin registry` Storing each user's preferred format is a hidden migration: duplicate
- Finding R-skeptic-6 [low / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` The preference requirement is not verifiable against any route: duplicate
- Finding R-skeptic-7 [high / low] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` No access control on an endpoint that exposes every invoice and customer: unreported (low confidence)
- Finding R-skeptic-8 [medium / high] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` Happy path only: CSV escaping, missing month and empty month are unspecified: **false positive**

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.48 USD, 66 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge key rests on an assumption the design itself contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Losing records are deleted outright, with no way back: duplicate
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The specs only cover the happy path: duplicate
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` No domain is ticked although the job deletes customers' personal data: **false positive**
- Finding R-skeptic-5 [medium / low] `proposal.md#Impact` The plan assumes customer and order tables the code does not show: unreported (low confidence)
- Finding R-skeptic-6 [low / high] `tasks.md#1. Merge` The task is verified only by the two happy-path tests: **false positive**

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Outcome: scored
- Cost: 0.24 USD, 21 s

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Outcome: scored
- Cost: 0.26 USD, 23 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

