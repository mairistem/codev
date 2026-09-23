# Design : `/codev-onboard`

## Contexte

Voir `proposal.md`. Un ajout de skill de plus dans le catalogue, sur le
même pattern que les six existantes — mais avec deux spécificités : un
rôle strictement informatif (lecture seule) et une place dans le
catalogue par défaut.

## Objectifs / Hors objectifs

Ce design cadre le contenu de la skill, ses `allowed-tools`, sa place
dans `DEFAULT_WORKFLOWS`, et les deux tests d'invariant. Il ne cadre
pas un tutoriel interactif ni une auto-détection MCP.

## Décisions

### Décision : `onboard` entre dans `DEFAULT_WORKFLOWS`

C'est la seule skill dont le rôle est de **s'expliquer elle-même** et
de guider un utilisateur qui n'a rien demandé. La cacher derrière un
opt-in serait absurde : celui qui aurait besoin de la découvrir ne
saurait pas l'activer.

**Alternative écartée** : garder `DEFAULT_WORKFLOWS = ["propose",
"explore"]` et laisser `onboard` en opt-in. Rejeté — casse le
principe même de la skill. Un utilisateur nouveau ne pense pas à
éditer `_codev/config.yaml` avant d'invoquer une skill.

### Décision : `allowed-tools = Bash(codev:*), Read, Glob`

Comparaison avec les six autres :

| Skill | `Bash(codev:*)` | `Read` | `Glob` | `Grep` | `Write` | `Edit` | `Bash` général |
|---|---|---|---|---|---|---|---|
| `propose` | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ |
| `explore` | ✓ | ✓ | ✓ | ✓ | ✗ | ✗ | ✗ |
| `apply` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `sync` | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ |
| `archive` | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ | ✗ |
| `update` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✗ |
| **`onboard`** | ✓ | ✓ | ✓ | ✗ | ✗ | ✗ | ✗ |

`Read` + `Glob` sans `Grep` : la skill ne cherche pas de motif dans le
code — elle regarde des chemins bien connus (`_codev/config.yaml`,
`_codev/decisions/`, listing des changes actifs). Pas besoin de
grep. **La règle « seule `apply` a le `Bash` général »** est
préservée.

### Décision : la skill lit la sortie **humaine** du CLI

`codev list`, `codev list --specs`, `codev status <change>` — sans
`--json`. Cohérent avec `apply` et `update` : ces skills sont
conversationnelles, elles ne consomment pas de contrat structuré. La
sortie humaine est plus courte, plus lisible pour l'agent, et
n'introduit pas de dépendance à un format versionné.

**Alternative écartée** : lire le JSON pour être robuste. Utile si un
jour la sortie humaine changeait sans crier gare — mais elle est
stable de fait, et les autres skills « guides » (`apply`, `update`) ne
consomment pas de JSON non plus.

### Décision : la skill **ne lance jamais** `codev init`

Un utilisateur qui découvre codev sur un dépôt non initialisé pourrait
attendre que la skill le fasse pour lui. Elle **refuse** : `init` est
une écriture sur disque, non triviale (scaffolding complet), et
demande une intention explicite de l'utilisateur. La skill affiche
donc `codev init` comme une **suggestion**, pas une action.

**Alignement** avec la décision de séparer geste et action — même
philosophie que `sync` qui invite à archiver mais n'archive pas.

### Décision : la skill s'accommode d'un dépôt non initialisé

Un `codev list` sur un dossier sans `_codev/` remonte une erreur avec
un code stable connu (`no_codev_root`). La skill intercepte ce cas
comme une **information** — pas une erreur — et bascule sur la branche
« suggère `codev init` ». Le rendu final reste utile.

### Décision : la recommandation d'action dépend d'un arbre de cas simple

Cinq branches, mutuellement exclusives, résolues dans l'ordre :

```
sans _codev/           → codev init
sans change actif      → /codev-propose <idée>
1 change, planif OK    → /codev-apply <nom>
1 change, planif KO    → /codev-propose <nom> (poursuivre)
≥ 2 changes actifs     → lister, laisser l'utilisateur choisir
```

Cet arbre vit dans le body markdown de la skill, pas dans du code
Rust. C'est l'agent qui l'exécute — la skill dit **quoi lire** et
**quoi recommander en fonction**, l'agent regarde, choisit, répond.

## Risques et compromis

- **La skill dit « lance `codev init` » mais l'utilisateur n'a pas
  installé le binaire.** → **Compromis assumé** : sans `codev` dans
  le PATH, la skill n'aurait pas pu être installée par un `codev
  update`. Cas de bord improbable, non traité.
- **La skill devient dense quand le projet a beaucoup de specs et
  changes.** → **Atténuation** : le rendu ne liste **pas** chaque
  spec ni chaque décision individuellement — il donne des **comptes**
  (« 5 specs », « 6 décisions »). L'utilisateur qui veut le détail
  lance `/codev-explore` ou les commandes `codev list --specs` /
  `codev decision list`.
- **Une évolution future du catalogue casse le contrat d'invariant
  « le body cite les trois blocs ».** → **Traité** par un test dédié
  `onboard_cite_ses_trois_blocs` qui vérifie la présence des mots-clés
  attendus (« description », « état », « action » — à ajuster selon la
  rédaction finale).

## Plan de migration

Aucune. Les projets qui déclarent explicitement `workflows:` dans leur
`config.yaml` gardent leur liste ; ils ajoutent `onboard` quand ils
veulent. Les projets neufs (sans clef `workflows:`) obtiennent la
skill au premier `codev init` / `codev update`.
