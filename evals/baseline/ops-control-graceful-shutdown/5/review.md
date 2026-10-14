# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is retried and may be delivered twice

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-50"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: Confirm in the `Mailer` implementation that a timeout after the message data was handed to the relay is not reported as `Unavailable`, or give each message a stable Message-ID so a re-send can be recognised; otherwise a slow relay that accepts the message after `SEND_TIMEOUT` makes the worker release the job and send the same e-mail again, up to `MAX_ATTEMPTS` times. Low confidence because the `Mailer` implementation is not part of the change.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
