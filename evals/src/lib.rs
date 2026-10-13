//! The evaluation suite of codev's reviewing roles.
//!
//! Cases seed defects in small projects (and controls hold none); the
//! runner has Claude Code review them with one role at a time; the scoring
//! counts, per role, what was found, missed and invented, and whether the
//! quoted evidence is real. Everything but the runner's shell part is pure
//! and tested without a model.

pub mod case;
pub mod checks;
pub mod lint;
pub mod observed;
pub mod report;
pub mod runner;
pub mod score;
pub mod suite;
