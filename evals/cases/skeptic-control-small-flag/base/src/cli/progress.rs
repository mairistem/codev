//! Progress lines of long commands, on standard error.

use std::io::Write;

/// Prints one line every `every` rows.
pub struct Progress {
    every: u64,
    seen: u64,
}

impl Progress {
    pub fn new() -> Self {
        Progress { every: 1000, seen: 0 }
    }

    /// Counts a row; prints a line when a block of rows is done.
    pub fn row(&mut self) {
        self.seen += 1;
        if self.seen % self.every == 0 {
            let _ = writeln!(std::io::stderr(), "imported {} rows", self.seen);
        }
    }
}
