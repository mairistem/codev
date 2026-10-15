# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay took the message is sent again

```yaml
severity: low
confidence: low
location: "src/worker.rs:46-50"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: Check how the mailer maps a timeout reached after the message data was sent; if it returns `Unavailable`, the job is handed out again and the recipient gets the e-mail twice. Map that case to a distinct error and do not retry it, or accept the duplicate as SMTP's usual at-least-once delivery.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
