# Bookmarks Specification

## Purpose

How the reading-list API saves links and lists them.

## Requirements

### Requirement: A bookmark is saved once

`POST /bookmarks` SHALL save a link with its title and answer 201; a link
already saved SHALL be answered with 409.

#### Scenario: The same link twice

- **GIVEN** a saved bookmark of `https://example.org/a`
- **WHEN** the same link is posted again
- **THEN** the API answers 409
