# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment through the client-supplied name

```yaml
severity: high
confidence: high
location: "app/attachments.py:30-32"
evidence: |
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
      return f.read()
recommendation: >-
  Validate `name` where it enters (CWE-22): reject it unless it equals
  `os.path.basename(name)` and is not empty, `.` or `..`, or better, look the
  name up in the `attachments` table for that `ticket_id` and only open a
  stored name; then resolve the final path with `os.path.realpath` and check it
  stays under `os.path.join(UPLOAD_DIR, str(ticket_id))`. Add a test that
  `../../../../etc/passwd`, `../<other ticket id>/<file>` and an absolute
  path are refused.
history:
  - status: open
    by: security
```

An agent who may read any one ticket calls `read_attachment` with that
ticket's id, which passes `may_read_ticket`, and `name` taken as the client
sends it. `name = "../../../../etc/passwd"` reads any file the service can
open; `name = "../42/contract.pdf"` reads an attachment of ticket 42, owned
by another team, bypassing the ownership check; an absolute `name` such as
`/etc/shadow` makes `os.path.join` discard the base entirely. Unlike
`save_attachment`, which applies `os.path.basename`, the read path does no
validation.

#### R-security-2 save_attachment enforces no authorization on the target ticket

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-15"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
      directory = os.path.join(UPLOAD_DIR, str(ticket_id))
recommendation: >-
  Take the calling `agent_id` and check `may_read_ticket` (or a write
  equivalent) before writing, as `read_attachment` does, so the upload
  operation is authorized on the server like the download. While there,
  refuse a `name` that is empty, `.` or `..` after `basename`, and decide
  whether an upload with an existing name may overwrite the previous file.
history:
  - status: open
    by: security
```

`read_attachment` checks that the agent's team owns the ticket, but
`save_attachment` takes no caller identity at all: whoever reaches it can
write files into, and overwrite existing attachments of, any ticket by
choosing `ticket_id`. No caller of `save_attachment` is in the change, so
whether an HTTP handler checks ownership before calling it could not be
verified — hence confidence `low`.

### newcomer

Disabled by configuration.
