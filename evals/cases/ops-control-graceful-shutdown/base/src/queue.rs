//! The job queue (a table polled with `FOR UPDATE SKIP LOCKED`).

use std::time::Duration;

pub struct Job {
    pub id: i64,
    /// How many times the job was handed out, 1 the first time.
    pub attempts: u32,
    pub message: String,
}

pub trait Queue {
    /// The next job, waiting at most `wait`; `None` when there is none.
    fn next(&self, wait: Duration) -> Option<Job>;
    /// The job is done.
    fn ack(&self, id: i64);
    /// Hand the job out again after `delay`.
    fn release(&self, id: i64, delay: Duration);
    /// Keep the job aside for a person to look at; it is not retried.
    fn dead_letter(&self, id: i64, reason: &str);
}
