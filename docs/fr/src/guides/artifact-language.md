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

Les skills écrivent la **structure** des fichiers en anglais : titres des
templates (`# Proposal:`, `## Why`, `## What Changes`, `## Context`,
`# Tasks`…), structure des specs et des deltas (`## Purpose`,
`### Requirement:`, `#### Scenario:`…), mots-clés des scénarios et mots-clés
normatifs (`**WHEN**`, `**THEN**`, `SHALL`, `MUST`).

codev **lit** cette structure en anglais comme en français, quelle que soit la
valeur de `language:` : une équipe peut donc aussi rédiger ses specs
entièrement en français. Chaque mot-clé a son texte dans chaque langue du
catalogue de mots-clés de codev :

| Anglais | Français |
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

Une exigence rédigée entièrement en français se présente donc ainsi :

```markdown
### Exigence : Le thème suit la préférence du système

L'application DOIT s'afficher avec le thème du système tant que l'utilisateur n'en a pas choisi.

#### Scénario : Aucun thème choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le thème
- **WHEN** son système est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

Comment codev les lit et les écrit :

- Les mots-clés français tolèrent la casse, les accents et l'espace —
  ordinaire ou insécable — avant les deux-points : `### Exigence: X` et
  `#### Scenario : Y` sont lus eux aussi. Les mots-clés anglais sont lus
  exactement comme avant.
- Les mots-clés normatifs ne comptent qu'en majuscules, dans les deux
  langues : `doit` en minuscules est de la prose ordinaire, et l'exigence
  échoue à la validation.
- Un document peut mélanger les deux langues.
- Quand `codev sync` ou `codev archive` modifie une spec, il en écrit les
  mots-clés dans la langue de cette spec ; une nouvelle spec prend la langue
  du delta qui la crée.
- Le changelog affiche les catégories de la note de version en anglais,
  quelle que soit la langue dans laquelle elles ont été écrites.
- Les noms de domaines (`security`, `migration`…) et les marqueurs du
  registre de revue restent en anglais. codev ne lit pas les lignes
  `**GIVEN**` / `**WHEN**` / `**THEN**`, que les skills écrivent en anglais.

> **Attention**
> Une version de codev antérieure à ce catalogue refuse les mots-clés
> français. Avant que votre équipe ne les écrive, relevez `codev_version`
> dans `_codev/config.yaml` jusqu'à la première version dont le changelog
> les annonce : chaque contributeur utilisera ainsi un codev qui les lit. Un
> titre dans une langue que codev ne connaît pas, comme `### Requisito:`, est
> signalé dans une section de delta (`delta_unexpected_heading`), et la
> synchronisation comme l'archivage refusent le change.

## Quelles skills l'utilisent

`codev instructions --json` expose la langue dans son champ `language`.
`/codev-propose` et `/codev-update` rédigent la prose des artefacts dans cette
langue, et `/codev-configure` y rédige également le `context:` et les
`rules:` qu'il propose. Le template d'ADR utilisé par les commandes
`codev decision` a des titres de sections en anglais ; rédigez-en le contenu
dans la langue de votre équipe.
