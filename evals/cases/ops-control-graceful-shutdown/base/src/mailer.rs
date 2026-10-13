//! The SMTP relay.

use std::time::Duration;

pub enum SendError {
    /// The relay refused the message for good (bad address, policy).
    Rejected(String),
    /// The relay could not be reached or answered a temporary failure.
    Unavailable,
}

pub trait Mailer {
    /// Sends one message, giving up after `timeout`.
    fn send(&self, message: &str, timeout: Duration) -> Result<(), SendError>;
}
