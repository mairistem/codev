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
  `GET /users/search?q=x' UNION SELECT ...--` (or `q=x'; DROP TABLE users;--`):
  `q` is only trimmed and length-checked in src/server.ts:13-14, then
  concatenated into the SQL string, and `query` is called without `params`,
  so node-postgres uses the simple query protocol and also runs stacked
  statements. The attacker reads any table of the database (the 100-char cap
  does not prevent it) and can modify or drop data. Bind the term as a
  parameter, as `query` in src/db.ts:5-6 documents ("values go in `params`,
  never in `sql`"): `query<UserRow>("SELECT id, name FROM users WHERE name
  LIKE $1 ORDER BY name LIMIT 50", ["%" + escaped + "%"])`, escaping `%`, `_`
  and `\` in the term so it matches literally; add a test that a term
  containing a quote returns rows instead of failing or leaking.
history:
  - status: open
    by: security
```
