## ADDED Requirements

### Requirement: ADR files with Windows line endings

codev SHALL parse an ADR whose lines end with CRLF exactly like the same
ADR with LF line endings: same frontmatter fields, same body. When a command
rewrites a decision's status (`decision accept`, or a predecessor marked
`superseded`), the body MUST stay byte for byte identical — line endings
included — so that its seal still matches, and the rewritten frontmatter
MUST use the same line-ending style as the file.

#### Scenario: A CRLF ADR is parsed like an LF one

- **GIVEN** an ADR saved with CRLF line endings by an editor on Windows
- **WHEN** codev parses it
- **THEN** its `id`, `title` and `status` are the same as for the LF version
- **AND** no `decision_missing_frontmatter` finding is reported

#### Scenario: Accepting a CRLF decision keeps its sealed body

- **GIVEN** a `proposed` ADR saved with CRLF line endings
- **WHEN** the user runs `codev decision accept <id>`
- **THEN** the frontmatter reads `status: accepted` with CRLF line endings
- **AND** the body is byte for byte identical to the one before
- **AND** `codev validate` reports no `decision_seal_mismatch`
