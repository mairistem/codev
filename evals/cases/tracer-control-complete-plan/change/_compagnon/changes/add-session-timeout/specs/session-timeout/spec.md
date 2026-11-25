## Purpose

Signs idle members out.

## ADDED Requirements

### Requirement: Idle sessions expire

A session SHALL expire after 30 minutes without a request; a request with an expired session SHALL be answered 401.

#### Scenario: An active session

- **WHEN** a member sends a request 10 minutes after the previous one
- **THEN** the request is served

#### Scenario: An idle session

- **WHEN** a member sends a request 31 minutes after the previous one
- **THEN** the service answers 401
