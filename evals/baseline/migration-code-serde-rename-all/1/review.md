# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 camelCase rename silently renames every existing field of the public order JSON

```yaml
severity: high
confidence: high
location: "src/api.rs:6-10"
evidence: |
  #[serde(rename_all = "camelCase")]
  pub struct OrderResponse {
      pub order_id: i64,
      pub customer_id: i64,
      pub total_cents: i64,
recommendation: >-
  Remove `#[serde(rename_all = "camelCase")]` so `GET /orders/{id}` keeps
  emitting `order_id`, `customer_id` and `total_cents` and only adds
  `currency`. Today a partner integration reading `order_id` from the
  response gets a missing field (`orderId` is sent instead) on the first
  deploy, with no BREAKING item in the release note and in breach of the
  rule in docs/api.md ("a field is never renamed or removed without two
  releases of deprecation"); docs/api.md also still documents the
  snake_case names. If camelCase is wanted, ship it as a separate change
  that emits both names during two releases of deprecation and marks it
  **BREAKING** in the release note.
history:
  - status: open
    by: migration
```

#### R-migration-2 Existing config files without `default_currency` no longer load

```yaml
severity: high
confidence: high
location: "src/config.rs:9-10"
evidence: |
  /// The currency of orders that do not name one (ISO 4217).
  pub default_currency: String,
recommendation: >-
  Give the field a default (`#[serde(default = "default_currency")]`
  returning `"EUR"`, the currency of every existing order) so a deployed
  `config.toml` holding only `database_url` and `listen` keeps loading.
  As written, `config::load` returns a `missing field default_currency`
  error and the service fails to start on upgrade, with no release-note
  item telling operators to add the key. Add a test that loads the old
  two-key config, or, if the key must stay required, mark it **BREAKING**
  in the release note with the line to add.
history:
  - status: open
    by: migration
```

### newcomer

Disabled by configuration.
