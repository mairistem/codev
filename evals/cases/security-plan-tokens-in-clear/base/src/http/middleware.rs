//! Every request goes through `authenticate` before reaching a handler.

/// The user a request acts for.
pub struct Caller {
    pub user_id: i64,
}

/// A request as the middleware sees it.
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
}

/// Reads the session cookie and returns the caller, or `None` for 401.
pub fn authenticate(request: &Request, sessions: &dyn Fn(&str) -> Option<i64>) -> Option<Caller> {
    let cookie = request
        .headers
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("cookie"))?;
    let session = cookie.1.strip_prefix("session=")?;
    sessions(session).map(|user_id| Caller { user_id })
}
