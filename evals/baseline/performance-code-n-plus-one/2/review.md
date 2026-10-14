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
  Fetch every line in one query with the existing batch method
  `Db::lines_of_orders(&ids)` (`WHERE order_id = ANY($1)`, api/src/db.rs),
  then group the lines by `order_id` (e.g. a `HashMap<i64, i64>` of totals)
  and build the summaries in the order `orders_of_customer` returned, so
  ordering and results stay identical. As written, each customer page view
  runs 1 + N queries, N being the customer's order count up to 100 — up to
  101 sequential round trips per request, growing with the customer's order
  history and multiplied by page traffic; the batch version runs 2.
history:
  - status: open
    by: performance
```
