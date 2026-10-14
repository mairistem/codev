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
  Log only the opaque member id (`log.info("new signup id=%s", user_id)`).
  As written, every sign-up copies the member's e-mail address and display
  name into the INFO log and wherever logs are shipped and retained; those
  copies escape the users table, so account erasure and the retention rule
  never reach them (GDPR Art. 5(1)(c) data minimisation, 5(1)(e) storage
  limitation, Art. 17 erasure). Add a test asserting the log record holds
  neither the e-mail nor the name.
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
raised_by: [privacy]
recommendation: >-
  Drop `email` from the event properties: the proposal's stated purpose is
  that "Sign-ups are counted in product analytics", which needs no e-mail
  address (and arguably not even the user id). As written, every sign-up
  sends the member's e-mail address to the third party at
  https://beacon.example.com/v1/events (app/analytics.py:5), beyond what the
  purpose needs (GDPR Art. 5(1)(b)-(c) purpose limitation and minimisation,
  Art. 25 by default); the copy at the processor is then out of reach of
  access and erasure requests (Art. 15, 17). Add a test asserting the
  tracked properties hold no e-mail.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
raised_by: [privacy]
recommendation: >-
  Question for the team: the change introduces Beacon as a new recipient of
  member data, yet the proposal's Impact and What Changes do not name it as
  a processor. Confirm a data processing agreement exists (GDPR Art. 28),
  where Beacon hosts the data and whether that is a transfer outside the EEA
  (Chapter V), and record it in the plan and the record of processing
  (Art. 30). Confidence is low: the contract and hosting region cannot be
  seen from the code.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose, lawful basis and retention of sign-up data not stated

```yaml
severity: low
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
raised_by: [privacy]
recommendation: >-
  Question for the team: the change starts collecting e-mail addresses and
  display names from self sign-up, but the proposal states neither the
  lawful basis (Art. 6), nor how long a member row is kept, nor how it is
  erased (backups included). State them in the proposal; only the
  controller can decide them, and the users table predates this change.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
