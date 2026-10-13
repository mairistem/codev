# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket authorization check

```yaml
severity: high
confidence: high
location: "app/attachments.py:30-32"
evidence: |
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
      return f.read()
recommendation: >
  CWE-22. `name` is the name the client asks for and is joined to the ticket
  directory as is. An agent authorized on their own ticket 7 asks for
  `name="../42/contract.pdf"` and reads an attachment of ticket 42 owned by
  another team, defeating `may_read_ticket`; `name="../../../../../etc/passwd"`
  or an absolute `name="/etc/passwd"` (which makes `os.path.join` discard the
  base) reads any file the service can open, including `helpdesk.db` with the
  API key hashes. Validate `name` where it enters: look it up in the
  `attachments` table for that `ticket_id` (allow-list of stored names) and
  reject anything else; additionally resolve the path with `os.path.realpath`
  and refuse it unless it stays inside `UPLOAD_DIR/<ticket_id>`.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment enforces no authorization on the target ticket

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
      directory = os.path.join(UPLOAD_DIR, str(ticket_id))
recommendation: >
  CWE-862. Unlike `read_attachment`, the upload takes no `agent_id` and checks
  nothing: if the handler that calls it (not part of this change) does not
  check ownership, any agent can write files into another team's ticket, and
  because the file is opened with "wb" under the client-chosen name, can
  silently overwrite an existing attachment of that ticket (e.g. replace
  `invoice.pdf`). Take the `agent_id` and call `may_read_ticket` (or a write
  permission) before writing, and refuse or rename on a name collision.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
