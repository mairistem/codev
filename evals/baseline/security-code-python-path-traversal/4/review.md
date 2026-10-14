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
  `os.path.basename(name)` and is not empty, `.` or `..`, and better, look it
  up in the `attachments` table for that `ticket_id` (the allow-list the
  upload already writes) before opening it. Additionally resolve the final
  path with `os.path.realpath` and check it stays under
  `os.path.join(UPLOAD_DIR, str(ticket_id))`. Add a test with `../1/x`,
  `../../../../etc/passwd` and `/etc/passwd`.
history:
  - status: open
    by: security
```

`name` is "the name the client asks for" and is joined unvalidated. An agent
allowed to read any one ticket passes `may_read_ticket` for that ticket, then
asks for `name="../<other_ticket_id>/invoice.pdf"` to read another team's
attachment (bypassing the very ownership check just made), or
`name="../../../../etc/passwd"` / `name="/etc/passwd"` — an absolute name makes
`os.path.join` discard the base — to read any file the server process can
read, including `helpdesk.db` and its API key hashes. `save_attachment` applies
`os.path.basename`; `read_attachment` does not.

#### R-security-2 save_attachment performs no authorization check

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-13"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
      """Stores an uploaded file under the ticket's directory; returns its name."""
      name = os.path.basename(filename)
recommendation: >-
  Take the `agent_id` and check `may_read_ticket` (or a write-specific check)
  on the server before writing, as `read_attachment` does. Also refuse to
  overwrite an existing attachment of the same name, which currently replaces
  another agent's file silently.
history:
  - status: open
    by: security
```

Unlike `read_attachment`, the upload takes no agent and checks nothing: any
caller that reaches it can write files into, and add rows to, any ticket by
id, including overwriting an existing attachment with the same name. The
handler calling it is not in the diff, so whether it checks ownership itself
could not be verified — hence confidence `low`.

### newcomer

Disabled by configuration.
