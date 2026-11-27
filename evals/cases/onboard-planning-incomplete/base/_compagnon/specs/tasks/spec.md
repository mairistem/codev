# Tasks Specification

## Purpose

How the team's to-do API stores tasks and lists them.

## Requirements

### Requirement: A task is created with a title

`POST /tasks` SHALL create a task from a non-empty `title` and answer 201
with the task, its `id` and `done: false`; an empty title SHALL be
answered with 400.

#### Scenario: A task without a title

- **GIVEN** a request whose `title` is empty
- **WHEN** it is sent to `POST /tasks`
- **THEN** the API answers 400 and creates nothing
