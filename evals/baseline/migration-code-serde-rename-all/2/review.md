# Review record: add-order-currency

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### migration

#### R-migration-1 rename_all = "camelCase" renames every existing order JSON field

```yaml
severity: high
confidence: high
location: "src/api.rs:6"
evidence: |
  #[serde(rename_all = "camelCase")]
recommendation: Remove `#[serde(rename_all = "camelCase")]` so `order_id`, `customer_id` and `total_cents` keep their names and only `currency` is added. Today `GET /orders/{id}` serializes them as `orderId`, `customerId` and `totalCents`, so every partner integration reading `order_id`/`customer_id`/`total_cents` gets missing fields right after the release, without any BREAKING item, contrary to docs/api.md ("a field is never renamed or removed without two releases of deprecation") and to the docs, which still list the snake_case names. If camelCase is wanted, emit both names during the deprecation period and announce it as BREAKING in the release note.
history:
  - status: open
    by: migration
```

#### R-migration-2 New required default_currency key makes existing config.toml fail to load

```yaml
severity: high
confidence: high
location: "src/config.rs:10"
evidence: |
  pub default_currency: String,
recommendation: Give the field a default (`#[serde(default = "default_currency")]` returning "EUR", the currency orders had before) and test that a config without the key still loads. Today every deployed `config.toml`, which holds only `database_url` and `listen`, makes `toml::from_str` return a "missing field `default_currency`" error at start-up, so the upgraded service does not start; the release note names no manual step. Otherwise mark it BREAKING in the release note with the key operators must add.
history:
  - status: open
    by: migration
```
