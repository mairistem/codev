# Auth Specification

## Purpose

Authentication and session management.

## Requirements

### Requirement: Login

The system SHALL emit a token upon successful login.

#### Scenario: Valid credentials

- **WHEN** the user submits valid credentials
- **THEN** a token is returned

### Requirement: Session Expiration

The system MUST expire sessions after 30 minutes.

#### Scenario: Idle timeout

- **WHEN** 30 minutes pass without activity
- **THEN** the session is invalidated

## Notes

Free-form notes that must survive any sync.
