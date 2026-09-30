## Purpose

Let users choose the color theme of the interface.

## ADDED Requirements

### Requirement: Theme follows the system preference by default

The application SHALL render with the operating system's color scheme until
the user picks a theme.

#### Scenario: No theme chosen yet

- **GIVEN** a user who never changed the theme
- **WHEN** their operating system uses a dark color scheme
- **THEN** the application renders with the dark theme

### Requirement: The chosen theme is remembered

The application SHALL restore the theme the user last picked, on every device
they sign in from.

#### Scenario: Returning user

- **GIVEN** a user who picked the dark theme on their laptop
- **WHEN** they sign in on their phone
- **THEN** the application renders with the dark theme
