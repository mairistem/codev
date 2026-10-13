# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

### privacy

#### R-privacy-1 E-mail address and display name written to the application log

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: Log only the opaque user id (`log.info("new signup id=%s", user_id)`); logs are copied to aggregators and kept outside any erasure path (GDPR Art. 5(1)(c) data minimisation, Art. 32).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 E-mail address sent to the Beacon analytics SaaS when the user id suffices

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
recommendation: Drop `email` from the event properties and send only `user_id` (or a pseudonymous id); counting sign-ups needs no e-mail address (GDPR Art. 5(1)(c), Art. 25).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 New third-party processor Beacon is not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:5"
evidence: |
  BEACON_URL = "https://beacon.example.com/v1/events"
recommendation: State in the proposal that Beacon is a new processor, confirm a data processing agreement exists and where it stores data (GDPR Art. 28, Chapter V if outside the EEA); a question for the team, not a code fix.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No retention or erasure defined for the new member data and its copies

```yaml
severity: medium
confidence: high
location: "app/signup.py:13-15"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
  track("signup", {"user_id": user_id, "email": email})
recommendation: Define how long the member record is kept and how account deletion reaches it, the log lines and the Beacon events, backups included (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Purpose and lawful basis of the collected data are not stated

```yaml
severity: low
confidence: high
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: Ask the team to state in the proposal the purpose and lawful basis for the e-mail address, the display name and the analytics event (GDPR Art. 5(1)(b), Art. 6), and whether the display name is needed at sign-up.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
