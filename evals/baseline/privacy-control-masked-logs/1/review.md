# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### privacy

#### R-privacy-1 The address pattern misses valid and encoded addresses, which reach the logs in full

```yaml
severity: high
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: >-
  The module promises it "never writes a full e-mail address" and the proposal
  says every address in the message and the fields is masked, but the pattern
  only matches an ASCII local part made of `A-Za-z0-9._%+-` followed by a
  literal `@` and an ASCII domain. Each of these goes to the hosting
  provider's logs (kept 90 days) unmasked or partly masked:
  `logInfo("GET /reset?email=ada.lovelace%40example.org")` (percent-encoded
  `@`, as in a logged URL or query string) is written whole;
  `ada.lovelace@exämple.org` (internationalized domain) and
  `"ada lovelace"@example.org` (quoted local part) do not match and are
  written whole; `o'brien@example.org` matches only from `b` and is logged as
  `o'b***@example.org`, leaking the start of the local part. GDPR Art. 5(1)(c)
  data minimisation and Art. 25 data protection by design. Fix: decode or also
  match `%40` (case-insensitive), widen the local part to the RFC 5322 atext
  characters and quoted strings, accept non-ASCII domain labels, and add a test
  per form.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Field keys are written to the log without masking

```yaml
severity: medium
confidence: low
location: "src/logger.ts:12-13"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
recommendation: >-
  Only field values go through `maskEmails`; the keys are copied as they come.
  A caller that keys fields by address, e.g.
  `logInfo("retries", { [user.email]: 3 })`, writes the full address to the
  logs. No caller exists yet, so this depends on future use. Mask the keys too
  (`safe[maskEmails(key)]`), or document that keys must never hold personal
  data.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 The mask keeps the first letter and the full domain

```yaml
severity: low
confidence: low
location: "src/logger.ts:7"
evidence: |
  return text.replace(EMAIL, "$1***@$2");
recommendation: >-
  `a***@example.org` keeps the initial and the whole domain. For a personal or
  small-organization domain (e.g. `a***@lovelace-family.org`) the masked form
  can still single out the person, so it remains personal data in logs kept
  90 days. Whether such domains appear depends on the customer base, not
  visible here. If the domain is not needed for debugging, mask it too, or
  replace the address by a keyed hash so log lines stay correlatable.
history:
  - status: open
    by: privacy
```
