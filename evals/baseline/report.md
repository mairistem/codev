# Role evaluation report

- Date: 2026-10-13
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 40
- Cost: 15.18 USD
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

One run per case: a score is a noisy point, compared with a baseline on the same model, not a proof.

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 5 (3/2) | 5 | 5 | 0 | 4 | 2 | 5 | 0 | 1.00 | 1.00 (16/0/0/0) | 5/5 | 1.94 |
| migration | 5 (3/2) | 5 | 5 | 0 | 5 | 1 | 4 | 0 | 1.00 | 1.00 (15/0/0/0) | 4/5 | 1.84 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 6 | 1 | 2 | 0 | 1.00 | 1.00 (11/0/0/0) | 0/2 | 1.05 |
| ops | 5 (3/2) | 6 | 6 | 0 | 11 | 5 | 4 | 0 | 1.00 | 1.00 (26/0/0/0) | 3/6 | 1.88 |
| performance | 5 (3/2) | 4 | 4 | 0 | 5 | 3 | 3 | 0 | 1.00 | 1.00 (15/0/0/0) | 1/4 | 1.84 |
| privacy | 6 (4/2) | 7 | 7 | 0 | 8 | 8 | 2 | 0 | 1.00 | 1.00 (25/0/0/0) | 4/7 | 2.48 |
| security | 6 (4/2) | 6 | 6 | 0 | 4 | 3 | 1 | 0 | 1.00 | 1.00 (14/0/0/0) | 6/6 | 2.16 |
| skeptic | 3 (2/1) | 3 | 3 | 0 | 9 | 2 | 3 | 0 | 1.00 | 1.00 (17/0/0/0) | 2/3 | 1.47 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.52 |

## Failures

- accessibility: 4 false positive(s) exceed 2
- migration: 5 false positive(s) exceed 2
- newcomer: 6 false positive(s) exceed 2
- ops: 11 false positive(s) exceed 2
- performance: 5 false positive(s) exceed 2
- privacy: 8 false positive(s) exceed 2
- security: 4 false positive(s) exceed 2
- skeptic: 9 false positive(s) exceed 2

## Per case

### accessibility-code-dropdown-keyboard-trap

- Role: accessibility at `code.review`, seeded
- Outcome: scored
- Cost: 0.37 USD, 30 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-2
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:21-30` Open sort menu traps keyboard focus: Tab cycles inside it and Escape does not close it: true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:34` Menu button always announces aria-expanded="false": true positive

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Outcome: scored
- Cost: 0.43 USD, 31 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete action is a clickable div, unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:14` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt attribute: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Delete target likely smaller than 24×24 CSS pixels: duplicate

### accessibility-control-custom-listbox

- Role: accessibility at `code.review`, control
- Outcome: scored
- Cost: 0.47 USD, 64 s
- Finding R-accessibility-1 [medium / high] `src/components/SizePicker.tsx:66` Custom combobox where a native select would do: **false positive**
- Finding R-accessibility-2 [medium / low] `src/components/SizePicker.tsx:86` Highlighted option has no visible style: unreported (low confidence)
- Finding R-accessibility-3 [medium / low] `src/components/SizePicker.tsx:79` Target size of the combobox and options is not set: unreported (low confidence)
- Finding R-accessibility-4 [low / high] `src/components/SizePicker.tsx:85` aria-selected marks the committed value, not the highlighted option: **false positive**
- Finding R-accessibility-5 [low / high] `src/components/SizePicker.tsx:11` Highlight is not reset to the current value when the list opens: **false positive**
- Finding R-accessibility-6 [low / high] `src/components/SizePicker.tsx:28-58` No type-ahead on the select-only combobox: **false positive**

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Outcome: scored
- Cost: 0.30 USD, 19 s

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.39 USD, 39 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Errors are conveyed by colour only, with no text message: true positive
- Finding R-accessibility-2 [high / high] `design.md#Decision: Invalid fields turn red` No programmatic invalid state or field–message association: duplicate
- Finding R-accessibility-3 [medium / high] `design.md#Decision: Invalid fields turn red` Removing the error summary leaves no feedback on submit: duplicate
- Finding R-accessibility-4 [medium / high] `specs/checkout-validation/spec.md#Scenario: An invalid postcode` Acceptance scenarios check only the border colour: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Outcome: scored
- Cost: 0.34 USD, 31 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-1
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-2
- Finding R-migration-1 [high / high] `migrations/0005_drop_legacy_email.sql:1-2` Irreversible DROP COLUMN with no down migration or backup: true positive
- Finding R-migration-2 [high / high] `src/api.rs:8-9` Account JSON field rename is a breaking API change not marked BREAKING: true positive
- Finding R-migration-3 [medium / high] `src/api.rs:8` Field removed in the same release as its replacement, no transition period: duplicate

