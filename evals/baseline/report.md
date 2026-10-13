# Role evaluation report

- Date: 2026-10-13
- Model: Claude Code's default
- Claude Code: 2.1.289 (Claude Code)
- codev: codev 0.6.1
- Cases: 40
- Cost: 14.23 USD
- Thresholds: recall ≥ 0.50, false positives ≤ 2, evidence validity ≥ 0.80
- Verdict: **fail**

One run per case: a score is a noisy point, compared with a baseline on the same model, not a proof.

## Per role

| Role | Cases (seeded/control) | Expected | True positives | Misses | False positives | Unreported | Duplicates | Cross-role | Recall | Evidence validity (found/moved/invalid/unchecked) | Severity agreement | Cost (USD) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| accessibility | 5 (3/2) | 5 | 5 | 0 | 0 | 2 | 4 | 0 | 1.00 | 1.00 (11/0/0/0) | 5/5 | 1.89 |
| migration | 5 (3/2) | 5 | 5 | 0 | 0 | 0 | 1 | 0 | 1.00 | 1.00 (6/0/0/0) | 5/5 | 1.70 |
| newcomer | 3 (2/1) | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 1.00 | 1.00 (2/0/0/0) | 2/2 | 0.83 |
| ops | 5 (3/2) | 6 | 6 | 0 | 1 | 1 | 1 | 0 | 1.00 | 1.00 (9/0/0/0) | 3/6 | 1.97 |
| performance | 5 (3/2) | 4 | 4 | 0 | 0 | 1 | 0 | 0 | 1.00 | 1.00 (5/0/0/0) | 2/4 | 1.76 |
| privacy | 6 (4/2) | 7 | 7 | 0 | 1 | 8 | 1 | 0 | 1.00 | 1.00 (17/0/0/0) | 4/7 | 2.24 |
| security | 6 (4/2) | 6 | 6 | 0 | 0 | 2 | 0 | 0 | 1.00 | 1.00 (8/0/0/0) | 6/6 | 2.04 |
| skeptic | 3 (2/1) | 3 | 3 | 0 | 5 | 0 | 1 | 0 | 1.00 | 1.00 (9/0/0/0) | 2/3 | 1.31 |
| tracer | 2 (1/1) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 1.00 | — (0/0/0/0) | 1/1 | 0.49 |

## Failures

- skeptic: 5 false positive(s) exceed 2

## Per case

### accessibility-code-dropdown-keyboard-trap

- Role: accessibility at `code.review`, seeded
- Outcome: scored
- Cost: 0.46 USD, 48 s
- Expected `tab-keyboard-trap` (keyboard-trap, high): found by R-accessibility-1
- Expected `expanded-state-hard-coded` (state-not-exposed, medium): found by R-accessibility-3
- Finding R-accessibility-1 [high / high] `src/components/SortMenu.tsx:23-25` Tab is captured inside the open sort menu (keyboard trap): true positive
- Finding R-accessibility-2 [medium / high] `src/components/SortMenu.tsx:26-29` Escape does not close the sort menu: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/SortMenu.tsx:34` aria-expanded is hard-coded to "false": true positive
- Finding R-accessibility-4 [low / low] `src/components/SortMenu.tsx:33` Focus visibility of the menu items is not shown in the change: unreported (low confidence) (at the location of `expanded-state-hard-coded`, another category)

### accessibility-code-icon-button

- Role: accessibility at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 32 s
- Expected `image-without-alt` (text-alternative, medium): found by R-accessibility-3
- Expected `div-button` (keyboard-and-name, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `src/components/Toolbar.tsx:13-15` Delete control is a clickable div unreachable by keyboard: true positive
- Finding R-accessibility-2 [high / high] `src/components/Toolbar.tsx:13-15` Icon-only delete control has no accessible name: duplicate
- Finding R-accessibility-3 [medium / high] `src/components/Toolbar.tsx:12` Owner avatar image has no alt text: true positive
- Finding R-accessibility-4 [low / low] `src/components/Toolbar.tsx:13` Focus style and target size of the icon button not visible in the change: duplicate

### accessibility-control-custom-listbox

- Role: accessibility at `code.review`, control
- Outcome: scored
- Cost: 0.39 USD, 37 s
- Finding R-accessibility-1 [medium / low] `src/components/SizePicker.tsx:92` Highlighted option relies on an unseen style: unreported (low confidence)

### accessibility-control-labelled-form

- Role: accessibility at `code.review`, control
- Outcome: scored
- Cost: 0.27 USD, 15 s

### accessibility-plan-colour-only

- Role: accessibility at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.41 USD, 34 s
- Expected `colour-only-errors` (use-of-colour, high): found by R-accessibility-1
- Finding R-accessibility-1 [high / high] `design.md#Decision: Invalid fields turn red` Field errors are conveyed by colour only: true positive
- Finding R-accessibility-2 [medium / high] `proposal.md#What Changes` Removing the error summary leaves a failed submit unannounced: duplicate

