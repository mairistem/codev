# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket authorization

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
  CWE-22. `name` comes from the client and is joined to the ticket directory
  as is. An agent authorized on their own ticket 7 asks for
  `name="../42/invoice.pdf"` and reads an attachment of ticket 42, owned by
  another team, past the `may_read_ticket` check; `name="../../../../etc/passwd"`
  or an absolute `name="/etc/passwd"` (which makes `os.path.join` discard the
  base) reads any file the server process can read, including `helpdesk.db`
  with the API key digests. Validate the name where it enters: reject any
  value where `os.path.basename(name) != name` or that is `.`/`..`, or better,
  look the name up in the `attachments` table for that `ticket_id` and only
  then open it; additionally resolve the final path with `os.path.realpath`
  and check it stays under `os.path.join(UPLOAD_DIR, str(ticket_id))`.
  Add a test that a `../` name and an absolute name are refused.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment performs no authorization check

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
      directory = os.path.join(UPLOAD_DIR, str(ticket_id))
recommendation: >-
  CWE-862. Unlike `read_attachment`, the upload operation takes no agent and
  checks nothing: if the handler that calls it (not in this change) does not
  check ownership itself, any authenticated agent can attach files to, or
  overwrite same-named attachments of, any team's ticket. Take `agent_id` and
  enforce `may_read_ticket` (or a write-specific check) inside
  `save_attachment`, as `read_attachment` does. Confidence low: the caller
  could not be seen.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
