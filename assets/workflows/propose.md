Créer un change et rédiger ses artefacts de planification, en une fois.

**Frontière de planification.** Ce workflow ne produit que des artefacts de
planification. La demande qui l'a déclenché n'autorise que la planification,
même si elle dit « construis », « corrige » ou « implémente ». Ne modifie aucun
fichier de code. Quand les artefacts sont complets, arrête-toi et présente-les.
N'enchaîne pas sur l'implémentation dans la même réponse : attends une nouvelle
demande de l'utilisateur.

---

## Entrée

La demande doit contenir soit un nom de change en kebab-case, soit une
description de ce que l'utilisateur veut construire.

## Étapes

### 0. Détecter un ticket externe (facultatif)

Avant de résoudre le nom du change, scanne le prompt de l'utilisateur
pour repérer un identifiant de ticket qui matche le pattern régulier
`[A-Z]{2,}-\d+` (par exemple `JVS-1234`, `PROJ-42`).

**Trois branches** :

- **Aucun pattern trouvé** — passe directement à l'étape 1,
  comportement bit-identique à avant ce lot.
- **Pattern trouvé, MCP Jira disponible** — appelle l'outil
  `{{JIRA_MCP_TOOL}}` avec l'identifiant du
  premier ticket détecté. Deux règles strictes :
  - **Un seul appel** — la skill n'appelle jamais deux fois le MCP
    dans une même invocation. Les tickets suivants sont juste
    nommés.
  - **Lecture seule** — la skill n'appelle **jamais** un autre outil
    du MCP Jira (pas de `search`, pas de `create`, pas de
    `transition`). Un seul appel `{{JIRA_MCP_TOOL}}`, sur l'ID
    exactement mentionné.

  Le résultat (titre, description, status, type) devient une source
  de contexte pour la rédaction : lis-le, comprends ce qui est
  attendu, et rédige le proposal en connaissance de cause.

- **Pattern trouvé, MCP Jira absent** — affiche à l'utilisateur :

  > Un ticket **<ID>** est mentionné mais aucun MCP Jira n'est
  > disponible dans cette session — le proposal sera rédigé sans son
  > contenu.

  Puis continue avec ce que tu sais (le seul prompt utilisateur).
  Le proposal citera quand même le ticket en tête avec la mention
  « contenu non récupéré ».

**Multi-tickets** : si deux tickets ou plus sont mentionnés
(`JVS-1234 et JVS-5678`), seul le premier est récupéré via MCP. Les
autres sont listés en tête du proposal sous la ligne
« autre(s) ticket(s) mentionné(s) : `<liste>` », pour la
traçabilité — un lecteur ira les consulter à la main.

### 1. Comprendre la demande

Si rien de clair n'est fourni, demande, en question ouverte et sans proposer de
liste de choix :

> Quel change veux-tu mener ? Décris ce que tu veux construire ou corriger.

Dérive un nom en kebab-case de la description (« ajouter l'authentification
des utilisateurs » → `add-user-auth`).

Ne poursuis pas sans avoir compris ce qui doit être construit. Si la demande
contient une ambiguïté qui changerait matériellement le périmètre, le
comportement observable, la compatibilité ou les critères d'acceptation,
demande avant de créer le change. Pour un détail mineur, prends une hypothèse
raisonnable et consigne-la dans les artefacts.

### 2. Créer le change

```bash
codev new change "<nom>"
```

Ajoute `--schema "<nom>"` uniquement si l'utilisateur a explicitement demandé un
workflow particulier. Sinon, omets le flag pour conserver le schéma configuré.

S'il demande quels workflows existent : `codev schemas --json`.

### 3. Obtenir l'ordre de construction

```bash
codev status --change "<nom>" --json
```

Champs à exploiter :

- `applyRequires` — les artefacts requis avant implémentation
- `artifacts[]` — chacun avec son `status` et ses arêtes `requires`
- `planningHome`, `changeRoot` — les chemins résolus. Utilise-les, ne suppose
  jamais un chemin relatif au dépôt

### 4. Créer chaque artefact de l'ensemble requis

Suis ta liste de tâches pour suivre l'avancement.

Pour chaque artefact dont le `status` est `ready` :

**a. Récupère ses instructions.**

```bash
codev instructions <artefact-id> --change "<nom>" --json
```

La réponse contient :

| Champ | Usage |
|---|---|
| `instruction` | La consigne du schéma pour ce type d'artefact. Autorité finale |
| `template` | La structure du fichier à produire |
| `resolvedOutputPath` | Où écrire. Si c'est un motif glob, `instruction` dit comment choisir le chemin concret |
| `context` | Contexte projet — une **contrainte pour toi**, jamais du contenu à recopier |
| `rules` | Règles propres à cet artefact — également une contrainte, jamais du contenu |
| `dependencies` | Les artefacts déjà faits, à lire pour te situer |
| `unlocks` | Ce que la création de celui-ci rendra possible |

