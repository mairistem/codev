## Purpose

Greets members on their birthday.

## ADDED Requirements

### Requirement: Members give their birth date and phone number

The profile form SHALL ask for the member's birth date and mobile phone number, both required.

#### Scenario: A complete profile

- **WHEN** a member saves a birth date and a phone number
- **THEN** both are stored on their profile

#### Scenario: A missing birth date

- **WHEN** a member saves the form without a birth date
- **THEN** the form is refused

### Requirement: Members receive a greeting on their birthday

On a member's birthday, the service SHALL send them a text message.

#### Scenario: Birthday

- **WHEN** the daily job runs on a member's birthday
- **THEN** a greeting is sent to their phone

#### Scenario: No phone

- **WHEN** the phone number was removed
- **THEN** no message is sent
