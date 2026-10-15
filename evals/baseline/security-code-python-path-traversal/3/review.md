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
  An agent allowed to read one ticket of their team passes the authorization
  check at line 28, then sends name="../43/contract.pdf" to read another
  team's attachment, name="../../../../etc/passwd" or an absolute path such as
  "/etc/passwd" (which makes os.path.join discard the base) to read any file
  the service can open (CWE-22). Do not open a path built from the request:
  look the name up in the attachments table for that ticket_id (the row
  save_attachment inserts) and refuse it when absent, and additionally reject
  any name that differs from os.path.basename(name) or whose resolved real
  path is not under os.path.join(UPLOAD_DIR, str(ticket_id)).
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment stores into any ticket without an authorization check

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-16"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
      directory = os.path.join(UPLOAD_DIR, str(ticket_id))
recommendation: >-
  The proposal says agents upload files to a ticket, yet the upload takes no
  agent_id and never calls may_read_ticket, unlike read_attachment; if the
  agent's ticket_id reaches it unchecked, an agent writes (and, with the same
  filename, overwrites) attachments of another team's ticket. The caller is
  not in the change, so this could not be confirmed. Take the agent_id and
  enforce the same ownership check as read_attachment before writing.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
