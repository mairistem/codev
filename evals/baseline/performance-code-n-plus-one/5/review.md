# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order_lines query per order (N+1) in order_summaries

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  Replace the per-order `db.lines_of_order(order.id)` call with one call to the
  existing batch method `db.lines_of_orders(&order_ids)` (api/src/db.rs:30-33,
  `WHERE order_id = ANY($1)`), then group the lines by `order_id` (e.g. a
  HashMap<i64, i64> of totals) and build the summaries in the order returned by
  `orders_of_customer`, keeping orders with no line at a total of 0. This turns
  1 + N queries (up to 101 per customer page view, as the proposal says the
  endpoint runs on every customer page view) into 2, without changing the
  ordering, the results or the error behavior. Alternatively, sum
  `quantity * unit_price_cents` with `GROUP BY order_id` in SQL.
history:
  - status: open
    by: performance
```

Scenario: a customer with 100 or more orders opens their page; `orders_of_customer` returns 100 orders and the loop issues 100 further `SELECT ... FROM order_lines WHERE order_id = $1` round trips (api/src/db.rs:25), so the cost per request grows linearly with the customer's order count up to 101 queries, on a user-facing path, while `lines_of_orders` can serve the whole set in one query.
