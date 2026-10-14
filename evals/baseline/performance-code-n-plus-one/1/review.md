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
  `db.lines_of_orders(&ids)` (api/src/db.rs:31, `WHERE order_id = ANY($1)`),
  collecting the ids from `orders` first, then group the lines by `order_id`
  (e.g. a HashMap<i64, i64> of totals) and build the summaries in the order of
  `orders`, so ordering and results stay unchanged. Cost today: 1 + N queries
  per request, N up to 100 orders, on a user-facing path that runs on every
  customer page view (proposal `## Domains`); with the batch call it is 2
  queries whatever N. Alternatively, compute the totals in SQL with
  `SUM(quantity * unit_price_cents) ... GROUP BY order_id`.
history:
  - status: open
    by: performance
```
