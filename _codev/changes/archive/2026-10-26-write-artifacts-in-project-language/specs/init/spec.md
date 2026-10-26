## MODIFIED Requirements

### Requirement: `codev init` records the artifact language

`codev init` SHALL write a `language:` key in the generated
`_codev/config.yaml`: the language in which skills write the artifacts —
their prose, and their structural keywords when the keyword catalog ships
the language — as an ISO 639 code optionally followed by one subtag (`en`,
`fr`, `pt-BR`). The comment above the key MUST say that the structural
keywords follow the language when codev ships it, and stay in English
otherwise.
The value MUST come, in order, from `--language <CODE>`, from the locale
(`LC_ALL`, then `LC_MESSAGES`, then `LANG`; `C` and `POSIX` carry no
language), then from the default `en`. A provenance comment MUST name its
source. An invalid `--language` MUST be refused before anything is written,
and an invalid `language:` in an existing config MUST be reported as an
invalid configuration.

#### Scenario: Language detected from the locale

- **GIVEN** an environment where `LANG=fr_FR.UTF-8` and `LC_ALL` is unset
- **WHEN** the user runs `codev init --yes`
- **THEN** `_codev/config.yaml` contains `language: fr`
- **AND** the line above it reads `# detected from LANG=fr_FR.UTF-8`
- **AND** the comment above the key says that the structural keywords
  follow the language

#### Scenario: The flag wins over the locale

- **GIVEN** an environment where `LANG=fr_FR.UTF-8`
- **WHEN** the user runs `codev init --yes --language de`
- **THEN** `_codev/config.yaml` contains `language: de`

#### Scenario: An invalid language is refused

- **WHEN** the user runs `codev init --yes --language 'fr_FR!'`
- **THEN** the command fails and no file is written

#### Scenario: No locale, English by default

- **GIVEN** an environment with no locale variable, or only `LC_ALL=C`
- **WHEN** the user runs `codev init --yes`
- **THEN** `_codev/config.yaml` contains `language: en`
