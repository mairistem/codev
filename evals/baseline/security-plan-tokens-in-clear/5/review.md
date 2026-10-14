# Review record: add-api-tokens

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### security

#### R-security-1 Request logging writes every bearer token to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Never log credential headers (CWE-532): redact `Authorization` and `Cookie`
  (log the header name and, at most, a non-secret token identifier or prefix
  stored alongside the token) before logging, and make test
  `logs_requests` (tasks.md 1.3) assert the redaction. As designed, anyone
  with read access to the info-level logs (support staff, log pipeline,
  log storage backups) copies a live token from a log line and calls the API
  as that user; the same line also leaks the `session=` cookie that
  `src/http/middleware.rs` authenticates today. Because revocation is a
  Non-Goal, a token leaked this way cannot be invalidated, so the deferral
  makes the leak permanent.
history:
  - status: open
    by: security
```

#### R-security-2 API tokens are stored in clear in the database

```yaml
severity: high
confidence: high
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  The `api_tokens` table holds `user_id`, `token` and `created_at`; the
  middleware looks a request's token up with `WHERE token = $1`. Storing the
  token as given keeps the lookup a simple indexed equality.
recommendation: >-
  Store only a hash of the token (CWE-256/CWE-312): since tokens are 32
  random bytes, a fast SHA-256 (or HMAC-SHA-256 with a server key) is enough
  and keeps the lookup an indexed equality on `token_hash`, so the stated
  performance reason for the clear storage does not hold. As designed,
  anyone who reads the `api_tokens` table — a SQL dump, a backup, a replica,
  a read-only SQL injection elsewhere — gets every user's live credential
  and can call the API as them, whereas passwords in the same database are
  bcrypt-hashed (`users.password_hash`). With no revocation, the exposure
  cannot be undone. Update tasks.md 1.1 and 1.2 accordingly.
history:
  - status: open
    by: security
```

#### R-security-3 Token lookup by database equality may leak timing

```yaml
severity: low
confidence: low
location: "design.md#Decision: Tokens are stored as given"
evidence: |
  middleware looks a request's token up with `WHERE token = $1`
recommendation: >-
  Looking up the hash of the presented token (see R-security-2) instead of
  the token itself removes any byte-by-byte timing signal from the index
  comparison; no separate fix is needed if R-security-2 is fixed. Whether the
  database comparison is observable remotely could not be verified.
history:
  - status: open
    by: security
```
