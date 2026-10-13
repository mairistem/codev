# Design: Birthday greetings

## Context

Profiles live in the `profiles` table (`user_id`, `display_name`). The
front-end is a React application; a daily job runner already exists.

## Decisions

### Decision: Birth dates live on the profile

Two columns, `birthday` (month and day, no year) and `phone`, are added to `profiles`, filled only for members who opt in. They are
kept as long as the profile exists, and copied with the rest of the
database into the nightly backups.

**Alternatives considered**: a separate table (no benefit).

### Decision: Greetings are sent by SMS through Textly

A daily job selects the profiles whose birthday is today and posts each
phone number and display name to Textly's API (hosted in the United
States), which sends the message.

**Alternatives considered**: e-mail (members asked for SMS).

## Risks / Trade-offs

- [Textly is down] → the job retries the next day.
