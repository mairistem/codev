# Proposal: Mask e-mail addresses in logs

## Why

Our logs are kept 90 days by the hosting provider and contain customers' e-mail addresses.

## What Changes

- A logger that masks every e-mail address in the message and the fields.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/logger.ts` (new) and its tests.

## Domains

- [ ] security
- [x] personal-data — removes personal data from logs
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Security: E-mail addresses no longer appear in full in the logs.
