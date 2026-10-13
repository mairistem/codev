//! Errors of the command line, on standard error, with their exit code.

use std::io::Write;

/// Prints the error and returns the exit code to use.
pub fn report(error: &dyn std::error::Error) -> i32 {
    let _ = writeln!(std::io::stderr(), "error: {error}");
    1
}