### migration-code-serde-rename-all

- Role: migration at `code.review`, seeded
- Outcome: scored
- Cost: 0.40 USD, 40 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2, another severity
- Finding R-migration-1 [high / high] `src/api.rs:5-13` `rename_all = "camelCase"` renames every existing field of the public order JSON: true positive
- Finding R-migration-2 [high / high] `src/config.rs:9-10` `default_currency` is a required key: existing `config.toml` files no longer load: true positive
- Finding R-migration-3 [medium / high] `proposal.md#Release note` Release note announces neither the breaking changes nor the new configuration key: duplicate
- Finding R-migration-4 [medium / high] `tasks.md#1. Implementation` No test covers the migration path for old configuration files or API consumers: **false positive**
- Finding R-migration-5 [low / low] `src/report.rs:3-7` Public `report::SalesRow` and its field `count` renamed with no deprecation: unreported (low confidence)

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Outcome: scored
- Cost: 0.35 USD, 26 s

### migration-control-serde-alias

- Role: migration at `code.review`, control
- Outcome: scored
- Cost: 0.36 USD, 34 s
- Finding R-migration-1 [medium / high] `src/config.rs:10-11` A file holding both `listen` and `bind_address` no longer loads: **false positive**
- Finding R-migration-2 [medium / high] `src/config.rs:14-16` The old-key migration path has no automated test: **false positive**
- Finding R-migration-3 [low / high] `src/config.rs:8-10` The deprecated `listen` key is read silently, with no warning or removal plan: **false positive**

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.39 USD, 35 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Removing the public `status` field breaks partner integrations with no deprecation: true positive
- Finding R-migration-2 [high / high] `proposal.md#Release note` The breaking rename is not marked BREAKING nor tells users what to do: duplicate
- Finding R-migration-3 [high / high] `tasks.md#1. Rename` No rollout order keeping old and new consumers working side by side: duplicate
- Finding R-migration-4 [medium / high] `openapi.yaml:4` The API version is not bumped for a breaking change: **false positive**

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.35 USD, 39 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1, another severity
- Finding R-newcomer-1 [high / high] `src/pricing.ts:10` The premium test is a bit trick that does not mean "10 euros and more": true positive
- Finding R-newcomer-2 [low / high] `src/pricing.ts:9-10` `bucket` is a dense one-liner with a comma expression and `||=` inside an index: duplicate
- Finding R-newcomer-3 [low / high] `src/pricing.ts:9-10` Names say nothing: `bucket`, `ps`, `a`, `p`, `"hi"` / `"lo"`: duplicate
- Finding R-newcomer-4 [medium / high] `src/pricing.ts:10` The return type `Record<string, Price[]>` hides its two keys and their possible absence: **false positive** (at the location of `clever-one-liner`, another category)
- Finding R-newcomer-5 [medium / high] `tasks.md#1. Implementation` No test describes the split; the only verification is looking at a page: **false positive**

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.37 USD, 36 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1, another severity
- Finding R-newcomer-1 [low / high] `src/import.rs:14-24` `import` nests six levels deep: true positive
- Finding R-newcomer-2 [low / high] `src/import.rs:31-37` Seat-adding loop duplicated in both match arms: **false positive** (at the location of `deep-nesting`, another category)
- Finding R-newcomer-3 [medium / low] `src/import.rs:18` Seats are only added to an existing account when its plan differs, without saying why: unreported (low confidence) (at the location of `deep-nesting`, another category)
- Finding R-newcomer-4 [low / high] `src/import.rs:15-21` Unexplained filters: `contains('@')` and empty email skip silently: **false positive** (at the location of `deep-nesting`, another category)
- Finding R-newcomer-5 [low / high] `tasks.md#1. Implementation` No test describes the import behavior: **false positive**

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Outcome: scored
- Cost: 0.33 USD, 25 s
- Finding R-newcomer-1 [low / high] `app/invoices.py:5-10` Docstring restates the payment term instead of naming the constant: **false positive**

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Outcome: scored
- Cost: 0.33 USD, 28 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge, without backoff: true positive
- Finding R-ops-3 [high / high] `app/payments.py:17-18` Charge failures are swallowed with no log or metric: duplicate
- Finding R-ops-4 [low / high] `app/payments.py:7` Provider URL is hard-coded: **false positive**
- Finding R-ops-5 [medium / low] `app/payments.py:10-11` No switch to turn automatic charging off: unreported (low confidence) (at the location of `unbounded-retry`, another category)

