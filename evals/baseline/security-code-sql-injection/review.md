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
  CWE-89. Any account with the `support` role sends `GET /users/search?q=` with
  a term such as `x' UNION SELECT id, password_hash FROM users --` (2 to 100
  characters, only trimmed at src/server.ts:13-14) and reads any table the
  database user can read; because `query` is called with no parameters, `pg`
  uses the simple query protocol, so stacked statements such as
  `x'; DROP TABLE users; --` run too. Bind the term as a parameter, as
  `src/db.ts` documents ("values go in `params`, never in `sql`"):
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE '%' || $1 || '%' ORDER BY name LIMIT 50", [term])`,
  escaping `%`, `_` and `\` in the term if they must match literally, and add
  a test that a term containing a quote returns no extra rows and no error.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
