//! The e-mail worker: sends queued e-mails until asked to stop.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::health::{Health, Outcome};
use crate::mailer::{Mailer, SendError};
use crate::queue::Queue;

/// How long the worker waits for a job before checking whether it must stop.
const POLL: Duration = Duration::from_secs(2);
/// How long one message may take to send.
const SEND_TIMEOUT: Duration = Duration::from_secs(10);
/// Delay before a job is handed out again, per attempt already made.
const RETRY_STEP: Duration = Duration::from_secs(30);
/// After this many attempts (half an hour of growing delays), a job the
/// relay still cannot take goes to the dead-letter queue.
const MAX_ATTEMPTS: u32 = 12;

/// Sends e-mails until `stop` is set, then returns after the e-mail in
/// progress. `main` sets `stop` on SIGTERM.
pub fn run(queue: &dyn Queue, mailer: &dyn Mailer, health: &dyn Health, stop: &AtomicBool) {
    tracing::info!(poll_s = POLL.as_secs(), send_timeout_s = SEND_TIMEOUT.as_secs(), max_attempts = MAX_ATTEMPTS, "worker started");
    while !stop.load(Ordering::SeqCst) {
        health.beat();
        let Some(job) = queue.next(POLL) else {
            continue;
        };
        match mailer.send(&job.message, SEND_TIMEOUT) {
            Ok(()) => {
                queue.ack(job.id);
                health.count(Outcome::Sent);
                tracing::info!(job_id = job.id, "e-mail sent");
            }
            Err(SendError::Rejected(reason)) => {
                queue.dead_letter(job.id, &reason);
                health.count(Outcome::DeadLettered);
                tracing::warn!(job_id = job.id, %reason, "e-mail rejected, kept in the dead-letter queue");
            }
            Err(SendError::Unavailable) if job.attempts >= MAX_ATTEMPTS => {
                queue.dead_letter(job.id, "relay unavailable after every attempt");
                health.count(Outcome::DeadLettered);
                tracing::error!(job_id = job.id, attempts = job.attempts, "relay unavailable, e-mail kept in the dead-letter queue");
            }
            Err(SendError::Unavailable) => {
                let delay = RETRY_STEP * job.attempts;
                queue.release(job.id, delay);
                health.count(Outcome::Released);
                tracing::warn!(job_id = job.id, attempts = job.attempts, delay_s = delay.as_secs(), "relay unavailable, job released");
            }
        }
    }
    tracing::info!("worker stopped");
}

/// Sets the returned flag on SIGTERM or SIGINT.
pub fn stop_on_signals() -> std::io::Result<Arc<AtomicBool>> {
    let stop = Arc::new(AtomicBool::new(false));
    for signal in [signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT] {
        signal_hook::flag::register(signal, Arc::clone(&stop))?;
    }
    Ok(stop)
}
