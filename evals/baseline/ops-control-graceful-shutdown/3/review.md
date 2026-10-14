# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay took the message is retried and may be delivered twice

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
            Err(SendError::Unavailable) => {
                let delay = RETRY_STEP * job.attempts;
                queue.release(job.id, delay);
recommendation: >-
  Sending an e-mail is not idempotent. `SendError::Unavailable` covers both "the relay could not be
  reached" and a temporary failure; if the mailer also returns it when `SEND_TIMEOUT` expires after
  the message data was transmitted, the relay may still deliver it while the job is released and sent
  again, up to MAX_ATTEMPTS times. Confirm how the mailer classifies a timeout after DATA; if it maps
  to `Unavailable`, give the message a stable Message-ID derived from the job id so duplicates can be
  recognised, or distinguish "outcome unknown" from "not sent" before retrying.
history:
  - status: open
    by: ops
```