### ops-code-retry-without-backoff

- Role: ops at `code.review`, seeded
- Outcome: scored
- Cost: 0.40 USD, 43 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-3, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-23` Retries fire back to back, with no backoff: true positive
- Finding R-ops-2 [medium / high] `src/webhooks.ts:17-19` Every non-2xx response is retried and never logged: duplicate
- Finding R-ops-3 [high / high] `src/webhooks.ts:24` Dropped deliveries are logged only at debug level, through console: true positive
- Finding R-ops-4 [medium / high] `src/webhooks.ts:33-35` Delivery runs inline in the payment event publish, blocking it for up to ~25 s: **false positive**
- Finding R-ops-5 [medium / high] `src/webhooks.ts:28` No switch to turn webhooks off without a redeploy: **false positive**
- Finding R-ops-6 [low / high] `src/webhooks.ts:4` Timeout and attempt limit are hard-coded: duplicate

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Outcome: scored
- Cost: 0.33 USD, 30 s
- Finding R-ops-1 [medium / low] `src/rates.rs:37` Final fetch failure is returned but never logged or counted: unreported (low confidence)
- Finding R-ops-2 [low / high] `src/rates.rs:18-37` Errors flattened to String, losing their source chain: **false positive**
- Finding R-ops-3 [low / high] `src/rates.rs:26` Permanent HTTP errors (4xx) are retried like transient ones: **false positive**
- Finding R-ops-4 [low / low] `src/rates.rs:28` Feed URL embedded in error messages: unreported (low confidence)

### ops-control-graceful-shutdown

- Role: ops at `code.review`, control
- Outcome: scored
- Cost: 0.44 USD, 50 s
- Finding R-ops-1 [high / high] `src/worker.rs:21-22` The worker exposes no health check, no metric and no start log: **false positive**
- Finding R-ops-2 [medium / high] `src/worker.rs:35-38` Temporary failures are retried forever, never dead-lettered: **false positive**
- Finding R-ops-3 [medium / low] `src/worker.rs:26` A send that times out after the relay accepted the message is retried and may duplicate the e-mail: unreported (low confidence)
- Finding R-ops-4 [medium / high] `src/worker.rs:38` The "relay unavailable" warning carries no cause: **false positive**
- Finding R-ops-5 [low / high] `src/worker.rs:11-17` Poll, timeout and retry settings are hard-coded: **false positive**

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.38 USD, 37 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-3, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` No way to turn Paylane off or fall back to Stripe: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: All traffic moves at deploy` Closing the Stripe account removes the redeploy rollback too: duplicate
- Finding R-ops-3 [high / high] `design.md#Decision: A worker settles payments nightly` Settlement worker has no health, metrics or failure alert: true positive
- Finding R-ops-4 [medium / high] `specs/payments/spec.md#Requirement: Card payments go through Paylane` No timeout or retry policy for Paylane calls: **false positive**
- Finding R-ops-5 [medium / high] `specs/payments/spec.md#Scenario: A declined card` Paylane errors other than a decline are not specified: **false positive**
- Finding R-ops-6 [low / low] `proposal.md#Impact` Paylane credentials and endpoint configuration are unplanned: unreported (low confidence)

### performance-code-hidden-n-plus-one

- Role: performance at `code.review`, seeded
- Outcome: scored
- Cost: 0.43 USD, 41 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1, another severity
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [high / high] `src/report.rs:23-26` One customer query per invoice (N+1): true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-45` The purchase-order regex is compiled once per invoice: true positive
- Finding R-performance-3 [low / high] `Cargo.toml:8` The regex crate is added for one fixed-shape pattern: **false positive**
- Finding R-performance-4 [low / high] `src/report.rs:23-25` Invoice numbers are cloned while the invoices are dropped: duplicate

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Outcome: scored
- Cost: 0.30 USD, 20 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` N+1 query: one order_lines query per order: true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Outcome: scored
- Cost: 0.30 USD, 22 s

