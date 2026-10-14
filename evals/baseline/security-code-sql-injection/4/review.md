# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 SQL injection in user search through the `q` query parameter

```yaml
severity: high
confidence: high
location: "src/users.ts:10"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
recommendation: >-
  Bind the term as a parameter instead of concatenating it (CWE-89):
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`,
  as `query` in src/db.ts already supports ("values go in `params`, never in `sql`").
  Escape `%`, `_` and `\` in the term (with an `ESCAPE` clause) so they match literally.
  The length check in src/server.ts:14 is not a defence: 100 characters are enough for a payload.
  Add a test that a term containing a quote returns no extra rows and raises no SQL error.
history:
  - status: open
    by: security
```

Attack: any caller holding the `support` role sends
`GET /users/search?q=x%' UNION SELECT id, password_hash FROM users --`;
`term` (2–100 characters, otherwise unchecked, src/server.ts:13-18) is
concatenated into the SQL text, so the injected `UNION` returns arbitrary
columns of any table the database user can read, beyond the `id, name`
the endpoint is meant to expose.

### newcomer

Disabled by configuration.
