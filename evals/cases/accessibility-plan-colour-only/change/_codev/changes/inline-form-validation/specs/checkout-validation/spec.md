## Purpose

Shows checkout errors next to the fields as the customer types.

## ADDED Requirements

### Requirement: Invalid fields are shown as the customer types

The checkout form SHALL validate each field when it loses focus and SHALL show an invalid field without submitting.

#### Scenario: An invalid postcode

- **WHEN** the customer leaves the postcode field with `ABC`
- **THEN** the field's border turns red

#### Scenario: A corrected field

- **WHEN** the customer fixes the postcode
- **THEN** the border returns to grey
