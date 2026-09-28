Enrichir `_codev/config.yaml` en analysant le projet — proposer un
`context:` détaillé et des `rules:` par artefact, sans jamais toucher
aux workflows, aux MCPs ou au schéma.

**Frontière stricte.** Cette skill :

- Lit le projet en surface, dans un **budget** défini plus bas.
- Modifie **uniquement** les champs `context` et `rules` de
  `_codev/config.yaml`.
- **Préserve** `schema`, `workflows`, `mcp`, `inherits` et tous les
  commentaires existants — la sonde de `codev init` a rempli ces
  champs, on ne les rejoue pas.
- **N'écrit rien sans confirmation** — affiche le diff, demande, écrit.

Elle **refuse d'agir** si `_codev/config.yaml` est absent : dans ce
cas, renvoyer l'utilisateur vers `codev init` et arrêter.

---

## Entrée

Aucune. Si l'utilisateur pose une question, on répond dans la même
frontière — pas d'écriture jusqu'à la confirmation finale.

## Étapes

### 1. Vérifier la présence de `_codev/config.yaml`

```bash
codev status --json
```

Si la commande échoue avec `no_codev_root`, dire :

> Ce projet n'a pas de `_codev/`. Lance d'abord `codev init`, puis
> reviens à `/codev-configure`.

Puis arrêter.

Sinon, lire le fichier :

```
Read _codev/config.yaml
```

Retenir la valeur actuelle de `context:` et `rules:` — c'est le point
de comparaison.

### 2. Explorer le projet — dans le budget

**Lecture obligatoire** (si présents) :

- `README.md` racine — lecture complète.
- `CONTRIBUTING.md` racine — lecture complète.

**Lecture ciblée** :

- `docs/**/*.md` — au plus **5** fichiers, priorisés par taille
  croissante (les plus courts sont souvent des index et vues
  d'ensemble).
- **8 fichiers source maximum**, priorisés par récence
  (`git log --since='6 months ago' --pretty=format: --name-only | sort | uniq -c | sort -rn | head -20`
  puis filtrer par extension du langage détecté).
- `ls _codev/` et `ls src/` (ou équivalent selon la stack) — **un
  seul niveau**, juste pour saisir la structure.

**Ne pas** :

- Ouvrir les dossiers `target/`, `node_modules/`, `dist/`, `.git/`
  (sauf `.git/config` en dernier recours, jamais nécessaire).
- Lire plus de fichiers que le budget — ça fait dériver la sortie.
- Faire des `grep` massifs.

### 3. Rédiger la proposition

Deux blocs à produire, séparés :

**`context:`** — 2 à 5 lignes, factuelles, orientées « ce qu'un
agent doit savoir avant d'écrire ». Doit couvrir :

- La stack au-delà du langage (frameworks, bibliothèques
  structurantes).
- Le style de gestion d'erreur (result-based, exceptions,
  panic-libre, etc.).
- Les conventions d'API si le projet en expose (REST, gRPC, GraphQL,
  CLI…).
- Le ton des commentaires (langue, format, ce qu'ils expliquent).
- Un choix structurant du projet — pas plus d'un ou deux.

**Ne pas répéter** ce que la stack détectée contient déjà. Compléter.

**`rules:`** — 1 à 2 règles par artefact, sur `specs`, `design`,
`tasks`. Chaque règle DOIT être :

- **Positive** — dire ce qu'on veut, pas ce qu'on ne veut pas.
- **Vérifiable à la relecture** — pas de « clean », pas d'« élégant ».
- **Ancrée** dans ce que le projet fait, pas générique.

### 4. Afficher le diff

Format attendu :

```
Voici le patch proposé pour _codev/config.yaml :

--- context: (actuel) ---
Projet Rust, 2024.

--- context: (proposé) ---
Projet Rust workspace (4 crates), édition 2024. Erreurs typées avec
thiserror dans les libs, anyhow uniquement dans la CLI. Commentaires
en français, ils expliquent le pourquoi.

--- rules: (actuel) ---
(vide)

--- rules: (proposé) ---
specs:
  - Décrire un comportement observable, jamais une implémentation.
design:
  - Citer une décision de _codev/decisions/ qui contraint le choix.
tasks:
  - Chaque tâche énonce comment vérifier qu'elle est faite.
```

### 5. Demander la confirmation, puis écrire

Question exacte :

> Applique ce patch à `_codev/config.yaml` ? [oui/non]

Si `oui` :

- **Editer** `_codev/config.yaml` — remplacer uniquement les
  sections `context:` et `rules:`. **Préserver** `schema`,
  `workflows`, `mcp`, `inherits`, et **tous les commentaires**.
- Ajouter un commentaire au-dessus de `context:` :
  `# rédigé par /codev-configure`. S'il y en avait déjà un
  (« détecté depuis Cargo.toml »), le remplacer par le nouveau.
- Confirmer : « ✓ `_codev/config.yaml` enrichi. »

Si `non` :

- Ne rien écrire.
- Dire : « Aucune modification. Relance `/codev-configure` quand tu
  veux réessayer. »

## Sortie

Le diff proposé, la question de confirmation, et selon la réponse :
un accusé d'écriture ou un refus poli.

## Garde-fous

- **Aucune écriture avant la confirmation explicite** — même partielle,
  même « juste pour tester ».
- **Champs préservés stricts** : `schema`, `workflows`, `mcp`,
  `inherits`. Si le patch touchait autre chose, c'est un bug de la
  skill, arrête et signale-le à l'utilisateur.
- **Refuse si `_codev/config.yaml` absent** — jamais de création
  ex nihilo par cette skill.
- **Respecte le budget de lecture** — 5 docs, 8 fichiers source
  maximum, un seul niveau de `ls`. Un projet plus gros ne mérite pas
  plus de lecture : ce qui compte tient dans les fichiers les plus
  vus.
