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
  Validate `name` where it enters (CWE-22): reject any name that differs from
  `os.path.basename(name)`, is empty, `.` or `..`, or contains a separator;
  better, look the name up in the `attachments` table for that `ticket_id` and
  refuse unknown names. Then resolve the path with `os.path.realpath` and check
  it stays under `os.path.join(UPLOAD_DIR, str(ticket_id))` before opening.
  Add a test that `read_attachment(agent, own_ticket, "../<other_ticket>/x")`
  and `read_attachment(agent, own_ticket, "/etc/passwd")` both raise.
history:
  - status: open
    by: security
```

`name` is "the name the client asks for" (line 27) and is joined to the
directory unchecked. An agent authorized for their own ticket 1 sends
`name="../2/contract.pdf"` and reads ticket 2's attachment, although
`may_read_ticket` only checked ticket 1; with `name="../../../../etc/passwd"`,
or an absolute `name="/etc/passwd"` (which makes `os.path.join` discard the
base), they read any file the server process can read, including
`helpdesk.db` and its API key hashes.

#### R-security-2 An upload silently overwrites an existing attachment of the same name

```yaml
severity: medium
confidence: high
location: "app/attachments.py:13-17"
evidence: |
    name = os.path.basename(filename)
    directory = os.path.join(UPLOAD_DIR, str(ticket_id))
    os.makedirs(directory, exist_ok=True)
    with open(os.path.join(directory, name), "wb") as f:
recommendation: >-
  Store each upload under a server-generated name (for example a UUID, kept in
  the `attachments` row next to the display name), or open with mode `"xb"`
  and refuse or rename on collision. Keep the client's file name only as
  display metadata.
history:
  - status: open
    by: security
```

The stored file name is the client's own `filename`, opened in mode `"wb"`.
Anyone who can upload to a ticket and sends `filename="invoice.pdf"` replaces
the `invoice.pdf` already attached to it; an agent who later downloads it gets
the attacker's content, while the `attachments` table holds two rows pointing
at the same file.

#### R-security-3 save_attachment takes no caller identity, so it enforces no authorization

```yaml
severity: medium
confidence: low
location: "app/attachments.py:11"
evidence: |
  def save_attachment(ticket_id: int, filename: str, data: bytes) -> str:
recommendation: >-
  Take the caller's `agent_id` and check `may_read_ticket` (or a dedicated
  write permission) before writing, as `read_attachment` does, so the
  operation is authorized server-side wherever it is called from.
history:
  - status: open
    by: security
```

Unlike `read_attachment`, the upload operation checks nothing: any caller that
reaches it can attach files to any `ticket_id`. The HTTP or client entry point
that calls it is not in the change, so whether a check happens upstream could
not be verified.

#### R-security-4 Uploaded data has no size or type limit

```yaml
severity: medium
confidence: low
location: "app/attachments.py:16-17"
evidence: |
    with open(os.path.join(directory, name), "wb") as f:
        f.write(data)
recommendation: >-
  Enforce a maximum size and an allow-list of accepted types (by extension and
  sniffed content) where the upload enters, and refuse the rest before writing.
history:
  - status: open
    by: security
```

`data` is written whatever its length or kind: repeated large uploads can fill
the disk holding `/var/lib/helpdesk/attachments`, and executable or HTML
content is stored and served back to agents as is. Whether the entry point
limits request size upstream could not be verified.

### newcomer

Disabled by configuration.
