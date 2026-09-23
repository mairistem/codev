# Auth Specification

## Purpose

Authentification et gestion de session pour l'application.

## Requirements

### Requirement: User Authentication

The system SHALL issue a JWT token upon successful login.

#### Scenario: Valid credentials

- **GIVEN** a user with valid credentials
- **WHEN** the user submits the login form
- **THEN** a JWT token is returned

#### Scenario: Invalid credentials

- **GIVEN** invalid credentials
- **WHEN** the user submits the login form
- **THEN** an error message is displayed

### Requirement: Session Expiration

The system MUST expire sessions after 30 minutes of inactivity.

#### Scenario: Idle timeout

- **GIVEN** an authenticated session
- **WHEN** 30 minutes pass without activity
- **THEN** the session is invalidated
