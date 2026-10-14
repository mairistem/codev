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
  The Why only asks that accountants stop copying invoices one by one into
  their spreadsheet ("Accountants copy invoices one by one into their
  spreadsheet at the end of each month."); a single CSV route answers it.
  Drop the plugin registry from the proposal, the design decision "Exports
  go through a plugin registry" (whose only argument against a single CSV
  function is "not extensible", for formats nobody requested) and task 1.2,
  which has no requirement or scenario behind it ("verified by `npm test --
  registry`"). Propose the registry in its own change when a second format
  is actually needed.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format is out of scope

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  Nothing in the Why asks for a per-user format preference, and with CSV as
  the only format the choice is empty. The requirement is also not
  verifiable as written: the export route is fixed to `/invoices.csv`, so
  "**THEN** exports use CSV" cannot observe the preference having any
  effect. Remove this requirement, the "A settings page where each user
  picks their preferred export format." bullet, `web/src/settings/` from
  Impact, the last sentence of the design decision and task 2.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains the plan touches are left unticked without a reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The change adds a route, `GET /invoices.csv?month=YYYY-MM`, that returns a
  month's invoices with their customer and total, yet `security` and
  `personal-data` are unticked with no reason, so no expert will review who
  may call it or what customer data leaves the system; the spec has no
  scenario for an unauthenticated or unauthorized caller. As planned, the
  settings page also touches `user-interface`, unticked. Tick each domain
  the plan touches, or give the reason it does not on its line (if
  R-skeptic-2 removes the settings page, `user-interface` can stay unticked
  with that reason).
history:
  - status: open
    by: skeptic
```

#### R-skeptic-4 Which invoices belong to a month is not defined

```yaml
severity: low
confidence: low
location: "specs/invoice-export/spec.md#Requirement: Invoices are exported as CSV"
evidence: |
  - **THEN** the CSV holds every invoice of September 2026
recommendation: >-
  The plan does not say which date places an invoice in a month (issue
  date, due date) nor in which time zone, so an invoice issued near
  midnight on a month boundary may land in either export. State the rule in
  the requirement and add a boundary scenario.
history:
  - status: open
    by: skeptic
```
