# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone kept after the member opts out

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Tie retention to the purpose: when a member opts out of birthday
  greetings, clear `birthday` and `phone` on their profile (and say so in
  the spec, with a scenario "A member opts out"), and add a task with its
  test. Today the columns are "filled only for members who opt in" but
  nothing empties them on withdrawal, so a member who opts out keeps both
  in `profiles` until the account is deleted (GDPR Art. 5(1)(e) storage
  limitation, Art. 7(3) withdrawal of consent, Art. 17 erasure).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention period for the new data in nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups holding `birthday` and `phone` are
  kept and how an erased or opted-out value leaves them (backup rotation
  period, or re-applying erasures on restore). As written, a member's
  birthday and phone number survive in backups for an undefined time
  after they opt out or delete their account (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Display name sent to Textly beyond what the greeting requires

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Send Textly only the phone number and the final message text, or, if a
  personalised greeting is wanted, state it in the spec and send no more
  than the first name the text needs. The spec only requires "a greeting
  is sent to their phone"; the display name posted to a US processor goes
  beyond that purpose (GDPR Art. 5(1)(c) data minimisation, Art. 25).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Transfer of phone numbers to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: confirm that a processor agreement with Textly
  exists (GDPR Art. 28) and which Chapter V mechanism covers the transfer
  to the United States (adequacy under the EU-US Data Privacy Framework,
  or standard contractual clauses), and record it in the design and the
  record of processing (Art. 30). Only the controller can say what was
  decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Lawful basis and purpose of the new data not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose (birthday greetings only, not
  other marketing use) and the lawful basis (consent through the opt-in,
  GDPR Art. 6(1)(a)) for `birthday` and `phone` in the proposal, so that
  "marketing wants" does not later justify reusing the phone numbers for
  other campaigns (Art. 5(1)(b) purpose limitation).
history:
  - status: open
    by: privacy
```
