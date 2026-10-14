# Review record: add-csv-export

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

#### R-skeptic-1 A plugin system for export formats nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
recommendation: >-
  The Why is only "Accountants copy invoices one by one into their spreadsheet at the end of each month";
  a CSV export answers it, and no PDF, XLSX, XML or JSON need is stated. Drop the plugin system from
  What Changes, replace design.md's "Decision: Exports go through a plugin registry" with a single CSV
  renderer behind the route (the rejected alternative's only flaw, "not extensible", serves no stated need),
  remove task 1.2 ("Plugin registry discovering `src/export/plugins/`"), and remove `src/export/plugins/`
  from Impact. Discovering modules at start-up from a source folder also rests on an unchecked assumption:
  the project builds with `tsc -p .`, so the plugins would have to be found in the compiled output, not in `src/`.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 A per-user settings page nobody asked for

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: >-
  Nothing in the Why asks users to choose a format, and with CSV as the only format the choice is empty.
  Drop this item, the requirement "Users choose their export format" in specs/invoice-export/spec.md,
  task 2.1, and `web/src/settings/` from Impact. The requirement is also not verifiable as written:
  scenario "A preference" says "exports use CSV" although the route is fixed at `GET /invoices.csv`,
  and scenario "No plugin" contradicts the design, where "CSV is the first plugin" and so is always installed.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Every domain unticked with no reason, though the plan exposes customer data and adds a page

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  The plan adds `GET /invoices.csv?month=YYYY-MM`, which serves every invoice of a month "with number,
  date, customer and total": a new endpoint exposing customer names, so security (who may export) and
  personal-data concern it, and the settings page touches user-interface. Tick the domains the plan
  touches so their experts review it, or keep each unticked with its reason on its line
  (`- [ ] <domain> — <reason>`). The spec has no scenario for an unauthenticated or unauthorized caller;
  whether it needs one is for the security expert once the domain is ticked.
history:
  - status: open
    by: skeptic
```
