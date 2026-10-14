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
  at the end of each month." — a single CSV export answers it. Drop the plugin
  system from What Changes, drop "Decision: Exports go through a plugin
  registry" from design.md (its only alternative, "a single CSV function", is
  rejected as "not extensible", an extensibility no requirement asks for), and
  drop task 1.2 "Plugin registry discovering `src/export/plugins/`", which no
  requirement of spec.md backs. Keep `src/export/` to the CSV renderer and the
  route; a registry can come in its own change when a second format is
  actually requested.
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
  only format there is nothing to choose. The requirement also cannot be
  verified as written: its scenario "WHEN a user picks CSV THEN exports use
  CSV" holds whatever the preference is, since the only route is
  `GET /invoices.csv`. Remove this requirement and its scenarios, the
  "A settings page where each user picks their preferred export format."
  line of proposal.md, the settings sentence of design.md, task 2.1, and
  `web/src/settings/` from Impact.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains the plan touches are all unticked with no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The new `GET /invoices.csv?month=YYYY-MM` route exposes a month's invoices
  with customer names and totals, yet `security` and `personal-data` are
  unticked with no reason, so no expert reviews who may call it — and the spec
  has no scenario for an unauthenticated or unauthorized caller. The settings
  page adds UI under `web/src/settings/` with `user-interface` unticked. Tick
  `security` and `personal-data` (or give a reason on each line), and tick
  `user-interface` unless the settings page is removed per R-skeptic-2.
history:
  - status: open
    by: skeptic
```
