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
  look a request up with `WHERE token_hash = $1` on the hash of the bearer
  value — still a single indexed equality, so the lookup stays fast. The
  clear token exists only in the creation response, shown once. As it
  stands, anyone who reads the `api_tokens` table — a database backup, a
  read replica, a support query, an SQL injection elsewhere — gets every
  user's live credential and can call the API as them (CWE-312 / CWE-256),
  while passwords in the same database are bcrypt-hashed. Update the
  design's alternatives and task 1.1 accordingly.
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
  Never log `Authorization` (nor `Cookie`, `Proxy-Authorization` or any
  other credential header): redact it with an allow-list of loggable
  headers, and to identify the token log a non-secret identifier instead
  — the token row's id or a short prefix of its hash. As designed, every
  request writes a usable bearer token to the `info` log, so anyone with
  log access (support, the log pipeline, a log aggregation vendor) can
  replay it and act as the user (CWE-532); passwords sent through the same
  middleware by today's integrations would be logged too. Combined with
  the absence of revocation, a token leaked through the logs cannot be
  stopped. Change task 1.3 so `logs_requests` asserts the header is
  redacted.
history:
  - status: open
    by: security
```

#### R-security-3 A leaked token cannot be invalidated until a later change

```yaml
severity: medium
confidence: low
location: "design.md#Risks / Trade-offs"
evidence: |
  - [A user loses a token] → they create another one; revoking the old one comes with the later revocation change.
recommendation: >-
  The proposal's own motive is that "a leaked integration exposes the
  account", yet a leaked token stays valid forever: creating another one
  does not stop the old one, and tokens have no expiry. Ship at least a
  minimal delete of a user's own token (server-side ownership check), or
  an expiry date on each token, with this change; or have a person accept
  the risk explicitly until the revocation change lands. Confidence is
  low because the impact depends on when that later change ships.
history:
  - status: open
    by: security
```

### skeptic

Disabled by configuration.
