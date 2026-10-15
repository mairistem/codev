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
  Bind the term as a parameter through the existing parameterized helper, e.g.
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ORDER BY name LIMIT 50", ["%" + escaped + "%"])`,
  escaping `%`, `_` and `\` in the term (with `ESCAPE '\'`) so they match literally. `src/db.ts` documents
  that values go in `params`, never in `sql`. Add a test sending a quote in `q` (CWE-89).
  Scenario: a user with the `support` role calls `GET /users/search?q=x' UNION SELECT id, password_hash FROM users --`;
  the term passes the 2-100 length check in `src/server.ts:14`, is concatenated into the SQL string, and the
  attacker reads any column of any table (or alters data) instead of searching names.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
