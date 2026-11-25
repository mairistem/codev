# Proposal: Export invoices as CSV

## Why

Accountants copy invoices one by one into their spreadsheet at the end of each month.

## What Changes

- `GET /invoices.csv?month=YYYY-MM` exports a month's invoices as CSV.
- A plugin system for export formats, so that PDF, XLSX, XML and JSON exports can be added later by dropping a module in `src/export/plugins/`.
- A settings page where each user picks their preferred export format.

## Capabilities

### New Capabilities

- `invoice-export`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/export/` (new), `src/server.ts`, `web/src/settings/`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Export a month's invoices as CSV.
