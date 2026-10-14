# Review record: add-ticket-attachments

<!-- codev-review: 1 -->

## code.review

Mode: inline.

### security

#### R-security-1 Path traversal in read_attachment bypasses the ticket authorization

```yaml
severity: high
confidence: high
location: "app/attachments.py:30-32"
evidence: |
  path = os.path.join(UPLOAD_DIR, str(ticket_id), name)
  with open(path, "rb") as f:
      return f.read()
recommendation: >-
  Validate `name` where it enters (CWE-22): reject any name that differs
  from `os.path.basename(name)` or is empty, `.` or `..`, and better, look
  it up in the `attachments` table for that `ticket_id` and only open a
  name recorded there; then check that the resolved real path stays under
  `UPLOAD_DIR/<ticket_id>`.
history:
  - status: open
    by: security
```

`name` comes from the client ("by the name the client asks for") and is joined to the
directory as is. An agent authorized on their own ticket 1 asks for
`name="../2/contract.pdf"` and reads an attachment of another team's ticket 2 — the
`may_read_ticket` check only covered ticket 1. `name="../../../../../etc/passwd"`, or an
absolute `name="/etc/passwd"` (which makes `os.path.join` discard the base), reads any
file the server process can read, such as `helpdesk.db` with every API key hash.
`save_attachment` applies `os.path.basename`; `read_attachment` does not.

#### R-security-2 save_attachment takes no agent identity and cannot enforce authorization

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11-17"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
recommendation: >-
  Take the `agent_id` as `read_attachment` does and check `may_read_ticket`
  (or a write permission) before writing; refuse or rename a name that
  already exists instead of overwriting it with `"wb"`.
history:
  - status: open
    by: security
```

Unlike `read_attachment`, the upload operation receives no agent and checks nothing, so
any agent able to reach it can write into any ticket's directory, and because the file is
opened with `"wb"` under the client's file name, overwrite an existing attachment of
another team's ticket (e.g. replace `invoice.pdf`). No caller is in the change, so whether
an upstream handler checks the ticket could not be verified — hence confidence `low`.

### newcomer

Disabled by configuration.
