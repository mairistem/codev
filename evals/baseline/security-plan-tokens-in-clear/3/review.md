# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### security

#### R-security-1 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store only a hash of the token (SHA-256 of the 32 random bytes is enough
  for high-entropy tokens; no bcrypt needed) in a `token_hash` column, and
  look a request up with `WHERE token_hash = $1` on the hash of the presented
  token — still a simple indexed equality, so the stated reason for storing
  it as given does not hold. Show the clear token only once at creation, as
  the spec requires. Anyone who reads the `api_tokens` table — a SQL
  injection elsewhere, a leaked backup or dump, a read-only DB account —
  otherwise gets bearer credentials usable at once against the API for every
  user (CWE-312 / CWE-256), unlike passwords which are bcrypt-hashed. Hashing
  also removes the timing side channel of comparing the secret itself in the
  lookup. Update task 1.1 accordingly.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, holding the bearer token, is logged

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log the `Authorization` header (nor `Cookie`); redact it, or log
  only a non-secret token identifier (a token id or a short prefix of the
  token's hash) so support can still tell which token was used. As written,
  every request writes a live bearer token into the logs at `info`, so
  anyone with access to the logs or the log pipeline (support staff, a log
  aggregator, a leaked log archive) can replay any integration's token and
  act as its user (CWE-532); the same line also logs session cookies and
  Basic credentials for password-based integrations. Adjust task 1.3 and its
  test `logs_requests` to assert the header is redacted.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be invalidated: revocation is deferred

```yaml
severity: medium
confidence: high
location: "design.md#Goals / Non-Goals"
evidence: |
  **Non-Goals:** OAuth, scopes, revocation (a later change).
recommendation: >-
  The proposal's motivation is that "a leaked integration exposes the
  account"; with no revocation and no expiry, a leaked token (from logs, see
  R-security-2, or from the integration's host) grants full, unscoped access
  to the account forever, and the user's only remedy listed — creating
  another token — leaves the leaked one valid. Either ship at least deletion
  of a token by its owner in this change, or give tokens an expiry, or record
  explicitly that the change must not be released before the revocation
  change.
history:
  - status: open
    by: security
```
