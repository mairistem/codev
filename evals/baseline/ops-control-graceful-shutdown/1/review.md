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
recommendation: Check how the mailer maps a timeout once the message data has been handed to the relay (`src/mailer.rs` only says it gives up after `timeout`). If that case returns `Unavailable`, the job is released and sent again, up to 12 times, so a slow relay that does deliver produces duplicate e-mails. Either have the mailer report an ambiguous outcome separately (e.g. dead-letter it or retry with a stable Message-ID the relay deduplicates), or document that duplicates are accepted.
history:
  - status: open
    by: ops
```
