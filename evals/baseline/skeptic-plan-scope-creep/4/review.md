# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 Plugin system for future export formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  Drop the plugin system from this change. The Why ("Accountants copy invoices
  one by one into their spreadsheet at the end of each month.") is met by a
  single CSV route; no PDF, XLSX, XML or JSON export is requested. Replace
  design.md "Decision: Exports go through a plugin registry" with a plain CSV
  renderer (the rejected alternative "a single CSV function (not extensible)"
  is the one the Why supports — extensibility is not a stated need), remove
  task 1.2 (`npm test -- registry`), and remove `src/export/plugins/` from
  Impact. The registry also adds a start-up module-discovery step whose only
  consumer is the CSV format; the "No plugin" scenario even describes a state
  (no CSV) the route `GET /invoices.csv` could not be in.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format nobody asked for

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  Remove this requirement, its scenarios, task 2.1, the What Changes bullet
  "A settings page where each user picks their preferred export format." and
  `web/src/settings/` from Impact. With CSV as the only format and an endpoint
  named `/invoices.csv`, a preference has nothing to choose between; the
  scenario "WHEN a user picks CSV THEN exports use CSV" cannot fail and so
  verifies nothing, and nothing says where the preference is stored (it would
  need a new per-user column the plan does not mention). The release note
  ("Added: Export a month's invoices as CSV.") does not mention it either. If
  a format preference is wanted later, it belongs in the change that adds a
  second format.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains touched by the plan are all unticked without a reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The plan adds `GET /invoices.csv`, a new endpoint that exports a month's
  invoices with their customer names in bulk, and changes the settings page
  (`web/src/settings/` in Impact). No spec scenario says who may call the
  export (no unauthorized case), and `src/server.ts` today only has
  `/health`, so no existing guard covers it. Tick `security` and
  `personal-data` so their experts review the endpoint's access control and
  the bulk export of customer data; tick `user-interface` if the settings page
  stays in scope (see R-skeptic-2), otherwise leave it unticked with that
  reason on its line.
history:
  - status: open
    by: skeptic
```
