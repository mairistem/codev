# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone kept after opt-out and in backups with no expiry

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Define retention and deletion for `profiles.birthday` and `profiles.phone`
  (storage limitation, GDPR Art. 5(1)(e); withdrawal of consent, Art. 7(3)):
  a member who opts out still has both columns kept for the profile's whole
  life, since the plan ties retention to the profile, not to the opt-in.
  Clear both columns when the member opts out, add a spec scenario
  "A member opts out" (THEN birthday and phone are erased), and state how
  long the nightly backups holding them are kept and when they are purged.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Transfer of phone numbers to Textly in the United States has no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: the plan names Textly as a new processor and a
  transfer outside the EEA, but states no processor agreement (GDPR
  Art. 28) nor transfer mechanism (Chapter V — adequacy decision or
  standard contractual clauses), nor how long Textly keeps the numbers and
  messages. Record the decision in the design, or pick an EEA-hosted
  provider.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Display name sent to Textly though the greeting does not need it

```yaml
severity: low
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Data minimisation (GDPR Art. 5(1)(c)): the spec only requires "a
  greeting is sent to their phone", so the phone number is the only detail
  the processor needs. Send a generic message, or state in the spec that
  the greeting is personalised by name if that is the intended purpose.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis for the birthday and phone processing is not stated

```yaml
severity: medium
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: the proposal gives a marketing purpose but no
  lawful basis (GDPR Art. 6) for collecting birthday and phone and sending
  promotional-style SMS. If it is the opt-in consent, say so in the
  proposal, and make sure the opt-in wording covers the SMS sending
  (ePrivacy Directive) and the transfer to Textly.
history:
  - status: open
    by: privacy
```

### skeptic

Disabled by configuration.
