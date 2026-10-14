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

The skills write the **structure** of the files in English: template headings
(`# Proposal:`, `## Why`, `## What Changes`, `## Context`, `# Tasks`…), spec
and delta structure (`## Purpose`, `### Requirement:`, `#### Scenario:`…), and
the scenario and normative keywords (`**WHEN**`, `**THEN**`, `SHALL`, `MUST`).

codev **reads** that structure in English or in French, whatever
`language:` says, so a team can also write its specs entirely in French. Each
keyword has its text in every language of codev's keyword catalog:

| English | French |
|---|---|
| `## Purpose` | `## Objet` |
| `## Requirements` | `## Exigences` |
| `### Requirement:` | `### Exigence :` |
| `#### Scenario:` | `#### Scénario :` |
| `## ADDED Requirements` | `## Exigences AJOUTÉES` |
| `## MODIFIED Requirements` | `## Exigences MODIFIÉES` |
| `## REMOVED Requirements` | `## Exigences SUPPRIMÉES` |
| `## RENAMED Requirements` | `## Exigences RENOMMÉES` |
| `**Reason**`, `**Migration**` | `**Raison**`, `**Migration**` |
| `FROM:` / `TO:` | `DE :` / `VERS :` |
| `SHALL`, `MUST` | `DOIT`, `DOIVENT` |
| `## What Changes` | `## Changements` |
| `## Domains`, `None` | `## Domaines`, `Aucun` |
| `## Release note`, `None` | `## Note de version`, `Aucune` |
| `Added`, `Changed`, `Deprecated`, `Removed`, `Fixed`, `Security` | `Ajouté`, `Modifié`, `Obsolète`, `Supprimé`, `Corrigé`, `Sécurité` |
| `## Decisions`, `### Decision:` | `## Décisions`, `### Décision :` |

A requirement written entirely in French therefore looks like this:

```markdown
### Exigence : Le thème suit la préférence du système

L'application DOIT s'afficher avec le thème du système tant que l'utilisateur n'en a pas choisi.

#### Scénario : Aucun thème choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le thème
- **WHEN** son système est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

How codev reads and writes them:

- French keywords forgive case, accents and the space — ordinary or
  no-break — before a colon: `### Exigence: X` and `#### Scenario : Y` are
  read too. English keywords are read exactly as before.
- Normative keywords count only in upper case, in both languages: `doit` in
  lower case is ordinary prose, and the requirement fails validation.
- A document may mix both languages.
- When `codev sync` or `codev archive` edits a spec, it writes the keywords
  in that spec's language; a new spec takes the language of the delta that
  creates it.
- The changelog shows release note categories in English, whatever language
  wrote them.
- Domain names (`security`, `migration`…) and the review record's markers
  stay in English. codev does not read the `**GIVEN**` / `**WHEN**` /
  `**THEN**` lines, which the skills write in English.

> **Warning**
> A codev release older than this catalog rejects French keywords. Before
> your team writes them, raise `codev_version` in `_codev/config.yaml` to
> the first release whose changelog announces them, so that every
> contributor runs a codev that reads them. A heading in a language codev
> does not know, such as `### Requisito:`, is reported in a delta section
> (`delta_unexpected_heading`), and the sync and the archive refuse the
> change.

## Which skills use it

`codev instructions --json` exposes the language in its `language` field.
`/codev-propose` and `/codev-update` write artifact prose in that language, and
`/codev-configure` writes the `context:` and `rules:` it proposes in it too.
The ADR template used by `codev decision` commands has English section
headings; write the body in your team's language.
