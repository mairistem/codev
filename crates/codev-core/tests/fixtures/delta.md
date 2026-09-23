## Purpose

Ajoute la double authentification et retire l'ancien mécanisme « Remember Me ».

## ADDED Requirements

### Requirement: Two-Factor Authentication

The system MUST support TOTP-based two-factor authentication.

#### Scenario: Enrolment

- **GIVEN** a user without 2FA enabled
- **WHEN** the user enables 2FA in settings
- **THEN** a QR code is displayed

#### Scenario: 2FA login

- **GIVEN** a user with 2FA enabled
- **WHEN** the user submits valid credentials
- **THEN** an OTP challenge is presented

## MODIFIED Requirements

### Requirement: Session Expiration

The system MUST expire sessions after 15 minutes of inactivity.

#### Scenario: Idle timeout

- **GIVEN** an authenticated session
- **WHEN** 15 minutes pass without activity
- **THEN** the session is invalidated

## REMOVED Requirements

### Requirement: Remember Me

**Reason**: Replaced by 2FA
**Migration**: Users must re-authenticate each session

## RENAMED Requirements

- FROM: Session Expiration
- TO: Session Timeout
