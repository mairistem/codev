# Review record: add-birthday-greetings

<!-- codev-review: 1 -->

## plan.challenge

Mode: inline.

### skeptic

Disabled by configuration.

### privacy

#### R-privacy-1 Opting out neither erases the birthday and phone nor stops the greetings

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in.
recommendation: >-
  Store the opt-in explicitly (a consent flag with its date) and define what
  withdrawing it does: clear `birthday` and `phone` from `profiles` when the
  member turns the option off, and make the daily job select only opted-in
  profiles. Add a spec scenario "A member opts out" (WHEN a member turns the
  option off THEN birthday and phone are erased and no greeting is sent) and a
  matching task. Today the plan adds only the two columns (tasks.md 1.1), the
  job selects "the profiles whose birthday is today" with no consent check, and
  nothing describes opting out: a member who withdraws keeps their birthday and
  phone in `profiles` and may keep receiving SMS sent through Textly
  (GDPR Art. 5(1)(c), 5(1)(e), Art. 7(3)).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Birthday and phone kept indefinitely in nightly backups, with no expiry or erasure

```yaml
severity: medium
confidence: high
location: "design.md#Decision: Birth dates live on the profile"
evidence: |
  kept as long as the profile exists, and copied with the rest of the
  database into the nightly backups.
recommendation: >-
  State the retention period of the nightly backups and how an erasure (opt-out
  or account deletion) reaches them — backup rotation within a stated delay, or
  re-applying erasures on restore. As written, a member's birthday and phone
  removed from `profiles` survive in every backup for an undefined time, and a
  restore brings them back (GDPR Art. 5(1)(e), Art. 17).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Phone numbers and names transferred to a US processor without a stated safeguard

```yaml
severity: medium
confidence: low
location: "design.md#Decision: Greetings are sent by SMS through Textly"
evidence: |
  phone number and display name to Textly's API (hosted in the United
  States), which sends the message.
recommendation: >-
  Question for the team: record Textly as a processor (Art. 28 agreement,
  record of processing under Art. 30) and name the transfer mechanism for the
  US (EU-US Data Privacy Framework certification or standard contractual
  clauses, GDPR Chapter V), or choose an EEA-hosted SMS provider. Also say
  whether Textly retains the phone numbers and message contents, and for how
  long. Only the controller can confirm what was agreed.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Purpose and lawful basis of the birthday SMS not stated

```yaml
severity: low
confidence: low
location: "proposal.md#Why"
evidence: |
  Members enjoy being remembered; marketing wants birthday messages.
recommendation: >-
  Question for the team: state the purpose (a greeting, or marketing) and the
  lawful basis — presumably consent through the opt-in (GDPR Art. 6(1)(a);
  ePrivacy Directive for marketing SMS) — and restrict the phone number to
  that purpose, so marketing does not reuse it for other campaigns
  (Art. 5(1)(b)).
history:
  - status: open
    by: privacy
```

#### R-privacy-5 What the job logs when Textly fails is not defined

```yaml
severity: low
confidence: low
location: "design.md#Risks / Trade-offs"
evidence: |
  - [Textly is down] → the job retries the next day.
recommendation: >-
  Say that the job's failure handling logs only the profile's `user_id` and the
  error, never the phone number, display name or the request body sent to
  Textly.
history:
  - status: open
    by: privacy
```
