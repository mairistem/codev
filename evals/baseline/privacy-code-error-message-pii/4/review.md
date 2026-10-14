# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Customer e-mail address written to the error log on a failed lookup

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `throw new NotFoundError("no customer for this e-mail")`).
  When an agent looks up an address that matches no customer, `errorHandler` (src/server.ts:14,
  `logger.error("request failed", { path: req.path, error: err.message })`) writes the typed e-mail address
  to the application log, which then holds personal data with no purpose or retention rule — GDPR Art. 5(1)(c)
  data minimisation and 5(1)(e) storage limitation; the criterion "No personal data goes to logs" fails.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor Helply receives phone, birth date, address and e-mail when only the name is needed

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
  The proposal's purpose is that the widget "greets the signed-in customer by name, so that the agent knows who
  they talk to": send only `id` and `name` to Helply, and drop `email`, `phone`, `birthDate` and `address`
  (and select only `id, name` in `customerById`, src/support.ts:28). Every signed-in customer who opens the chat
  currently has their phone number, date of birth and postal address shipped to the third-party chat vendor
  through `GET /chat/settings` — GDPR Art. 5(1)(c) data minimisation, Art. 25 data protection by default.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply is a new processor with no stated agreement or transfer location

```yaml
severity: medium
confidence: low
location: "src/support.ts:37-40"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
  export function chatWidgetSettings(customer: Customer) {
    return {
      appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: is Helply already a processor under contract (GDPR Art. 28), listed in the records of
  processing (Art. 30), and where does it process the data — inside the EEA, or under a Chapter V transfer
  mechanism? The proposal names "the chat vendor" but neither the processor nor the transfer.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns birth date, phone and address beyond identifying the customer

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
      "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The stated purpose is that agents stop asking customers "to repeat who they are"; `POST /support/customers/lookup`
  returns the whole record, birth date and postal address included, to every support agent. Return only the
  fields agents need to identify the customer (id, name, e-mail), or state in the plan why the others are needed
  — GDPR Art. 5(1)(c). Confidence low: what agents need was not visible in the plan.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
