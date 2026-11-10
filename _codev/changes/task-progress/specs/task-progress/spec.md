## Purpose

Measure how far the implementation of a change has gone, from the
checkboxes of the file its schema tracks, so that a person, a skill or a
tool driving codev can read the progress without parsing Markdown itself.

## ADDED Requirements

### Requirement: The tracked file's checkboxes are counted from the Markdown structure alone

The file whose progress is measured SHALL be the one named by the schema's
`apply.tracks`, relative to the change folder; no other file of the change
is read for progress, whatever its name. A value that is an absolute path
or holds a `..` segment SHALL be treated as an absent file, so that nothing
outside the change folder is read. A tracked file that exists but cannot be
read as UTF-8 text (or names a folder) SHALL NOT make `codev status` fail:
`tasks` is `null`, the JSON `status` carries one entry of level `warning`
and code `tasks_unreadable` naming the file, the human output says so on the
`Tasks:` line, and the exit code stays 0. `codev archive` SHALL refuse such a
change with the code `tasks_unreadable`, naming the file, and write nothing,
with or without `--strict`: the boxes it must report cannot be counted.

A line SHALL be a box when, after any leading spaces or tabs, it starts with
`- [`, then a single space, `x` or `X`, then `]`, followed by the end of the
line or by whitespace. `- [ ]` is open; `- [x]` and `- [X]` are checked. The
indentation does not matter: a box nested under another one counts as a box
of its own. Any other line is not a box and is never counted, among them
`- [-]`, `-[ ]`, `* [ ]`, `1. [ ]` and `- [x]done`. A line ending in `\r\n`
SHALL be read as the same line ending in `\n`.

A line SHALL NOT be counted while it sits inside a fenced code block or an
HTML comment:

- a fence opens at a line whose first characters after leading whitespace
  are at least three backticks, or at least three tildes, and closes at the
  next line made only of whitespace and at least as many of the same
  character; a fence left open runs to the end of the file;
- a comment opens at a line whose first characters after leading whitespace
  are `<!--`, and closes at the first line, the same one included, that
  contains `-->` after the opening.

Boxes SHALL be grouped under the last level-2 heading (`## ` at the start
of the line, outside a fence or a comment) that precedes them; a `#` or
`###` heading neither opens nor closes a group. A group's `title` is the
heading's text, trimmed; when that text starts with digits followed by a
`.` and then whitespace or the end of the line, the group's `id` is those
digits and its `title` is the rest of the text, trimmed; otherwise its `id`
is `null`. Boxes placed before the first level-2 heading form a group whose
`id` and `title` are both `null`. Groups SHALL be listed in the order of the
file, and a heading under which no box is counted SHALL NOT be listed. No
word of any language is interpreted: a heading's text is only copied.

#### Scenario: Boxes are counted per numbered group

- **GIVEN** a change whose `tasks.md` holds `## 1. Core` followed by
  `- [x] 1.1 Parser` and `- [ ] 1.2 Engine`, then `## 2. Docs` followed by
  `- [X] 2.1 Guide`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `{ "checked": 2, "total": 3, "groups": [{ "id": "1",
  "title": "Core", "checked": 1, "total": 2 }, { "id": "2", "title": "Docs",
  "checked": 1, "total": 1 }] }`

#### Scenario: A box nested under a task counts

- **GIVEN** a `tasks.md` holding `- [x] 1.1 Parser` and, indented by four
  spaces under it, `- [ ] edge cases`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.total` is 2 and `tasks.checked` is 1

#### Scenario: Boxes inside a code fence are not counted

- **GIVEN** a `tasks.md` holding `- [x] 7.4 Create the PR template:`,
  then, indented by six spaces, a fence opened by three backticks holding
  `- [ ] cargo test green` and closed by three backticks
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.total` is 1 and `tasks.checked` is 1

#### Scenario: Boxes inside an HTML comment are not counted

- **GIVEN** a `tasks.md` holding a comment opened by `<!--` on one line,
  then `- [ ] example task`, then `-->`, then `- [ ] 1.1 Real task`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.total` is 1 and `tasks.checked` is 0

#### Scenario: Other markers are not boxes

- **GIVEN** a `tasks.md` holding `- [-] 9.1 Bump the version after the
  merge`, `* [ ] 9.2 Tag`, `-[ ] 9.3 Push` and `- [ ] 9.4 Changelog`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.total` is 1 and `tasks.checked` is 0

#### Scenario: Boxes before the first heading form a group without id or title

- **GIVEN** a `tasks.md` holding `# Tasks`, `- [ ] Read the plan`, then
  `## Wrap-up` and `- [x] Validate`
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.groups` is `[{ "id": null, "title": null, "checked": 0,
  "total": 1 }, { "id": null, "title": "Wrap-up", "checked": 1, "total": 1
  }]`

#### Scenario: A heading without a box is not listed

- **GIVEN** a `tasks.md` holding `## 1. Core`, `- [ ] 1.1 Parser`, then
  `## Notes` and a paragraph without any box
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.groups` holds only the group whose `id` is `"1"`

#### Scenario: The schema's tracked file is read, not tasks.md

- **GIVEN** a project schema whose `apply.tracks` is `plan.md`, and a
  change holding `plan.md` with one checked box and `tasks.md` with two open
  boxes
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks.total` is 1 and `tasks.checked` is 1

#### Scenario: A tracked path leaving the change folder is not read

- **GIVEN** a project schema whose `apply.tracks` is `../shared.md`, and a
  file `_codev/changes/shared.md` holding one box
- **WHEN** the user runs `codev status --change add-auth --json`
- **THEN** `tasks` is `null`

#### Scenario: An unreadable tracked file warns in status and blocks the archive

- **GIVEN** a change whose `tasks.md` holds bytes that are not valid UTF-8
- **WHEN** the user runs `codev status --change add-auth --json`, then
  `codev archive --change add-auth --json`
- **THEN** `status` exits with code 0, `tasks` is `null`, and its `status`
  holds a `warning` entry with code `tasks_unreadable` naming `tasks.md`
- **AND** `archive` exits with a non-zero code, `status[0].code` is
  `tasks_unreadable`, and the change is still active

#### Scenario: Windows line endings give the same count

- **GIVEN** two changes whose `tasks.md` differ only by `\r\n` instead of
  `\n` line endings, each holding `## 1. Core`, `- [x] 1.1 Parser` and
  `- [ ] 1.2 Engine`
- **WHEN** the user runs `codev status --json` on each
- **THEN** both `tasks` values are identical, with the group title `Core`
