# Payment Gateway Specification

## Purpose

How the payments service calls the acquiring bank's gateway, and what it
does when the gateway is slow or down.

## Requirements

### Requirement: A timed-out call is retried with exponential backoff

A call to the gateway that times out SHALL be retried up to four times,
waiting 200 ms, then 400 ms, 800 ms and 1.6 s, each wait with up to 20 %
of random jitter; an answer other than a timeout SHALL NOT be retried.

#### Scenario: Two timeouts then an answer

- **GIVEN** a gateway that times out twice, then authorizes the payment
- **WHEN** the service charges a card
- **THEN** the charge succeeds after two retries, having waited about 200 ms then 400 ms

#### Scenario: A declined card

- **GIVEN** a gateway that declines the card
- **WHEN** the service charges it
- **THEN** the call is not retried
