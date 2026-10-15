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
raised_by: [privacy]
recommendation: >-
  Log only the opaque user id (`log.info("new signup id=%s", user_id)`).
  Every sign-up currently copies the member's e-mail address and display
  name into the INFO log, and from there into every log sink, rotation and
  backup, outside the retention and erasure that apply to the users table
  (GDPR Art. 5(1)(c) data minimisation, 5(1)(e) storage limitation, Art. 17
  erasure). Add a test asserting the log record does not contain the e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the Beacon analytics SaaS to count sign-ups

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
raised_by: [privacy]
recommendation: >-
  Drop `email` from the event properties: the stated purpose ("Sign-ups are
  counted in product analytics") needs no e-mail address, and the user id
  already identifies the event. As written, every sign-up ships the member's
  e-mail to a third-party processor (beacon.example.com) where account
  erasure will not reach it (GDPR Art. 5(1)(c) data minimisation, Art. 17,
  Art. 28). Prefer a pseudonymous id over the raw database id if Beacon must
  not link events to members. Add a test asserting the tracked properties
  carry no e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New processor Beacon not identified in the plan, transfer location unknown

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
raised_by: [privacy]
recommendation: >-
  Question for the team: the proposal's Impact and Domains do not name Beacon
  as a new processor, nor whether it hosts data outside the EEA. Record the
  processor, its data processing agreement (GDPR Art. 28), its hosting
  region (Chapter V) and an entry in the records of processing (Art. 30)
  before events with personal data flow to it.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose, lawful basis and retention of the sign-up data are not stated

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
raised_by: [privacy]
recommendation: >-
  Question for the team: the change starts collecting e-mail addresses and
  display names through self sign-up but states neither the lawful basis
  (GDPR Art. 6), nor how long the `users` rows are kept, nor how a member's
  erasure or access request reaches them, backups included (Art. 5(1)(e),
  Art. 15–17). Only the controller can decide; state it in the proposal.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
