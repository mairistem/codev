# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is retried and duplicates the e-mail

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-50"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: Make sure the mailer reports a timeout that happens after the message was handed to the relay differently from a refused connection, or send each job with a stable Message-ID so the relay drops duplicates. Otherwise a slow relay makes the worker resend the same e-mail up to twelve times. This depends on how `Mailer::send` maps a timeout, which this change does not show.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
