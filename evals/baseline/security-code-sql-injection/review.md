# Review record: add-user-search

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 SQL injection through the search term (CWE-89)

```yaml
severity: high
confidence: high
location: "src/users.ts:10"
evidence: |
  const sql = "SELECT id, name FROM users WHERE name LIKE '%" + term + "%' ORDER BY name LIMIT 50";
recommendation: >
  Pass the term as a bound parameter instead of concatenating it
  (e.g. "... WHERE name LIKE ? ESCAPE '\\' ..." with ["%" + escaped + "%"]),
  escaping the LIKE wildcards % and _ in the term. Any authenticated "support"
  user can send GET /users/search?q=x' UNION SELECT id, password FROM users --
  (only trimmed and length-checked in src/server.ts:13-14) and read or alter
  any table the database account can reach.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
