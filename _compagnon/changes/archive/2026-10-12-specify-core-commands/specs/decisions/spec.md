## MODIFIED Requirements

### Requirement: Showing a decision through the CLI

`codev decision show <id>` MUST display the full content of the requested
decision. Resolution accepts a short `id` when it is unambiguous, or a
qualified identifier `<origin>/<id>` in case of collision.

The human output SHALL start with a header — `ID:` with the short and the
qualified identifier, `Title:`, `Status:`, `Date:`, then `Tags:`,
`Supersedes:` and `Superseded by:` only when they have a value, and
`File:` with the decision's path, relative to the project for a file inside
it and absolute otherwise — followed by a blank line and the file's content,
verbatim. With `--json`, the document SHALL carry `root`, `decision` — the
same object as an entry of `codev decision list --json` — and `content`,
the file's full content. An identifier that matches no decision fails with
`unknown_decision_id`, whose message points to `codev decision list`, and
exits with code 1.

#### Scenario: Show of an id without collision

- **GIVEN** a project containing a unique ADR `0001`
- **WHEN** the user runs `codev decision show 0001`
- **THEN** the output contains the frontmatter header (`id`, `title`,
  `status`, `date`) followed by the markdown body of the decision

#### Scenario: Show of an ambiguous id

- **GIVEN** a project containing a local ADR `0007` **and** an ADR `0007`
  in an inherited source `path:~/shared`
- **WHEN** the user runs `codev decision show 0007`
- **THEN** no content is displayed
- **AND** the `ambiguous_decision_id` error message lists both
  qualified identifiers (`project/0007`, `path:~/shared/0007`) and asks
  the user to be more specific

#### Scenario: Show with a qualified identifier

- **GIVEN** the same context
- **WHEN** the user runs `codev decision show path:~/shared/0007`
- **THEN** the inherited decision is displayed

#### Scenario: Header of a local decision

- **GIVEN** a local ADR `0001` titled `Foundation`, `accepted`, tagged
  `arch`, at `_codev/decisions/0001-foundation.md`
- **WHEN** the user runs `codev decision show 0001`
- **THEN** the output starts with `ID:     0001 (project/0001)`, then the
  title, status, date and `Tags:   arch` lines
- **AND** `File:` gives `_codev/decisions/0001-foundation.md`
- **AND** the file's content follows, unchanged

#### Scenario: An inherited decision

- **GIVEN** a project inheriting from a source `path:/shared` outside the
  project, holding an ADR `0100`
- **WHEN** the user runs `codev decision show 0100 --json`
- **THEN** `decision.origin` is `path:/shared`, `decision.qualifiedId` is
  `path:/shared/0100`, and `decision.path` is the absolute path of the file
- **AND** `content` is the file's full content

#### Scenario: Unknown identifier

- **WHEN** the user runs `codev decision show 9999`
- **THEN** the command fails with `unknown_decision_id` and a message
  naming `codev decision list`
