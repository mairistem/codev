# Review record: add-mail-worker

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### ops

#### R-ops-1 A send that timed out is retried and may deliver the e-mail twice

```yaml
severity: medium
confidence: low
location: "src/worker.rs:46-48"
evidence: |
  Err(SendError::Unavailable) => {
      let delay = RETRY_STEP * job.attempts;
      queue.release(job.id, delay);
recommendation: >-
  `SendError::Unavailable` covers both "the relay could not be reached" and a
  send that hit `SEND_TIMEOUT`; if the relay accepted the message before the
  timeout fired, releasing the job sends it again (up to 11 times while the
  relay is slow). Check that the mailer returns `Unavailable` only when the
  relay provably did not accept the message, or make the send idempotent
  (e.g. a stable Message-ID per job that the relay deduplicates).
  Confidence is low: the mailer implementation is not in the change.
history:
  - status: open
    by: ops
```

#### R-ops-2 Registering the stop flag disables the default kill on a second signal

```yaml
severity: low
confidence: low
location: "src/worker.rs:60-61"
evidence: |
  for signal in [signal_hook::consts::SIGTERM, signal_hook::consts::SIGINT] {
      signal_hook::flag::register(signal, Arc::clone(&stop))?;
recommendation: >-
  `flag::register` replaces the default disposition, so a second SIGTERM or
  Ctrl-C no longer terminates the process; if a send ever outlives its
  timeout, the worker can only be stopped by SIGKILL. Also register
  `signal_hook::flag::register_conditional_shutdown` so a second signal exits
  immediately. Confidence is low: shutdown stays bounded (POLL + SEND_TIMEOUT)
  as long as the mailer honours its timeout, which is not visible here.
history:
  - status: open
    by: ops
```

### newcomer

Disabled by configuration.
