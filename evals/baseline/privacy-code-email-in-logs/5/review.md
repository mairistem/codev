# Review record: add-signup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Sign-up logs the member's e-mail address and display name

```yaml
severity: high
confidence: high
location: "app/signup.py:14"
evidence: |
  log.info("new signup email=%s name=%s id=%s", email, display_name, user_id)
recommendation: >-
  Log only the opaque id (`log.info("new signup id=%s", user_id)`). As written,
  every sign-up writes the e-mail address and display name in clear to the
  application logs, which are kept, shipped and read outside any erasure path,
  against data minimisation and storage limitation (GDPR Art. 5(1)(c), 5(1)(e)).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 The analytics event sends the e-mail address to the Beacon SaaS

```yaml
severity: high
confidence: high
location: "app/signup.py:15"
evidence: |
  track("signup", {"user_id": user_id, "email": email})
recommendation: >-
  Drop `email` from the event properties: the stated purpose, counting sign-ups
  in product analytics, needs no e-mail address, and `user_id` already
  identifies the event. As written, every sign-up sends the member's e-mail
  address to a third party (`https://beacon.example.com/v1/events`), where
  account erasure will never reach it (GDPR Art. 5(1)(c), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Beacon is a new processor that the plan does not identify

```yaml
severity: medium
confidence: low
location: "app/analytics.py:1"
evidence: |
  """Product analytics, sent to the Beacon SaaS (beacon.example.com)."""
recommendation: >-
  Question for the team: name Beacon as a processor of member data (even the
  `user_id` alone is a pseudonymous identifier), confirm the processing
  agreement (GDPR Art. 28), where it hosts the data (Chapter V if outside the
  EEA) and its retention, and record them in the proposal. The proposal only
  says "Sign-ups are counted in product analytics".
history:
  - status: open
    by: privacy
```

#### R-privacy-4 No retention or erasure defined for the collected e-mail and display name

```yaml
severity: medium
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - `signup(email, display_name)` creates a member.
recommendation: >-
  State in the plan how long a member's e-mail address and display name are
  kept and how an account is erased (users table, backups, and any copy in
  logs or Beacon). The change starts collecting this data from members
  themselves, but nothing in it defines retention or deletion (GDPR Art. 5(1)(e),
  Art. 17); whether an existing purge covers the `users` table could not be
  verified.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
