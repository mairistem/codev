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
  every call to `signup("ada@example.org", "Ada")` writes the member's e-mail
  address and display name to the application log, where they escape the
  members table's retention and erasure (GDPR Art. 5(1)(c) data minimisation,
  Art. 5(1)(e) storage limitation, Art. 17 erasure) and reach whoever or
  whatever reads or ships the logs.
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
  Drop `email` from the event properties and send only `user_id` (or no
  identifier at all, since the proposal only needs sign-ups to be counted).
  As written, each sign-up posts the member's e-mail address to
  https://beacon.example.com/v1/events (app/analytics.py:5,9), a third-party
  processor, although the stated purpose — "Sign-ups are counted in product
  analytics" — does not need it (GDPR Art. 5(1)(b)-(c), Art. 25). The copy at
  Beacon is also out of reach of the member's access and erasure rights
  (Art. 15, 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  Question for the team: the change introduces Beacon as a processor receiving
  member data (at least `user_id`), but the proposal's Impact and Domains
  sections do not name it. State whether a processor agreement exists
  (GDPR Art. 28), where Beacon processes the data (Chapter V transfers outside
  the EEA) and add it to the record of processing (Art. 30). Could not be
  verified from the repository.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose, lawful basis and retention of the sign-up data not stated

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: >-
  Question for the team: the proposal says the change collects e-mail
  addresses and display names but states neither their lawful basis
  (GDPR Art. 6), nor how long the member record is kept, nor how it is
  deleted, backups included (Art. 5(1)(e), Art. 17). Only the controller can
  tell what was decided; record it in the proposal.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
