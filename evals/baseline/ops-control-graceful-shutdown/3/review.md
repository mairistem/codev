# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it may be retried and duplicated

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-50"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: Confirm how the mailer maps a send that exceeds `SEND_TIMEOUT` after the message data was transmitted. If it returns `SendError::Unavailable`, the job is released and sent again, so the recipient gets a duplicate e-mail. Either map an ambiguous timeout to a distinct outcome that is not retried blindly, or document that delivery is at-least-once. This is low confidence because the mailer implementation was not visible.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
