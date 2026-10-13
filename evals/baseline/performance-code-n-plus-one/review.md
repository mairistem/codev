# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

### performance

#### R-performance-1 N+1 query: one `lines_of_order` query per order

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Collect the order ids and fetch every line in one query with the existing
  `Db::lines_of_orders(&ids)` (api/src/db.rs:31, `WHERE order_id = ANY($1)`),
  then group the lines by `order_id` (e.g. a `HashMap<i64, i64>` of totals)
  while keeping the orders' `id DESC` order and an order with no line at 0.
  Cost today: 1 + N queries per customer page view (up to 101), growing with
  the customer's order count on a user-facing path; after the fix, 2.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
