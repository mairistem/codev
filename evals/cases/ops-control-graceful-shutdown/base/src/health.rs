//! What the notifier's admin server reports: `/healthz` fails when no beat
//! came for a minute, `/metrics` exports the counters.

/// How a job ended, counted on `/metrics` as `notifier_jobs_total{outcome}`.
pub enum Outcome {
    Sent,
    Released,
    DeadLettered,
}

pub trait Health {
    /// The worker is alive and polling.
    fn beat(&self);
    /// Counts one job's outcome.
    fn count(&self, outcome: Outcome);
}
