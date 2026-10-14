# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### privacy

#### R-privacy-1 Looked-up e-mail address written to the error log

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Drop the e-mail from the message (`no customer for this e-mail`). The
  existing `errorHandler` (src/server.ts:14) logs `err.message`, so every
  lookup that finds nobody writes the typed e-mail address to the error log,
  breaking "no personal data goes to logs" (GDPR Art. 5(1)(c), 32). If a
  correlation is needed, log the customer-free request id instead.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address beyond the greeting purpose

```yaml
severity: high
confidence: high
location: "src/support.ts:41-48"
evidence: |
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
recommendation: >-
  The proposal's purpose is that the Helply widget greets the customer by
  name so the agent knows who they talk to: send `id` and `name` only (and
  `email` only if the team states why the widget needs it). Phone, birth
  date and postal address go to a third-party processor with no stated use
  (GDPR Art. 5(1)(c) data minimisation, Art. 25, Art. 28). Also stop
  selecting the unused columns in `customerById` (line 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Lookup returns birth date, phone and address to support agents

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The lookup's stated purpose is that agents stop asking customers who they
  are; the endpoint (src/routes.ts:7) returns the whole record, birth date
  included. Return only the fields the support console needs, or confirm
  with the team that agents need each of them (Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply as processor and its transfer location not identified

```yaml
severity: medium
confidence: low
location: "proposal.md#Domains"
evidence: |
  - [x] personal-data — customers' records are read and the chat vendor receives who is chatting
recommendation: >-
  Question for the team: is Helply covered by a processing agreement
  (Art. 28), recorded in the records of processing (Art. 30), and does it
  host data outside the EEA (Chapter V)? State the lawful basis for sending
  customer data to it. Only the controller can answer.
history:
  - status: open
    by: privacy
```
