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
  Keep the e-mail address out of the error message (for example
  `throw new NotFoundError("no customer for this e-mail")`). As it stands,
  every lookup that matches no customer reaches `errorHandler` in
  src/server.ts, which runs `logger.error("request failed", { path: req.path,
  error: err.message })`, so the address an agent typed — a customer's or a
  mistyped one — lands in the application logs with no retention rule
  (GDPR Art. 5(1)(c) data minimisation, Art. 32).
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address it does not need

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
  Send Helply only what the stated purpose needs — greeting the signed-in
  customer by name so the agent knows who they talk to: the opaque id and
  the name (the e-mail only if the agent console needs it to match the
  account). Drop `phone`, `birthDate` and `address`; also stop selecting
  them in `customerById` (src/support.ts:28) since nothing else uses them.
  Today `GET /chat/settings` hands the full record to the browser and the
  widget forwards it to the third-party processor (GDPR Art. 5(1)(c), Art. 25,
  Art. 28).
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Support lookup returns the whole customer record

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The proposal's purpose is that agents stop asking customers who they are;
  confirm with the team which fields an agent needs to identify the
  customer and return only those from `POST /support/customers/lookup`
  (src/routes.ts:7). Birth date, phone and postal address go to every
  support agent on each lookup without a stated need (GDPR Art. 5(1)(c)).
  Low confidence: the agents' actual need is not stated in the plan.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Helply as processor: transfer, retention and erasure not identified

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
  A question for the team, not a code fix: state in the plan whether Helply
  is covered by a processor agreement (GDPR Art. 28), where it stores data
  (Chapter V if outside the EEA), how long it keeps chat users' data, and
  how a customer's erasure or access request reaches the copy Helply holds
  (Art. 15–17). Only the controller can say what was decided.
history:
  - status: open
    by: privacy
```

#### R-privacy-5 Customer id written to the error log

```yaml
severity: low
confidence: high
location: "src/support.ts:32"
evidence: |
  throw new NotFoundError(`no customer ${id}`);
recommendation: >-
  Use a message without the identifier ("no customer for this session");
  `errorHandler` logs `err.message`, so the signed-in customer's id ends up
  in the logs. The impact is limited to a session whose customer no longer
  exists.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
