# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 SQL injection through the search term

```yaml
severity: high
confidence: high
location: "src/users.ts:10-11"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
  return query<UserRow>(sql);
recommendation: >-
  CWE-89. Any holder of the support role sends `GET /users/search?q=` with a
  term such as `x' UNION SELECT id, password_hash FROM users --` (the route
  only checks its length, src/server.ts:13-17) and reads or alters any table
  the database user can reach. Bind the term as a parameter, as `query` in
  src/db.ts expects: `query<UserRow>("SELECT id, name FROM users WHERE name
  LIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`, and add a test with
  a term containing a quote.
history:
  - status: open
    by: security
```

#### R-security-2 LIKE wildcards in the term are not escaped

```yaml
severity: low
confidence: high
location: "src/users.ts:10"
evidence: |
  WHERE name LIKE '%" + term + "%'
recommendation: >-
  Once the term is bound, `%` and `_` in it still act as wildcards, so
  `q=%%` matches every user instead of names containing `%`. Escape `\`,
  `%` and `_` in the term before wrapping it in `%…%` (with `ESCAPE '\'`).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
