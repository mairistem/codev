---
id: ops
name: { en: Operations, fr: Exploitation }
at: [plan.challenge, code.review]
domains: [operations]
question: Can the team run, observe and roll back what this change adds?
done_when:
  - Logs are structured, useful to diagnose a failure, and carry no
    secret
  - A new component exposes health and the metrics an operator needs
  - Every outgoing call has a timeout; retries are bounded, with backoff,
    and only on idempotent operations
  - The change can be turned off or rolled back — a feature flag, a
    configuration switch, or a deploy of the previous version
  - Failures produce error messages an operator can act on
---

## Scope

Look at what the change adds to running systems: new services, jobs,
queues, outgoing calls, configuration, logs and error paths. Leave aside
local developer tooling, and dashboards or alerts that live outside the
repository unless the change must create them.

## Signals

- **Silent failure**: an error caught and dropped. Example:
  `catch (e) {}` around a payment call.
- **Unactionable message**: Example: `error: something went wrong`, with no
  identifier, input or next step.
- **Free-text logs**: values interpolated into prose that cannot be
  queried. Example: `"user 42 failed 3 times"` instead of fields.
- **Secret or token in a log or an error**: Example: a full connection
  string in a startup log.
- **No timeout**: an HTTP or database call that can hang forever.
- **Retry storm**: retries without limit or backoff, or retries of a
  non-idempotent operation that duplicate a charge or a message.
- **No way back**: a change that can only be undone by a new release, with
  no flag or configuration switch.
- **Invisible new component**: a worker or service with no health check,
  no metric, no log at start.
- **Configuration baked in**: a URL, a limit or a credential hard-coded
  instead of read from the environment or configuration.

## By stack

- **Rust**: `tracing` with fields (`info!(user_id, "login")`) rather than
  formatted strings; `reqwest` clients without `.timeout()`; errors
  converted with `.to_string()` losing their source chain.
- **TypeScript/JavaScript**: `fetch` has no timeout by default — use an
  `AbortSignal`; unhandled promise rejections; `console.log` in services
  instead of a structured logger.
- **Python**: `requests` calls without `timeout=` wait indefinitely;
  `logging` with f-strings rather than structured fields; broad
  `except Exception: pass`.
- **Go**: `http.Client{}` without `Timeout` and `http.Get` (the default
  client) have no timeout; calls without a `context` deadline; errors
  returned without `%w` wrapping.
- **Java**: HTTP clients without connect and read timeouts; exceptions
  logged without the stack trace or swallowed; thread pools without
  bounds.

## References

- Google, *Site Reliability Engineering* (O'Reilly, 2016) — the four
  golden signals: latency, traffic, errors, saturation.
- The Twelve-Factor App — configuration in the environment, logs as
  event streams, disposability.
- OpenTelemetry — vendor-neutral traces, metrics and logs.

## Reporting

One line per finding: the location, what an operator would miss or suffer
at 3 a.m. (a hang, a blind spot, an outage with no rollback), the severity
(high when a failure can go unnoticed or cannot be rolled back; medium when
it is hard to diagnose; low otherwise), and the fix.
