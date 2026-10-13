# Orders API

`GET /orders/{id}` returns an order as JSON: `order_id`, `customer_id`,
`total_cents` and `status`. Partners' integrations read these fields; a
field is never renamed or removed without two releases of deprecation.
