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
  Anyone who reads the `api_tokens` table — a leaked backup or SQL dump, a
  read-only replica, a support account, an SQL injection elsewhere — gets
  every user's working bearer credential and calls the API as them
  (CWE-312 / CWE-256); since revocation is a Non-Goal, nothing can cut that
  access off. Store only a hash of the token (SHA-256 is enough for a 32-byte
  random value) in a `token_hash` column and look it up with
  `WHERE token_hash = $1`: the lookup stays a single indexed equality, so the
  stated reason for storing it as given does not hold. Update task 1.1 and
  1.2 and the spec accordingly (the token is shown once and never stored).
history:
  - status: open
    by: security
```

#### R-security-2 The Authorization header, and so every token, is written to the logs

```yaml
severity: high
confidence: high
location: "design.md#Decision: Every request is logged with its headers"
evidence: |
  To diagnose integration problems, the middleware logs each request's
  method, path and headers at `info` level, including `Authorization`, so
  support can see which token an integration sent.
recommendation: >-
  Every request writes its bearer token (and, since all headers are logged,
  the `Cookie` session too — see `src/http/middleware.rs:21`) into the info
  logs; anyone with log access — support, a log aggregator, a shipped log
  archive — replays it and acts as the user until the token is revoked,
  which this change cannot do (CWE-532, CWE-200). Never log `Authorization`
  or `Cookie`: redact them (e.g. `Bearer ***`), and if support must identify
  a token, log a non-secret identifier such as the token's row id or a short
  prefix of its hash. Change task 1.3 so `logs_requests` asserts the
  redaction.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
