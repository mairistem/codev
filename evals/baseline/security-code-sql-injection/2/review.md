# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

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
  CWE-89. Any caller holding the `support` role sends
  `GET /users/search?q=x' UNION SELECT ...--` (src/server.ts:13-18 only trims
  and checks the length, 2 to 100 characters) and the term is concatenated
  into the SQL text, so they read any table of the database and, since the
  query runs without parameters (pg simple protocol allows several
  statements), can chain `; UPDATE ...` / `; DROP ...`. Bind the term as a
  parameter, as `query` in src/db.ts is documented to require ("values go in
  `params`, never in `sql`"): `query<UserRow>("SELECT id, name FROM users
  WHERE name LIKE $1 ESCAPE '\\' ORDER BY name LIMIT 50", ['%' +
  term.replace(/[\\%_]/g, '\\$&') + '%'])`, escaping `%`, `_` and `\` so the
  term is matched literally as the proposal's "name contains the term"
  requires. Add a test sending a quote in `q`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
