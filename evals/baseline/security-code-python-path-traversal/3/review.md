# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket check

```yaml
severity: high
confidence: high
location: "app/attachments.py:30-32"
evidence: |
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
recommendation: >-
  The client-supplied `name` is joined to the ticket directory unvalidated
  (CWE-22). An agent allowed to read ticket 1 asks for
  `name="../2/invoice.pdf"` and gets ticket 2's attachment, defeating
  `may_read_ticket`; `name="../../../../etc/passwd"` or an absolute
  `name="/etc/passwd"` (which makes `os.path.join` discard the base) reads any
  file the service can read. Reduce `name` with `os.path.basename` as
  `save_attachment` does, or better look it up in the `attachments` table for
  that `ticket_id`, then resolve the path with `os.path.realpath` and refuse it
  unless it lies under `UPLOAD_DIR/<ticket_id>/`. Add a test with `../` and
  absolute names.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment checks no authorization

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
recommendation: >-
  Unlike `read_attachment`, the upload takes no `agent_id` and calls no
  authorization check, so whatever entry point exposes it lets any caller write
  files into any ticket's directory (and silently overwrite an existing
  attachment of the same name). The caller was not visible in the change, so
  this depends on it enforcing access; take `agent_id` and check
  `may_read_ticket` (or a write permission) here, on the server.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
