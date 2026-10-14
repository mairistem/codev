# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Typed e-mail address written to the log platform through the error message

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (for example
  `throw new NotFoundError("no customer for this e-mail")`). Every lookup
  that misses reaches `errorHandler` (src/server.ts:14), which runs
  `logger.error("request failed", { path: req.path, error: err.message })`,
  and the logger ships each line to the log platform. Each failed support
  search therefore stores the searched person's e-mail address in the logs
  (GDPR Art. 5(1)(c) minimisation, Art. 32), including addresses of people
  who are not customers or were mistyped.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date, address and e-mail when greeting needs a name

```yaml
severity: high
confidence: high
location: "src/support.ts:41-48"
evidence: |
    user: {
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
    },
recommendation: >-
  Send Helply only what the stated purpose needs — the proposal says the
  widget "greets the signed-in customer by name, so that the agent knows who
  they talk to": an opaque id and the name. Drop `email`, `phone`,
  `birthDate` and `address` from the widget settings (and from the
  `customerById` select if nothing else uses them). As written, every
  signed-in customer who loads `/chat/settings` hands their phone number,
  date of birth and postal address to the third-party chat processor
  (GDPR Art. 5(1)(b)-(c), Art. 25, Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Support lookup returns birth date, phone and address with no stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The proposal says only "Agents look a customer up by e-mail address.",
  without saying which fields an agent needs.
  Confirm with the team which fields the support screen uses and select
  only those; birth date and postal address are likely beyond the purpose
  (GDPR Art. 5(1)(c)). Confidence low: the support front-end that consumes
  this endpoint is not visible.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply processor and its location not identified in the plan

```yaml
severity: low
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — customers' records are read and the chat vendor receives who is chatting
recommendation: >-
  A question for the team: record Helply as a processor (data processing
  agreement, GDPR Art. 28, record of processing Art. 30) and state where it
  hosts the data — if outside the EEA, the Chapter V transfer mechanism.
  Only the controller can confirm what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Customer id written to the log platform on a missing record

```yaml
severity: low
confidence: high
location: "src/support.ts:32"
evidence: |
  throw new NotFoundError(`no customer ${id}`);
recommendation: >-
  The message reaches the logs through `errorHandler` (src/server.ts:14);
  the customer identifier is personal data. Drop the id from the message,
  or log it deliberately as a structured field if operations need it.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
