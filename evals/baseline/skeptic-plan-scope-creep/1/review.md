# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 A plugin system for formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is accountants copying invoices one by one into a spreadsheet; a
  single CSV route answers it. Drop the plugin system from What Changes, the
  "Exports go through a plugin registry" decision in design.md (whose only
  rejected alternative, "a single CSV function (not extensible)", is
  dismissed for an extensibility no requirement needs) and task 1.2. As it
  stands, task 1.2 is verified by `npm test -- registry` while no spec
  requirement describes the registry, so its done state has nothing to be
  checked against; and discovering modules from `src/export/plugins/` at
  start-up is an unverified assumption on how the TypeScript build ships
  those files. Add the plugin system in its own change when a second format
  is actually requested.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 A per-user export-format setting nobody asked for

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  Nothing in the Why asks for a format preference, and with CSV as the only
  format there is nothing to choose. Remove this requirement, the settings
  page item of What Changes, the sentence "A settings page stores each
  user's preferred format." in design.md and task 2.1. If it were kept, its
  scenario "WHEN a user picks CSV / THEN exports use CSV" cannot be told
  apart from no preference, since `GET /invoices.csv` is CSV whatever is
  stored; the plan does not say where the preference is stored (a new
  column or table, i.e. a migration); and the `user-interface` and
  `migration` domains would have to be ticked in proposal.md#Domains.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 security left unticked for a route exposing invoices

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
recommendation: >-
  The change adds `GET /invoices.csv?month=YYYY-MM`, which returns every
  invoice of a month with customer and total, and the spec has no scenario
  for a caller who is not authenticated or not an accountant. Tick
  `security` (and decide on `personal-data` for customer names) so that its
  expert reviews who may call the route, and add an unauthorized-caller
  scenario to "Invoices are exported as CSV".
history:
  - status: open
    by: skeptic
```
