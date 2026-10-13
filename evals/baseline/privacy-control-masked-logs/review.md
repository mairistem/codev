# Review record: mask-emails-in-logs

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 Masked address keeps the full domain and first letter

```yaml
severity: medium
confidence: high
location: "src/logger.ts:6-8"
evidence: |
  export function maskEmails(text: string): string {
    return text.replace(EMAIL, "$1***@$2");
  }
recommendation: Mask the domain too (or replace the address with a keyed hash or opaque user id); a personal or small-company domain plus an initial still identifies the person, so the logs keep personal data for 90 days (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e) storage limitation).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Addresses the pattern misses still reach the logs in full or in part

```yaml
severity: medium
confidence: high
location: "src/logger.ts:3"
evidence: |
  const EMAIL = /([A-Za-z0-9._%+-])[A-Za-z0-9._%+-]*@([A-Za-z0-9.-]+\.[A-Za-z]{2,})/g;
recommendation: Also mask URL-encoded addresses (`ada%40example.org`, common in logged URLs and query strings), internationalised domains, and local parts with RFC 5322 characters such as `'` (`o'brien@example.com` becomes `o'b***@example.com`, leaking the prefix); add a test per case. GDPR Art. 5(1)(f), Art. 25.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Field keys are written unmasked

```yaml
severity: low
confidence: low
location: "src/logger.ts:12-15"
evidence: |
  for (const [key, value] of Object.entries(fields)) {
    safe[key] = typeof value === "string" ? maskEmails(value) : value;
  }
  console.log(JSON.stringify({ level: "info", message: maskEmails(message), ...safe }));
recommendation: |
  Mask the keys as well (`safe[maskEmails(key)]`), since a caller building fields keyed by address (e.g. `{ [user.email]: attempts }`) would log the address in full.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No test proves that log fields are masked

```yaml
severity: low
confidence: high
location: "src/logger.test.ts:4-11"
evidence: |
  describe("maskEmails", () => {
recommendation: Add a test on `logInfo` capturing `console.log` and checking that an address in `message` and in a string field never appears in full — the guarantee the proposal promises ("masks every e-mail address in the message and the fields").
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Logs already written keep full addresses for 90 days

```yaml
severity: medium
confidence: low
location: "proposal.md#Why"
evidence: |
  Our logs are kept 90 days by the hosting provider and contain customers' e-mail addresses.
recommendation: Decide with the team whether existing logs at the hosting provider (a processor, GDPR Art. 28) must be purged or left to expire, and state it in the change; the masking only protects future lines (Art. 5(1)(e)).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
