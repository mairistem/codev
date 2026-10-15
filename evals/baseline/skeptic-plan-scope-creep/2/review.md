# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 A plugin system for future formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is only "Accountants copy invoices one by one into their spreadsheet at the end
  of each month." — a single CSV route solves it. Drop the plugin system from What Changes,
  replace "Decision: Exports go through a plugin registry" in design.md (the rejected
  "single CSV function" is the simpler solution the Why calls for) and remove task 1.2
  ("Plugin registry discovering `src/export/plugins/`"), which traces to no requirement of
  specs/invoice-export/spec.md. This also removes the unverified assumption that
  `registry.ts` can discover modules in `src/export/plugins/` at start-up once compiled by
  `tsc -p .`. A plugin system can be proposed in its own change when a second format is
  actually requested.
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
  Nothing in the Why asks for a format preference, and with CSV as the only format the
  choice has no effect: the route is `GET /invoices.csv`, so the scenario "A preference"
  ("**WHEN** a user picks CSV / **THEN** exports use CSV") cannot fail and verifies nothing.
  Remove this requirement and its scenarios, the "A settings page where each user picks
  their preferred export format." line of proposal.md#What Changes, `web/src/settings/`
  from proposal.md#Impact, the last sentence of the design decision ("A settings page
  stores each user's preferred format.") and task 2.1. Note that it would also need a
  storage for the preference that the plan never describes, and a user-visible change
  absent from the release note.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Domains the plan touches are all unticked, with no reason

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The new route `GET /invoices.csv?month=YYYY-MM` exposes a whole month of invoices, with
  each customer's name, and neither the proposal nor the spec says who may call it (no
  "unauthorized" scenario): tick `security` and `personal-data` so that their experts
  review it, or give the reason on the line. While the settings page stays in scope
  (see R-skeptic-2), tick `user-interface` too — the plan modifies `web/src/settings/`.
history:
  - status: open
    by: skeptic
```
