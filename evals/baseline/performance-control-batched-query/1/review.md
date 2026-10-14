# Review record: batch-cart-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 The id list's bound rests on a validation not visible in the repository

```yaml
severity: low
confidence: low
location: "src/cart.ts:18-19"
evidence: |
  const ids = [...new Set(items.map((item) => item.productId))];
  const rows = await query<PriceRow>("SELECT id, price_cents FROM products WHERE id = ANY($1)", [ids]);
recommendation: The batched query is correct and its result is bounded by the number of distinct product ids in the cart. The proposal says carts are limited to 100 items by an existing validation, but no such validation is in the repository (src/server.ts has no cart route); check that the limit is enforced before `cartTotal` is reached, otherwise the `ANY($1)` array grows with whatever a client sends. Also confirm with `EXPLAIN` that the lookup on `products.id` uses its primary-key index (the schema is not in the repository).
history:
  - status: open
    by: performance
```

#### R-performance-2 The speed-up is not backed by a measurement

```yaml
severity: low
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `cartTotal` with one query, verified by the cart page of a test account
recommendation: The change exists because large carts were slow, but its verification only shows the cart page works for a test account. Time `cartTotal` (or the cart page) on a large cart, e.g. 100 items, before and after, and record the numbers so the improvement is measured rather than assumed.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
