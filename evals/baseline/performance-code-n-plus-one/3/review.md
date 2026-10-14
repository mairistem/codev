# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order (N+1) on the customer page

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
      for order in orders {
          let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Fetch the lines of every order in one query with the existing batch method
  `Db::lines_of_orders(&ids)` (`WHERE order_id = ANY($1)`, api/src/db.rs:30-33),
  then group them by `order_id` (e.g. a `HashMap<i64, i64>` of totals) and build
  the summaries in the original order of `orders`. Cost today: 1 + N queries per
  customer page view, N up to 100 — 101 round-trips on a user-facing path that
  runs on every view (proposal `## Domains`); after the fix, 2 queries whatever N.
  Keep the order of the summaries and the error behavior unchanged; orders
  without lines must still get a total of 0.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
