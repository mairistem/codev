# Review record: batch-cart-prices

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 The speed gain is not backed by a measurement

```yaml
severity: low
confidence: high
location: "tasks.md#1. Implementation"
evidence: |
  - [x] 1.1 `cartTotal` with one query, verified by the cart page of a test account
recommendation: The proposal says large carts were slow, but the only check is loading the cart page of a test account, so nothing shows the gain. Time `cartTotal` (or the cart request) on a large cart, for example 100 items, before and after the change, run `EXPLAIN` on `SELECT id, price_cents FROM products WHERE id = ANY($1)`, and put the numbers in the change.
history:
  - status: open
    by: performance
```

#### R-performance-2 The cart-size bound the query relies on is not in the repository

```yaml
severity: low
confidence: low
location: "src/cart.ts:18-19"
evidence: |
  const ids = [...new Set(items.map((item) => item.productId))];
  const rows = await query<PriceRow>("SELECT id, price_cents FROM products WHERE id = ANY($1)", [ids]);
recommendation: The size of the `ANY($1)` array and of the result set grows with the number of distinct products in the cart. The proposal says "Carts are limited to 100 items by the existing validation", but no such validation exists in `src/` (only `cart.ts`, `db.ts` and `server.ts` are there), and `cartTotal` accepts any `CartItem[]`. Check that the 100-item limit is enforced before `cartTotal` is called. Also check that `products.id` is indexed (it is probably the primary key; the schema is not in the repository).
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
