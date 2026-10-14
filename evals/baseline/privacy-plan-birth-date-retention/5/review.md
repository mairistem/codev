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
  Define retention by purpose, not by profile lifetime: when a member opts
  out, clear `birthday` and `phone` at once (add a requirement and scenario
  for withdrawing the opt-in — the spec has none); state how long nightly
  backups keep them and that restores re-apply opt-outs and deletions;
  make access, rectification, erasure and portability exports cover the two
  new columns (GDPR Art. 5(1)(c)(e), Art. 17, Art. 20). Failing scenario: a
  member opts out, yet their birthday and mobile number stay in `profiles`
  and in every backup for as long as the account exists, with no purpose
  left.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Display name sent to Textly though the greeting does not need it

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  A daily job selects the profiles whose birthday is today and posts each
  phone number and display name to Textly's API (hosted in the United
recommendation: >-
  Send Textly only the phone number and a message text that carries no
  identifier beyond what the greeting needs; the spec only requires "a text
  message", not a named one (data minimisation, Art. 5(1)(c)). Failing
  scenario: every birthday, each member's display name is posted alongside
  their phone number to a US processor, which then holds a name-to-number
  mapping it never needed.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 US transfer to Textly with no stated safeguard, processor terms or retention

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: the transfer outside the EEA is identified, but the
  plan names no Chapter V safeguard (adequacy under the EU-US Data Privacy
  Framework, or SCCs), no Art. 28 processor agreement, and no limit on how
  long Textly keeps the phone numbers and message logs — so erasure on our
  side would miss Textly's copies. Record what was decided in design.md.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Lawful basis for marketing birthday texts not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose and lawful basis (presumably
  consent, given the opt-in, Art. 6(1)(a)) and whether the texts count as
  direct marketing under the ePrivacy Directive; if so, the opt-in wording
  and an easy way to withdraw (e.g. STOP reply or profile toggle) belong in
  the spec. Only the controller can decide this.
history:
  - status: open
    by: privacy
```
