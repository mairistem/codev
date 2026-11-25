# Design: Idle session timeout

## Decisions

### Decision: The last request time lives in the session row

Each request updates `sessions.last_seen_at`; the middleware refuses a
session whose `last_seen_at` is more than 30 minutes old.

**Alternatives considered**: a sliding cookie expiry (the server could not
enforce it).
