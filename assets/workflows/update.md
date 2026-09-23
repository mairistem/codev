Réviser un artefact de planification déjà écrit d'un change codev actif —
proposal, specs, design ou tasks — en préservant la cohérence avec les
autres artefacts.

**Frontière stricte.** Cette skill lit et écrit **uniquement** des
fichiers sous `_codev/changes/<nom>/`. Elle **ne modifie pas** de code du
projet — ça, c'est `/codev-apply` après révision. Elle **ne crée pas**
d'artefact manquant — ça, c'est `/codev-propose`. Elle **ne touche pas**
à un change déjà archivé — un archivé est de l'histoire.

---

## Entrée

Deux morceaux, dans cet ordre :

1. L'identifiant de l'artefact à réviser — `proposal`, `specs`, `design`
   ou `tasks`. Si l'utilisateur ne le nomme pas, demande explicitement
   lequel — ne devine pas.
2. Une description libre de la révision demandée.

Optionnel : `--change <nom>` si plusieurs changes sont actifs.

## Étapes

### 1. Résoudre le change et vérifier son état

```bash
codev list
```

- Un seul change actif → c'est celui-là.
- Plusieurs → si l'utilisateur n'a pas nommé, demande.
- Aucun → dis-le et arrête-toi.

Si le nom donné ne figure pas dans `codev list` — parce qu'il est
archivé, ou qu'il n'existe pas — **refuse**. Rappelle qu'un archivé est
figé ; corriger demande de le dé-archiver à la main.

Puis :

```bash
codev status --change "<nom>" --json
```

Repère l'artefact demandé dans `artifacts[]` :

- statut `done` → OK, on peut réviser.
- statut `ready` ou `blocked` → l'artefact n'existe pas encore.
  **Refuse** et invite explicitement à `/codev-propose` (ou
  `/codev-continue` en profil étendu) pour le créer.
- statut `skipped` → **refuse** et explique que cet artefact est
  neutralisé par `skip_specs` dans le `change.yaml`.

### 2. Lire l'existant

Lis, depuis le disque (jamais depuis la conversation) :

- l'artefact à réviser lui-même ;
- les autres artefacts du change qui pourraient être impactés.

Cette lecture sert au repérage du ripple à l'étape 4 — c'est pour ça
qu'on la fait **avant** d'écrire, pas après.

### 3. Appliquer la révision

Deux formes selon l'ampleur :

- **Modification ciblée** — un paragraphe à ajuster, une décision à
  remplacer, une tâche à reformuler → `Edit` avec un `old_string` précis.
- **Réécriture complète** — un artefact qui doit être largement refait →
  `Write`, mais garde en tête que les autres artefacts vont s'appuyer sur
  sa nouvelle forme.

Reste dans le contrat de l'artefact — sections attendues, format des
scénarios, cases à cocher. Le changement porte sur le **contenu**, pas
sur la structure.

### 4. Détecter et signaler le ripple

Après l'écriture, compare l'état du change avec ce qui a changé :

- **`proposal` révisé** :
  - une capacité listée dans les « Nouvelles capacités » ou « Capacités
    modifiées » qui disparaît → nomme le fichier
    `specs/<capa>/spec.md` qui devient orphelin ;
  - une nouvelle capacité qui apparaît → nomme le fichier `specs/<capa>/`
    qui manque désormais.
- **`specs` révisé** :
  - une exigence supprimée qui était citée par une tâche → nomme la
    tâche concernée dans `tasks.md` ;
  - un nouveau nom de scénario différent — un `tasks.md` qui citait
    l'ancien nom est signalé.
- **`design` révisé** :
  - une décision citée qui disparaît d'un côté et une contrainte de
    l'autre → signale, mais laisse l'utilisateur trancher (`codev
    decision list` peut aider).
- **`tasks` révisé** :
  - une tâche qui contredit une exigence de `specs/` → nomme
    l'exigence en cause.

**Signale**, **propose** l'action suivante (souvent : « lance
`/codev-update specs …` » ou « supprime `_codev/changes/<nom>/specs/<capa>/spec.md` »),
mais **n'agis pas** sans confirmation explicite.

### 5. Garde-fou final — `codev validate`

Quel que soit le ripple, à la fin, lance :

```bash
codev validate "<nom>"
```

Relaye le rapport tel quel. Si `validate` remonte des erreurs, cite les
codes stables ; ne réécris pas les messages humains.

### 6. Résumé final

Une ou deux phrases :

- le ou les fichiers touchés ;
- le verdict de `codev validate` ;
- la prochaine action recommandée — souvent `/codev-apply` (si la
  révision affecte l'implémentation) ou `/codev-archive` (si l'écart
  reconnu est terminé).

## Sortie

Le résumé de l'étape 6, précédé du signalement de ripple s'il y en a un
et de son statut (accepté par l'utilisateur, refusé, reporté à un
`/codev-update` séparé).

## Garde-fous

- **Pas de code** — tous les chemins écrits par cette skill vivent sous
  `_codev/changes/<nom>/`. Refuse si l'utilisateur demande d'ajuster du
  code : c'est `/codev-apply` qui le fait après la révision.
- **Pas de création** — si l'artefact demandé n'existe pas, refuse et
  renvoie vers `/codev-propose`. Ne crée jamais un `proposal.md`,
  `design.md` ou `tasks.md` de zéro depuis cette skill.
- **Pas d'archivé** — un change qui vit sous `changes/archive/` est de
  l'histoire. Refuse et rappelle que le corriger demande de le
  dé-archiver à la main.
- **Pas de cascade** — un ripple détecté est signalé, jamais appliqué
  sans confirmation.
- **Toujours `validate` en fin** — c'est le seul garde-fou automatique.
