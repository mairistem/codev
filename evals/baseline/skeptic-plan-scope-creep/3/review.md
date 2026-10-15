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
  The Why only names accountants copying invoices into their spreadsheet
  ("Accountants copy invoices one by one into their spreadsheet at the end of
  each month."); a CSV route answers it. Nothing asks for PDF, XLSX, XML or
  JSON. Drop the plugin system from the proposal, replace design.md's
  "Decision: Exports go through a plugin registry" by a single CSV renderer
  (the alternative design.md rejects only as "not extensible", which the Why
  never requires), and remove task 1.2 ("Plugin registry discovering
  `src/export/plugins/`, verified by `npm test -- registry`"). Start-up
  discovery of modules in a TypeScript project built with `tsc` is also an
  unverified assumption the plan would no longer need. Propose the registry in
  its own change when a second format is actually requested.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format is scope nobody asked for

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  The Why asks for a monthly export, not a per-user preference; with CSV as
  the only format, the choice has a single option. Remove this requirement,
  its scenarios, the proposal line "A settings page where each user picks
  their preferred export format.", the sentence "A settings page stores each
  user's preferred format." in design.md, `web/src/settings/` from Impact, and
  task 2.1. As written the requirement is also unverifiable: scenario "A
  preference" says "exports use CSV" while the only route is the fixed
  `/invoices.csv`, so no observable behavior depends on the preference, and
  no requirement says where the preference is stored.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Invoice export endpoint touches security and personal data with every domain unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
recommendation: >-
  The change exposes a new route, `GET /invoices.csv?month=YYYY-MM`, that
  returns every invoice of a month with its customer name and total, yet no
  domain is ticked, so no expert reviews who may call it or what customer data
  leaves the system; the spec has no unauthorized-caller scenario either.
  Tick `security` and `personal-data`, or give the reason on each line. If
  the settings page stays despite R-skeptic-2, `user-interface` must be ticked
  too, since `web/src/settings/` is in Impact.
history:
  - status: open
    by: skeptic
```
