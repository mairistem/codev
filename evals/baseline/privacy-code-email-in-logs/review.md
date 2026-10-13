# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 E-mail address and display name written to application logs

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: Log only the opaque user id (e.g. `log.info("new signup id=%s", user_id)`); logs escape the erasure and retention rules of the users table (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e) storage limitation).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the third-party analytics vendor Beacon

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
recommendation: Drop `email` from the event and send only `user_id`; counting sign-ups needs no e-mail address (GDPR Art. 5(1)(c) data minimisation, Art. 25 data protection by default).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New processor Beacon not identified, nor its location or contract

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1-5"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""

  import requests

  BEACON_URL = "https://beacon.example.com/v1/events"
recommendation: State in the proposal that Beacon is a new processor, confirm a data processing agreement (GDPR Art. 28) and whether data leaves the EEA (Chapter V); question for the team, not a code fix.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No retention period nor erasure path for the stored e-mail and display name

```yaml
severity: medium
confidence: high
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
recommendation: Define how long member records are kept and how account deletion removes them, including log and analytics copies and backups (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Purpose and lawful basis of the collected data not stated

```yaml
severity: low
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: State in the proposal the purpose of each field and its lawful basis (GDPR Art. 6) — question for the controller.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
