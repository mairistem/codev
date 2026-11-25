## Purpose

Lets integrations call the API with a personal token instead of a password.

## ADDED Requirements

### Requirement: A user creates an API token

The service SHALL let a signed-in user create an API token, shown once, and SHALL accept it in the `Authorization: Bearer` header of later requests.

#### Scenario: A token is created

- **WHEN** a signed-in user asks for a new token
- **THEN** the token is shown once and can call the API

#### Scenario: An unknown token

- **WHEN** a request carries a token that was never issued
- **THEN** the service answers 401
