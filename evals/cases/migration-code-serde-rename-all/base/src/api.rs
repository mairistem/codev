//! The JSON of the public orders API, read by partners' integrations.

use serde::Serialize;

#[derive(Serialize)]
pub struct OrderResponse {
    pub order_id: i64,
    pub customer_id: i64,
    pub total_cents: i64,
    pub status: String,
}
