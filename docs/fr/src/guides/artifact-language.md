# Langue des artefacts

codev lui-même s'exprime en anglais : ses commandes, ses messages, ses skills et
ses templates sont en anglais. Les artefacts de planification que votre équipe
lit et approuve — proposals, specs, designs, tâches — peuvent en revanche être
rédigés dans n'importe quelle langue.

## Définir la langue

La clé `language:` de `_codev/config.yaml` définit la langue de la prose des
artefacts, sous la forme d'un code ISO 639 :

```yaml
language: fr
```

Les valeurs acceptées sont un code de langue en minuscules de deux ou trois
lettres, éventuellement suivi d'une sous-étiquette, telle qu'une région ou une
écriture : `en`, `fr`, `de`, `pt-BR`, `zh-Hant`. Toute autre valeur rend la
configuration invalide. En l'absence de la clé, la langue est l'anglais
(`en`).

## Comment `codev init` la choisit

`codev init` écrit la clé `language:` pour vous, avec un commentaire indiquant
la provenance de la valeur. Il retient, dans l'ordre :

1. l'option `--language <CODE>` ;
2. à défaut, votre locale — la première des variables `LC_ALL`,
   `LC_MESSAGES` et `LANG` qui désigne une langue. `C` et `POSIX` sont
   ignorées : `LC_ALL=C` ne masque donc pas une variable `LANG` pertinente ;
3. à défaut, `en`.

```bash
codev init --language pt-BR
```

```yaml
# set with `codev init --language`
language: pt-BR
```

La détection ne conserve que la partie langue de la locale — `fr_FR.UTF-8`
donne `fr` —, sauf lorsque la région change la langue écrite : `pt_BR` donne
`pt-BR`, `zh_TW` et `zh_HK` donnent `zh-Hant`, `zh_CN` et `zh_SG` donnent
`zh-Hans`. Passez `--language` pour enregistrer toute autre région. `--no-detect` désactive la détection de la locale
en même temps que le reste de l'analyse de l'environnement.

Sur un projet qui possède déjà un `_codev/config.yaml`, `codev init` ne touche
pas au fichier. Pour changer de langue par la suite, modifiez la clé.

## Ce qui est traduit, et ce qui ne l'est pas

Les skills rédigent chaque phrase dans la langue configurée, quelle que soit la
langue dans laquelle vous échangez avec Claude : les fichiers constituent la
mémoire de l'équipe, et la langue est le choix de l'équipe.

La **structure** des fichiers reste toujours en anglais, car codev l'analyse :

- les titres des templates : `# Proposal:`, `## Why`, `## What Changes`,
  `## Capabilities`, `## Impact`, `## Context`, `## Decisions`, `# Tasks`… ;
- la structure des specs et des deltas : `## Purpose`, `## Requirements`,
  `## ADDED Requirements` et les autres sections de delta,
  `### Requirement:`, `#### Scenario:`, `**Reason**`, `**Migration**`,
  `FROM:` / `TO:` ;
- les mots-clés des scénarios et les mots-clés normatifs : `**GIVEN**`,
  `**WHEN**`, `**THEN**`, `**AND**`, `SHALL`, `MUST`.

Une exigence rédigée en français se présente donc ainsi — la prose en
français, les mots-clés en anglais :

```markdown
### Requirement: Le thème suit la préférence du système

L'application SHALL s'afficher avec le thème du système tant que l'utilisateur n'en a pas choisi.

#### Scenario: Aucun thème choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le thème
- **WHEN** son système est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

> **Attention**
> Un mot-clé traduit, c'est un artefact cassé. codev ne reconnaît pas
> `### Exigence :` comme une exigence : dans une section de delta, la
> validation le signale (`delta_unexpected_heading`), et la synchronisation
> comme l'archivage refusent le change. Une exigence dépourvue de `SHALL` ou de
> `MUST` échoue elle aussi à la validation.

## Quelles skills l'utilisent

`codev instructions --json` expose la langue dans son champ `language`.
`/codev-propose` et `/codev-update` rédigent la prose des artefacts dans cette
langue, et `/codev-configure` y rédige également le `context:` et les
`rules:` qu'il propose. Le template d'ADR utilisé par les commandes
`codev decision` a des titres de sections en anglais ; rédigez-en le contenu
dans la langue de votre équipe.
