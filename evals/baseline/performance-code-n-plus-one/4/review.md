# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 One order-lines query per order (N+1) on every customer page view

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
    for order in orders {
        let lines = db.lines_of_order(order.id)?;
recommendation: >-
  `order_summaries` runs one `orders_of_customer` query, then one
  `lines_of_order` query per order — up to 101 database round trips per call,
  on a path the proposal says runs on every customer page view; the cost grows
  linearly with the number of orders (capped at 100) and with traffic. `Db`
  already offers `lines_of_orders(&[i64])` (`api/src/db.rs:31`,
  `WHERE order_id = ANY($1)`): collect the order ids, fetch all lines in one
  query, sum the totals per `order_id` in a `HashMap`, then build the summaries
  in the orders' original order (`ORDER BY id DESC`), so the output order and
  the error behaviour stay the same. Two queries per call whatever the number
  of orders.
history:
  - status: open
    by: performance
```

### newcomer

Disabled by configuration.
