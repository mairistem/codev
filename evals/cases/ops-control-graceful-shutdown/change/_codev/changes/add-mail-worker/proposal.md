# Proposal: A worker that sends queued e-mails

## Why

E-mails are sent inside web requests, which time out when the relay is slow.

## What Changes

- Services queue e-mails; a worker sends them, retries temporary failures with a growing delay, keeps rejected ones and those still failing after half an hour aside, reports its health and counts, and stops cleanly on deployment.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/worker.rs` (new).

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — a new long-running worker, its retries, logs and shutdown
- [ ] migration

## Release note

- Changed: E-mails are sent in the background; a slow mail relay no longer slows pages down.
