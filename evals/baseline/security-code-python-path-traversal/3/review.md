# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment through the client-supplied name

```yaml
severity: high
confidence: high
location: "app/attachments.py:30-31"
evidence: |
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
recommendation: >-
  Validate `name` where it enters (CWE-22): reject it unless it equals
  `os.path.basename(name)` and is not empty, `.` or `..`, or better, look it up
  in the `attachments` table for this `ticket_id` and open only the stored
  name; then resolve the path with `os.path.realpath` and check it stays under
  the ticket's directory. Add a test that `../`, absolute and other-ticket
  names are refused.
history:
  - status: open
    by: security
```

An agent allowed to read any one ticket calls `read_attachment(agent, own_ticket, "../42/invoice.pdf")`
and gets another ticket's attachment, bypassing `may_read_ticket`; `"../../../../etc/passwd"` or an
absolute name such as `"/etc/passwd"` (which `os.path.join` keeps, discarding the base) reads any
file the server process can read, secrets included. Unlike `save_attachment`, the name is not passed
through `os.path.basename`.

#### R-security-2 save_attachment enforces no authorization on the ticket written to

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
recommendation: >-
  Take the caller's `agent_id` and check it may write to `ticket_id` on the
  server, as `read_attachment` does with `may_read_ticket`, unless the caller
  of this function is shown to enforce it.
history:
  - status: open
    by: security
```

Any caller reaching this function can add a file to any ticket, and a file with an existing name
silently overwrites that ticket's attachment (`open(..., "wb")`). Confidence is low: the handler that
calls it is not part of the change, so whether it checks authorization could not be verified.

### newcomer

Disabled by configuration.
