# Design: Magic links

## Decisions

### Decision: Links carry a random token stored hashed

A 32-byte random token, stored as its SHA-256 with an expiry 15 minutes
ahead; opening the link compares digests and deletes the row.

**Alternatives considered**: signed JWTs (cannot be made single-use).
