# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Plugin system for future formats is scope nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: Drop the plugin system from this change; the Why (accountants copying invoices into a spreadsheet) is met by a single CSV export. Propose a registry in its own change once a second format is actually requested.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Per-user export format preference is scope nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: Remove the settings page, its requirement "Users choose their export format" and task 2.1; with a single format there is nothing to choose.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Design rejects the simple solution only for an extensibility nobody needs

```yaml
severity: low
confidence: high
location: "design.md#Decision: Exports go through a plugin registry"
evidence: |
  **Alternatives considered**: a single CSV function (not extensible).
recommendation: Choose the single CSV function, which fully answers the Why; extensibility is not a stated need, so it cannot be the reason to reject the simpler option.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Preference scenario has no observable effect: the route hardcodes CSV

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  - **WHEN** a user picks CSV
  - **THEN** exports use CSV
recommendation: If the requirement stays, say which endpoint honours the preference — `GET /invoices.csv` names its format in the path, so "exports use CSV" holds whatever the user picks and cannot fail; otherwise drop the requirement (see R-skeptic-2).
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 Storing a per-user preference is a hidden schema migration

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Exports go through a plugin registry"
evidence: |
  A settings page stores each user's preferred format.
recommendation: Either drop the preference (R-skeptic-2), or state where it is stored, the schema change it needs, the default for existing users, and tick `migration` in the Domains.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-6 No access-control case for an export of every invoice of a month

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
recommendation: Say who may call `GET /invoices.csv` and add a scenario for an unauthenticated or unauthorized caller; the export returns customer names and totals in bulk, so tick `security` and `personal-data` or give the reason they do not apply.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-7 Date and total formats of the CSV are unspecified

```yaml
severity: low
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  `GET /invoices.csv?month=YYYY-MM` SHALL return the month's invoices, one per line after a header line, with number, date, customer and total; a field holding a comma, a quote or a line break SHALL be quoted as RFC 4180 says.
recommendation: Fix the header names, the date format and whether the total is in cents (`totalCents` in `src/invoices.ts`) or a decimal amount, so a test can check a line exactly and the spreadsheet reads the numbers.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-8 Registry task is verified by a test no requirement defines

```yaml
severity: low
confidence: high
location: "tasks.md#1. Export"
evidence: |
  - [ ] 1.2 Plugin registry discovering `src/export/plugins/`, verified by `npm test -- registry`
recommendation: Drop the task with the plugin system (R-skeptic-1); if kept, back it with a requirement and scenarios so `npm test -- registry` checks a specified behavior.
history:
  - status: open
    by: skeptic
```
