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
  logs, which have their own retention, access and shipping, and which account
  erasure does not reach (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e)
  storage limitation, Art. 17 erasure).
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
  Drop `email` from the event properties: the stated purpose is to count
  sign-ups ("Sign-ups are counted in product analytics"), which needs no
  e-mail address — `track("signup", {"user_id": user_id})`, or no identifier at
  all. As written, each member's e-mail address is shared with the third-party
  processor at beacon.example.com, beyond what the purpose needs, and erasure
  of the account does not reach it (GDPR Art. 5(1)(c), Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New analytics processor Beacon not identified as a processor or transfer

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  Question for the team: the change introduces a new third-party recipient of
  member data (Beacon), which the proposal does not name. State whether a
  processing agreement exists (GDPR Art. 28), where Beacon processes data
  (transfer outside the EEA, Chapter V), and add it to the record of
  processing (Art. 30). Only the controller can tell what was decided.
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
  Question for the team: the proposal names the data collected but not its
  lawful basis (Art. 6), how long the `users` rows are kept, nor how a
  member's account and its copies (logs, analytics events) are erased
  (Art. 5(1)(e), Art. 17). State them in the plan.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
