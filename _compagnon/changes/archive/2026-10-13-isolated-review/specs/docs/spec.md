## ADDED Requirements

### Requirement: Isolated review is documented in both languages

The Roles chapter of each language SHALL explain the two review modes,
what a reviewer receives in isolated mode, the fallback to inline, and
the meeting — how findings are merged, credited and ranked. The
configuration reference of each language SHALL document `review.mode`,
its values, its default and the per-change override, how to switch
modes, and that a project using the key raises `codev_version` to the
release that reads it. The skills chapter of each language SHALL say which skills
request the sub-agent tool and when. No error case applies: the
requirement is on the repository's documentation, checked by its tests.

#### Scenario: Both languages document the review modes

- **GIVEN** the repository's `docs/en` and `docs/fr`
- **WHEN** the test suite of `codev-cli` runs
- **THEN** it fails if the Roles chapter or the configuration reference
  of either language does not name `review.mode`, `inline` and
  `isolated`, if the configuration reference's `review` section does not
  name `codev_version`, or if the skills chapter of either language does
  not name the `Agent` tool
