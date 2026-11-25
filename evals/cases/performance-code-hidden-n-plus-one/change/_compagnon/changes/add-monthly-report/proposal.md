# Proposal: Monthly invoice report

## Why

Finance rebuilds the month's invoice list in a spreadsheet; about 3 000 invoices a month, from 800 customers.

## What Changes

- A monthly report lists each invoice with its customer and purchase-order number, and the month's total.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/report.rs` (new), `Cargo.toml` (the `regex` crate).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [x] performance — a report over every invoice of a month
- [ ] operations
- [ ] migration

## Release note

- Added: A monthly invoice report for finance.
