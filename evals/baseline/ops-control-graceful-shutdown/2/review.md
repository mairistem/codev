# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that times out after the relay accepted it is retried, duplicating the e-mail

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
              Err(SendError::Unavailable) => {
                  let delay = RETRY_STEP * job.attempts;
                  queue.release(job.id, delay);
recommendation: >-
  Check how the `Mailer` implementation maps a timeout reached after DATA was sent. If it returns `Unavailable`, the relay may already have accepted the message and the release sends it again (up to 12 times during an outage with slow answers). Either distinguish an ambiguous timeout from a refused connection in the worker's handling, or make the send idempotent (a stable Message-ID per job that the relay deduplicates). Not verified: the mailer implementation is not in the change.
history:
  - status: open
    by: ops
```

#### R-ops-2 The health beat does not depend on the queue being reachable

```yaml
severity: medium
confidence: low
location: "src/worker.rs:26-29"
evidence: |
          health.beat();
          let Some(job) = queue.next(POLL) else {
              continue;
          };
recommendation: >-
  `Queue::next` returns `None` both for an empty queue and, depending on its implementation, for a database it cannot reach; the worker beats before polling, so `/healthz` stays green and no log is written while no job can be taken. If the implementation swallows database errors, beat only after a successful poll (or have `next` return a `Result` and log the error with fields). Not verified: the queue implementation is not in the change.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
