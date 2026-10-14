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
  Pass the term as a bound parameter instead of concatenating it (CWE-89):
  `query<UserRow>("SELECT id, name FROM users WHERE name ILIKE $1 ORDER BY name LIMIT 50", ["%" + term + "%"])`,
  as `src/db.ts` documents ("values go in `params`, never in `sql`"). The length
  check in `src/server.ts:14` (2 to 100 characters) does not prevent the attack.
  Add a test that a term containing a quote returns matching rows without error.
  Escaping `%` and `_` in the term (with `ESCAPE`) would also keep them literal.
history:
  - status: open
    by: security
```

Attack: any caller holding the `support` role sends
`GET /users/search?q=' UNION SELECT id, password_hash FROM users --` (under
100 characters); the term is spliced into the SQL string, so the query reads
any table or column the database user can reach and returns it in the JSON
response — data beyond the `id`/`name` the endpoint is meant to expose. A
compromised or malicious support account is enough; no further privilege is
needed.

### newcomer

Disabled by configuration.
