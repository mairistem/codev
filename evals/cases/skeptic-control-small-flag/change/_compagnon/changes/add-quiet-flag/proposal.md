# Proposal: A --quiet flag for imports

## Why

Nightly scripts run `ledger import` and their mail reports are flooded with progress lines.

## What Changes

- `ledger import --quiet` prints no progress; errors and exit codes are unchanged.

## Capabilities

### New Capabilities

- `cli-import`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/main.rs`, `src/cli/import.rs`, `src/cli/progress.rs`, `docs/cli.md`.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: `ledger import --quiet` silences the progress lines.
