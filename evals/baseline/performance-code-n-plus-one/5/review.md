# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One query per order to load its lines (N+1)

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Load every line in one query with the existing batch method
  `db.lines_of_orders(&ids)` (`api/src/db.rs:31`, `WHERE order_id = ANY($1)`),
  then group the lines by `order_id` (a `HashMap<i64, i64>` of totals) and
  build the summaries in the order `orders_of_customer` returned, keeping
  orders with no line at a total of 0. This turns up to 101 queries per
  customer page view into 2, without changing results or ordering.
history:
  - status: open
    by: performance
```

`order_summaries` runs on every customer page view (proposal, `## Domains`). It runs one `orders_of_customer` query, then one `lines_of_order` query per order returned — up to 100 more round-trips per request, growing with the customer's order count, while `Db` already offers `lines_of_orders` to serve the whole set in one call. Breaks the criterion "No query or remote call runs once per item of a loop when one call can serve the whole set".

### newcomer

Disabled by configuration.
