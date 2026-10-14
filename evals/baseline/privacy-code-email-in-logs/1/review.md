# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 E-mail address and display name written to application logs

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: Log only the opaque `user_id` (e.g. `log.info("new signup id=%s", user_id)`). Every sign-up currently copies the member's e-mail and display name into the log files and whatever log pipeline collects them, where they escape the member's retention and erasure (GDPR Art. 5(1)(c) data minimisation, Art. 5(1)(e) storage limitation, Art. 17 erasure).
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
recommendation: Drop `email` from the event properties; counting sign-ups, the purpose the proposal states, needs no more than the event itself (or an opaque/pseudonymous id). As written every sign-up ships the member's e-mail to a third party (beacon.example.com) where account erasure cannot reach it (GDPR Art. 5(1)(c), Art. 25, Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor not identified in the plan

```yaml
severity: medium
confidence: low
location: "app/analytics.py:5"
evidence: |
  BEACON_URL = "https://beacon.example.com/v1/events"
recommendation: Question for the team — state in the proposal that Beacon is a new processor, whether a data processing agreement covers it (GDPR Art. 28), where it hosts data (Chapter V, transfer outside the EEA?), and add it to the records of processing (Art. 30). The proposal's Impact lists the file but not the vendor.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No retention period or erasure path for the collected e-mail and display name

```yaml
severity: medium
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
recommendation: Define how long member e-mail and display name are kept, how an account is erased (database, backups, and the log and Beacon copies of R-privacy-1 and R-privacy-2), and how access/portability reach them (GDPR Art. 5(1)(e), Art. 15–20). `app/db.py` is outside the diff, so the existing storage policy could not be checked.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Purpose and lawful basis of the collected data not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — collects e-mail addresses and display names
recommendation: Question for the controller — state in the proposal why the e-mail and the display name are needed (e.g. login and contact; public display) and on which lawful basis (GDPR Art. 6), so that collection can be checked against that purpose.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
