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
  The `q` query-string parameter of `GET /users/search` (src/server.ts:13) is
  only trimmed and length-checked (2 to 100 characters), then concatenated
  into the SQL text. An authenticated user with the `support` role who sends
  `q=x' UNION SELECT id, password_hash FROM users --` reads arbitrary columns
  of any table (or alters data, depending on the database grants), well within
  the 100-character limit (CWE-89). Bind the term as a parameter, as
  `query()` in src/db.ts already supports:
  `query<UserRow>("SELECT id, name FROM users WHERE name ILIKE '%' || $1 || '%' ORDER BY name LIMIT 50", [term])`,
  and escape `%`, `_` and `\` in the term (with `ESCAPE '\'`) so they match
  literally instead of acting as wildcards. Add a test that sends a quote in
  `q` and checks it is matched literally.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
