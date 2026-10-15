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
  Bind the term as a parameter, as `query` in src/db.ts expects ("values go in
  `params`, never in `sql`"): `query<UserRow>("SELECT id, name FROM users WHERE
  name LIKE '%' || $1 || '%' ORDER BY name LIMIT 50", [term])`, and add a test
  that a term containing a quote returns no extra rows. A caller with the
  `support` role sends `GET /users/search?q=' OR 1=1 UNION SELECT ...--` (only the
  2–100 length check in src/server.ts applies) and reads or alters any table the
  database user can reach (CWE-89).
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
