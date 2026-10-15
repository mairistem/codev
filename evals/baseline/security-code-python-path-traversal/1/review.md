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
  Validate `name` where it enters (CWE-22): reject any name that differs from
  `os.path.basename(name)`, is empty, `.` or `..`, or contains a separator;
  better, look the name up in the `attachments` table for this `ticket_id`
  (the rows `save_attachment` writes) and refuse anything not listed. As a
  second barrier, resolve the path with `os.path.realpath` and check it stays
  under `os.path.join(UPLOAD_DIR, str(ticket_id))`. Add a test that
  `read_attachment(agent, ticket, "../<other>/x")` and `"/etc/passwd"` are
  refused.
history:
  - status: open
    by: security
```

An agent of any team, authorized on one of its own tickets, sends
`name="../42/invoice.pdf"` and reads an attachment of ticket 42 owned by
another team, bypassing `may_read_ticket`; `name="../../../../etc/passwd"`
or an absolute `name="/etc/passwd"` (which `os.path.join` lets discard the
base) reads any file the service can open. The upload side sanitizes with
`os.path.basename`, the read side does not.

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
recommendation: >-
  Take the `agent_id` as `read_attachment` does and check
  `may_read_ticket(agent_id, ticket_id)` (or a write-specific check) before
  writing, so an agent cannot add files to, or overwrite same-named files of,
  another team's ticket (CWE-862). Confidence is low because the caller of
  `save_attachment` is not in the repository: it may enforce this itself.
history:
  - status: open
    by: security
```

### newcomer

Disabled by configuration.
