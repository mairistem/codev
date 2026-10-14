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
  The Why is only "Accountants copy invoices one by one into their spreadsheet
  at the end of each month." — a single CSV export answers it, and the design
  itself names that simpler option ("a single CSV function") and rejects it only
  as "not extensible", for formats no one requested. Drop the plugin registry
  from proposal.md#What Changes, the "Decision: Exports go through a plugin
  registry" in design.md and task 1.2 ("Plugin registry discovering
  `src/export/plugins/`"); implement CSV as a plain function behind the route.
  Start-up discovery of modules from a directory is also an unverified
  load-bearing mechanism that would vanish with it. Propose extensibility in its
  own change when a second format is actually requested.
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
  Nothing in the Why asks for a per-user format preference, and with CSV as the
  only format the choice has nothing to choose between. The requirement is also
  not checkable as written: the scenario "A preference" says "**THEN** exports
  use CSV", yet the only export is `GET /invoices.csv`, which is CSV whatever the
  preference, so the test passes without the feature doing anything. Remove the
  requirement and its two scenarios, the third bullet of proposal.md#What
  Changes ("A settings page where each user picks their preferred export
  format."), the sentence "A settings page stores each user's preferred format."
  in design.md, `web/src/settings/` from proposal.md#Impact, and task 2.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Every domain unticked while the plan exposes invoice data and adds a UI page

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  `GET /invoices.csv?month=YYYY-MM` exposes a whole month of invoices with
  customer names and totals in one request, and the spec has no scenario for who
  may call it (no unauthorized case); the plan also adds a settings page
  (`web/src/settings/`) storing a per-user preference. Every box is unticked with
  no reason, so no expert reviews them. Tick security and personal-data (or give
  the reason on the line), and tick user-interface as long as the settings page
  stays in scope (see R-skeptic-2); if it is removed, leave it unticked with that
  reason.
history:
  - status: open
    by: skeptic
```
