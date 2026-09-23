# Design : `/codev-update`

## Contexte

Voir `proposal.md` pour la motivation. Le pattern est celui déjà éprouvé
par les cinq skills existantes : une entrée dans le `CATALOG` et un
fichier markdown sous `assets/workflows/`. La spécificité tient à ce que
la skill lit **et** écrit sous `_codev/changes/`, comme `apply` — mais
sans jamais toucher au code du projet.

## Objectifs / Hors objectifs

Ce design cadre le contenu de la skill, ses `allowed-tools`, et un test
d'invariant qui verrouille sa frontière. Il ne cadre ni l'édition en
batch, ni un mode `--dry-run`, ni l'ajout à `DEFAULT_WORKFLOWS`.

## Décisions

### Décision : `allowed-tools` = `Bash(codev:*), Read, Write, Edit, Glob, Grep`

Trois outils d'écriture : `Write` pour réécrire un artefact en entier,
`Edit` pour une modification ponctuelle, et `Bash(codev:*)` pour relancer
`codev validate` en garde-fou final. Pas de `Bash` général — la skill
n'exécute jamais de commande de vérification côté projet.

**Comparaison** :

| Skill | `Bash(codev:*)` | Édition | `Bash` général |
|---|---|---|---|
| `propose` | ✓ | ✓ | ✗ |
| `explore` | ✓ | ✗ | ✗ |
| `apply` | ✓ | ✓ | ✓ |
| `sync` | ✓ | ✗ | ✗ |
| `archive` | ✓ | ✗ | ✗ |
| **`update`** | ✓ | ✓ | ✗ |

La règle « seule `apply` a le `Bash` général » — verrouillée par
`workflows::apply_est_dans_le_catalogue_et_a_les_bons_outils` — reste
préservée. Alignement direct avec la décision
[0004](../../decisions/0004-une-seule-identite-skill-et-commande.md) sur
l'identité unique skill/commande : chaque skill fait une chose distincte,
avec un `allowed-tools` distinct.

### Décision : la skill lit et écrit du markdown à la main

Elle n'invoque **aucune commande CLI d'édition** — il n'en existe pas et
il n'y en aura pas tant que le format markdown reste la source de vérité,
comme la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) le
demande (le cœur ne touche pas au disque). La skill utilise `Edit`
pour une modification ciblée et `Write` pour une réécriture complète —
choix laissé à son jugement selon l'ampleur de la révision.

**Alternative écartée** : une commande `codev update <artefact>
<révision>` qui produirait un plan d'édition. Utile un jour, mais demande
un parseur markdown plus intelligent que celui livré par
`parse-specs-and-deltas` (qui repère les blocs mais ne les remplace pas
sémantiquement). Reportable au premier besoin réel.

### Décision : ripple signalé, jamais appliqué en cascade automatique

Un `Write` sur `proposal.md` qui retire une capacité laisse
`specs/<capa>/spec.md` orphelin. La skill le **détecte** (parcours simple
des dossiers `specs/` avant et après), le **signale** à l'utilisateur, et
**propose** la suite (supprimer le fichier ou lancer un
`/codev-update specs …` séparé) — mais **n'agit pas** sans confirmation.
Idem quand la révision d'un `design.md` invalide une tâche : la skill
nomme la tâche et propose l'action, l'utilisateur tranche.

**Rationale** : une skill qui applique en cascade des modifications que
l'utilisateur n'a pas explicitement demandées finit par produire ce que
l'utilisateur n'attendait pas — pattern paternaliste qu'on refuse depuis
`sync` (qui, elle, invite à archiver mais n'archive pas).

### Décision : `codev validate` relancé en fin, systématiquement

Après la dernière écriture, la skill lance `codev validate <change>` et
affiche le rapport tel quel. C'est le seul garde-fou automatique — il
attrape ce que la skill n'a pas vu.

**Alternative écartée** : ne relancer `validate` que sur demande. Trop de
place laissée au « j'ai oublié ». Le coût de la commande est négligeable.

### Décision : la skill refuse un change archivé, matériellement

Elle liste `codev list` (qui ne montre que les actifs) et refuse tout
nom qui n'y apparaît pas — même s'il existe sous `changes/archive/`. Un
change archivé est **de l'histoire** ; le réécrire depuis une skill
compromettrait la trace que l'archivage a précisément pour but de
préserver.

## Risques et compromis

- **Une révision mal formulée casse la cohérence** entre proposal et
  design sans que la skill le voie — surtout si le changement de sens est
  subtil (« reformuler » vs « restreindre »). → **Atténuation** : la
  skill lit **d'abord** les dépendances du change avant d'écrire, et
  `codev validate` en fin de traitement remonte les défauts structurels.
  Le vrai filet reste l'utilisateur qui relit le diff.
- **Un utilisateur pourrait faire dériver une décision d'architecture
  par une révision de design.** → **Compromis assumé** : le design cite
  les décisions en vigueur (injection déjà en place), l'agent est censé
  respecter cette contrainte. La skill ne fait rien pour l'empêcher — le
  contrat social entre l'utilisateur et l'agent le fait.
- **Le ripple sur les tests est ignoré** — une révision qui remplacerait
  une exigence par une autre dans `specs/` pourrait rendre caduque une
  ligne de `tasks.md` qui cite le nom du scénario. → **Compromis
  assumé** : la skill signale que `tasks.md` cite un nom disparu et
  propose l'appel séparé.

## Plan de migration

Sans objet — nouvelle skill. Un projet qui a déjà `workflows: [propose,
explore, apply, sync, archive]` dans son `config.yaml` doit y ajouter
`update` et relancer `codev update` pour l'installer.
