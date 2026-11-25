## Purpose

Moves card payments to Paylane.

## ADDED Requirements

### Requirement: Card payments go through Paylane

Every card payment SHALL be authorized and captured through Paylane once the change is deployed.

#### Scenario: A payment

- **WHEN** a customer pays by card
- **THEN** Paylane authorizes and captures it

#### Scenario: A declined card

- **WHEN** Paylane declines the card
- **THEN** the customer sees that the payment was declined

### Requirement: Payments are settled nightly

A worker SHALL reconcile the day's Paylane payments with the orders every night.

#### Scenario: Settlement

- **WHEN** the worker runs
- **THEN** each captured payment is matched to its order

#### Scenario: A payment without order

- **WHEN** a payment matches no order
- **THEN** it is written to the `unmatched_payments` table and the finance channel is alerted with its Paylane reference
