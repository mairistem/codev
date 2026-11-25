## ADDED Requirements

### Requirement: `config.roles` names the roles active by domain

The message of the `config.roles` check SHALL list, besides the active and
the disabled roles, the roles whose activation is `on_domain` — with the
default settings, the six experts — each with the domain or domains that
activate it, under `on domain`. A project setting that turns an expert to
`always` or `never` moves it to the active or the disabled list.

#### Scenario: The experts in the doctor report

- **GIVEN** a project with no `roles:` key
- **WHEN** the user runs `codev doctor`
- **THEN** `config.roles` is `ok` and its message names `security`
  (`security`) and `privacy` (`personal-data`) under `on domain`

#### Scenario: An expert forced by the project

- **GIVEN** a project whose configuration holds
  `roles: { security: always, ops: never }`
- **WHEN** the user runs `codev doctor`
- **THEN** the message of `config.roles` lists `security` among the active
  roles and `ops` among the disabled ones
