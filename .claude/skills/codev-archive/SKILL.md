---
name: codev-archive
description: "Clore un change codev : fusionner ses deltas dans les specs principales et déplacer le dossier vers l'archive datée. Refuse d'agir si la validation remonte des erreurs, et renvoie alors vers `codev validate` pour le détail."
allowed-tools: "Bash(codev:*), Read"
license: MIT
metadata:
  generator: codev
  version: "0.1.0"
---

Clore un change codev : fusionner ses deltas dans les specs principales et
déplacer le dossier vers `_codev/changes/archive/<date>-<nom>/`.

**Quand l'utiliser.** Après un `codev-apply` réussi, quand toutes les tâches
de `tasks.md` sont cochées et que le change est prêt à être classé.

**Refus strict en cas d'erreur de validation.** Le CLI fait un pré-flight
`validate` interne ; s'il remonte une erreur, la skill n'insiste pas et
renvoie vers `codev validate` pour le détail.

---

## Entrée

Un nom de change en argument, ou rien (résolution implicite s'il n'y en a
qu'un seul actif).

## Étapes

### 1. Résoudre le change et vérifier la planification

Sans nom explicite :

```bash
codev list
```

- Un seul change actif → c'est celui-là.
- Plusieurs → demande à l'utilisateur, en listant les noms.
- Aucun → dis-le et arrête-toi.

Puis :

```bash
codev status --change "<nom>" --json
```

Si `isPlanningComplete` est `false`, arrête : la planification n'est pas
prête. Nomme ce qui manque et propose `/codev-propose` ; ne lance pas
`archive`.

### 2. Lancer l'archive en mode JSON

```bash
codev archive --change "<nom>" --json
```

En cas de **succès** (exit 0), le JSON reçu est un `ArchiveReportV1` —
contrat public, versionné. Champs à utiliser :

- `changeName` — pour confirmer sur quoi on a agi ;
- `created[]` — chemins des specs principales qui viennent d'être créées ;
- `updated[]` — chemins des specs principales qui viennent d'être modifiées ;
- `unchanged[]` — chemins des specs déjà à jour au moment de la fusion ;
- `movedTo` — chemin d'archive datée du dossier de change ;
- `status[]` — vide.

En cas d'**exit non nul**, lis `status[0].code` :

- Si `code == "validation_failed"` → réponds **exactement** :
  > Le change a des erreurs. Lance `codev validate "<nom>"` pour voir le
  > détail.
  Rien de plus. Ne retente pas. Ne devine pas. Ne cite pas le message
  humain (qui peut être reformulé).
- Pour **tout autre code** → relaye `status[0].message` tel quel, et
  arrête-toi. La skill n'interprète pas.

### 3. Rendre compte à l'utilisateur (succès)

Résumé attendu, une ligne :

> ✓ Archive de « `<changeName>` » — `N` spec(s) créée(s), `M` mise(s) à
> jour, `K` inchangée(s).

Puis les listes par catégorie si non vides (comme `sync`).

Enfin, sur sa propre ligne :

> Déplacé vers : `<movedTo>`

## Sortie

Le rendu de succès de l'étape 3, ou le refus court en cas d'erreur de
validation, ou le message brut du CLI en cas d'autre erreur.

## Garde-fous

- **N'écris rien toi-même** — tout passe par `codev archive`. La skill ne
  modifie ni les specs ni les dossiers en direct.
- **Ne contourne pas un refus de validation** — un `validation_failed`
  arrête la skill. Corriger, c'est le travail de l'utilisateur guidé par
  `codev validate`, pas de la skill.
- **Ne parse pas d'autre JSON que celui de `archive`** — la skill ne
  connaît la forme d'aucun autre contrat.
- **Ne réinvente pas les messages du CLI** — pour tout code d'erreur autre
  que `validation_failed`, le `message` du JSON est relayé tel quel.