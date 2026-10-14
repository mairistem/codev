# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Looked-up e-mail address written to the error log

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep the e-mail address out of the error message, e.g.
  `throw new NotFoundError("no customer with that e-mail")`. The route
  hands the error to `errorHandler` (src/routes.ts:9 `next(error);`), which
  runs `logger.error("request failed", { path: req.path, error: err.message })`
  (src/server.ts), so every lookup that misses — a typo, a former customer,
  a person who never was one — writes the typed e-mail address to the
  application log, where it has no retention rule and escapes erasure
  (GDPR Art. 5(1)(c) and 5(1)(e)). Add a test asserting the logged message
  does not contain the address.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date, address and e-mail to greet by name

```yaml
severity: high
confidence: high
location: "src/support.ts:42-47"
evidence: |
      id: String(customer.id),
      name: customer.name,
      email: customer.email,
      phone: customer.phone,
      birthDate: customer.birth_date,
      address: customer.address,
recommendation: >-
  The proposal's purpose is that the Helply widget "greets the signed-in
  customer by name, so that the agent knows who they talk to": send only
  `id` and `name` to the widget, and drop `email`, `phone`, `birthDate`
  and `address`. As written, `GET /chat/settings` exposes the whole record
  to the browser and to the third-party Helply SDK for every signed-in
  customer, whether or not they ever open a chat — data minimisation
  (GDPR Art. 5(1)(c)) and sharing with a processor beyond the purpose
  (Art. 28). Also narrow the `customerById` SELECT to the columns used.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Support lookup returns birth date and address with no stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The proposal says agents look a customer up so they need not ask "who
  they are"; it does not state why an agent needs the birth date or postal
  address. Either return only the fields the support workflow needs (id,
  name, e-mail) or state in the proposal the purpose that requires the rest
  (e.g. identity verification). Whether agents need them is for the team
  to confirm (GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply as a new processor is not identified

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
  The change starts sending customer data to Helply, but the plan does not
  name it as a processor nor say where it hosts data (inside or outside the
  EEA), nor whether a processing agreement covers it and what retention it
  applies (GDPR Art. 28, Art. 30, Chapter V). A question for the team:
  record the processor, its location and lawful basis in the proposal.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
