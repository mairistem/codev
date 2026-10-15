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
  for a high-entropy secret, no bcrypt needed), and look it up with
  `WHERE token_hash = $1` on the hash of the presented token: the lookup stays
  a single indexed equality, so the "lookup must be fast" argument does not
  require the clear value. As written, anyone who reads the `api_tokens`
  table — a database backup, a read replica, a support SQL console, a later
  injection — obtains working bearer credentials for every integration of
  every user, and since revocation is a Non-Goal they cannot be invalidated
  (CWE-312, CWE-522). Update tasks 1.1 and 1.2 to match.
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
  Redact secret headers before logging: drop or mask `Authorization` and
  `Cookie` (the middleware already reads the session from `Cookie`), and log
  instead a non-secret token identifier (a token id or a short prefix of the
  token's hash) so support can still tell which token an integration sent.
  As written, every bearer token and every session cookie lands in the logs:
  anyone with access to the log pipeline or a log export replays them and
  acts as the user, with no revocation to stop it (CWE-532). Update task 1.3
  so `logs_requests` asserts the secret headers are absent.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be invalidated

```yaml
severity: medium
confidence: high
location: "design.md#Risks / Trade-offs"
evidence: |
  - [A user loses a token] → they create another one; revoking the old one comes with the later revocation change.
recommendation: >-
  The proposal's motivation is that "a leaked integration exposes the
  account", yet with revocation deferred and no expiry a leaked token keeps
  working forever: an attacker holding it keeps full API access to the
  account and the user has no way to cut it off — creating another token does
  not disable the old one. Either ship a minimal revocation (delete the
  token's row) in this change, or give tokens an expiry until the revocation
  change lands, and state the residual risk in the design.
history:
  - status: open
    by: security
```
