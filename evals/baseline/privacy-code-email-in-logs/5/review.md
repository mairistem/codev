# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 E-mail address and display name written to the application log

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: >-
  Log only the opaque id (`log.info("new signup id=%s", user_id)`). As written,
  every sign-up copies the member's e-mail address and display name into the
  application logs, which have no retention rule of their own and are not
  reached by access or erasure requests (GDPR Art. 5(1)(c) data minimisation,
  Art. 5(1)(e) storage limitation, Art. 17 erasure).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the Beacon analytics SaaS

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
recommendation: >-
  Drop `email` from the event properties. The stated purpose is that
  "Sign-ups are counted in product analytics", which needs no e-mail address;
  with this line every sign-up sends the member's e-mail address to the
  third-party processor at https://beacon.example.com (app/analytics.py:5),
  beyond the purpose (GDPR Art. 5(1)(b)-(c), Art. 25). Consider whether even
  `user_id` is needed to count sign-ups; if it is kept, Beacon still receives
  a pseudonymous identifier (see R-privacy-3).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New processor Beacon not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  The change introduces a new processor (Beacon, a SaaS at beacon.example.com)
  receiving member data, but the proposal does not name it, its data
  processing agreement (GDPR Art. 28), nor where it hosts the data (Chapter V
  transfers outside the EEA). Question for the team: record Beacon in the
  records of processing (Art. 30) and state its location, or keep only
  anonymous counts in the events. Confidence low: a register outside this
  repository may already cover it.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis and retention of sign-up data not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: >-
  Question for the team: the proposal states what is collected but not the
  lawful basis (Art. 6) for collecting the e-mail address and display name,
  nor how long the member record is kept and how it is erased, backups
  included. Only the controller can say; record it in the proposal.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
