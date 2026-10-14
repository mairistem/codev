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
  CWE-22. An agent allowed to read ticket 1 calls read_attachment with
  name "../2/invoice.pdf" and gets another ticket's attachment, bypassing
  may_read_ticket; with "../../../../etc/passwd", or an absolute name such as
  "/etc/passwd" (os.path.join discards the base), it reads any file the
  service can read. Validate the name where it enters: reject anything that
  is not equal to os.path.basename(name), or empty, "." or "..", or better
  look it up in the attachments table for that ticket_id; then resolve the
  path with os.path.realpath and check it stays under
  UPLOAD_DIR/<ticket_id> before opening it.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment enforces no authorization on the ticket

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
recommendation: >-
  CWE-862. Unlike read_attachment, the upload takes no agent id and checks
  nothing, so any caller can attach files to (or overwrite same-named files
  of) any ticket, unless the unseen caller checks it. Take the agent id and
  enforce the write permission on the server here, as read_attachment does
  with may_read_ticket.
history:
  - status: open
    by: security
```
