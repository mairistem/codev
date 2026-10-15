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
recommendation: >
  The authorization check covers only ticket_id; name comes from the client
  and is joined as is (CWE-22). An agent allowed to read one ticket asks for
  name="../17/contract.pdf" and reads another team's ticket attachment, or
  name="../../../../etc/passwd" (or the absolute "/etc/passwd", which makes
  os.path.join discard the base) and reads any file the service can read,
  helpdesk.db included. Validate name at the boundary: reject anything that
  differs from os.path.basename(name), "." and "..", then look the name up in
  the attachments table for that ticket_id (only stored names are readable),
  and as defence in depth check that os.path.realpath(path) stays under the
  ticket's directory. Add a test reading "../<other ticket>/<file>" and an
  absolute path, both refused.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment writes to any ticket without an authorization check

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
  Unlike read_attachment, save_attachment takes no agent_id and checks no
  permission, so the server-side authorization for the upload operation
  depends entirely on a caller this change does not show (no handler calls
  it yet). If the upload endpoint passes the request's ticket_id straight
  through, any authenticated agent can attach files to, or overwrite an
  existing same-name attachment of, another team's ticket (CWE-862). Take
  the agent_id and check may_read_ticket (or a write permission) as
  read_attachment does, and do not silently overwrite an existing file.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
