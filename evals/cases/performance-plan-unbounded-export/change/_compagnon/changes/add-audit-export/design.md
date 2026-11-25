# Design: Audit log export

## Context

Audit events are rows of `audit_events` (about 40 000 per organization and
per month, kept seven years). The API is a Node.js service with one event
loop per instance.

## Decisions

### Decision: The export loads every event, then writes the CSV

The handler runs `SELECT * FROM audit_events WHERE org_id = $1 ORDER BY
created_at`, loads the rows into an array, builds the whole CSV string with
`rows.map(toCsvLine).join("\n")`, and sends it.

**Alternatives considered**: none; the code is the simplest.

### Decision: Only administrators may export

The route checks the `admin` role on the server.

**Alternatives considered**: hiding the button for members (not enough).
