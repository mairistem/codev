# Artifact language

codev itself speaks English: its commands, messages, skills and templates are
in English. The planning artifacts your team reads and approves — proposals,
specs, designs, tasks — can be written in any language.

## Setting the language

The `language:` key of `_codev/config.yaml` sets the language of the prose in
artifacts, as an ISO 639 code:

```yaml
language: fr
```

Accepted values are a two- or three-letter lowercase language code,
optionally followed by one subtag such as a region or a script: `en`, `fr`,
`de`, `pt-BR`, `zh-Hant`. Any other value makes the configuration invalid.
When the key is absent, the language is English (`en`).

## How `codev init` chooses it

`codev init` writes the `language:` key for you, with a comment saying where
the value came from. It uses, in order:

1. the `--language <CODE>` option;
2. otherwise your locale — the first of `LC_ALL`, `LC_MESSAGES` and `LANG`
   that names a language. `C` and `POSIX` are skipped, so `LC_ALL=C` does not
   hide a meaningful `LANG`;
3. otherwise `en`.

```bash
codev init --language pt-BR
```

```yaml
# set with `codev init --language`
language: pt-BR
```

Detection keeps only the language part of the locale — `fr_FR.UTF-8` gives
`fr` — except where the region changes the written language: `pt_BR` gives
`pt-BR`, `zh_TW` and `zh_HK` give `zh-Hant`, `zh_CN` and `zh_SG` give
`zh-Hans`. Pass `--language` to record any other region. `--no-detect` disables locale detection along with the rest
of the environment probe.

On a project that already has a `_codev/config.yaml`, `codev init` does not
touch the file. To change the language later, edit the key.

## What is translated, and what is not

The skills write every sentence in the configured language, whatever language
you chat with Claude in: the files are the team's record, and the language is
the team's choice.

The **structure** of the files always stays in English, because codev parses
it:

- template headings: `# Proposal:`, `## Why`, `## What Changes`,
  `## Capabilities`, `## Impact`, `## Context`, `## Decisions`, `# Tasks`…;
- spec and delta structure: `## Purpose`, `## Requirements`,
  `## ADDED Requirements` and the other delta sections, `### Requirement:`,
  `#### Scenario:`, `**Reason**`, `**Migration**`, `FROM:` / `TO:`;
- scenario and normative keywords: `**GIVEN**`, `**WHEN**`, `**THEN**`,
  `**AND**`, `SHALL`, `MUST`.

A requirement written in French therefore looks like this:

```markdown
### Requirement: Le thème suit la préférence du système

L'application SHALL s'afficher avec le thème du système tant que l'utilisateur n'en a pas choisi.

#### Scenario: Aucun thème choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le thème
- **WHEN** son système est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

> **Warning**
> A translated keyword is a broken artifact. codev does not recognize
> `### Exigence :` as a requirement: in a delta section, validation reports it
> (`delta_unexpected_heading`), and the sync and the archive refuse the change.
> A requirement without `SHALL` or `MUST` fails validation too.

## Which skills use it

`codev instructions --json` exposes the language in its `language` field.
`/codev-propose` and `/codev-update` write artifact prose in that language, and
`/codev-configure` writes the `context:` and `rules:` it proposes in it too.
The ADR template used by `codev decision` commands has English section
headings; write the body in your team's language.
