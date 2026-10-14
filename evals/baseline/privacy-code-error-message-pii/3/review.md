# Review record: add-support-lookup

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### privacy

#### R-privacy-1 Looked-up e-mail address written to the logs through the error message

```yaml
severity: high
confidence: high
location: "src/support.ts:20"
evidence: |
  throw new NotFoundError(`no customer with e-mail ${email}`);
recommendation: >-
  Keep personal data out of the error message: throw `new NotFoundError("no customer")`
  (or any constant text). `errorHandler` in `src/server.ts` logs `err.message`
  (`logger.error("request failed", { path: req.path, error: err.message })`), and
  `src/logger.ts` ships every line to the log platform, so each lookup of an unknown
  address by a support agent stores that typed e-mail address — of a customer or of
  anyone — in the logs with no retention rule (GDPR Art. 5(1)(c), 5(1)(e), 32). The
  same applies, with less impact, to `no customer ${id}` at `src/support.ts:32`, which
  logs the customer identifier: drop it too.
history:
  - status: open
    by: privacy
```

#### R-privacy-2 Chat vendor Helply receives phone, birth date, address and e-mail when the purpose needs the name

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
  Send Helply only what the stated purpose needs — the proposal says the widget
  "greets the signed-in customer by name, so that the agent knows who they talk to":
  `id` (opaque) and `name`. Remove `email`, `phone`, `birthDate` and `address`; the
  agent can look the rest up internally by id. As written, every signed-in customer who
  opens a page with the chat has their full record shared with a third-party processor
  and returned to the browser by `GET /chat/settings` (GDPR Art. 5(1)(c), Art. 25, Art. 28).
  Load only the needed columns in `customerById` for this route.
history:
  - status: open
    by: privacy
```

#### R-privacy-3 Helply is a new processor with no stated legal basis, contract or transfer location

```yaml
severity: medium
confidence: low
location: "src/support.ts:37"
evidence: |
  /** What the Helply chat widget is started with, for the signed-in customer. */
recommendation: >-
  Question for the team: is Helply covered by a processor agreement (GDPR Art. 28),
  recorded in the records of processing (Art. 30), and where does it store data — a
  transfer outside the EEA needs Chapter V safeguards? State the lawful basis of sharing
  chat identity with it, and how erasure and access requests reach the copies Helply keeps.
  Only the controller can answer; nothing in the change or the repository says.
history:
  - status: open
    by: privacy
```

#### R-privacy-4 Support lookup returns birth date, address and phone without a stated need

```yaml
severity: medium
confidence: low
location: "src/support.ts:16"
evidence: |
  "SELECT id, email, name, phone, birth_date, address FROM customers WHERE lower(email) = lower($1)",
recommendation: >-
  The proposal's purpose is that agents stop asking customers who they are; it does not
  say agents need the birth date, postal address or phone of every customer they look up.
  Return only the fields the support console shows (likely `id`, `name`, `email`), or
  state in the proposal why each further field is needed (e.g. identity checks) — GDPR
  Art. 5(1)(c). Whether the console uses them could not be verified from the repository.
history:
  - status: open
    by: privacy
```

### newcomer

Disabled by configuration.
