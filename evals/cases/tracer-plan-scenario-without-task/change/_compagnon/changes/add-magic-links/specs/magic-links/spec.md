## Purpose

Lets members sign in with a link sent by e-mail.

## ADDED Requirements

### Requirement: Members sign in with a magic link

The service SHALL e-mail a single-use sign-in link valid for 15 minutes, and SHALL sign the member in when they open it in time.

#### Scenario: A valid link

- **WHEN** a member opens their link within 15 minutes
- **THEN** they are signed in

#### Scenario: An expired link

- **WHEN** a member opens their link after 15 minutes
- **THEN** they are not signed in and are asked to request a new link
