# Appointments Specification

## Purpose

How patients book appointments with the clinic, and how they are told
about them.

## Requirements

### Requirement: A booking is confirmed by mail

When a patient books an appointment, the system SHALL send them a
confirmation through the clinic's mailer, naming the practitioner, the
day and the time.

#### Scenario: A booking for Monday

- **GIVEN** a patient booking Dr. Martin on Monday at 9:30
- **WHEN** the booking is saved
- **THEN** the patient receives a confirmation naming Dr. Martin, Monday and 9:30
