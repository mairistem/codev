# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is retried and may be delivered twice

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: >-
  Check how the mailer maps a timeout that happens after the relay received the
  message (after DATA): if it returns `SendError::Unavailable`, the job is handed
  out again and the recipient gets the e-mail twice, with nothing in the logs or
  `notifier_jobs_total` to tell the operator. Send a stable Message-ID derived from
  `job.id` so the relay or the recipient can deduplicate, or map a post-DATA
  timeout to its own outcome. Confidence is low: the mailer implementation is not
  part of this change and could not be checked.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