### migration-code-drop-and-rename

- Role: migration at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 39 s
- Expected `irreversible-drop` (irreversible-migration, high): found by R-migration-2
- Expected `renamed-json-field` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `src/api.rs:8-9` Account JSON field rename is an unmarked breaking change: true positive
- Finding R-migration-2 [high / high] `migrations/0005_drop_legacy_email.sql:2` DROP COLUMN legacy_email is irreversible with no down migration or backup: true positive

### migration-code-serde-rename-all

- Role: migration at `code.review`, seeded
- Outcome: scored
- Cost: 0.31 USD, 21 s
- Expected `public-fields-renamed` (breaking-api, high): found by R-migration-1
- Expected `required-config-key` (config-compat, medium): found by R-migration-2
- Finding R-migration-1 [high / high] `src/api.rs:5-12` `rename_all = "camelCase"` renames every existing order JSON field: true positive
- Finding R-migration-2 [medium / high] `src/config.rs:9-10` Existing `config.toml` files without `default_currency` no longer load: true positive

### migration-control-additive-field

- Role: migration at `plan.challenge`, control
- Outcome: scored
- Cost: 0.36 USD, 33 s

### migration-control-serde-alias

- Role: migration at `code.review`, control
- Outcome: scored
- Cost: 0.34 USD, 25 s

### migration-plan-breaking-rename

- Role: migration at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.34 USD, 23 s
- Expected `unannounced-breaking-rename` (breaking-api, high): found by R-migration-1
- Finding R-migration-1 [high / high] `design.md#Decision: The field is renamed in place` Public field removed with no deprecation period: true positive
- Finding R-migration-2 [medium / high] `proposal.md#Release note` Breaking rename not marked BREAKING in the release note: duplicate

