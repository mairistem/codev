# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 camelCase renames every existing order JSON field

```yaml
severity: high
confidence: high
location: "src/api.rs:6"
evidence: |
  #[serde(rename_all = "camelCase")]
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id`,
  `total_cents` and `status` keep their snake_case names and only `currency`
  is added. Today `GET /orders/{id}` returns `order_id`, `customer_id` and
  `total_cents`; after this change it returns `orderId`, `customerId` and
  `totalCents`, so every partner integration reading those fields gets a
  missing field on the first response after deploy. `docs/api.md` promises
  that "a field is never renamed or removed without two releases of
  deprecation", the proposal only announces an added field, and the release
  note has no **BREAKING** item. If camelCase is wanted, it needs its own
  change: emit both names for two releases, then mark the removal BREAKING.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config.toml without default_currency no longer loads

```yaml
severity: high
confidence: high
location: "src/config.rs:10"
evidence: |
  pub default_currency: String,
recommendation: >-
  Give the field a default — `#[serde(default = "...")]` returning `"EUR"`,
  the currency every existing order is in — so a deployed `config.toml`
  holding only `database_url` and `listen` keeps loading. As written,
  `toml::from_str` fails with a missing-field error on such a file and the
  service refuses to start after the upgrade; neither the release note nor
  the docs tell operators to add the key. If the key must stay required,
  announce it as a **BREAKING** item in the release note with the line to
  add, and add a test that loads the pre-change `config.example.toml`.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
