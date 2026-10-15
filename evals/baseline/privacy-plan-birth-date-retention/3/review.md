# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Birthday and phone are kept after the member opts out

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  The only purpose of `birthday` and `phone` is the greeting (proposal.md#What Changes),
  yet retention is tied to the profile, not to the opt-in. A member who turns the option off
  keeps both values in `profiles` indefinitely, with no purpose left (GDPR Art. 5(1)(c) and (e),
  Art. 7(3) withdrawal of consent). State in the design that opting out clears both columns,
  add a spec scenario "A member opts out → birthday and phone are erased", and a task with its test.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or purge rule for the copies in the nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  The design copies birthdays and phone numbers into the nightly backups but defines neither how
  long those backups are kept nor how an erasure (opt-out or profile deletion) reaches them, so a
  deleted phone number survives in every backup for an unknown time (GDPR Art. 5(1)(e), Art. 17).
  State the backup retention period and that erased values are not restored from a backup
  (or are re-erased after a restore).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Phone numbers and names sent to a US processor with no transfer safeguard or processor terms stated

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Textly is a new processor receiving members' phone numbers and display names, outside the EEA.
  The plan names the vendor but states no data processing agreement (GDPR Art. 28), no transfer
  mechanism (Chapter V — adequacy decision or standard contractual clauses), and no limit on how
  long Textly keeps the numbers and messages. Question for the team: which safeguards apply, and
  is an EU-hosted provider an option? Also confirm the display name is needed in the message; if
  not, send only the number. Record the answer in the design and in the proposal's Impact, which
  only says "Textly account".
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis of the new processing is not stated

```yaml
severity: low
confidence: low
location: "proposal.md#What Changes"
evidence: |
  - Members can opt in to birthday greetings; they then give their birthday (day and month) and a mobile phone number.
recommendation: >-
  The opt-in suggests consent (GDPR Art. 6(1)(a)), but no artifact states the lawful basis, and
  the Why mentions marketing's interest. Question for the controller: is the basis consent, and
  is the message purely a greeting (no marketing content, which would also bring ePrivacy rules
  for SMS)? State the basis and purpose in the proposal.
history:
  - status: open
    by: privacy
```
