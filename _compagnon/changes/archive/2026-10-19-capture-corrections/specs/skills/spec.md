## ADDED Requirements

### Requirement: The `propose`, `apply` and `update` skills note the author's corrections

The `codev-propose`, `codev-apply` and `codev-update` skills SHALL tell
the agent that, when the author corrects what it did with an instruction
about how the project does things — a convention, a technical choice, an
expectation on an artifact — it appends one entry to the change's
`corrections.md` in the format the `corrections` capability defines: the
step, where, what it did and what the author asked, one or two lines,
never a transcript of the conversation. The agent writes the entry once,
without reading the file again beyond finding the next free identifier,
and then quotes the entry it wrote — `where`, `did` and `asked` — to the
author, who may have it changed or removed before it is committed.

The skills SHALL give examples of what counts and what does not. It
counts when the instruction would apply to the next change too: "errors
go through `thiserror` in the libraries, `anyhow` stays in the CLI";
"every scenario is covered by a test named after the behavior"; "a new
JSON field is additive, never renamed". It does not count: a one-off
preference about this change ("call this variable `total`"), a typo, a
change of scope or of the request, a factual correction of what the
agent misunderstood about the request, or a personal preference about
how the agent talks or works with this author.

The skills SHALL say where an instruction goes: project knowledge,
shared with the team through git, goes to `corrections.md` and not to
the agent's personal memory; a personal preference stays in the
personal memory and never goes to `corrections.md`. An entry SHALL name
no person and quote no personal data or secret from the conversation:
it states the instruction, not who gave it.

#### Scenario: The skills tell the agent to note a correction

- **GIVEN** the `codev-propose`, `codev-apply` and `codev-update` skills
  as `codev update` renders them
- **WHEN** their text is read
- **THEN** each tells the agent to append an entry to `corrections.md`
  when the author corrects how the project does things, with `step`,
  `where`, `did` and `asked`, and to quote the entry it wrote to the
  author

#### Scenario: The skills say what counts as a correction

- **GIVEN** the same skills
- **WHEN** their text is read
- **THEN** each gives examples of instructions that count and of
  one-off preferences, typos, scope changes and personal preferences
  that do not

#### Scenario: Project knowledge and personal memory are kept apart

- **GIVEN** the same skills
- **WHEN** their text is read
- **THEN** each says that a project instruction goes to `corrections.md`
  and not to the personal memory, that a personal preference never goes
  to `corrections.md`, and that an entry names no person and quotes no
  personal data or secret

#### Scenario: A run without correction writes no file

- **GIVEN** a `codev-evals` planning run, in which no author corrects
  the agent
- **WHEN** the run ends
- **THEN** the change holds no `corrections.md`

#### Scenario: Other skills do not note corrections

- **GIVEN** the `codev-archive`, `codev-sync` and `codev-explore` skills
  as `codev update` renders them
- **WHEN** their text is read
- **THEN** none mentions `corrections.md`
