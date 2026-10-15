# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 SQL injection through the `q` search term

```yaml
severity: high
confidence: high
location: "src/users.ts:10-11"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
  return query<UserRow>(sql);
recommendation: >-
  Bind the term as a parameter instead of concatenating it (CWE-89), as
  `query` in src/db.ts expects ("values go in `params`, never in `sql`"):
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`.
  Attack today: any session with the `support` role sends
  `GET /users/search?q=x' UNION SELECT id, password_hash FROM users --`
  (src/server.ts:12-18 only trims and checks the length, 2 to 100
  characters, which fits the payload) and reads any column of any table the
  database user can see, or alters data, beyond the id and name the endpoint
  is meant to expose. Consider also escaping `%`, `_` and `\` in the term so
  a search for them matches literally.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
