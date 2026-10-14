# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is sent again

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: >-
  An SMTP send is not idempotent: when the relay accepts the message but its
  answer arrives after SEND_TIMEOUT, the mailer gives up and, if it reports
  that as `Unavailable`, the job is released and the recipient gets the
  e-mail twice (up to twelve times while the relay stays slow — the very
  situation this change is for). Give each job a stable Message-ID derived
  from `job.id` so retries are recognisable downstream, or have the mailer
  distinguish a timeout after DATA from a refused connection and not retry
  it blindly. Confidence is low: the mailer implementation, which decides
  how a timeout is reported, is not part of the diff.
history:
  - status: open
    by: ops
```

#### R-ops-2 No metric shows the backlog or the send latency

```yaml
severity: medium
confidence: low
location: "src/worker.rs:30-34"
evidence: |
  match mailer.send(&job.message, SEND_TIMEOUT) {
      Ok(()) => {
          queue.ack(job.id);
          health.count(Outcome::Sent);
recommendation: >-
  The worker exports only outcome counters (`notifier_jobs_total{outcome}`)
  and a liveness beat. When it falls behind — a slow relay taking close to
  SEND_TIMEOUT per message, or more e-mails queued than one worker sends —
  `/healthz` stays green and the counters keep rising, so an operator sees
  nothing until users report late e-mails. Record the send duration (a
  histogram) and expose the queue depth or the age of the oldest pending
  job. Confidence is low: such a gauge may already be exported outside the
  diff.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
