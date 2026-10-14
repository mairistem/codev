# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### newcomer

Disabled by configuration.

### privacy

#### R-privacy-1 Customer e-mail address written to the error log on a failed lookup

```yaml
severity: high
confidence: high
location: "src/support.ts:19-21"
evidence: |
  if (rows.length === 0) {
    throw new NotFoundError(`no customer with e-mail ${email}`);
  }
recommendation: >-
  Keep the e-mail address out of the error message (e.g. `throw new NotFoundError("no customer with this e-mail")`). Every lookup of an unknown address reaches `errorHandler` in src/server.ts, which runs `logger.error("request failed", { path: req.path, error: err.message })`, so the typed address — personal data, and possibly a third person's address mistyped by the agent — lands in the application logs with no retention rule tied to it (GDPR Art. 5(1)(c) data minimisation, Art. 32). Add a test that a not-found lookup logs no e-mail address.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Phone, birth date, address and e-mail sent to the Helply chat vendor when only the name is needed

```yaml
severity: high
confidence: high
location: "src/support.ts:38-49"
evidence: |
  user: {
    id: String(customer.id),
    name: customer.name,
    email: customer.email,
    phone: customer.phone,
    birthDate: customer.birth_date,
    address: customer.address,
  },
recommendation: The proposal's purpose is that the widget "greets the signed-in customer by name, so that the agent knows who they talk to"; send only `id` and `name` to Helply (an opaque id lets the agent use the support lookup for the rest). Every signed-in customer who opens the chat today has their phone number, birth date, postal address and e-mail address handed to a third-party processor beyond what the purpose needs (GDPR Art. 5(1)(c), Art. 25, Art. 28). Also select only the needed columns in `customerById` for this route.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Support lookup returns birth date, phone and postal address the identification purpose may not need

```yaml
severity: medium
confidence: low
location: "src/support.ts:15-18"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: The proposal states agents look a customer up so they stop asking "who they are"; returning the full record (birth date, phone, postal address) to every agent with the `support` role goes beyond identification unless the team states why agents need each field. Confirm which fields the support screen uses and return only those (GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply not identified as a processor, nor its hosting location

```yaml
severity: medium
confidence: low
location: "src/support.ts:37-41"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
  export function chatWidgetSettings(customer: Customer) {
    return {
      appId: process.env.HELPLY_APP_ID,
recommendation: Question for the team — the change starts sharing customer data with Helply, but the plan does not state the lawful basis, that a processor agreement (GDPR Art. 28) is in place, nor whether Helply transfers data outside the EEA (Chapter V). Record it in the proposal or a decision; only the controller can answer.
history:
  - status: open
    by: privacy
```
