# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 A plugin system for four future formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is accountants copying invoices one by one into a spreadsheet; one CSV
  route answers it. Drop the plugin system from What Changes, the registry
  decision from design.md ("Exports go through a plugin registry", whose only
  rejected alternative — "a single CSV function (not extensible)" — is the one
  the Why calls for), task 1.2 (`npm test -- registry`, which no spec scenario
  backs, so its done state cannot be observed) and `src/export/plugins/` from
  Impact. Add other formats in their own change when someone asks for them.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 A per-user export-format settings page nobody asked for

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  Nothing in the Why asks for a format preference, and with CSV the only format
  the choice has no effect: the route is `GET /invoices.csv`, so the scenario
  "WHEN a user picks CSV THEN exports use CSV" cannot fail and checks nothing.
  The design's "A settings page stores each user's preferred format" also needs
  a stored per-user value the plan never places (no table or column in Impact,
  `migration` unticked) and touches `web/src/settings/` with `user-interface`
  unticked. Drop the requirement, the What Changes bullet, the design sentence,
  task 2.1 and `web/src/settings/` from Impact.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 The export route exposes customer data with every domain unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
recommendation: >-
  `GET /invoices.csv` returns every invoice of a month with its customer and
  total, and the spec has no scenario for who may call it (no unauthorized
  case: an unauthenticated request would get the whole month). Tick `security`
  and `personal-data` so their experts review the route, or write the reason on
  each line if access is already guarded elsewhere; and add the unauthorized
  scenario to "Invoices are exported as CSV".
history:
  - status: open
    by: skeptic
```
