# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 SQL injection through the search term in searchUsers

```yaml
severity: high
confidence: high
location: "src/users.ts:10-11"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
  return query<UserRow>(sql);
recommendation: >-
  Bind the term as a parameter instead of concatenating it, as `query` in
  src/db.ts:5-6 requires ("values go in `params`, never in `sql`"):
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`.
  The length check in src/server.ts:14 (2 to 100 characters) does not
  prevent the attack. Also escape `%`, `_` and `\` in the term (with an
  `ESCAPE` clause) so they match literally, and add a test sending a quote
  in `q` that fails without the fix. CWE-89. An authenticated user with the
  `support` role sends `GET /users/search?q=' UNION SELECT id, password FROM users--`
  (well under 100 characters) and gets back any column of any table the
  database user can read, or alters data through stacked statements.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
