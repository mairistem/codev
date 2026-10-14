# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 A plugin system for export formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why only covers accountants copying invoices into a spreadsheet each month ("Accountants copy invoices one by one into their spreadsheet at the end of each month."),
  and one CSV route solves that. Drop the plugin registry from What Changes, from
  "design.md#Decision: Exports go through a plugin registry" (it turns down the single CSV function only because it is "not extensible",
  but nothing in the plan needs extensibility) and drop task 1.2. Task 1.2 also cannot be checked as written: "verified by `npm test -- registry`"
  names no scenario, and no requirement in the spec describes the registry. Scanning `src/export/plugins/` at start-up also assumes the
  source folder exists at run time, which the `tsc -p .` build does not show. If more formats are needed later, put them in a change of their own.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 A settings page for a preferred export format nobody asked for, with no effect

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  Nothing in the Why asks for this requirement, and the only route the plan adds is `GET /invoices.csv?month=YYYY-MM`, which is CSV whatever is chosen.
  So the scenario "A preference" (WHEN a user picks CSV, THEN exports use CSV) cannot fail and does not check anything.
  Remove the requirement, its scenarios, the "A settings page" bullet in What Changes, `web/src/settings/` in Impact, the
  settings sentence in the design, and task 2.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The security domain is unticked although the change adds a route that exposes invoices

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
recommendation: >-
  `GET /invoices.csv?month=YYYY-MM` returns a whole month of invoices, and the spec has no scenario about who may call it: it only covers the happy path for an
  "accountant". Tick `security` so its expert reviews access to the route, or give the reason on the line
  (`- [ ] security — <reason>`), and add the scenario for an unauthorized caller that the security review asks for.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 The personal-data domain is unticked although the export contains customer names

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] personal-data
recommendation: >-
  The requirement exports "number, date, customer and total", and `customer` can be a person's name (`src/invoices.ts` `customer: string;`).
  The CSV file then leaves the application to spreadsheets. Tick `personal-data` so its expert reviews this, or give the reason on the line.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-5 The user-interface domain is unticked although the plan changes the settings page

```yaml
severity: low
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] user-interface
recommendation: >-
  Impact lists `web/src/settings/`, and task 2.1 adds a control to the settings page, but `user-interface` is unticked with no reason.
  If R-skeptic-2 is fixed by removing the page, give that reason on the line. Otherwise, tick it.
history:
  - status: open
    by: skeptic
```