### performance-control-preloaded-helper

- Role: performance at `code.review`, control
- Outcome: scored
- Cost: 0.42 USD, 54 s
- Finding R-performance-1 [low / high] `src/report.rs:43-46` The month total costs a second scan of the month's invoices: **false positive**
- Finding R-performance-2 [low / high] `Cargo.toml:8` The `regex` crate is added for one fixed-shape pattern: **false positive**
- Finding R-performance-3 [low / high] `src/report.rs:51-55` The customer label is formatted once per invoice, not once per customer: **false positive**
- Finding R-performance-4 [low / high] `src/report.rs:35-37` Invoice numbers are cloned while the invoices are dropped right after: **false positive**
- Finding R-performance-5 [low / low] `src/report.rs:25` The month's invoices are loaded without a bound, on a filter whose index is not visible: unreported (low confidence)

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.39 USD, 52 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` The export loads the whole audit log of an organization into memory: true positive
- Finding R-performance-2 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Building the whole CSV string blocks the event loop: duplicate
- Finding R-performance-3 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is planned for the filter and sort of the export query: unreported (low confidence) (at the location of `unbounded-export`, another category)
- Finding R-performance-4 [medium / high] `specs/audit-export/spec.md#Requirement: Administrators export the audit log` The spec requires an unbounded export with no performance criterion: duplicate
- Finding R-performance-5 [low / low] `design.md#Decision: The export loads every event, then writes the CSV` `SELECT *` reads every column for a fixed CSV: unreported (low confidence) (at the location of `unbounded-export`, another category)

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.46 USD, 34 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-4
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to the application log: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS when the user id suffices: duplicate
- Finding R-privacy-3 [medium / low] `app/analytics.py:5` New third-party processor Beacon is not identified in the plan: unreported (low confidence)
- Finding R-privacy-4 [medium / high] `app/signup.py:13-15` No retention or erasure defined for the new member data and its copies: true positive
- Finding R-privacy-5 [low / high] `proposal.md#Domains` Purpose and lawful basis of the collected data are not stated: **false positive**

### privacy-code-error-message-pii

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.45 USD, 46 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the logs through the error message: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:42-47` Chat vendor receives phone, birth date and address when only the name is needed: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:37-40` Helply is a new processor with no transfer, retention or erasure stated: duplicate
- Finding R-privacy-4 [medium / low] `src/support.ts:16` Support lookup returns birth date, phone and address with no stated purpose: unreported (low confidence)

### privacy-code-python-soft-delete

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.38 USD, 34 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps every personal field of the member: true positive
- Finding R-privacy-2 [medium / high] `tests/test_account.py:10-11` Tests do not check that personal data is erased: **false positive**
- Finding R-privacy-3 [medium / low] `app/account.py:11-13` Erasure does not reach analytics data linked by analytics_id: unreported (low confidence)
- Finding R-privacy-4 [medium / low] `proposal.md#What Changes` Retention of deleted accounts and backups is not defined: unreported (low confidence)

### privacy-control-ids-in-logs

- Role: privacy at `code.review`, control
- Outcome: scored
- Cost: 0.37 USD, 32 s

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Outcome: scored
- Cost: 0.40 USD, 58 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:6-8` Masked address still identifies the person: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:3` Encoded or non-ASCII addresses are not masked: unreported (low confidence)
- Finding R-privacy-3 [low / low] `src/logger.ts:12-13` Field names are written without masking: unreported (low confidence)
- Finding R-privacy-4 [low / high] `src/logger.test.ts:4-11` Field masking is not covered by a test: **false positive**
- Finding R-privacy-5 [medium / high] `proposal.md#Why` Addresses already in the 90-day logs are left in place: **false positive**
- Finding R-privacy-6 [low / low] `proposal.md#Why` Hosting provider as log processor is not identified: unreported (low confidence)

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.41 USD, 53 s
- Expected `no-retention` (retention, medium): found by R-privacy-2
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `design.md#Decision: Greetings are sent by SMS through Textly` Phone numbers and names go to a US processor with no transfer safeguard identified: true positive
- Finding R-privacy-2 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone are kept for the profile's lifetime, including after opt-out, and in backups with no expiry: true positive
- Finding R-privacy-3 [medium / high] `specs/birthday-greetings/spec.md#Requirement: Members opt in to birthday greetings` No requirement for opting out or erasing the new data: **false positive**
- Finding R-privacy-4 [medium / high] `proposal.md#Why` Purpose and lawful basis of the new data are not stated: **false positive**
- Finding R-privacy-5 [low / high] `design.md#Decision: Greetings are sent by SMS through Textly` Display name is sent to Textly where the phone number alone would do: **false positive** (at the location of `processor-transfer`, another category)
- Finding R-privacy-6 [low / low] `design.md#Risks / Trade-offs` No rule keeps phone numbers out of the job's logs and retry state: unreported (low confidence)

