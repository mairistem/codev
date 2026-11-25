## Purpose

Greets members on their birthday.

## ADDED Requirements

### Requirement: Members opt in to birthday greetings

The profile form SHALL offer birthday greetings as an option, off by default; a member who opts in SHALL give their birthday (day and month) and a mobile phone number.

#### Scenario: A member opts in

- **WHEN** a member opts in with a birthday and a phone number
- **THEN** both are stored on their profile

#### Scenario: A birthday without phone

- **WHEN** a member opts in without a phone number
- **THEN** the form is refused

### Requirement: Members receive a greeting on their birthday

On a member's birthday, the service SHALL send them a text message.

#### Scenario: Birthday

- **WHEN** the daily job runs on a member's birthday
- **THEN** a greeting is sent to their phone

#### Scenario: No phone

- **WHEN** the phone number was removed
- **THEN** no message is sent
