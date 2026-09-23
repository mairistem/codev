Faire entrer les deltas d'un change codev dans les specs principales, **sans
déplacer le change**. Le change reste actif à son emplacement.

**Quand l'utiliser.** Quand une capacité nouvelle doit apparaître dans les
specs avant qu'un autre change ne s'y appuie, ou quand tu veux relire le
merge avant d'archiver. Dans le cas normal, `codev-archive` fait déjà le
sync en pré-flight — inutile de `sync` puis `archive` séparément.

---

## Entrée

Un nom de change en argument, ou rien (auquel cas le change est résolu
implicitement s'il n'y en a qu'un seul actif).

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
prête. Nomme les artefacts manquants et propose `/codev-propose` ou
`/codev-continue` (selon ce qui existe déjà). Ne lance pas `sync`.

### 2. Lancer la fusion en mode JSON

```bash
codev sync --change "<nom>" --json
```

Le JSON reçu est un `SyncReportV1` — contrat public, versionné. Champs à
utiliser :

- `changeName` — pour confirmer sur quoi on agit ;
- `created[]` — chemins des specs principales qui viennent d'être créées ;
- `updated[]` — chemins des specs principales qui viennent d'être modifiées ;
- `unchanged[]` — chemins des specs qui étaient déjà à jour ;
- `status[]` — vide en cas de succès.

En cas d'exit non nul, lis `status[0].code` et `status[0].message`, relaye
le message tel quel et arrête-toi.

### 3. Rendre compte à l'utilisateur

Résumé attendu, une ligne :

> ✓ Sync de « `<changeName>` » — `N` spec(s) créée(s), `M` mise(s) à jour,
> `K` inchangée(s).

Puis, si au moins l'une des listes `created` ou `updated` est non vide,
liste-les par catégorie, chacune sur sa propre ligne :

```
Créé(s) :
  <chemin>
Mis à jour :
  <chemin>
```

### 4. Fin — inviter à archive si quelque chose a changé

Si `created` ou `updated` est non vide, ajoute **une seule ligne**, sans
injonction :

> Le change est prêt à être archivé si tu veux clore le cycle.

Si les deux listes sont vides (`unchanged` seulement), ne suggère rien —
un sync no-op n'appelle pas d'archive.

## Sortie

Ce que tu as rendu à l'étape 3 + éventuellement la ligne de l'étape 4.
Rien de plus. La skill s'arrête là ; c'est l'utilisateur qui décide de la
suite (relire, archiver, ou autre).

## Garde-fous

- **Ne déplace jamais le change** — le pouvoir de `codev sync` s'arrête à
  la fusion. Le déplacement est le travail de `codev-archive`.
- **N'archive pas** — si l'utilisateur voulait archiver, il aurait tapé
  `/codev-archive`.
- **Ne parse pas d'autre JSON que celui de `sync`** — la skill ne connaît
  la forme d'aucun autre contrat.
- **Ne masque pas une erreur** — si le CLI retourne un exit non nul,
  relaye ; ne retente pas silencieusement.
