## Purpose

Merges customer records that belong to the same person.

## ADDED Requirements

### Requirement: Duplicate customers are merged

A nightly job SHALL merge customer records that share an e-mail address into the oldest one, moving their orders to it.

#### Scenario: Two records, one e-mail

- **WHEN** two customers have the e-mail `ada@example.org`
- **THEN** one record remains, holding the orders of both

#### Scenario: No duplicate

- **WHEN** every e-mail is distinct
- **THEN** nothing changes
