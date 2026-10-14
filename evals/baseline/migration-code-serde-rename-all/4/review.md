# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 `rename_all = "camelCase"` renames every existing order JSON field
```yaml
severity: high
confidence: high
location: "src/api.rs:6"
evidence: |
  #[serde(rename_all = "camelCase")]
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id` and
  `total_cents` keep their snake_case names and only `currency` is added. Today
  `GET /orders/{id}` serializes `orderId`, `customerId` and `totalCents`, so every
  partner integration reading `order_id`, `customer_id` or `total_cents` gets a
  missing field on the first response after deployment, with no warning. This
  breaks the documented contract in docs/api.md ("a field is never renamed or
  removed without two releases of deprecation"), is not marked **BREAKING**,
  and is absent from the release note, which only announces the added
  `currency`. If camelCase is ever wanted, emit both names during two releases
  of deprecation and announce it as BREAKING.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing `config.toml` files without `default_currency` no longer load
```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
      /// The currency of orders that do not name one (ISO 4217).
      pub default_currency: String,
recommendation: >-
  Give the new key a default that matches today's behavior, e.g.
  `#[serde(default = "default_currency")]` returning `"EUR"`, and add a test
  loading a config without the key. As written, `Config` derives `Deserialize`
  with a required `default_currency`, so every deployed `config.toml` written
  before this change (only `database_url` and `listen`) makes `config::load`
  fail with "missing field `default_currency`" and the service does not start
  after the upgrade. Only config.example.toml was updated; no migration step or
  release-note item tells operators to add the key.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
