# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket authorization

```yaml
severity: high
confidence: high
location: "app/attachments.py:28-31"
evidence: |
  if not may_read_ticket(agent_id, ticket_id):
      raise PermissionError(f"agent {agent_id} may not read ticket {ticket_id}")
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
recommendation: >-
  CWE-22. `name` comes from the client and is joined as is: an agent allowed on
  ticket 1 asks for `../42/contract.pdf` and reads another team's attachment, or
  `/etc/passwd` (an absolute path makes os.path.join discard the base) and reads
  any file the service can read. Look the attachment up in the `attachments`
  table by (ticket_id, name) and serve only a recorded name, or reject any name
  that differs from os.path.basename(name) and check that the resolved path
  (os.path.realpath) stays under the ticket's directory.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment checks no authorization

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-12"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
recommendation: >-
  CWE-862. Unlike read_attachment, the upload takes no agent and calls no
  may_read_ticket: unless every caller checks it (no caller is in the change),
  any authenticated agent can attach files to any team's ticket. Take the
  agent_id and enforce the same ownership check on the server before writing.
history:
  - status: open
    by: security
```

#### R-security-3 Uploaded file name is not allow-listed and overwrites existing attachments

```yaml
severity: medium
confidence: high
location: "app/attachments.py:13-17"
evidence: |
  name = os.path.basename(filename)
  directory = os.path.join(UPLOAD_DIR, str(ticket_id))
  os.makedirs(directory, exist_ok=True)
  with open(os.path.join(directory, name), "wb") as f:
      f.write(data)
recommendation: >-
  basename strips directories but accepts `..`, `.`, an empty name, control
  characters and any extension, and an upload under an existing name silently
  replaces the stored file (with a second row in `attachments`), so one uploader
  can swap a file another agent will open. Validate the name against an
  allow-list (characters, length, extensions), reject collisions or store under
  a generated id, and bound the size of `data`.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
