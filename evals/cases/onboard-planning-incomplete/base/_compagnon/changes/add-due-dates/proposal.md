# Proposal: Tasks carry a due date

## Why

The team plans its week in the to-do API, but a task has no date: what is
due on Friday is lost among the rest. A due date, and a list of the tasks
due soon, would let each member see what comes first.

## What Changes

- A task MAY carry a `due` date, set at creation or later.
- `GET /tasks?due=week` lists the tasks not done whose due date falls in
  the next seven days, the soonest first.

## Capabilities

### New Capabilities

### Modified Capabilities

- tasks

### Removed Capabilities

## Impact

- `src/tasks.ts` and `src/server.ts`; the tests in `src/__tests__/`.

## Domains

None

## Release note

- Added: Tasks can carry a due date, and `GET /tasks?due=week` lists those due in the next seven days.
