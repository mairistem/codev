## Purpose

Lets accountants export a month's invoices.

## ADDED Requirements

### Requirement: Invoices are exported as CSV

`GET /invoices.csv?month=YYYY-MM` SHALL return the month's invoices, one per line after a header line, with number, date, customer and total; a field holding a comma, a quote or a line break SHALL be quoted as RFC 4180 says.

#### Scenario: A month

- **WHEN** an accountant exports 2026-09
- **THEN** the CSV holds every invoice of September 2026

#### Scenario: An invalid month

- **WHEN** the month is `2026-13`, or missing
- **THEN** the service answers 400

#### Scenario: A month without invoice

- **WHEN** an accountant exports a month without invoice
- **THEN** the CSV holds the header line only

#### Scenario: A customer name with a comma

- **WHEN** an invoice's customer is `Smith, Jones & Co`
- **THEN** the field is quoted in the CSV

### Requirement: Users choose their export format

The settings page SHALL let a user choose their preferred export format among the installed plugins.

#### Scenario: A preference

- **WHEN** a user picks CSV
- **THEN** exports use CSV

#### Scenario: No plugin

- **WHEN** no plugin is installed
- **THEN** the settings page says so