### newcomer-code-clever-one-liner

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.27 USD, 28 s
- Expected `clever-one-liner` (cleverness, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/pricing.ts:9-10` `bucket` packs its grouping rule into an unreadable one-liner: true positive

### newcomer-code-deep-nesting

- Role: newcomer at `code.review`, seeded
- Outcome: scored
- Cost: 0.30 USD, 22 s
- Expected `deep-nesting` (nesting, medium): found by R-newcomer-1
- Finding R-newcomer-1 [medium / high] `src/import.rs:12-42` `import` nests six levels deep and hides its seat rules: true positive

### newcomer-control-plain-code

- Role: newcomer at `code.review`, control
- Outcome: scored
- Cost: 0.26 USD, 25 s

### ops-code-no-timeout

- Role: ops at `code.review`, seeded
- Outcome: scored
- Cost: 0.33 USD, 29 s
- Expected `no-timeout` (timeout, high): found by R-ops-1
- Expected `unbounded-retry` (retry, high): found by R-ops-2
- Finding R-ops-1 [high / high] `app/payments.py:14` Payment call has no timeout: true positive
- Finding R-ops-2 [high / high] `app/payments.py:12-18` Unbounded retry of a non-idempotent charge: true positive
- Finding R-ops-3 [medium / high] `app/payments.py:17-18` Payment failures are swallowed without a log: duplicate
- Finding R-ops-4 [medium / high] `app/payments.py:7` Provider URL hard-coded: **false positive**

### ops-code-retry-without-backoff

- Role: ops at `code.review`, seeded
- Outcome: scored
- Cost: 0.53 USD, 64 s
- Expected `retries-without-backoff` (retry, high): found by R-ops-1, another severity
- Expected `dropped-delivery-invisible` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [medium / high] `src/webhooks.ts:9-11` Webhook retries fire back-to-back with no backoff: true positive
- Finding R-ops-2 [high / high] `src/webhooks.ts:24` A dropped webhook is invisible to operators: true positive
- Finding R-ops-3 [medium / low] `src/webhooks.ts:34` Webhook delivery blocks the payment event publisher for up to 25 s: unreported (low confidence)

### ops-control-bounded-retries

- Role: ops at `code.review`, control
- Outcome: scored
- Cost: 0.30 USD, 20 s

### ops-control-graceful-shutdown

- Role: ops at `code.review`, control
- Outcome: scored
- Cost: 0.42 USD, 42 s

### ops-plan-no-rollback

- Role: ops at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.38 USD, 30 s
- Expected `no-rollback` (rollback, high): found by R-ops-1
- Expected `invisible-worker` (observability, medium): found by R-ops-2, another severity
- Finding R-ops-1 [high / high] `design.md#Decision: All traffic moves at deploy` The switch to Paylane has no way back: true positive
- Finding R-ops-2 [high / high] `design.md#Decision: A worker settles payments nightly` The nightly settlement worker can fail unseen: true positive

### performance-code-hidden-n-plus-one

- Role: performance at `code.review`, seeded
- Outcome: scored
- Cost: 0.36 USD, 28 s
- Expected `customer-query-per-invoice` (query-per-item, medium): found by R-performance-1
- Expected `regex-compiled-per-invoice` (repeated-work, low): found by R-performance-2, another severity
- Finding R-performance-1 [medium / high] `src/report.rs:23-26` One customer query per invoice in the monthly report: true positive
- Finding R-performance-2 [medium / high] `src/report.rs:44-46` The purchase-order regex is compiled once per invoice: true positive

### performance-code-n-plus-one

- Role: performance at `code.review`, seeded
- Outcome: scored
- Cost: 0.31 USD, 20 s
- Expected `n-plus-one` (query-per-item, medium): found by R-performance-1, another severity
- Finding R-performance-1 [high / high] `api/src/orders.rs:14-15` One query per order to load its lines (N+1): true positive

### performance-control-batched-query

- Role: performance at `code.review`, control
- Outcome: scored
- Cost: 0.32 USD, 22 s

### performance-control-preloaded-helper

- Role: performance at `code.review`, control
- Outcome: scored
- Cost: 0.33 USD, 22 s

### performance-plan-unbounded-export

- Role: performance at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.44 USD, 32 s
- Expected `unbounded-export` (unbounded-result, high): found by R-performance-1
- Finding R-performance-1 [high / high] `design.md#Decision: The export loads every event, then writes the CSV` Export loads the whole audit log into memory and builds the CSV on the event loop: true positive
- Finding R-performance-2 [medium / low] `design.md#Decision: The export loads every event, then writes the CSV` No index is shown for the export's filter and sort: unreported (low confidence) (at the location of `unbounded-export`, another category)

### privacy-code-email-in-logs

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.33 USD, 31 s
- Expected `email-in-logs` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Expected `email-to-analytics` (personal-data-to-processor, medium): found by R-privacy-2, another severity
- Finding R-privacy-1 [high / high] `app/signup.py:14` E-mail address and display name written to application logs: true positive
- Finding R-privacy-2 [high / high] `app/signup.py:15` E-mail address sent to the Beacon analytics SaaS: true positive
- Finding R-privacy-3 [medium / low] `proposal.md#Domains` No retention, erasure or lawful basis stated for the collected member data: unreported (low confidence)

### privacy-code-error-message-pii

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.38 USD, 33 s
- Expected `profile-to-chat-vendor` (personal-data-to-processor, high): found by R-privacy-2
- Expected `email-in-error-message` (personal-data-in-logs, medium): found by R-privacy-1, another severity
- Finding R-privacy-1 [high / high] `src/support.ts:20` Looked-up e-mail address written to the error log: true positive
- Finding R-privacy-2 [high / high] `src/support.ts:41-48` Chat vendor receives phone, birth date and address to greet by name: true positive
- Finding R-privacy-3 [medium / low] `src/support.ts:16` Support lookup returns the full customer record: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/support.ts:40` Helply as a new processor is not identified: duplicate

### privacy-code-python-soft-delete

- Role: privacy at `code.review`, seeded
- Outcome: scored
- Cost: 0.35 USD, 26 s
- Expected `soft-delete-keeps-personal-data` (erasure, high): found by R-privacy-1
- Finding R-privacy-1 [high / high] `app/account.py:18-21` Account deletion keeps the member's email, name, phone and postal address: true positive
- Finding R-privacy-2 [low / low] `app/account.py:11-13` New analytics identifier added with no use and outside the erasure: unreported (low confidence)

### privacy-control-ids-in-logs

- Role: privacy at `code.review`, control
- Outcome: scored
- Cost: 0.36 USD, 28 s

### privacy-control-masked-logs

- Role: privacy at `code.review`, control
- Outcome: scored
- Cost: 0.38 USD, 42 s
- Finding R-privacy-1 [medium / high] `src/logger.ts:3` Addresses with non-ASCII characters are logged in full: **false positive**
- Finding R-privacy-2 [medium / low] `src/logger.ts:6-8` Percent-encoded addresses escape the mask: unreported (low confidence)
- Finding R-privacy-3 [low / low] `src/logger.ts:5` Masked form keeps the first letter and the full domain: unreported (low confidence)
- Finding R-privacy-4 [low / low] `src/logger.ts:12-13` Field keys are written unmasked: unreported (low confidence)

### privacy-plan-birth-date-retention

- Role: privacy at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.43 USD, 43 s
- Expected `no-retention` (retention, medium): found by R-privacy-1
- Expected `processor-transfer` (processor-transfer, medium): found by R-privacy-3
- Finding R-privacy-1 [medium / high] `design.md#Decision: Birth dates live on the profile` Birthday and phone kept after opt-out and in backups with no expiry: true positive
- Finding R-privacy-2 [medium / low] `specs/birthday-greetings/spec.md#Requirement: Members opt in to birthday greetings` Opt-out and data-subject rights not specified for the new fields: unreported (low confidence)
- Finding R-privacy-3 [medium / low] `design.md#Decision: Greetings are sent by SMS through Textly` Phone numbers and names sent to a US processor without a stated transfer safeguard: true positive
- Finding R-privacy-4 [low / low] `proposal.md#Why` Purpose and lawful basis of the new data not stated: unreported (low confidence)

### security-code-detail-endpoint-authz

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.37 USD, 33 s
- Expected `detail-without-tenant-check` (access-control, high): found by R-security-1
- Expected `stack-trace-to-client` (information-disclosure, medium): found by R-security-2
- Finding R-security-1 [high / high] `src/invoices.ts:32-35` GET /invoices/:id returns any organization's invoice: true positive
- Finding R-security-2 [medium / high] `src/server.ts:15-18` Error handler returns the error message and stack trace to the client: true positive
- Finding R-security-3 [low / low] `src/invoices.ts:27-28` Invoice id is not restricted to a positive safe integer: unreported (low confidence) (at the location of `detail-without-tenant-check`, another category)

### security-code-python-path-traversal

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.35 USD, 29 s
- Expected `download-path-traversal` (path-traversal, high): found by R-security-1
- Finding R-security-1 [high / high] `app/attachments.py:30-32` Path traversal in read_attachment bypasses the ticket authorization check: true positive
- Finding R-security-2 [medium / low] `app/attachments.py:11-17` save_attachment enforces no authorization on the target ticket: unreported (low confidence)

### security-code-sql-injection

- Role: security at `code.review`, seeded
- Outcome: scored
- Cost: 0.35 USD, 28 s
- Expected `sql-injection` (injection, high): found by R-security-1
- Finding R-security-1 [high / high] `src/users.ts:10-11` SQL injection through the search term: true positive

### security-control-docs-only

- Role: security at `code.review`, control
- Outcome: scored
- Cost: 0.30 USD, 19 s

### security-control-dynamic-query

- Role: security at `code.review`, control
- Outcome: scored
- Cost: 0.31 USD, 23 s

### security-plan-tokens-in-clear

- Role: security at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.35 USD, 26 s
- Expected `token-stored-in-clear` (credential-storage, high): found by R-security-1
- Expected `token-in-logs` (secret-in-logs, high): found by R-security-2
- Finding R-security-1 [high / high] `design.md#Decision: Tokens are stored as given` API tokens are stored in clear in the database: true positive
- Finding R-security-2 [high / high] `design.md#Decision: Every request is logged with its headers` The Authorization header, carrying the bearer token, is logged at info level: true positive

### skeptic-control-small-flag

- Role: skeptic at `plan.challenge`, control
- Outcome: scored
- Cost: 0.41 USD, 37 s

### skeptic-plan-scope-creep

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.41 USD, 35 s
- Expected `scope-creep` (scope, medium): found by R-skeptic-1
- Finding R-skeptic-1 [medium / high] `proposal.md#What Changes` Plugin system for future export formats nobody asked for: true positive
- Finding R-skeptic-2 [medium / high] `specs/invoice-export/spec.md#Requirement: Users choose their export format` Per-user export-format preference on the settings page nobody asked for: duplicate
- Finding R-skeptic-3 [medium / high] `proposal.md#Domains` New invoice endpoint exposes customer data with security and personal-data unticked: **false positive**

### skeptic-plan-unsupported-assumption

- Role: skeptic at `plan.challenge`, seeded
- Outcome: scored
- Cost: 0.49 USD, 57 s
- Expected `email-is-not-a-person` (assumption, high): found by R-skeptic-1
- Expected `irreversible-merge` (rollback, medium): found by R-skeptic-2, another severity
- Finding R-skeptic-1 [high / high] `design.md#Decision: Customers are merged by e-mail` The merge rests on "one e-mail = one person", which the design's own context contradicts: true positive
- Finding R-skeptic-2 [high / high] `design.md#Decision: Customers are merged by e-mail` Records are deleted with no way back: true positive
- Finding R-skeptic-3 [medium / high] `specs/customer-merge/spec.md#Requirement: Duplicate customers are merged` The spec covers only the happy path: **false positive** (at the location of `email-is-not-a-person`, another category)
- Finding R-skeptic-4 [medium / high] `proposal.md#Domains` Domains the plan touches are left unticked without a reason: **false positive**
- Finding R-skeptic-5 [low / high] `proposal.md#Why` The Why names the cause, but the plan only cleans up after it: **false positive**
- Finding R-skeptic-6 [low / high] `tasks.md#1. Merge` No task makes the job run nightly: **false positive**

### tracer-control-complete-plan

- Role: tracer at `plan.trace`, control
- Outcome: scored
- Cost: 0.23 USD, 19 s

### tracer-plan-scenario-without-task

- Role: tracer at `plan.trace`, seeded
- Outcome: scored
- Cost: 0.26 USD, 22 s
- Expected `_codev/changes/add-magic-links/tasks.md contains "expired"` (traceability, check): found by _codev/changes/add-magic-links/tasks.md

