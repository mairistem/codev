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
  The `q` query-string parameter of `GET /users/search` (src/server.ts:13) is only trimmed and
  length-checked (2 to 100 characters), then concatenated into the SQL text. Any caller holding the
  `support` role can send `q=' UNION SELECT id, password_hash FROM users --` (or any other table) and
  read arbitrary data, or alter the query, through the endpoint (CWE-89). Bind the term as a parameter
  through the existing helper, which `src/db.ts` documents as "values go in `params`, never in `sql`":
  `query<UserRow>("SELECT id, name FROM users WHERE name LIKE $1 ESCAPE '\\' ORDER BY name LIMIT 50", ["%" + escaped + "%"])`,
  where `escaped` escapes `\`, `%` and `_` in the term so that user wildcards match literally. Add a test
  showing that a term containing a quote is matched literally and does not break the query.
history:
  - status: open
    by: security
```
