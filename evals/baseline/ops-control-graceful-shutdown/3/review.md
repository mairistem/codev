# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is retried and may duplicate the e-mail

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
              Err(SendError::Unavailable) => {
                  let delay = RETRY_STEP * job.attempts;
                  queue.release(job.id, delay);
recommendation: Confirm how the mailer maps a timeout reached after the message data was handed to the relay; if it returns `Unavailable`, the job is released and sent again, so the recipient gets the e-mail twice. Either have the mailer distinguish "not delivered" from "outcome unknown", or send with a stable Message-ID the relay can deduplicate on.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
