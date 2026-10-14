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
  CWE-89. Any caller holding the `support` role sends `GET /users/search?q=`
  with a value such as `x' UNION SELECT id, password_hash FROM users --`
  (2 to 100 characters, so it passes the length check in src/server.ts:14);
  `req.query.q` reaches `searchUsers` unescaped and is concatenated into the
  SQL, so the attacker reads any table the database user can read (or alters
  the query). Bind the term as a parameter, as `query` in src/db.ts already
  supports ("values go in `params`, never in `sql`"):
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE '%' || $1 || '%' ORDER BY name LIMIT 50", [term])`,
  and escape `%`, `_` and `\` in the term (with `ESCAPE '\'`) so they match
  literally. Add a test with a quote in `q`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
