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
recommendation: >-
  Log only the opaque id (`log.info("new signup id=%s", user_id)`). As written,
  every sign-up copies the member's e-mail address and display name into the
  application logs, which have their own retention, access and shipping, and
  which account erasure does not reach (GDPR Art. 5(1)(c) data minimisation,
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
  Drop `email` from the event properties: the stated purpose ("Sign-ups are
  counted in product analytics", proposal.md#What Changes) needs no e-mail
  address — a count needs at most an opaque or pseudonymised id. As written,
  every new member's e-mail address goes to a third-party processor
  (`https://beacon.example.com/v1/events`, app/analytics.py:5) where account
  erasure cannot reach it (GDPR Art. 5(1)(c), Art. 17, Art. 28). Consider
  whether `user_id` itself is needed for a count.
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
  Question for the team: the change adds a new processor (Beacon) receiving
  per-member events, but the proposal names neither the processor, its
  processing agreement, nor where it hosts the data (possible transfer
  outside the EEA, GDPR Art. 28, Chapter V). State them in the proposal and
  the record of processing (Art. 30).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No purpose, lawful basis, retention or erasure stated for the new member data

```yaml
severity: medium
confidence: low
location: "app/signup.py:13"
evidence: |
  user_id = insert_user(email=email, display_name=display_name)
recommendation: >-
  Question for the team: the proposal collects e-mail addresses and display
  names (proposal.md#Domains) but states no lawful basis, no retention period
  and no deletion path (backups included), and nothing shows that access,
  erasure and portability reach the stored member (GDPR Art. 5(1)(e), Art. 6,
  Art. 15–20). `app/db.insert_user` is not in the diff, so the storage could
  not be checked.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
