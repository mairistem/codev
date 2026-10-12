## ADDED Requirements

### Requirement: The `configure` skill proposes testing rules grounded in the project's tests

In addition to the one or two rules it proposes for `tasks`, the
`configure` skill SHALL propose testing rules for the project's language:
the test framework, where tests live, how they are named and the commands
that run and check them. These rules MUST describe the practice of the
project's existing tests when there are any; the existing practice wins
over any general convention. Only when the project has no tests yet SHALL
the skill fall back on the mainstream conventions of the language, from a
reference it carries for at least Rust, TypeScript/JavaScript, Python, Go
and Java.

#### Scenario: Testing rules follow the existing tests

- **GIVEN** a TypeScript project whose `package.json` runs Jest and whose
  tests sit in `__tests__/` directories
- **WHEN** the user runs `/codev-configure`
- **THEN** the proposed `tasks` rules name Jest and `__tests__/`
- **AND** they do not recommend another runner

#### Scenario: A project without tests gets the language's conventions

- **GIVEN** a Go project with no `_test.go` file
- **WHEN** the user runs `/codev-configure`
- **THEN** the proposed `tasks` rules describe table-driven `_test.go`
  tests in the same package, run with `go test ./...` and checked with
  `go vet`

#### Scenario: The skill carries the conventions reference

- **GIVEN** the codev workflow catalog
- **WHEN** the `configure` workflow is resolved
- **THEN** its `body` states that the project's existing practice wins
- **AND** it carries conventions for Rust, TypeScript/JavaScript, Python, Go
  and Java
