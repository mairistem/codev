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
  Keep the e-mail address out of the error message — e.g. `throw new
  NotFoundError("no customer with this e-mail")`. `errorHandler`
  (src/server.ts) logs `error: err.message` for every failed request, so
  each lookup of an unknown address writes the typed e-mail (a customer's
  or a mistyped one) to the application logs, with no purpose and no
  retention rule (GDPR Art. 5(1)(c), 5(1)(e)). The same applies to any
  future message built from personal data.
history:
  - status: open
    by: privacy
```

Scenario: an agent posts `{"email": "jane@example.com"}` to
`/support/customers/lookup` for an address with no customer; `customerByEmail`
throws, `errorHandler` runs `logger.error("request failed", { path: req.path, error: err.message })`,
and the log line holds `no customer with e-mail jane@example.com`.

#### R-privacy-2 Chat vendor Helply receives phone, birth date and address to greet by name

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
  Send Helply only what the stated purpose needs — the proposal says the
  widget "greets the signed-in customer by name, so that the agent knows
  who they talk to": the opaque `id` and `name` (and `email` only if the
  team decides agents need it in the chat). Drop `phone`, `birthDate` and
  `address` from `chatWidgetSettings`, and select only the needed columns
  in `customerById`. Sending more to a processor than the purpose needs
  breaks data minimisation (GDPR Art. 5(1)(c), Art. 25); these settings
  are also served to the browser by `GET /chat/settings`, where the
  widget script can read them.
history:
  - status: open
    by: privacy
```

Scenario: any signed-in customer opens a page with the chat widget;
`GET /chat/settings` returns their phone number, birth date and postal
address, and the Helply widget forwards them to the vendor — data the
greeting never uses.

#### R-privacy-3 Support lookup returns the full customer record, birth date and address included

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The purpose is that agents stop asking customers to repeat who they are;
  confirm with the team which fields an agent needs to identify the
  customer, and return only those (likely `id`, `name`, `email`) instead of
  the whole record, birth date and address included (GDPR Art. 5(1)(c)).
  Confidence is low: what agents need is not stated in the plan.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply is a new processor with no stated agreement or transfer location

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
  Question for the team: the proposal names the chat vendor as a recipient
  but not whether a processing agreement (GDPR Art. 28) covers Helply, where
  it stores the data (Chapter V if outside the EEA), how long it keeps chat
  identities, and how erasure requests reach it. Record these in the plan
  or the record of processing (Art. 30); the code cannot answer them.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
