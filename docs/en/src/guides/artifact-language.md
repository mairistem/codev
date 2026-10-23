# Artifact language

codev itself speaks English: its commands, messages and skills are in English.
The planning artifacts your team reads and approves — proposals, specs,
designs, tasks — can be written in any language, and entirely in French:
prose and structure alike.

## Setting the language

The `language:` key of `_codev/config.yaml` sets the language of the
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

The skills write the **structure** of the files — template headings, spec and
delta structure, the scenario lines and the normative keywords — in the
project's **keyword language**: the language of `language:` when codev ships
its keywords, matched on its first part (`fr`, `fr-CA` → French), and
English otherwise (`pt-BR` → English keywords, Portuguese prose). codev ships
English and French. `codev instructions` serves the template in that
language, and gives the text of every keyword in it (`keywordLanguage`,
`keywords`), so the skills never translate a keyword themselves.

With `language: fr`, a new proposal starts with `# Proposition :`,
`## Pourquoi`, `## Changements`, and a delta spec uses `## Exigences
AJOUTÉES`, `### Exigence :`, `#### Scénario :`, `**QUAND**` / `**ALORS**` and
`DOIT`. To keep English keywords with French prose, keep `language: en`:
there is no separate setting for the keyword language.

codev **reads** the structure in English or in French, whatever
`language:` says. Each keyword has its text in every language of codev's
keyword catalog:

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

- **ÉTANT DONNÉ** un utilisateur qui n'a jamais modifié le thème
- **QUAND** son système est en mode sombre
- **ALORS** l'application s'affiche avec le thème sombre
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
- Domain names (`security`, `migration`…), the review record and
  `corrections.md` stay in English. codev does not read the scenario lines
  (`**WHEN**` / `**THEN**`, `**QUAND**` / `**ALORS**`).

## Upgrading a project already in French

A project with `language: fr` got English headings under French prose until
this release; its next artifacts are written in French. Two kinds of mixing
follow, both valid, since codev reads both languages in one document:

- A change planned before the upgrade stays as it is: `/codev-update` keeps
  the keyword language of the artifact it revises, and only a new artifact
  takes the project's.
- A main spec keeps its own language. A merge writes its headings in that
  spec's language, but copies a requirement's body as written, so a French
  `DOIT` or `**QUAND**` can land under an English `### Requirement:`. To
  avoid it, keep `language: en`, or rewrite a main spec's keywords in French
  once — codev never rewrites a spec on its own.

> **Warning**
> A codev release older than this catalog rejects French keywords. Before
> your team writes them, raise `codev_version` in `_codev/config.yaml` to
> the first release whose changelog announces them, so that every
> contributor runs a codev that reads them. A heading in a language codev
> does not know, such as `### Requisito:`, is reported in a delta section
> (`delta_unexpected_heading`), and the sync and the archive refuse the
> change.

## Which skills use it

`codev instructions --json` exposes the language in its `language` field, the
keyword language in `keywordLanguage`, and the text of each keyword in
`keywords`; its human output says both on its `Language:` line.
`/codev-propose` and `/codev-update` write artifacts in that language, prose
and keywords, and `/codev-configure` writes the `context:` and `rules:` it
proposes in it too. A project schema may hold its templates per language,
under `templates/<language>/`; without, `templates/<file>` serves every
language ([Custom schemas](custom-schemas.md)).
The ADR template used by `codev decision` commands has English section
headings; write the body in your team's language.
