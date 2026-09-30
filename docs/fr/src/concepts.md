# Concepts

codev conserve trois types d'enregistrements dans le dépôt : les **specs**
disent ce que fait le système, les **décisions** expliquent pourquoi il est
construit ainsi, et les **changes** sont le travail en cours qui modifie les
unes et les autres.

```text
_codev/
├── config.yaml     # configuration du projet
├── specs/          # contrats de comportement — CE QUE fait le système
├── decisions/      # décisions d'architecture — POURQUOI il est construit ainsi
├── changes/        # travail en cours, qui modifie les deux dossiers ci-dessus
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   changes terminés, datés
├── schemas/        # schémas de workflow personnalisés (facultatif)
└── codev.lock      # sources git héritées et épinglées (le cas échéant)
```

codev trouve ce dossier en remontant l'arborescence depuis le répertoire
courant : chaque commande fonctionne donc depuis n'importe quel
sous-répertoire du projet.

## Capacité

Une capacité est une unité de comportement observable qu'offre le système —
l'authentification des utilisateurs, le réglage du thème, la commande
`validate`. Une capacité est identifiée par un chemin de segments en
kebab-case, comme `ui/theme` ou `identity/user-auth`, qui indique aussi
l'emplacement de sa spec. Regroupez les capacités selon l'organisation de
votre domaine métier, et non selon celle de votre code.

## Spec

La spec principale d'une capacité se trouve dans
`_codev/specs/<capability>/spec.md`. Elle comporte une section `## Purpose` —
une ou deux phrases sur la raison d'être de la capacité — et une section
`## Requirements` :

```markdown
# Theme Specification

## Purpose

Permettre aux utilisateurs de choisir le thème de couleurs de l'interface.

## Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème.

#### Scenario: Aucun thème encore choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le réglage du thème
- **WHEN** son système d'exploitation est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre
```

Chaque exigence est normative (elle emploie `SHALL` ou `MUST`) et comporte au
moins un scénario décrivant un comportement observable. Une spec décrit un
comportement, jamais une implémentation : si le code peut changer sans que le
comportement change, la spec ne doit pas changer non plus.

On ne modifie jamais directement une spec principale. Elle n'évolue que
lorsqu'un change est synchronisé ou archivé, ce qui rend son historique
lisible.

## Change

Un change est un dossier `_codev/changes/<name>/` qui porte une évolution du
système : une proposal, des deltas de specs, un design et une liste de tâches,
ainsi qu'un fichier `change.yaml` contenant ses métadonnées. Les artefacts que
comporte un change sont définis par son [schéma](guides/custom-schemas.md) ;
le schéma par défaut, `spec-driven`, est décrit dans
[Le workflow](workflow.md).

Un change est **actif** tant qu'il se trouve dans `_codev/changes/<name>/`.
L'archivage le déplace dans `_codev/changes/archive/<YYYY-MM-DD>-<name>/`, où
il devient la trace du pourquoi et du comment de l'évolution des specs.

## Delta

Un delta est la partie d'un change qui modifie une spec. Il se trouve dans
`_codev/changes/<name>/specs/<capability>/spec.md` — le même chemin de
capacité que la spec principale qu'il cible — et s'organise en quatre
opérations au plus :

| Section | Effet lors de la fusion |
|---|---|
| `## ADDED Requirements` | Ajoute les exigences à la fin de la spec principale |
| `## MODIFIED Requirements` | Remplace sur place l'exigence de même nom |
| `## REMOVED Requirements` | Supprime l'exigence, avec une **Reason** (raison) et une **Migration** |
| `## RENAMED Requirements` | Change le titre d'une exigence sans toucher à son contenu |

Voici un delta qui modifie une exigence et en renomme une autre :

```markdown
## MODIFIED Requirements

### Requirement: Le thème suit par défaut la préférence du système

L'application SHALL s'afficher avec le jeu de couleurs du système d'exploitation tant que l'utilisateur n'a pas choisi de thème, et SHALL basculer en direct lorsque cette préférence change.

#### Scenario: Aucun thème encore choisi

- **GIVEN** un utilisateur qui n'a jamais modifié le réglage du thème
- **WHEN** son système d'exploitation est en mode sombre
- **THEN** l'application s'affiche avec le thème sombre

## RENAMED Requirements

- FROM: Le thème choisi est mémorisé
- TO: Le thème choisi suit l'utilisateur
```

Un bloc `MODIFIED` remplace l'exigence tout entière : copiez donc le bloc
complet depuis la spec principale — scénarios compris — avant de le modifier.
Tout contenu omis est perdu lors de la fusion. Si vous ajoutez un comportement
sans modifier l'existant, utilisez `ADDED`.

La fusion ne touche que les exigences nommées dans le delta. Tout le reste de
la spec principale — autres exigences, sections libres, commentaires — reste
tel quel.

Le delta d'une **nouvelle** capacité commence par sa propre section
`## Purpose` ; sa fusion crée la spec principale. Supprimer entièrement une
capacité — un delta `REMOVED` qui vide une spec — exige
`retire_capabilities: true` dans `change.yaml`, car cette suppression ne peut
être annulée que depuis git.

La syntaxe exacte figure dans [Formats de fichiers](reference/file-formats.md).

## Décision

Une décision, ou ADR (*architecture decision record*), est un fichier
`_codev/decisions/NNNN-<slug>.md` qui consigne un choix d'architecture : son
contexte, la décision elle-même, ses conséquences et les alternatives
envisagées.

