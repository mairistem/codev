# Langue des artefacts

codev lui-même s'exprime en anglais : ses commandes, ses messages et ses skills
sont en anglais. Les artefacts de planification que votre équipe lit et
approuve — proposals, specs, designs, tâches — peuvent en revanche être
rédigés dans n'importe quelle langue, et entièrement en français : la prose
comme la structure.

## Définir la langue

La clé `language:` de `_codev/config.yaml` définit la langue des artefacts,
sous la forme d'un code ISO 639 :

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

Les skills écrivent la **structure** des fichiers — titres des templates,
structure des specs et des deltas, lignes des scénarios et mots-clés
normatifs — dans la **langue des mots-clés** du projet : la langue de
`language:` quand codev en fournit les mots-clés, reconnue sur sa première
partie (`fr`, `fr-CA` → français), et l'anglais sinon (`pt-BR` → mots-clés
anglais, prose portugaise). codev fournit l'anglais et le français.
`codev instructions` sert le template dans cette langue et donne le texte de
chaque mot-clé (`keywordLanguage`, `keywords`) : les skills ne traduisent
jamais un mot-clé elles-mêmes.

Avec `language: fr`, une nouvelle proposition commence par `# Proposition :`,
`## Pourquoi`, `## Changements`, et un delta de spec utilise `## Exigences
AJOUTÉES`, `### Exigence :`, `#### Scénario :`, `**QUAND**` / `**ALORS**` et
`DOIT`. Pour garder des mots-clés anglais avec une prose française, gardez
`language: en` : il n'y a pas de réglage séparé pour la langue des mots-clés.

codev **lit** la structure en anglais comme en français, quelle que soit la
valeur de `language:`. Chaque mot-clé a son texte dans chaque langue du
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

- **ÉTANT DONNÉ** un utilisateur qui n'a jamais modifié le thème
- **QUAND** son système est en mode sombre
- **ALORS** l'application s'affiche avec le thème sombre
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
- Les noms de domaines (`security`, `migration`…), le registre de revue et
  `corrections.md` restent en anglais. codev ne lit pas les lignes des
  scénarios (`**WHEN**` / `**THEN**`, `**QUAND**` / `**ALORS**`).

## Mettre à jour un projet déjà en français

Un projet en `language: fr` recevait jusqu'ici des titres anglais sous une
prose française ; ses prochains artefacts sont écrits en français. Deux
mélanges en découlent, tous deux valides, puisque codev lit les deux langues
dans un même document :

- Un change planifié avant la mise à jour reste tel quel : `/codev-update`
  garde la langue des mots-clés de l'artefact qu'il révise, et seul un
  nouvel artefact prend celle du projet.
- Une spec principale garde sa propre langue. Une fusion écrit ses titres
  dans la langue de cette spec, mais copie le corps d'une exigence tel qu'il
  est écrit : un `DOIT` ou un `**QUAND**` français peut ainsi se retrouver
  sous un `### Requirement:` anglais. Pour l'éviter, gardez `language: en`,
  ou réécrivez une fois en français les mots-clés d'une spec principale —
  codev ne réécrit jamais une spec de lui-même.

> **Attention**
> Une version de codev antérieure à ce catalogue refuse les mots-clés
> français. Avant que votre équipe ne les écrive, relevez `codev_version`
> dans `_codev/config.yaml` jusqu'à la première version dont le changelog
> les annonce : chaque contributeur utilisera ainsi un codev qui les lit. Un
> titre dans une langue que codev ne connaît pas, comme `### Requisito:`, est
> signalé dans une section de delta (`delta_unexpected_heading`), et la
> synchronisation comme l'archivage refusent le change.

## Quelles skills l'utilisent

`codev instructions --json` expose la langue dans son champ `language`, la
langue des mots-clés dans `keywordLanguage`, et le texte de chaque mot-clé
dans `keywords` ; sa sortie lisible donne les deux sur sa ligne `Language:`.
`/codev-propose` et `/codev-update` rédigent les artefacts dans cette langue,
prose et mots-clés, et `/codev-configure` y rédige également le `context:` et les
`rules:` qu'il propose. Un schéma de projet peut contenir ses templates par langue, sous
`templates/<langue>/` ; sans eux, `templates/<fichier>` sert toutes les
langues ([Schémas personnalisés](custom-schemas.md)). Le template d'ADR utilisé par les commandes
`codev decision` a des titres de sections en anglais ; rédigez-en le contenu
dans la langue de votre équipe.
