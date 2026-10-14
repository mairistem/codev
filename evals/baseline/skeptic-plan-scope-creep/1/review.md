# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 Plugin system for export formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is only "Accountants copy invoices one by one into their spreadsheet at the end of each month." —
  a single CSV export answers it. No PDF, XLSX, XML or JSON export is requested, yet the plan builds a
  start-up module discovery (design.md#Decision: Exports go through a plugin registry, which dismisses
  "a single CSV function (not extensible)" on speculative extensibility alone) and task 1.2, whose check
  "verified by `npm test -- registry`" names no observable behaviour. Drop the plugin system from What
  Changes, replace the design decision by a single CSV serializer next to `invoicesOfMonth`
  (src/invoices.ts), and remove task 1.2. A registry can come with the second format, when one is asked for.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: >-
  Nothing in the Why asks for a per-user format choice, and with a single format the choice is empty.
  The requirement it brings is not verifiable either: specs/invoice-export/spec.md "A preference" says
  "**THEN** exports use CSV", but the only export is `GET /invoices.csv`, CSV by its URL, so the
  preference changes nothing observable; and the design ("A settings page stores each user's preferred
  format") never says where the preference is stored — src/db.ts and the existing SettingsPage.tsx hold
  no such field, so a schema change is implied but unplanned. Remove the bullet, the requirement
  "Users choose their export format" with its two scenarios, task 2.1, and `web/src/settings/` from Impact.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains all unticked while the plan exposes invoices and changes the UI

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The plan adds an endpoint returning every invoice of a month with its customer
  (spec: "with number, date, customer and total"), and src/server.ts has no authentication middleware
  — only `app.use(express.json());` and `/health` — so who may call `GET /invoices.csv` is decided
  nowhere, and the spec has no unauthorized-caller scenario. Tick `security` and `personal-data` so
  their experts review the exposure, or give the reason on each line. If the settings page stays
  (see R-skeptic-2), tick `user-interface` too, and `migration` for the stored preference.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Which date places an invoice in a month is not stated

```yaml
severity: low
confidence: low
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  - **THEN** the CSV holds every invoice of September 2026
recommendation: >-
  The scenario does not say which date decides the month, nor in which time zone; src/invoices.ts uses
  `to_char(issued_on, 'YYYY-MM')`, so the plan probably means the issue date in the database's zone.
  State it ("invoices issued in the month") so the test of "A month" can be written without guessing,
  and say whether `invoicesOfMonth` is reused.
history:
  - status: open
    by: skeptic
```
