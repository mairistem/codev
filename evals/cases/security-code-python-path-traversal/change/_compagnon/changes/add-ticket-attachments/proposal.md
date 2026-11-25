# Proposal: Attachments on support tickets

## Why

Customers e-mail screenshots that agents then forward by hand; tickets need attachments.

## What Changes

- Agents upload files to a ticket and download them from the desktop client.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `app/attachments.py` (new).

## Domains

- [x] security — files uploaded by customers and read back by agents
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Support tickets hold attachments.
