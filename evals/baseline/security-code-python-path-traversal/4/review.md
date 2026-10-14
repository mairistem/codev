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
  `os.path.basename(name)` and is not empty, `.` or `..`, or better look it up
  in the `attachments` table for this `ticket_id` and open only a stored name;
  then check that the resolved path (`os.path.realpath`) stays under
  `UPLOAD_DIR/<ticket_id>`. The `may_read_ticket` check on line 28 only covers
  `ticket_id`, so without this fix it protects nothing. Add a test that
  `read_attachment` refuses `../2/x.png`, `../../../../etc/passwd` and `/etc/passwd`.
history:
  - status: open
    by: security
```

An agent allowed to read ticket 1 asks for `name="../2/contract.pdf"` and gets
an attachment of ticket 2 they may not read; with `name="../../../../etc/passwd"`,
or an absolute `name="/etc/passwd"` (which makes `os.path.join` discard the base),
they read any file the server process can read, configuration and secrets included.

#### R-security-2 save_attachment takes no caller and checks no authorization

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
recommendation: >-
  Give `save_attachment` the caller's identity and check on the server that it
  may write to `ticket_id` (CWE-862), as `read_attachment` does with
  `may_read_ticket`; otherwise any caller of the upload entry point attaches
  files to any ticket. Confidence low: the HTTP handler calling it is not in
  the change, so whether it checks ownership could not be verified.
history:
  - status: open
    by: security
```

#### R-security-3 An upload silently overwrites an earlier attachment of the same name

```yaml
severity: medium
confidence: low
location: "app/attachments.py:16-21"
evidence: |
  with open(os.path.join(directory, name), "wb") as f:
      f.write(data)
  _conn.execute(
      "INSERT INTO attachments (ticket_id, name, size) VALUES (?, ?, ?)",
recommendation: >-
  Store each upload under a server-generated name (a UUID, or the row id) and
  keep the client's filename only as display metadata, or refuse an existing
  name (open with mode `"xb"`). As written, whoever can upload to a ticket
  replaces a file agents already trust — e.g. a screenshot swapped for a
  malicious document under the same name — and the table keeps two rows
  pointing at one file with a stale `size`. Confidence low: who may upload
  to a ticket depends on the caller, not visible in the change.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
