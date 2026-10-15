# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket check

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
  The `name` the client asks for is joined to the upload directory unchecked
  (CWE-22). An agent who may read ticket 1 asks for `name="../2/contract.pdf"`
  and gets ticket 2's attachment of another team, past `may_read_ticket`;
  `name="../../../../etc/passwd"` or an absolute `name="/etc/passwd"` (which
  makes `os.path.join` discard the base) returns any file the service can read.
  Validate `name` where it enters: reject it unless it equals
  `os.path.basename(name)` and is not `.`/`..`/empty, or better look it up in
  the `attachments` table for that `ticket_id` and open only the stored name;
  then check that `os.path.realpath(path)` stays under
  `os.path.join(UPLOAD_DIR, str(ticket_id))`. Add a test that a `../` name and
  an absolute name are refused.
history:
  - status: open
    by: security
```

#### R-security-2 save_attachment enforces no authorization on the target ticket

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
  The upload operation takes no `agent_id` and checks nothing, while
  `read_attachment` checks `may_read_ticket` in this same module: unless every
  caller checks first (no caller is in the change), any agent can write files
  into any team's ticket, including tickets that do not exist. Take the
  caller's `agent_id` and check it on the server as `read_attachment` does
  (CWE-862). Confidence is low because the entry point that calls this function
  is not part of the change.
history:
  - status: open
    by: security
```

#### R-security-3 An upload with an existing name silently overwrites the stored attachment

```yaml
severity: medium
confidence: high
location: "app/attachments.py:13-20"
evidence: |
      name = os.path.basename(filename)
      directory = os.path.join(UPLOAD_DIR, str(ticket_id))
      os.makedirs(directory, exist_ok=True)
      with open(os.path.join(directory, name), "wb") as f:
          f.write(data)
recommendation: >-
  The stored file name is the client's file name and the file is opened with
  `"wb"`: a second upload of `screenshot.png` to the same ticket — or a
  customer's file deliberately named like an earlier one — replaces the first
  file's bytes, while the `INSERT` adds a second `attachments` row that now
  points to the same file, so the first attachment is lost and its row lies
  about its size. Store each upload under a server-generated name (for example
  the new row id or a random token) and keep the client's file name only as a
  display column, or open with `"xb"` and refuse or rename on collision.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