```bash
codev decision new "Use PostgreSQL for persistence"
codev decision accept 0001
```

Une décision a l'un des cinq statuts suivants : `accepted`, `superseded`,
`proposed`, `deprecated` ou `rejected`. Toutes les commandes qui créent une
décision — `codev decision new`, [`supersede`](#remplacement),
[`deviate`](#écart) et [`promote`](#promotion) — la créent `proposed`, afin que
vous puissiez la rédiger librement ; `codev decision accept` la passe à
`accepted` une fois son texte définitif. L'acceptation est la seule étape qui
fait entrer une décision en vigueur : une décision `proposed` ne remplace encore
rien et ne s'écarte encore de rien. Une décision est **en vigueur**
lorsqu'elle est `accepted` et qu'aucune autre ne la remplace. Les décisions en
vigueur sont transmises à l'agent chaque fois qu'il rédige un `design.md`,
afin que les designs les respectent au lieu de les remettre en débat.

Chaque décision possède un identifiant court (`0007`) et un identifiant
qualifié qui indique sa provenance : `project/0007` pour une décision du projet
lui-même, `path:~/shared/0100` ou `git:git@github.com:acme/shared.git/0100`
pour une décision héritée d'un autre dépôt.

## Sceau

Une décision acceptée est immuable. Lorsqu'une décision devient acceptée par
une commande `codev decision` — `accept`, ou `new --status accepted` — codev
enregistre une empreinte SHA-256 du contenu de la décision dans
`_codev/decisions/seal.yaml` ; `codev decision seal` fait
de même pour une décision que vous avez acceptée à la main.
`codev validate` compare le contenu de chaque décision locale `accepted` ou
`superseded` à son empreinte, et signale une modification par l'erreur
`decision_seal_mismatch` et un sceau manquant par l'avertissement
`decision_unsealed`. Une décision `proposed` n'est pas scellée.

Le frontmatter n'est pas couvert par le sceau : un statut peut donc passer de
`accepted` à `superseded` sans le briser.

> **Remarque**
> Scellez une décision lorsque son texte est définitif : rédigez d'abord la
> décision `proposed`, puis lancez `codev decision accept <ID>`, qui change
> le statut et enregistre le sceau en une seule étape. Si vous avez modifié
> volontairement une décision scellée, `codev decision seal <ID> --force`
> enregistre le nouveau contenu ; pour changer ce que dit une décision
> acceptée, [remplacez-la](#remplacement) plutôt.

## Remplacement

Pour changer une décision, remplacez-la plutôt que de la modifier :

```bash
codev decision supersede 0001 "Use SQLite for persistence"
```

codev crée une nouvelle décision `proposed` dont le frontmatter indique
`supersedes: ["0001"]`, et ne touche pas à `0001` : elle reste `accepted`, et en
vigueur, pendant que vous rédigez la nouvelle. Accepter la nouvelle décision
marque `0001` comme `superseded` dans la même étape :

```bash
codev decision accept 0002
```

Le contenu de l'ancienne décision reste strictement identique : son sceau reste
valide et l'historique du raisonnement est préservé. Seule une décision
`accepted` peut être remplacée : si une autre décision a remplacé `0001`
entre-temps, `codev decision accept` refuse avec `predecessor_not_accepted` et
n'écrit rien.

## Écart

Une décision [héritée](#source-héritée) ne peut pas être remplacée depuis
votre projet, puisque vous ne pouvez pas écrire dans sa source. Consignez
plutôt un écart local :

```bash
codev decision deviate path:~/shared/0100 "Services log in logfmt"
```

Cette commande crée une décision locale `proposed` dont le champ
`deviates_from` pointe vers la décision héritée. L'écart entre en vigueur
lorsque vous l'acceptez avec `codev decision accept` : la décision héritée reste
alors visible dans `codev decision list`, mais elle n'est plus en vigueur pour
votre projet et n'est plus transmise à l'agent lorsqu'il rédige des designs.
D'ici là, la décision héritée reste en vigueur.

## Promotion

Un design contient souvent une décision qui mérite de survivre au change.
Rédigez-la dans `design.md` sous la forme d'un bloc `### Decision: <title>`,
puis promouvez-la :

```bash
codev decision promote add-audit-log "Append-only audit table"
```

codev crée un ADR `proposed` dont la section `## Decision` reprend tel quel le
contenu du bloc, et remplace le contenu du bloc par une référence au nouvel
ADR. Relisez-le et répartissez le texte promu dans les sections de l'ADR, puis
acceptez-le avec `codev decision accept`, qui le scelle — idéalement avant
d'archiver le change.

## Source héritée

Un projet peut hériter, en lecture seule, du contenu d'autres dépôts codev —
un dossier partagé sur votre machine (`path:`) ou un dépôt git épinglé sur un
commit (`git:`). Chaque source apporte son `context:`, ses `rules:` et ses
décisions. Rien d'exécutable n'est jamais hérité, et rien n'est lu depuis une
branche mouvante : les sources git sont épinglées dans `_codev/codev.lock`.
Voir [Sources héritées](guides/inherited-sources.md).

## Schéma

Un schéma définit les artefacts que comporte un change, les prérequis de
chacun, ainsi que les instructions et les templates que suit l'agent pour les
rédiger. codev embarque un schéma, `spec-driven`. Un projet peut définir les
siens dans `_codev/schemas/`. Voir
[Schémas personnalisés](guides/custom-schemas.md).
