# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 SQL injection through the search term

```yaml
severity: high
confidence: high
location: "src/users.ts:10"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
recommendation: >-
  Any caller with the support role sends GET /users/search?q=x' UNION SELECT id, password FROM users --
  (the route only checks length 2–100 in src/server.ts:14) and the term is concatenated into the SQL,
  so they read or alter arbitrary tables (CWE-89). Bind the term as a parameter, as src/db.ts's
  `query(sql, params)` intends: `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY
  name LIMIT 50", ["%" + term + "%"])`, escaping `%`, `_` and `\` in the term with an `ESCAPE` clause so
  they match literally; add a test that a term containing a quote returns no row and no SQL error.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
