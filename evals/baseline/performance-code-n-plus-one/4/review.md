# Review record: add-order-summaries

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### performance

#### R-performance-1 N+1 queries: one order_lines query per order

```yaml
severity: high
confidence: high
location: "api/src/orders.rs:14-15"
evidence: |
  for order in orders {
      let lines = db.lines_of_order(order.id)?;
recommendation: >-
  `order_summaries` runs on every customer page view (proposal, Domains:
  performance) and issues one `orders_of_customer` query plus one
  `lines_of_order` query per order returned — up to 101 sequential round
  trips per request for a customer with 100+ orders, growing with the
  customer's order count up to the limit and multiplied by traffic. The
  `Db` already offers `lines_of_orders(&[i64])`
  (`SELECT ... FROM order_lines WHERE order_id = ANY($1)`, api/src/db.rs:31):
  collect the order ids, fetch all lines in one call, group them by
  `order_id` in a `HashMap<i64, i64>` of totals, then build the summaries
  in the original order of `orders` (keeping orders with no line at 0) so
  ordering and results stay unchanged. Two queries per request instead of
  up to 101.
history:
  - status: open
    by: performance
```
