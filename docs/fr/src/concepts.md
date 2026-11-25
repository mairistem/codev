# Concepts

Compagnon conserve trois types d'enregistrements dans le dépôt de votre
projet, à côté du code :

- les **spécifications** disent ce que fait le système ;
- les **décisions** expliquent pourquoi il est construit ainsi ;
- les **modifications** sont le travail en cours qui fait évoluer les
  spécifications et les décisions.

Voici où se trouve chacun d'eux :

```text
_compagnon/
├── config.yaml     # configuration du projet
├── specs/          # contrats de comportement — CE QUE fait le système
├── decisions/      # décisions d'architecture — POURQUOI il est construit ainsi
├── changes/        # travail en cours, qui modifie les deux dossiers ci-dessus
│   ├── <name>/     #   proposal.md, design.md, tasks.md, specs/** (deltas)
│   └── archive/    #   changes terminés, datés
├── schemas/        # schémas de workflow personnalisés (facultatif)
└── compagnon.lock      # sources git héritées et épinglées (le cas échéant)
```

Vous pouvez lancer chaque commande Compagnon depuis n'importe quel dossier du
projet. Compagnon remonte de dossier en dossier jusqu'à trouver `_compagnon/`.

## Capacité

Une [capacité](glossary.md#capacité) est une chose que fait le système et que
l'on peut constater de l'extérieur. Par exemple : la connexion des
utilisateurs, le réglage du thème ou la commande `validate`.

Chaque capacité porte un nom fait de parties séparées par des barres
obliques, comme `ui/theme` ou `identity/user-auth`. Chaque partie est écrite
en [kebab-case](glossary.md#kebab-case) : des mots en minuscules reliés par
des traits d'union. Ce nom est aussi celui du dossier où est rangée la
spécification de la capacité.

Regroupez vos capacités selon l'organisation de votre métier, et non selon
celle de votre code.

## Spécification

Chaque capacité a une spécification, rangée dans
`_compagnon/specs/<capability>/spec.md`. C'est la référence du projet pour ce
que fait cette capacité aujourd'hui. Elle comporte deux sections :

- `## Purpose` : une ou deux phrases sur la raison d'être de la capacité ;
- `## Requirements` : les règles que suit la capacité.

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

Chaque exigence est une règle ferme : elle emploie `SHALL` ou `MUST` (`DOIT`
ou `DOIVENT` dans une spécification rédigée en français). Chaque exigence
comporte au moins un scénario : un court exemple de ce que l'on peut voir se
produire. Dans l'exemple ci-dessus, quand l'ordinateur est en mode sombre,
l'application affiche le thème sombre.

Une spécification décrit ce que fait le système, jamais la façon dont le code
le fait. Un test simple : si le code peut évoluer sans que le comportement
évolue, la spécification ne doit pas évoluer non plus.

Vous ne modifiez jamais directement les spécifications du projet. Elles
n'évoluent que lorsqu'une modification est synchronisée ou archivée. Ainsi,
chaque retouche d'une spécification arrive avec la modification qui
l'explique, et son historique reste lisible.

## Modification

Une modification est un travail sur le système, rangé dans son propre dossier
`_compagnon/changes/<name>/`. Elle contient :

- une proposition : pourquoi ce travail est nécessaire et ce qu'il change ;
- les retouches prévues des spécifications, appelées
  [deltas](glossary.md#delta) ;
- une conception : comment le travail sera fait ;
- une liste de tâches ;
- un fichier `change.yaml`, qui contient les informations de la modification
  (ses métadonnées).

Ces documents sont les [artefacts](glossary.md#artefact) de la modification.
Lesquels elle comporte, c'est son [schéma](glossary.md#schéma) qui le fixe :
voir [Schémas personnalisés](guides/custom-schemas.md). Le schéma par défaut,
`spec-driven`, fixe le [déroulé](glossary.md#déroulé), c'est-à-dire les
étapes de la demande jusqu'à l'archive, décrit dans
[Le déroulé](workflow.md).

Une modification est **active** tant que son dossier se trouve dans
`_compagnon/changes/<name>/`. L'archivage déplace le dossier dans
`_compagnon/changes/archive/<YYYY-MM-DD>-<name>/`. Elle y reste comme la
trace du pourquoi et du comment de l'évolution des spécifications.

## Delta

Un delta est la partie d'une modification qui retouche une spécification. Il
se trouve dans `_compagnon/changes/<name>/specs/<capability>/spec.md`, sous
le même nom de capacité que la spécification du projet qu'il retouche. Il
contient au plus quatre sortes de retouches, chacune dans sa propre section :

| Section | Ce qu'elle fait lors de la fusion dans la spécification du projet |
|---|---|
| `## ADDED Requirements` | Ajoute les exigences à la fin de la spécification |
| `## MODIFIED Requirements` | Remplace sur place l'exigence de même nom |
| `## REMOVED Requirements` | Supprime l'exigence, en donnant une **Reason** (raison) et une **Migration** (que faire à la place) |
| `## RENAMED Requirements` | Change le titre d'une exigence sans toucher à son texte |

Voici un delta qui modifie une exigence et en renomme une autre :

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

Attention avec `MODIFIED` : ce bloc remplace l'exigence tout entière. Copiez
d'abord l'exigence complète depuis la spécification du projet, scénarios
compris, puis modifiez la copie. Tout ce que vous omettez est perdu lors de
la fusion. Si vous ajoutez seulement un comportement, sans modifier
l'existant, utilisez `ADDED`.

La fusion ne touche que les exigences nommées dans le delta. Tout le reste de
la spécification du projet reste tel quel : autres exigences, sections
libres, commentaires.

Le delta d'une **nouvelle** capacité commence par sa propre section
`## Purpose`. Sa fusion crée la spécification de la capacité. Supprimer
entièrement une capacité, avec un delta `REMOVED` qui vide une
spécification, exige `retire_capabilities: true` dans `change.yaml`. Cette
sécurité existe parce que seul git, l'historique des versions du projet,
peut annuler la suppression.

La syntaxe exacte figure dans [Formats de fichiers](reference/file-formats.md).

## Décision

Une décision, ou ADR (*architecture decision record*), est un fichier
`_compagnon/decisions/NNNN-<slug>.md`. Elle consigne un choix sur la façon
dont le système est construit : ce qui y a mené (le contexte), le choix
lui-même, ses conséquences et les autres options envisagées.

```bash
compagnon decision new "Use PostgreSQL for persistence"
compagnon decision accept 0001
```

Une décision a l'un des cinq statuts suivants : `accepted`, `superseded`,
`proposed`, `deprecated` ou `rejected`.

Toutes les commandes qui créent une décision la créent `proposed` :
`compagnon decision new`, [`supersede`](#remplacement),
[`deviate`](#écart) et [`promote`](#promotion). Tant qu'elle est `proposed`,
vous pouvez la rédiger et la reprendre librement. Une fois son texte
définitif, `compagnon decision accept` la passe à `accepted`.

L'acceptation est la seule étape qui fait entrer une décision en vigueur.
Une décision `proposed` ne remplace encore aucune autre décision et ne
s'en écarte pas encore. Une décision est **en vigueur** lorsqu'elle est
`accepted` et qu'aucune autre ne la remplace.

L'agent reçoit les décisions en vigueur chaque fois qu'il rédige un
`design.md`. Ainsi, les conceptions les respectent au lieu de les remettre
en débat.

Chaque décision possède un identifiant court, comme `0007`, et un
identifiant complet qui dit aussi d'où elle vient : de votre projet, ou d'un
autre dépôt dont il hérite.

<details>
<summary>Sous le capot : l'identifiant complet d'une décision</summary>

L'identifiant qualifié indique la provenance de la décision : `project/0007`
pour une décision du projet lui-même, `path:~/shared/0100` ou
`git:git@github.com:acme/shared.git/0100` pour une décision héritée d'un
autre dépôt.

</details>

## Sceau

Une décision acceptée ne doit plus être modifiée. Compagnon le vérifie grâce
à un sceau : une empreinte du texte de la décision, enregistrée au moment où
elle est acceptée. Si quelqu'un modifie le texte ensuite, `compagnon validate`
voit que l'empreinte ne correspond plus et le signale. Il avertit aussi
lorsqu'une décision acceptée n'a pas de sceau. Une décision `proposed` n'est
pas scellée.

Le sceau ne couvre pas le [frontmatter](glossary.md#frontmatter) de la
décision, le court en-tête en haut du fichier qui contient son statut. Le
statut peut donc passer de `accepted` à `superseded` sans briser le sceau.

> **Remarque**
> Scellez une décision lorsque son texte est définitif : rédigez d'abord la
> décision `proposed`, puis lancez `compagnon decision accept <ID>`, qui change
> le statut et enregistre le sceau en une seule étape. Si vous avez modifié
> volontairement une décision scellée, `compagnon decision seal <ID> --force`
> enregistre le nouveau texte ; pour changer ce que dit une décision
> acceptée, [remplacez-la](#remplacement) plutôt.

<details>
<summary>Sous le capot : comment le sceau est enregistré et vérifié</summary>

Lorsqu'une décision devient acceptée par une commande `compagnon decision` —
`accept`, ou `new --status accepted` — Compagnon enregistre une empreinte
SHA-256 du contenu de la décision dans `_compagnon/decisions/seal.yaml`.
`compagnon decision seal` fait de même pour une décision que vous avez
acceptée à la main. `compagnon validate` compare le contenu de chaque décision
locale `accepted` ou `superseded` à son empreinte, et signale une
modification par l'erreur `decision_seal_mismatch` et un sceau manquant par
l'avertissement `decision_unsealed`.

</details>

## Remplacement

Pour changer une décision, ne la modifiez pas : remplacez-la par une
nouvelle. La commande s'appelle `supersede` (remplacer).

```bash
compagnon decision supersede 0001 "Use SQLite for persistence"
```

Compagnon crée une nouvelle décision `proposed` dont le frontmatter nomme
celle qu'elle remplace : `supersedes: ["0001"]`. Il ne touche pas à `0001`,
qui reste `accepted`, et en vigueur, pendant que vous rédigez la nouvelle.
Quand vous acceptez la nouvelle décision, `0001` est marquée `superseded`
dans la même étape :

```bash
compagnon decision accept 0002
```

Le texte de l'ancienne décision reste strictement identique. Son sceau reste
donc valide, et l'historique du raisonnement est préservé.

Seule une décision `accepted` peut être remplacée. Si une autre décision a
remplacé `0001` entre-temps, `compagnon decision accept` refuse, avec
l'erreur `predecessor_not_accepted`, et n'écrit rien.

## Écart

Certaines décisions viennent d'un autre dépôt : elles sont
[héritées](#source-héritée). Vous ne pouvez pas en remplacer une depuis votre
projet, puisque vous ne pouvez pas écrire dans sa source. Consignez plutôt
une exception locale, appelée écart :

```bash
compagnon decision deviate path:~/shared/0100 "Services log in logfmt"
```

Cette commande crée une décision `proposed` dans votre projet. Son champ
`deviates_from` pointe vers la décision héritée.

L'écart entre en vigueur lorsque vous l'acceptez avec
`compagnon decision accept`. Dès lors, la décision héritée apparaît toujours
dans `compagnon decision list`, mais elle ne s'applique plus à votre projet,
et l'agent ne la reçoit plus lorsqu'il rédige des conceptions. Tant que vous
n'avez pas accepté l'écart, la décision héritée reste en vigueur.

## Promotion

Pendant qu'on prépare une modification, sa conception fait souvent un choix
qui mérite de lui survivre. Rédigez ce choix dans `design.md` sous la forme
d'un bloc `### Decision: <title>`, puis promouvez-le en décision à part
entière :

```bash
compagnon decision promote add-audit-log "Append-only audit table"
```

Compagnon crée un ADR `proposed`. Sa section `## Decision` reprend le texte
du bloc, mot pour mot. Dans `design.md`, le texte du bloc est remplacé par
une référence au nouvel ADR.

Relisez ensuite l'ADR et répartissez le texte promu dans ses sections
(contexte, décision, conséquences). Enfin, acceptez-le avec
`compagnon decision accept`, qui le scelle. Idéalement, faites-le avant
d'archiver la modification.

## Source héritée

Un projet peut reprendre le contenu d'autres dépôts Compagnon, sans pouvoir
le modifier. Une source peut être :

- un dossier partagé sur votre ordinateur (`path:`) ;
- un dépôt git (`git:`), épinglé sur un enregistrement précis (commit).

Chaque source apporte son `context:`, ses `rules:` et ses décisions. Rien de
ce qui s'exécute comme un programme n'est jamais hérité. Rien n'est lu non
plus depuis une branche qui continue de bouger : les sources git sont
épinglées dans `_compagnon/compagnon.lock`.
Voir [Sources héritées](guides/inherited-sources.md).

## Schéma

Le schéma définit les artefacts que comporte une modification, ce que chacun
doit contenir, ainsi que les instructions et les modèles que suit l'agent
pour les rédiger. Compagnon fournit un schéma, `spec-driven`. Un projet peut
définir les siens dans `_compagnon/schemas/`. Voir
[Schémas personnalisés](guides/custom-schemas.md).

Dans `spec-driven`, chaque instruction commence par nommer le rôle qui porte
l'artefact, et ses critères « Terminé quand » (*Done when*), qui disent
quand l'artefact est complet :

- le responsable produit (product owner), pour la proposition (pourquoi et
  quoi) ;
- l'analyste QA, pour les spécifications (le comportement que l'on peut
  observer) ;
- l'architecte, pour la conception (comment) ;
- le référent technique (tech lead), pour les tâches (leur ordre, et comment
  vérifier chacune).