### security-code-detail-endpoint-authz

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 27 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler sends the exception message and stack trace to the client: true positive

### security-code-python-path-traversal

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 40 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:28-31` Path traversal in read_attachment bypasses the ticket authorization: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-12` save_attachment checks no authorization: unreported (low confidence)
- Finding R-security-3 [medium / high] `app/attachments.py:13-17` Uploaded file name is not allow-listed and overwrites existing attachments: **false positive**

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.31 USD, 24 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term: true positive
- Finding R-security-2 [low / high] `src/users.ts:10` LIKE wildcards in the term are not escaped: duplicate

### security-control-docs-only

- Role: security at `code.review`, control
- Outcome: scored
- Cost: 0.43 USD, 46 s
- Finding R-security-1 [medium / high] `README.md:13-20` Exposed token stays valid while the new one is deployed: **false positive**
- Finding R-security-2 [low / high] `README.md:13-14` Non-expiring tokens documented without any routine rotation: **false positive**
- Finding R-security-3 [medium / low] `README.md:16-20` Security guarantees asserted with no code in the repository backing them: unreported (low confidence)

### security-control-dynamic-query

- Role: security at `code.review`, control
- Outcome: scored
- Cost: 0.31 USD, 19 s

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.39 USD, 35 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, carrying the bearer token, is logged at info level: true positive
- Finding R-security-3 [medium / high] `design.md#Goals / Non-Goals` A leaked token cannot be revoked and never expires: **false positive**
- Finding R-security-4 [medium / low] `specs/api-tokens/spec.md#Requirement: A user creates an API token` Token creation is not restricted to session authentication: unreported (low confidence)

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Outcome: scored
- Cost: 0.40 USD, 40 s
- Finding R-skeptic-1 [medium / high] `tasks.md#1. Flag` Task 1.2's check passes even if `--quiet` is never declared: **false positive**
- Finding R-skeptic-2 [medium / high] `specs/cli-import/spec.md#Scenario: A quiet import` The "A quiet import" scenario passes without `--quiet` on a small file: **false positive**
- Finding R-skeptic-3 [low / low] `design.md#Decision: The progress reporter takes a `quiet` switch` The plan does not say how `--quiet` gets from the command line to `run`: unreported (low confidence)

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.47 USD, 40 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future formats is scope nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `proposal.md#What Changes` Per-user export format preference is scope nobody asked for: duplicate
- Finding R-skeptic-3 [low / high] `design.md#Decision: Exports go through a plugin registry` Design rejects the simple solution only for an extensibility nobody needs: duplicate
- Finding R-skeptic-4 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Preference scenario has no observable effect: the route hardcodes CSV: **false positive** (at the location of `scope-creep`, another category)
- Finding R-skeptic-5 [medium / high] `design.md#Decision: Exports go through a plugin registry` Storing a per-user preference is a hidden schema migration: **false positive** (at the location of `scope-creep`, another category)
- Finding R-skeptic-6 [medium / low] `proposal.md#Domains` No access-control case for an export of every invoice of a month: unreported (low confidence)
- Finding R-skeptic-7 [low / high] `specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV` Date and total formats of the CSV are unspecified: **false positive**
- Finding R-skeptic-8 [low / high] `tasks.md#1. Export` Registry task is verified by a test no requirement defines: duplicate

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.60 USD, 49 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge rests on "one e-mail = one person", which the design itself refutes: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Records are deleted with no rollback, dry run or audit trail: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `design.md#Decision: Customers are merged by e-mail` What happens to the merged records' name and billing address is unspecified: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-5 [low / high] `design.md#Decision: Customers are merged by e-mail` The cause of the duplicates stays in place, and the simpler fix is not weighed: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-6 [low / high] `tasks.md#1. Merge` A single task with only the two happy-path tests: **false positive**

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Outcome: scored
- Cost: 0.21 USD, 21 s

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Outcome: scored
- Cost: 0.31 USD, 24 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

