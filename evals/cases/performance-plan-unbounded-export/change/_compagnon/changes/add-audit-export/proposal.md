# Proposal: Audit log export

## Why

Auditors ask administrators for the audit log; today it is copied by hand.

## What Changes

- `GET /admin/audit-events.csv` exports the organization's audit log.

## Capabilities

### New Capabilities

- `audit-export`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/admin/audit.ts` (new), a route in `src/server.ts`.

## Domains

- [x] security — administrators only
- [ ] personal-data
- [ ] user-interface
- [x] performance — reads the whole audit log
- [ ] operations
- [ ] migration

## Release note

- Added: Administrators can export the audit log as CSV.
