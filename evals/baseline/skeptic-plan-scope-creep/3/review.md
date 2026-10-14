# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 Plugin system for future export formats is scope nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why only names accountants copying invoices into a spreadsheet one by one
  ("Accountants copy invoices one by one into their spreadsheet at the end of each
  month."); a single CSV route answers it. Drop the plugin system from the proposal,
  the design decision "Exports go through a plugin registry" (whose only rejected
  alternative, "a single CSV function (not extensible)", is rejected for an
  extensibility nobody requested) and task 1.2 ("Plugin registry discovering
  `src/export/plugins/`, verified by `npm test -- registry`", which names no
  observable behaviour to verify). Move PDF/XLSX/XML/JSON formats to a later change
  if a need appears.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format is scope nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: >-
  Nothing in the Why asks for a per-user format choice, and with CSV as the only
  format the choice has nothing to choose from. The requirement "Users choose their
  export format" is also not verifiable as written: its scenario "**THEN** exports
  use CSV" cannot be observed differently from today, since the only route is
  `GET /invoices.csv`, whose format is fixed by its path. Remove the requirement,
  its scenarios "A preference" and "No plugin", task 2.1 and `web/src/settings/`
  from Impact. If it were kept, the `user-interface` domain would have to be ticked
  since `web/src/settings/SettingsPage.tsx` changes.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 New endpoint exposing invoice and customer data with security and personal-data unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
recommendation: >-
  The change exposes every invoice of a month, with its customer, through the new
  route `GET /invoices.csv?month=YYYY-MM`, yet every domain is unticked with no
  reason, so no expert reviews who may call the route or what customer data leaves
  the system. The spec has no unauthorized scenario either: it only covers
  "An invalid month" (400). Tick `security` and `personal-data` (or give a reason
  on their line), and add a scenario for an unauthenticated or unauthorized caller.
history:
  - status: open
    by: skeptic
```
