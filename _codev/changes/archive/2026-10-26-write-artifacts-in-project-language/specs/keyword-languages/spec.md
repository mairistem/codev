## ADDED Requirements

### Requirement: A project writes its keywords in its keyword language

codev SHALL resolve a project's keyword language from its `language:`
setting: the language whose code is the setting's primary subtag, compared
without regard to case (`fr`, `fr-CA` and `FR` give `fr`), when the keyword
catalog ships that language, and English (`en`) otherwise, including when
`language:` is absent. The keyword language is the language of the
structural keywords codev's templates and skills write in a new artifact;
the prose stays in the `language:` itself. Reading is unchanged: a keyword
of any catalog language is still recognized wherever codev reads one.

#### Scenario: A shipped language

- **GIVEN** a configuration with `language: fr`
- **WHEN** codev resolves the keyword language
- **THEN** it is `fr`

#### Scenario: A regional variant of a shipped language

- **GIVEN** a configuration with `language: fr-CA`
- **WHEN** codev resolves the keyword language
- **THEN** it is `fr`

#### Scenario: An upper-case setting

- **GIVEN** a configuration with `language: FR`
- **WHEN** codev resolves the keyword language
- **THEN** it is `fr`

#### Scenario: A language the catalog does not ship

- **GIVEN** a configuration with `language: de`
- **WHEN** codev resolves the keyword language
- **THEN** it is `en`

#### Scenario: No configured language

- **GIVEN** a configuration without `language:`
- **WHEN** codev resolves the keyword language
- **THEN** it is `en`
