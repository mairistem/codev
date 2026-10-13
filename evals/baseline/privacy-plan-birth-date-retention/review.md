# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

### privacy

#### R-privacy-1 Birthday and phone kept after opt-out and in backups with no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Tie retention to the purpose (GDPR Art. 5(1)(e), storage limitation): clear
  `birthday` and `phone` when the member opts out, and state how long the
  nightly backups keep them (backup rotation period) so a purged value
  disappears from backups within a defined delay. Add a spec scenario
  "A member opts out → birthday and phone are erased from the profile" and a
  matching task. Scenario that fails today: a member opts in, later turns
  greetings off; their birth date and mobile number stay in `profiles` for the
  life of the account and in every backup indefinitely, with no purpose left.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Opt-out and data-subject rights not specified for the new fields

```yaml
severity: medium
confidence: low
location: "specs/birthday-greetings/spec.md#Requirement: Members opt in to birthday greetings"
evidence: |
  The profile form SHALL offer birthday greetings as an option, off by default; a member who opts in SHALL give their birthday (day and month) and a mobile phone number.
recommendation: >-
  Add a requirement that a member can withdraw (opt out) at any time and that
  withdrawal stops the messages (Art. 7(3), Art. 21 for direct marketing), and
  confirm that access, rectification, erasure and portability (Art. 15–20)
  cover `birthday` and `phone`. Confidence low: the repository shows no
  existing export or account-deletion path to check whether it would reach the
  new columns.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Phone numbers and names sent to a US processor without a stated transfer safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: Textly is a new processor receiving mobile numbers
  and display names, and the transfer goes outside the EEA (GDPR Art. 28,
  Chapter V). State in the design the processor agreement and the transfer
  mechanism (adequacy decision / Data Privacy Framework certification, or
  SCCs), Textly's retention of the numbers and message content, and record it
  in the records of processing (Art. 30). Confidence low: only the controller
  knows whether a DPA and safeguards are in place.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose and lawful basis of the new data not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose (member greeting vs. marketing) and
  the lawful basis (consent via the opt-in, presumably) for `birthday` and
  `phone` (Art. 5(1)(b), Art. 6). If the messages are marketing, the SMS
  channel also needs prior consent under the ePrivacy rules, and the opt-in
  wording must say so; the phone number must not be reused for other
  campaigns without a new basis.
history:
  - status: open
    by: privacy
```
