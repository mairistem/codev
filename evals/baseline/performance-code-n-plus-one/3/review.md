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
  `order_summaries` runs one query for the orders, then one `lines_of_order`
  query per order: up to 101 round trips on every customer page view (the
  proposal's Domains line), growing with the number of orders shown. Collect
  the order ids and load every line in one query with the existing
  `db.lines_of_orders(&ids)` (`api/src/db.rs:31`, `WHERE order_id = ANY($1)`),
  then sum the totals per `order_id` (e.g. into a `HashMap<i64, i64>`), and
  build the summaries in the orders' original order so the result is
  unchanged — orders with no line keep a total of 0.
history:
  - status: open
    by: performance
```
