# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket authorization check

```yaml
severity: high
confidence: high
location: "app/attachments.py:26-32"
evidence: |
  def read_attachment(agent_id: int, ticket_id: int, name: str) -> bytes:
      """An attachment of a ticket the agent may read, by the name the client asks for."""
      if not may_read_ticket(agent_id, ticket_id):
          raise PermissionError(f"agent {agent_id} may not read ticket {ticket_id}")
      path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
recommendation: >-
  CWE-22. The `name` the client asks for is joined to the ticket directory as is.
  An agent authorized on their own ticket 1 sends name `../7/contract.pdf` and
  reads an attachment of ticket 7, owned by another team, so the
  `may_read_ticket` check is bypassed; `../../../../../etc/passwd` or an
  absolute name such as `/etc/shadow` (which makes `os.path.join` discard the
  base) reads any file the service can read. Validate `name` where it enters:
  reject anything that is not equal to `os.path.basename(name)`, empty, `.` or
  `..`; better, look the name up in the `attachments` table for this
  `ticket_id` and only open names recorded there. As defence in depth, resolve
  the final path with `os.path.realpath` and check it stays under
  `os.path.join(UPLOAD_DIR, str(ticket_id))`. Add a test that requests
  `../<other ticket>/<file>` and an absolute path and expects a refusal.
history:
  - status: open
    by: security
```

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
  CWE-862. Unlike `read_attachment`, the upload takes no `agent_id` and never
  calls `may_read_ticket`, so any authenticated agent can attach files to (and,
  with the same name, overwrite the stored file of) a ticket owned by another
  team, unless a caller not part of this change checks it. Take the agent id and
  enforce the same ownership check on the server before writing.
  Confidence is low because the HTTP handler calling this function is not in the
  diff.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
