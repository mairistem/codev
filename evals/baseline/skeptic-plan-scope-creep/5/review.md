# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 Plugin registry for formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is only "Accountants copy invoices one by one into their spreadsheet
  at the end of each month." — a single CSV export answers it. Nothing in the
  proposal asks for PDF, XLSX, XML or JSON; the design rejects the simple
  solution only as "a single CSV function (not extensible)", which is
  speculative. Drop the plugin system from What Changes, replace
  "design.md#Decision: Exports go through a plugin registry" with a plain CSV
  renderer behind `GET /invoices.csv`, and remove task 1.2 ("Plugin registry
  discovering `src/export/plugins/`"), which no spec requirement backs. A
  registry can come in its own change when a second format is actually
  requested.
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
  The Why gives no need for a per-user format preference, and with CSV as the
  only format the choice is empty. The requirement is also unverifiable as
  written: "**THEN** exports use CSV" while the route is `/invoices.csv`
  itself, so the preference changes nothing observable; and "**WHEN** no plugin
  is installed" contradicts the design, where CSV is always "the first plugin".
  Remove the requirement and its scenarios, the "A settings page where each
  user picks their preferred export format." line of What Changes, the
  sentence "A settings page stores each user's preferred format." of the
  design, `web/src/settings/` from Impact, and task 2.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains all unticked though the plan exposes invoices and changes a UI page

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The plan adds `GET /invoices.csv`, a new endpoint returning every invoice of
  a month with its customer, yet `security` and `personal-data` are unticked
  with no reason, and no spec scenario says who may export (the spec only
  names "an accountant"; `src/server.ts` has no authentication). The plan also
  edits `web/src/settings/` (Impact) with `user-interface` unticked. Tick
  `security` and `personal-data` so their experts review who can call the
  endpoint and what customer data it returns; tick `user-interface` too if the
  settings page survives R-skeptic-2, otherwise remove it from Impact. Give a
  reason on the line of any domain left unticked.
history:
  - status: open
    by: skeptic
```
