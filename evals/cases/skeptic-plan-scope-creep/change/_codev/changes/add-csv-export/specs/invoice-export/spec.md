## Purpose

Lets accountants export a month's invoices.

## ADDED Requirements

### Requirement: Invoices are exported as CSV

`GET /invoices.csv?month=YYYY-MM` SHALL return the month's invoices, one per line, with number, date, customer and total.

#### Scenario: A month

- **WHEN** an accountant exports 2026-09
- **THEN** the CSV holds every invoice of September 2026

#### Scenario: An invalid month

- **WHEN** the month is `2026-13`
- **THEN** the service answers 400

### Requirement: Users choose their export format

The settings page SHALL let a user choose their preferred export format among the installed plugins.

#### Scenario: A preference

- **WHEN** a user picks CSV
- **THEN** exports use CSV

#### Scenario: No plugin

- **WHEN** no plugin is installed
- **THEN** the settings page says so
