# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### privacy

#### R-privacy-1 Birthday and phone outlive the opt-in, in the table and in backups

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  Tie retention to the purpose (GDPR Art. 5(1)(e), Art. 25): when a member
  opts out, clear `birthday` and `phone` on their profile, and add a spec
  requirement and scenario for opting out (the spec only covers opting in
  and "the phone number was removed", which leaves the birthday stored).
  State how long the nightly backups are kept and that a cleared value
  disappears from them once they rotate, so erasure reaches every copy.
  Failing scenario: a member opts in, then turns greetings off; nothing
  sends them anything any more, yet their birthday and mobile number stay
  in `profiles` for the life of the account and in every nightly backup
  with no stated expiry.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Phone numbers and names go to a US processor with no stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  The transfer is identified; record, as a question for the team, the
  processor agreement with Textly (GDPR Art. 28), the transfer mechanism
  for the United States (Chapter V: adequacy decision or standard
  contractual clauses), and how long Textly keeps the numbers and message
  logs. If the greeting does not need the display name, send only the
  phone number and a generic text. Confidence is low: the contract and
  Textly's terms are not in the material.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Purpose and lawful basis are not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Ask the team to state the purpose and lawful basis in the proposal
  (GDPR Art. 5(1)(b), Art. 6): presumably consent through the opt-in,
  limited to the birthday greeting. "Marketing wants" suggests other uses
  of the birthday and phone; say whether any other use is excluded, or
  that it needs its own basis. Only the controller can decide this.
history:
  - status: open
    by: privacy
```

### skeptic

Disabled by configuration.
