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
  Bind the term as a parameter instead of concatenating it (CWE-89), as the
  `query` helper in src/db.ts requires ("values go in `params`, never in
  `sql`"): `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1
  ORDER BY name LIMIT 50", ["%" + term + "%"])`. Any user holding the
  `support` role sends `GET /users/search?q=x' UNION SELECT ...--` (the
  route in src/server.ts:12-18 only trims and checks 2-100 characters) and
  reads any table the database user can reach, or alters data through
  stacked statements. Add a test that a term containing a quote returns
  matching rows and no SQL error.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
