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
  `os.path.basename(name)` and is not empty, `.` or `..` — or better, look it up
  in the `attachments` table for that `ticket_id` and open only the stored name.
  Then resolve the path and check it stays under
  `os.path.join(UPLOAD_DIR, str(ticket_id))`. The authorization check on line 28
  only covers the ticket, not the file the name reaches; `save_attachment`
  already applies `basename`, so the read side should be at least as strict.
history:
  - status: open
    by: security
```

An agent who may read any one ticket asks for that ticket with
`name="../<other ticket id>/invoice.pdf"` and reads another team's attachment,
bypassing `may_read_ticket`; with `name="../../../../etc/passwd"` or an absolute
name such as `"/etc/passwd"` (which makes `os.path.join` discard the base), they
read any file the server process can read, `helpdesk.db` with its API key hashes
included.

#### R-security-2 save_attachment writes to any ticket without an authorization check

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
recommendation: >-
  Take the agent's id like `read_attachment` does and check it may write to the
  ticket before touching the disk (CWE-862), so the check cannot be forgotten by
  a caller. The upload also silently overwrites an existing attachment of the
  same name on that ticket.
history:
  - status: open
    by: security
```

Unlike `read_attachment`, the upload path takes no agent and checks nothing: an
agent who uploads to a `ticket_id` of another team adds or replaces that
ticket's files. Whether a caller checks first could not be verified — the
change adds no caller — hence confidence `low`.

### newcomer

Disabled by configuration.
