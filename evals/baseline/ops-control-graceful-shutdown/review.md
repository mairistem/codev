# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

### ops

#### R-ops-1 The worker exposes no health check, no metric and no start log

```yaml
severity: high
confidence: high
location: "src/worker.rs:21-22"
evidence: |
  pub fn run(queue: &dyn Queue, mailer: &dyn Mailer, stop: &AtomicBool) {
      while !stop.load(Ordering::SeqCst) {
recommendation: Log a structured "worker started" event with its settings, and expose a liveness signal (a heartbeat updated each loop turn, or a health endpoint) plus counters for sent, rejected and released jobs and the queue depth or oldest job age, so a stuck or dead worker is noticed before e-mails pile up.
history:
  - status: open
    by: ops
```

#### R-ops-2 Temporary failures are retried forever, never dead-lettered

```yaml
severity: medium
confidence: high
location: "src/worker.rs:35-38"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts.clamp(1, MAX_STEPS);
      queue.release(job.id, delay);
recommendation: Bound the retries — after a maximum number of attempts (or a maximum age), dead-letter the job with a reason such as "relay unavailable after N attempts" — and add jitter to the delay so that every released job does not hit the relay at the same moment when it comes back.
history:
  - status: open
    by: ops
```

#### R-ops-3 A send that times out after the relay accepted the message is retried and may duplicate the e-mail

```yaml
severity: medium
confidence: low
location: "src/worker.rs:26"
evidence: |
  match mailer.send(&job.message, SEND_TIMEOUT) {
recommendation: Confirm how the mailer classifies a timeout after DATA; if it maps to `Unavailable`, make the retry idempotent (a stable Message-ID per job, checked or deduplicated by the relay) or treat an ambiguous timeout separately from a refused connection.
history:
  - status: open
    by: ops
```

#### R-ops-4 The "relay unavailable" warning carries no cause

```yaml
severity: medium
confidence: high
location: "src/worker.rs:38"
evidence: |
  tracing::warn!(job_id = job.id, attempts = job.attempts, delay_s = delay.as_secs(), "relay unavailable, job released");
recommendation: Carry the cause in `SendError::Unavailable` (connection refused, DNS, timeout, SMTP 4xx code and text) and log it as a field, so an operator can tell a relay outage from a network or credential problem.
history:
  - status: open
    by: ops
```

#### R-ops-5 Poll, timeout and retry settings are hard-coded

```yaml
severity: low
confidence: high
location: "src/worker.rs:11-17"
evidence: |
  const POLL: Duration = Duration::from_secs(2);
  /// How long one message may take to send.
  const SEND_TIMEOUT: Duration = Duration::from_secs(10);
  /// Delay before a job is handed out again, per attempt already made.
  const RETRY_STEP: Duration = Duration::from_secs(30);
  /// The delay stops growing after this many attempts (five minutes).
  const MAX_STEPS: u32 = 10;
recommendation: Read these values from the environment or configuration with the current values as defaults, so the send timeout or retry pace can be tuned during an incident without a release.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
