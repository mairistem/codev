# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Plugin system for future export formats is scope creep

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: Drop the plugin system and the registry task (1.2); ship the CSV export as a single function, and propose a registry in its own change once a second format is actually requested.
history:
  - status: open
    by: skeptic
```

The Why only describes accountants copying invoices into a spreadsheet each month; no one asked for PDF, XLSX, XML or JSON. The release note itself mentions only the CSV export.

#### R-skeptic-2 The simpler solution was dismissed only because of speculative extensibility

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Exports go through a plugin registry"
evidence: |
  **Alternatives considered**: a single CSV function (not extensible).
recommendation: Choose the single CSV function, which fully answers the Why; extensibility is not a requirement of any spec scenario.
history:
  - status: open
    by: skeptic
```

Runtime discovery of `src/export/plugins/` at start-up also rests on an unverified assumption: the project builds with `tsc -p .`, and the design does not say which directory (sources or compiled output) is scanned in production.

#### R-skeptic-3 Per-user export-format settings page is scope creep

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: Remove the settings page, the requirement "Users choose their export format" and task 2.1; with a single CSV format there is nothing to choose.
history:
  - status: open
    by: skeptic
```

With CSV as the only format, the preference has a single possible value. The release note does not mention this feature either.

#### R-skeptic-4 The impacted frontend `web/src/settings/` does not exist in the project

```yaml
severity: medium
confidence: high
location: "proposal.md#Impact"
evidence: |
  - `src/export/` (new), `src/server.ts`, `web/src/settings/`.
recommendation: If the settings page is kept, say where the frontend lives or that it must be created; otherwise remove `web/src/settings/` from the Impact.
history:
  - status: open
    by: skeptic
```

The repository holds only `src/db.ts` and `src/server.ts`, an Express API (`package.json` name `shop-api`), and no `web/` directory: the settings page assumes a frontend that is not there.

#### R-skeptic-5 Storing each user's preferred format is a hidden migration

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Exports go through a plugin registry"
evidence: |
  first plugin. A settings page stores each user's preferred format.
recommendation: If the preference is kept, state where it is stored (table, column, default value for existing users) and add the migration as a task; otherwise drop it with R-skeptic-3.
history:
  - status: open
    by: skeptic
```

No artifact says where the preference is persisted, what the default is for existing users, or what happens to a stored format whose plugin is removed.

#### R-skeptic-6 The preference requirement is not verifiable against any route

```yaml
severity: low
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  - **WHEN** a user picks CSV
  - **THEN** exports use CSV
recommendation: If kept, name the route that honours the preference and the observable output; the only export route, `/invoices.csv`, is CSV whatever the preference.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-7 No access control on an endpoint that exposes every invoice and customer

```yaml
severity: high
confidence: low
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  `GET /invoices.csv?month=YYYY-MM` SHALL return the month's invoices, one per line, with number, date, customer and total.
recommendation: Add a requirement and scenario for unauthenticated (401) and non-accountant (403) requests, and reconsider the unticked `security` and `personal-data` domains.
history:
  - status: open
    by: skeptic
```

The spec says the export is for accountants, yet no scenario covers an unauthenticated or unauthorized caller, and `src/server.ts` shows no authentication middleware. The proposal leaves `security` and `personal-data` unticked although the CSV carries customer names.

#### R-skeptic-8 Happy path only: CSV escaping, missing month and empty month are unspecified

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  #### Scenario: A month

  - **WHEN** an accountant exports 2026-09
  - **THEN** the CSV holds every invoice of September 2026
recommendation: Add scenarios for a customer name containing a comma, quote or newline (and a leading `=`, `+`, `-`, `@` for spreadsheet formula injection), a missing `month` parameter, and a month with no invoice; specify header row, separator, encoding and number/date formats.
history:
  - status: open
    by: skeptic
```

The file is meant to be opened in a spreadsheet, so escaping and formula injection decide whether the output is correct and safe.
