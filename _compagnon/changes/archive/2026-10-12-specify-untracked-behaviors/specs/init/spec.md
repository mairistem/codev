## MODIFIED Requirements

### Requirement: `codev init --inherit` declares inherited sources

`codev init` SHALL accept `--inherit <SOURCE>`, any number of times. A
SOURCE is a git source when it starts with `git@`, `https://`, `http://`,
`ssh://` or `file://`, or ends with `.git` once an optional `@<ref>`
suffix is removed; anything else is a local path. A git source MAY carry an
`@<ref>` suffix (`https://example.com/acme/conventions.git@v2`); the `@`
of a user name before the host (`git@github.com:…`) is not a ref
separator. Without a suffix the ref is `main`.

When `codev init` writes `_codev/config.yaml`, it MUST declare every
source under `inherits:`, in the order given, as `git:` + `ref:` or as
`path:`, with the comment `# set with codev init --inherit` above the
key. A relative local path MUST be written as an absolute path, so that it
resolves the same from any folder of the project, and with forward slashes
on every platform (Windows accepts them), so that the file stays readable
and identical whatever machine ran `codev init`. The written file MUST
stay readable by every other codev command.

The command MUST refuse, before writing anything, with the stable code
`invalid_inherit`:

- an empty value;
- a git source with an empty ref (`…/conventions.git@`) or an empty URL;
- a local path that does not exist or is not a folder.

When `_codev/config.yaml` already exists, `codev init` MUST NOT modify it:
the sources are not written, and a warning with the code
`inherit_not_written` names them and tells the user to add them under
`inherits:` by hand.

No source is declared when `--inherit` is absent.

#### Scenario: A git source with a ref and a local path

- **GIVEN** a new project and an existing folder `/home/you/conventions`
- **WHEN** the user runs `codev init --yes --inherit
  git@github.com:acme/conventions.git@v2 --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` contains, under `inherits:`, an entry
  `git: git@github.com:acme/conventions.git` with `ref: v2`, then an entry
  `path: /home/you/conventions`
- **AND** the line above `inherits:` reads
  `# set with codev init --inherit`
- **AND** `codev status` and `codev sources list` read the file without
  error

#### Scenario: A git source without a ref gets `main`

- **WHEN** the user runs `codev init --yes --inherit
  https://example.com/acme/conventions.git`
- **THEN** the declared source carries `ref: main`

#### Scenario: An invalid value is refused before anything is written

- **GIVEN** a folder without `_codev/`
- **WHEN** the user runs `codev init --yes --inherit ./does-not-exist`
- **THEN** the command fails with the code `invalid_inherit`
- **AND** no file or folder has been created

#### Scenario: An existing configuration is left untouched

- **GIVEN** a project whose `_codev/config.yaml` already exists
- **WHEN** the user runs `codev init --yes --inherit /home/you/conventions`
- **THEN** `_codev/config.yaml` is identical byte for byte
- **AND** a warning `inherit_not_written` names the source

#### Scenario: A relative local path is written with forward slashes

- **GIVEN** a project folder and an existing folder `../conventions` next to it
- **WHEN** the user runs `codev init --yes --inherit ../conventions`, on any
  platform
- **THEN** the declared `path:` is the absolute path of that folder, written
  with `/` separators and without quotes
