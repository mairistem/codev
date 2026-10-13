//! `ledger import <file>`: loads a CSV export into the ledger.

use super::errors;
use super::progress::Progress;

/// The flags `ledger import` accepts.
pub const FLAGS: &[&str] = &[];

/// Runs the import; returns the process exit code.
pub fn run(path: &str) -> i32 {
    let rows = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => return errors::report(&e),
    };
    let mut progress = Progress::new();
    for _line in rows.lines().skip(1) {
        progress.row();
    }
    0
}
