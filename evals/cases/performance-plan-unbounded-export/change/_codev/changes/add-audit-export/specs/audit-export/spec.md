## Purpose

Lets administrators export the audit log as CSV.

## ADDED Requirements

### Requirement: Administrators export the audit log

`GET /admin/audit-events.csv` SHALL return every audit event of the organization as CSV, oldest first.

#### Scenario: An export

- **WHEN** an administrator requests the export
- **THEN** the response holds every event, one per line

#### Scenario: Not an administrator

- **WHEN** a member requests the export
- **THEN** the service answers 403