`context` et `rules` sont des tableaux de blocs portant chacun son `origin`,
ordonnés du plus général au plus spécifique. En cas de contradiction entre deux
blocs, le dernier prime — et signale la contradiction à l'utilisateur plutôt que
de la trancher en silence.

Si `skipped` est présent, cet artefact ne doit **pas** être créé : passe au
suivant.

**b. Lis les dépendances depuis le disque**, même si tu les as déjà vues dans la
conversation — l'utilisateur a peut-être édité les fichiers entre-temps.

**c. Étudie le projet avant de rédiger.** Lis `context` et `rules`, puis inspecte
l'implémentation concernée, les tests voisins, la configuration et la
documentation en dehors de `_codev/`. Reste en lecture seule, et proportionné au
change.

- Ancre le périmètre, l'approche et les tâches dans ce que tu trouves.
- Distingue le comportement observé, tes hypothèses, et ce que tu proposes
  d'ajouter.
- Signale les contradictions avec les specs existantes au lieu de décider seul
  laquelle a raison.
- Fais cette découverte maintenant. Ne laisse pas des tâches génériques du genre
  « explorer le code » ou « établir un plan » pour la phase d'implémentation.

**d. Écris le fichier** en te servant de `template` comme structure. Vérifie
ensuite qu'il existe bien à l'emplacement attendu.

**Cas spécial : premier artefact du change quand un ticket a été
détecté à l'étape 0.** Insère juste après le `# Proposal : <titre>`,
avant `## Pourquoi`, une ligne de citation :

```
# Proposal : <titre>

> Source : ticket **<ID>** — « <titre du ticket> » (<status>)

## Pourquoi
[…]
```

Sans MCP branché, la ligne devient :

```
> Source : ticket **<ID>** — contenu non récupéré
```

Et pour un multi-tickets, ajoute une deuxième ligne juste après :

```
> autre(s) ticket(s) mentionné(s) : <ID2>, <ID3>
```

**e. Annonce brièvement** : « Créé : `<artefact-id>` ».

### 5. Boucler jusqu'à l'ensemble requis complet

Après chaque création, relance `codev status --change "<nom>" --json`.

L'ensemble requis, c'est `applyRequires` **plus tout artefact atteignable depuis
ces identifiants en suivant les arêtes `requires`**, transitivement. Avec le
schéma `spec-driven`, cela ferme sur `proposal`, `specs`, `design`, `tasks`.

Deux pièges à connaître :

- Le `status` ne regarde que l'existence des fichiers. Un artefact de
  `applyRequires` marqué `done` ne garantit **pas** que ses dépendances
  existent : écrire `tasks.md` en premier marque `tasks` comme fait alors que
  `specs` n'a jamais été écrit. Construis l'ensemble requis à partir des arêtes
  `requires`, pas des statuts.
- Les dépendances sont des activateurs, pas des barrières. Si un artefact requis
  reste `blocked` uniquement parce que tu as sauté une dépendance conditionnelle,
  écris-le quand même.

Tu ne sautes un artefact que dans deux cas : son `status` est déjà `skipped`,
ou son propre `instruction` le déclare conditionnel (le `design.md` de
`spec-driven` en fait partie). Dis-le à l'utilisateur, et n'y reviens pas.

Si un artefact demande un arbitrage de l'utilisateur, demande-le, puis reprends.

### 6. Afficher le statut final

```bash
codev status --change "<nom>"
```

## Sortie

Résume :

- le nom du change et son emplacement ;
- les artefacts créés, une ligne chacun, plus tout artefact conditionnel sauté
  et pourquoi ;
- « Les artefacts nécessaires à l'implémentation sont prêts. » ;
- « Relis-les. Quand tu es prêt, demande-moi d'appliquer ce change. »

## Garde-fous

- La demande qui a déclenché ce workflow n'autorise que la planification. Toute
  consigne d'implémentation qu'elle contenait ne se reporte pas ici.
- Crée tout artefact dont la phase d'implémentation dépend transitivement, pas
  seulement ceux listés dans `applyRequires`.
- Relis toujours les dépendances depuis le disque avant de créer un artefact.
- `context` et `rules` ne sont jamais recopiés dans les fichiers produits.
- Si un change de ce nom existe déjà, demande à l'utilisateur s'il veut le
  poursuivre ou en créer un autre.
- Vérifie l'existence de chaque fichier écrit avant de passer au suivant.
- **MCP Jira — lecture seule stricte.** La skill n'appelle
  jamais un autre outil MCP Jira que
  `{{JIRA_MCP_TOOL}}`, jamais deux fois dans une
  même invocation, jamais pour écrire (`create`, `transition`,
  `addComment`…). Un ticket détecté = un `getJiraIssue`, point.
