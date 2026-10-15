# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Addresses with a non-ASCII character are logged in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >
  The pattern only accepts ASCII letters on both sides of the `@`, and its first group needs
  an ASCII character right before the `@`. `logInfo("signup", { user: "josé@exemple.fr" })`
  finds no match (the character before `@` is `é`) and writes `josé@exemple.fr` whole to the
  logs the hosting provider keeps 90 days — the very leak the proposal promises to stop
  (GDPR Art. 5(1)(c), Art. 32). Same for an internationalized domain (`ada@exämple.org`).
  Match on `\S+@\S+` (or Unicode classes with the `u` flag, `[\p{L}\p{N}._%+-]`) and add
  tests with an accented local part and an IDN domain.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Field keys are written to the log without masking

```yaml
severity: medium
confidence: high
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >
  Only the values go through `maskEmails`; the keys are copied as they are. A caller that
  indexes fields by address — `logInfo("quota", { [user.email]: 3 })` — writes
  `{"ada.lovelace@example.org":3}` to the logs, against the module's own promise ("never
  writes a full e-mail address") and GDPR Art. 5(1)(c). Mask the key too
  (`safe[maskEmails(key)] = …`) and cover it with a test.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 URL-encoded addresses escape the mask

```yaml
severity: medium
confidence: low
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >
  The pattern requires a literal `@`. A logged URL or query string such as
  `/reset?email=ada.lovelace%40example.org` keeps the address in full. Whether callers log
  request URLs could not be checked in this change. Also match the encoded form `%40`
  (case-insensitive), or decode before masking.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 The kept domain and first letter can still identify the person

```yaml
severity: low
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >
  `a***@example.org` keeps the full domain. For a personal or small company domain
  (`l***@nicaise.fr`) the masked value still points to one person, so the logs keep
  personal data (GDPR Art. 4(1), Art. 5(1)(c)). Depends on the customer base, which could not
  be seen. If re-identification matters, mask the domain too or replace the address with a
  keyed hash.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
