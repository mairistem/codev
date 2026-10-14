# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

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
  for a high-entropy secret; no bcrypt needed) in a `token_hash` column with
  a unique index, and look requests up with `WHERE token_hash = $1` on the
  hash of the presented token — the lookup stays a single indexed equality,
  so the stated reason for storing it as given does not hold. Optionally
  keep a short non-secret prefix to let users identify a token. Update task
  1.1 and the spec (a token is "shown once" only if the service cannot show
  it again). Anyone who reads the `api_tokens` table — a SQL injection
  elsewhere, a leaked backup or replica, a support engineer with read access
  — gets every live token and can call the API as each user (CWE-312,
  CWE-256); with no revocation (R-security-3) the access cannot be cut.
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header is logged at info level

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log credential-bearing headers: redact `Authorization`, `Cookie` and
  `Set-Cookie` (log at most the token's non-secret prefix or an id of the
  token row resolved by the middleware), and make task 1.3's test
  `logs_requests` assert that the secret values are absent from the output.
  As designed, every bearer token — and every session cookie, since all
  headers are logged and `authenticate` reads the session from `cookie` —
  ends up in plaintext logs: anyone with access to the logs, the log
  pipeline or its retention store replays them to act as the user
  (CWE-532).
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be revoked and never expires

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [A user loses a token] → they create another one; revoking the old one comes with the later revocation change.
recommendation: >-
  The proposal's own motivation is that "a leaked integration exposes the
  account"; with this change, a leaked token (from logs per R-security-2,
  from the table per R-security-1, or from the integration itself) grants
  API access indefinitely, and changing the password does not stop it since
  tokens are independent of `users.password_hash`. Either bring a minimal
  revocation (delete a token by id, scoped to its owner) into this change,
  or give tokens an expiry (`expires_at`, checked in the lookup) until the
  revocation change ships, and state in the spec what the user can do when
  a token leaks.
history:
  - status: open
    by: security
```
