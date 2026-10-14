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
  The Why is only "Accountants copy invoices one by one into their spreadsheet
  at the end of each month." — a CSV route answers it. The design itself names
  the simpler option ("a single CSV function") and rejects it only as "not
  extensible", for formats no one requested. Drop the plugin registry from
  What Changes, the design decision "Exports go through a plugin registry" and
  task 1.2 ("Plugin registry discovering `src/export/plugins/`"), and implement
  the CSV export directly; propose the registry in its own change when a
  second format is actually asked for.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-2 Settings page for a preferred export format is scope nobody asked for and has no observable effect

```yaml
severity: medium
confidence: high
location: "proposal.md#What Changes"
evidence: |
  - A settings page where each user picks their preferred export format.
recommendation: >-
  Nothing in the Why asks for a per-user format choice, and with only CSV
  there is nothing to choose. The requirement "Users choose their export
  format" cannot be checked as written: the only route, `GET /invoices.csv`,
  is CSV by its URL, so the scenario "WHEN a user picks CSV THEN exports use
  CSV" holds whatever the setting says, and "WHEN no plugin is installed THEN
  the settings page says so" describes a state that cannot occur once CSV
  ships. It also pulls `web/src/settings/` into the Impact and the
  user-interface domain. Remove the settings-page bullet, the requirement
  "Users choose their export format" with its two scenarios, and task 2.1.
history:
  - status: open
    by: skeptic
```

#### R-skeptic-3 Every domain is unticked with no reason, though the plan exposes customer invoice data and changes a page

```yaml
severity: medium
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [ ] security
  - [ ] personal-data
  - [ ] user-interface
recommendation: >-
  `GET /invoices.csv?month=YYYY-MM` returns every invoice of a month with
  "number, date, customer and total" — customer names leave the system in a
  file, and the spec has no scenario for who may call the route (an
  unauthenticated or non-accountant caller). Tick security and personal-data
  so their experts review the endpoint, or give the reason on each line
  (`- [ ] <domain> — <reason>`). If the settings page stays (see
  R-skeptic-2), user-interface must be ticked too, since `web/src/settings/`
  is in the Impact.
history:
  - status: open
    by: skeptic
```
