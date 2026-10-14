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
recommendation: Check whether the `Mailer` implementation maps a timeout reached after the message data was sent (no reply to the final `.`) to `Unavailable`. If it does, the retry sends the e-mail twice; give that case its own error variant and dead-letter it or retry it only with a Message-ID the relay deduplicates, and count it apart on `/metrics`. The `Mailer` implementation is not in the repository, hence the low confidence.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
