# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 SQL injection through the search term of GET /users/search

```yaml
severity: high
confidence: high
location: "src/users.ts:10-11"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
  return query<UserRow>(sql);
recommendation: >-
  The query-string value `q` (src/server.ts:13, only trimmed and length-checked)
  is concatenated into the SQL text. A caller with the `support` role sends
  `GET /users/search?q=' UNION SELECT 1, password FROM users --` (under 100
  characters) and gets columns of any table the database user can read, or
  alters the query at will (CWE-89). `src/db.ts` already offers parameters:
  use `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ESCAPE '\\'
  ORDER BY name LIMIT 50", ["%" + escaped + "%"])`, escaping `\`, `%` and `_` in
  the term so it matches literally, and add a test sending a quote in `q`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
