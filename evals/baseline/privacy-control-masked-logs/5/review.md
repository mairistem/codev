# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with non-ASCII characters are logged in clear

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern only accepts ASCII letters on both sides of the `@`. For
  `logInfo("signup", { email: "josé@example.com" })` the character right
  before `@` is `é`, so no match starts anywhere and the full address goes
  to the 90-day hosting logs; `ada@exämple.com` fails the same way on the
  domain side. This breaks the proposal's promise ("masks every e-mail
  address") and GDPR Art. 5(1)(c) / Art. 25. Match Unicode letters
  (`\p{L}\p{N}` with the `u` flag) in the local part and the domain, and
  add tests for an internationalised local part and domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 URL-encoded addresses are not masked

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3-7"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The pattern requires a literal `@`. A logged request path such as
  `logInfo("GET /reset?email=ada%40example.org")` contains the address
  percent-encoded (`%40`), so `maskEmails` leaves it untouched and the full
  address reaches the logs (GDPR Art. 5(1)(c)). Also match `%40` as the
  separator (case-insensitive), or decode before masking, and cover it with
  a test.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field names are written to the log without masking

```yaml
severity: medium
confidence: high
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
  console.log(JSON.stringify({ level: "info", message: maskEmails(message), ...safe }));
recommendation: >-
  Only values go through `maskEmails`; keys are copied as is. A caller that
  keys fields by user, e.g. `logInfo("quota", { [user.email]: count })`,
  writes `{"ada.lovelace@example.org":3}` to the logs, against the docblock
  "never writes a full e-mail address" and the proposal's "the message and
  the fields". Mask the key too: `safe[maskEmails(key)] = …`.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The masked form keeps the first letter and the full domain

```yaml
severity: low
confidence: low
location: "src/logger.ts:5-7"
evidence: |
  /** `ada.lovelace@example.org` becomes `a***@example.org`. */
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  `a***@example.org` is still personal data when the domain is personal or
  small (e.g. `j***@janedoe.fr`), and it stays 90 days at the hosting
  provider. Whether that residue is acceptable depends on the domains the
  customers use, which the reviewer could not see. A question for the team:
  if the first letter and domain are not needed for debugging, replace the
  whole address with a fixed token or a keyed hash (GDPR Art. 5(1)(c),
  Art. 25).
history:
  - status: open
    by: privacy
```
