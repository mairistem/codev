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
  The Why only asks to stop accountants copying invoices one by one into
  their spreadsheet; one CSV export answers it. Drop the plugin system from
  What Changes, replace "Decision: Exports go through a plugin registry" in
  design.md (whose only rejected alternative, "a single CSV function (not
  extensible)", is the simpler design the Why calls for), and remove task
  1.2 with its `npm test -- registry` check. Start-up discovery of modules
  in `src/export/plugins/` also rests on an unverified assumption about how
  the `tsc`-built server loads files; it disappears with the registry.
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
  Nothing in the Why asks for a per-user format preference, and with CSV as
  the only format there is nothing to choose. Remove the requirement and its
  scenarios, the settings-page bullet of proposal.md#What Changes and
  `web/src/settings/` from Impact, the sentence "A settings page stores each
  user's preferred format." in design.md, and task 2.1. As written, the
  scenario "A preference" (THEN exports use CSV) cannot be checked either:
  the only route is `GET /invoices.csv`, so the preference changes nothing
  observable.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains all unticked while the plan exposes invoice data and changes a page

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The new route `GET /invoices.csv` exposes every invoice of a month,
  including customer names, and the plan modifies the settings page; yet
  every domain is unticked with no reason. Tick security and personal-data
  (so their experts review who may export and what customer data leaves the
  system), and tick user-interface while the settings page stays in scope
  — or give each unticked domain its reason on its line.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Export requirement has no scenario for an unauthorized caller

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  `GET /invoices.csv?month=YYYY-MM` SHALL return the month's invoices, one per line after a header line, with number, date, customer and total; a field holding a comma, a quote or a line break SHALL be quoted as RFC 4180 says.
recommendation: >-
  The scenarios cover a month, an invalid month, an empty month and a comma,
  but none says what happens when the request comes from someone who is not
  signed in or not allowed to see invoices; as written, any caller gets the
  whole month's invoices. State who may export and add a scenario for the
  refused caller, with its status code, and a matching test in task 1.1.
history:
  - status: open
    by: skeptic
```
