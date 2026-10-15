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
  CWE-89. Any user with the `support` role sends `GET /users/search?q=` with a
  2-100 character value such as `x' UNION SELECT id, password_hash FROM users --`;
  the route in src/server.ts only checks its length, and searchUsers concatenates
  it into the SQL text, so the attacker reads any table the database user can
  reach (or alters data if stacked statements are allowed). `query` in src/db.ts
  already takes parameters ("values go in `params`, never in `sql`"): use
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`,
  escaping `%`, `_` and `\` in `term` (with `ESCAPE '\'`) so the term matches
  literally. Add a test that a quote in `q` returns no error and no extra rows.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
