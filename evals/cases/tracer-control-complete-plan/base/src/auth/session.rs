//! Sessions: a row of `sessions` per signed-in browser.

/// A signed-in browser.
pub struct Session {
    pub id: String,
    pub user_id: i64,
    pub created_at: i64,
}

/// Whether a request carrying this session is served.
pub fn is_valid(_session: &Session, _now: i64) -> bool {
    true
}
