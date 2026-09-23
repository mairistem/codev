---
name: codev-onboard
description: "Présenter codev à un utilisateur qui le découvre : ce que fait l'outil, l'état actuel du projet, et la prochaine action recommandée. Strictement en lecture — ne modifie ni ne crée rien."
allowed-tools: "Bash(codev:*), Read, Glob"
license: MIT
metadata:
  generator: codev
  version: "0.1.0"
---

Présenter codev à un utilisateur qui le découvre : ce que fait l'outil,
l'état actuel du projet, et la prochaine action recommandée.

**Frontière stricte lecture.** Cette skill **ne modifie rien**. Elle ne
crée pas de change, ne lance pas `codev init` à la place de
l'utilisateur, ne coche pas de tâche. Elle **guide** — l'utilisateur
agit.

---

## Entrée

Rien. La skill s'invoque sans argument. Si l'utilisateur passe une
question, elle informe sur codev, elle ne l'implémente pas.

## Étapes

### 1. Décrire codev (bloc 1 — « codev, c'est »)

Trois phrases exactement, à afficher au format markdown :

> **codev** est un outil de planification versionnée pour un projet
> logiciel : chaque évolution passe par un cycle **propose → apply →
> archive** documenté dans `_codev/`. Chaque étape est pilotée par une
> skill Claude Code (`/codev-<étape>`), et le CLI `codev` fait le
> travail atomique en dessous. Ta planification vit dans le dépôt,
> aux côtés du code.

### 2. Lire l'état du projet (bloc 2 — « ici, tu as »)

**Vérifie d'abord si le dépôt est initialisé** :

```bash
codev list
```

Cette commande liste les changes actifs. Trois cas :

- **Succès** → le dépôt est initialisé. Continue.
- **Échec avec code `no_codev_root`** → aucun `_codev/` sous le
  dossier courant. Bascule directement à l'étape 3, branche « `codev
  init` ».
- **Autre échec** → relaie le message tel quel et arrête-toi.

Si le dépôt est initialisé, complète l'état :

```bash
codev list --specs           # capacités déjà spécifiées
codev decision list          # décisions locales et héritées
ls _codev/changes/archive/   # nombre de changes archivés (facultatif)
```

Compte les entrées de la forme `<date>-<nom>/` dans
`_codev/changes/archive/` (ignore `.gitkeep` et les fichiers cachés).

Affiche un résumé compact — **compte, ne liste pas exhaustivement** :

> Ici, tu as :
> - N spec(s) principale(s) : `<liste des ids>` (jusqu'à 5 ; sinon
>   « et K autres »)
> - M décision(s) locale(s) en vigueur, P héritée(s)
> - Q change(s) actif(s) : `<liste des noms>`
> - R change(s) archivé(s) *(**cette ligne uniquement si R > 0**, pour
>   ne pas polluer un projet neuf)*

Un projet fraîchement initialisé (aucune spec, aucun change, aucune
décision) est un cas normal — dis-le : « projet fraîchement
initialisé, prêt pour ton premier change ».

### 3. Recommander la prochaine action (bloc 3 — « la suite »)

Résolution ordonnée, du plus contraignant au plus général. **Un seul
cas s'applique** :

| État | Recommandation |
|---|---|
| `_codev/` absent | `codev init` |
| Aucun change actif | **Lire `README.md`** pour prendre le pouls, puis `/codev-propose <une-idée>` ; `/codev-explore <sujet>` en alternative |
| Un seul change actif, planification incomplète | `/codev-propose <ce-nom>` pour poursuivre |
| Un seul change actif, planification complète | `/codev-apply <ce-nom>` |
| Plusieurs changes actifs | Les lister ; laisser l'utilisateur choisir la skill |

Comment déterminer si la planification d'un change est complète :

```bash
codev status --change "<nom>"
```

La dernière ligne dit « Planification : N/N artefacts » et « La
planification est complète. » quand tout est prêt.

**Rends la suggestion actionnable** : cite la commande exacte à taper,
pas juste « lance `codev-propose` ». Exemple utile sur un projet sans
change actif :

> **La suite** : commence par lire `README.md` pour prendre le pouls
> du projet. Puis, quand une idée émerge, tape
> `/codev-propose <une-idée>`. Alternative si tu as une question mais
> pas encore d'idée d'action : `/codev-explore <sujet>`.

Et sur un change actif :

> **La suite** : tu peux taper `/codev-propose add-user-auth` pour
> planifier ton premier change.

Un cas de non-lieu : si l'utilisateur a déjà tapé `/codev-onboard`
pour la Nième fois, l'état est stable, tu redis la même chose sans
t'en excuser — le rôle de la skill est de rester **prévisible**.

## Sortie

Les **trois blocs** dans l'ordre : description, état, suite. Séparés
par une ligne blanche. Pas d'introduction (« Voici… »), pas de
conclusion (« J'espère que… »). Le lecteur veut la carte du terrain,
pas un tour guidé.

## Garde-fous

- **Aucune écriture** — ni fichier, ni skill, ni change, ni décision.
  `allowed-tools` ne contient que `Bash(codev:*), Read, Glob`.
- **Pas de `codev init` lancé** — si le dépôt n'est pas initialisé,
  affiche la commande, laisse l'utilisateur la taper.
- **Pas de sur-détail** — nombres, pas listes exhaustives. Un
  utilisateur qui veut le détail lance `/codev-explore` ou les
  commandes `codev decision list` / `codev list --specs`.
- **Pas de suggestion d'une skill absente** — ne recommande
  `/codev-<truc>` que si le workflow existe dans le catalogue
  installé. En pratique, cette skill est toujours livrée avec les
  workflows du catalogue par défaut ; les opt-in (`apply`, `sync`,
  `archive`, `update`) peuvent ne pas être installés. Si tu recommandes
  `/codev-apply` alors qu'il n'est pas installé, l'utilisateur ne
  trouvera pas la skill — vérifie avant en listant les skills de
  `.claude/skills/`.