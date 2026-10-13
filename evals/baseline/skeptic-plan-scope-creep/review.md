# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

### skeptic

#### R-skeptic-1 Plugin system for future export formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why only asks to stop copying invoices one by one into a spreadsheet
  ("Accountants copy invoices one by one into their spreadsheet at the end of
  each month."); a CSV export answers it. No PDF, XLSX, XML or JSON export is
  requested. Drop the plugin system from What Changes and Impact
  (`src/export/` can shrink to one CSV module), replace the design's
  "Decision: Exports go through a plugin registry" with a single CSV
  function — the alternative it rejected only as "not extensible" — and
  remove task 1.2 ("Plugin registry discovering `src/export/plugins/`").
  Add the registry in the change that brings a second format.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Per-user export-format preference on the settings page nobody asked for

```yaml
severity: medium
confidence: high
location: "specs/invoice-export/spec.md#Requirement: Users choose their export format"
evidence: |
  The settings page SHALL let a user choose their preferred export format among the installed plugins.
recommendation: >-
  With CSV as the only format, a choice of format has nothing to choose
  between, and the Why says nothing about preferences. Remove this
  requirement and its scenarios, the What Changes bullet "A settings page
  where each user picks their preferred export format.", `web/src/settings/`
  from Impact, the last sentence of the design decision ("A settings page
  stores each user's preferred format.") and task 2.1. As written the
  requirement is also unverifiable: "exports use CSV" cannot be told apart
  from the default when CSV is the only format, and it needs a stored
  per-user preference the plan never places anywhere.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 New invoice endpoint exposes customer data with security and personal-data unticked

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
recommendation: >-
  `GET /invoices.csv?month=YYYY-MM` is a new input-taking route that returns
  every invoice of a month with its customer, yet no domain is ticked, so no
  expert reviews who may call it or what customer data leaves the system;
  the spec has no unauthorized-caller scenario. Tick `security` and
  `personal-data` (or write the reason on each line if they do not apply).
  If the settings page stays despite R-skeptic-2, tick `user-interface` too.
history:
  - status: open
    by: skeptic
```
