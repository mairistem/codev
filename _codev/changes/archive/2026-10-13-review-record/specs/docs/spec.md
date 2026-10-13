## ADDED Requirements

### Requirement: The review record is documented in both languages

The documentation of each language SHALL hold a "Review record" guide —
the file `guides/review-record.md`, listed in its table of contents — that
describes the record's layout, a finding's fields, the statuses and who
may set them, the answer loop, the checks, the archive gate and what to do
for a change started with an earlier codev. The Roles chapter of each
language SHALL give the severity and confidence scales; the English
chapter MUST give each level's definition exactly as codev's skills
receive it.

#### Scenario: The guide exists in both languages

- **GIVEN** the repository's `docs/en` and `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if either table of contents does not list
  `guides/review-record.md`

#### Scenario: The scale in the docs is the scale in the skills

- **GIVEN** the English `roles.md` and the severity and confidence scales
  the skills receive
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if a level's definition is missing from the chapter,
  or if a level is missing from the French chapter
