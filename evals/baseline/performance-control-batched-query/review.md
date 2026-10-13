# Review record: batch-cart-prices

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 The speed claim is not backed by a measurement

```yaml
severity: low
confidence: high
location: "proposal.md#Release note"
evidence: |
  - Changed: Large carts load faster.
recommendation: Measure cartTotal before and after on a large cart (e.g. 100 items, with EXPLAIN on the ANY($1) query) and record the figures, or soften the release note; the task's only verification is `npm test -- cart` and no test or benchmark for cart exists in the repository.
history:
  - status: open
    by: performance
```

#### R-performance-2 The 100-item cart bound the query relies on is not found in the code

```yaml
severity: low
confidence: low
location: "src/cart.ts:18-19"
evidence: |
  const ids = [...new Set(items.map((item) => item.productId))];
  const rows = await query<PriceRow>("SELECT id, price_cents FROM products WHERE id = ANY($1)", [ids]);
recommendation: The batched query is sized by the cart (one row per distinct product), which is bounded only if carts are capped; the proposal cites "Carts are limited to 100 items by the existing validation", but no such validation is in src/ (server.ts only serves /health). Point to the validation, or cap items in cartTotal's caller.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
