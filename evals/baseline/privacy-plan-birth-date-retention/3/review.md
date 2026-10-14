# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Opting out does not erase the birthday and phone number

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
  kept as long as the profile exists, and copied with the rest of the
recommendation: >-
  Tie retention to the purpose, not to the profile (GDPR Art. 5(1)(e), Art. 17).
  Add a requirement and a scenario to specs/birthday-greetings/spec.md: when a
  member opts out, `birthday` and `phone` are cleared on their profile. Today the
  spec only covers opting in, so a member who withdraws keeps both fields stored
  for the life of the profile with no purpose left. Add the matching task in
  tasks.md.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 No retention or purge rule for the copies in nightly backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  State how long the nightly backups holding `birthday` and `phone` are kept and
  how an erasure (opt-out or account deletion) reaches them — for instance a
  bounded backup rotation after which erased values are gone, documented in the
  design (GDPR Art. 5(1)(e), Art. 17). As written, a member's birthday and phone
  number erased from `profiles` survive in every backup with no stated expiry.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Transfer of phone numbers and names to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: record Textly as a processor (Art. 28 contract) and the
  Chapter V mechanism for the transfer to the United States (adequacy decision /
  Data Privacy Framework certification or standard contractual clauses), and add
  Textly to the records of processing (Art. 30). Also confirm the display name is
  needed in the message; if the text can be generic, send the phone number only.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis for marketing text messages not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose and lawful basis of the processing
  (presumably consent given by the opt-in, Art. 6(1)(a) and Art. 7) and whether
  these texts count as direct marketing under the ePrivacy Directive; if so, the
  opt-in wording must say so and withdrawal must be as easy as giving consent
  (see R-privacy-1).
history:
  - status: open
    by: privacy
```
