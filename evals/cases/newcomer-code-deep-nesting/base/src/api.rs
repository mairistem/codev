//! The JSON types of the public HTTP API.

use serde::Serialize;

#[derive(Serialize)]
pub struct Account {
    pub id: i64,
    pub user_name: String,
}
