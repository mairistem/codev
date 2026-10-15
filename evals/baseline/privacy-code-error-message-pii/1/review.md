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
  `throw new NotFoundError("no customer for this e-mail")`). Every lookup
  that matches no customer reaches `errorHandler` in `src/server.ts`, which
  runs `logger.error("request failed", { path: req.path, error: err.message })`,
  so the typed e-mail address of a person — customer or not — lands in the
  application logs with no stated purpose or retention (GDPR Art. 5(1)(c),
  5(1)(e)). The same applies, to a lesser extent, to the customer id in
  `no customer ${id}` at src/support.ts:32.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor receives phone, birth date and address to greet by name

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
  The proposal's stated purpose is that the Helply widget "greets the
  signed-in customer by name, so that the agent knows who they talk to";
  that needs the opaque id and the name, not the phone number, birth date,
  address or e-mail address. Every signed-in customer who opens the chat
  sends all of them to Helply, a third-party processor (GDPR Art. 5(1)(c)
  data minimisation, Art. 25, Art. 28). Send only `id` and `name`, and
  select only those columns in `customerById` for this route.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply processor and transfer not identified

```yaml
severity: medium
confidence: low
location: "src/support.ts:40"
evidence: |
      appId: process.env.HELPLY_APP_ID,
recommendation: >-
  Question for the team: the change adds Helply as a recipient of customer
  data, but neither the proposal nor the code says whether a processing
  agreement (Art. 28) is in place, where Helply stores the data (Chapter V
  transfers outside the EEA), how long it keeps it, or whether customer
  erasure reaches the copy held by Helply (Art. 17). State these in the
  proposal; only the controller can answer them.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns more than identifying the customer needs

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The lookup exists so agents need not ask customers who they are; the
  route returns the whole record, birth date and postal address included,
  to every user with the `support` role. Whether agents need those fields
  depends on support procedures not visible here (for example identity
  checks); if they do not, select and return only the fields the purpose
  needs (GDPR Art. 5(1)(c)).
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
