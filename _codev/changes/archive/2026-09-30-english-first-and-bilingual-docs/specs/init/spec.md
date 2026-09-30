## ADDED Requirements

### Requirement: `codev init` records the artifact language

`codev init` SHALL write a `language:` key in the generated
`_codev/config.yaml`: the language in which skills write artifact prose, as
an ISO 639 code optionally followed by one subtag (`en`, `fr`, `pt-BR`).
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

#### Scenario: The flag wins over the locale

- **GIVEN** an environment where `LANG=fr_FR.UTF-8`
- **WHEN** the user runs `codev init --yes --language de`
- **THEN** `_codev/config.yaml` contains `language: de`

#### Scenario: No locale, English by default

- **GIVEN** an environment with no locale variable, or only `LC_ALL=C`
- **WHEN** the user runs `codev init --yes`
- **THEN** `_codev/config.yaml` contains `language: en`
