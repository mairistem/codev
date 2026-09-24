# Design : tutoriel + diagrammes Mermaid

## Contexte

Voir `proposal.md`. Change **contenu de documentation** — un fichier
modifié (`docs/codev.md`), zéro Rust, `skip_specs: true`.

## Décisions

### Décision : le tutoriel simule un cas Rust concret, pas un exemple abstrait

Le tutoriel propose « ajouter une option `--json` à `codev list` » comme
change fictif. Deux raisons :

- **Il parle à un dev Rust** — c'est exactement le type de contribution
  qu'un lecteur peut avoir en tête.
- **Il est vérifiable** — le lecteur peut littéralement suivre la
  démarche sur codev lui-même, c'est du dogfooding pédagogique. Il
  aboutit à un delta réaliste (une exigence `MODIFIED` sur la
  capacité `cli-status` ou `cli-list`, plus un scénario `--json`).

**Alternative écartée** : un exemple hors-Rust (« ajouter une route
`/health` à une API »). Rejeté — casse la cohérence, oblige à
inventer un projet fictif complet. Le lecteur qui code en Rust
comprend, celui qui code ailleurs traduit sans effort.

### Décision : le tutoriel intègre les deux voies (Claude Code / CLI pur) en parallèle

Chaque étape du tutoriel liste **les deux** commandes possibles :

```markdown
Dans Claude Code :

    /codev-propose add-list-json

Ou en CLI pur :

    codev new change add-list-json --goal "…"
    $EDITOR _codev/changes/add-list-json/proposal.md
```

Ça évite de dupliquer le tutoriel en deux versions, et ça montre que
codev fonctionne **sans** Claude Code (l'agent n'est pas obligatoire).

**Alternative écartée** : deux tutoriels séparés (un « Claude Code »
et un « CLI pur »). Coûteux à maintenir, double du travail à jour
pour tout changement.

### Décision : Mermaid, malgré la dégradation en HTML embarqué

`pulldown-cmark` rend les blocs ` ```mermaid` comme du texte préformaté,
pas comme un diagramme. Dans le HTML de `codev docs`, l'utilisateur
voit donc le **source Mermaid** comme un bloc de code — lisible mais
pas graphique.

C'est acceptable parce que :

- Sur **GitHub** (la surface principale de lecture), Mermaid rend
  nativement depuis 2022 — public visé principal.
- Le **source Mermaid est lisible** en texte brut pour qui connaît la
  syntaxe (`stateDiagram-v2`, `graph LR`, `sequenceDiagram`).
- Le manuel HTML embarqué reste **utilisable hors-ligne**, le
  diagramme y sert de description textuelle plutôt que graphique.

**Alternative écartée A** : ajouter un rendu JS Mermaid (via CDN) au
HTML embarqué. Rejeté — casse l'exigence « HTML autonome, aucune
requête externe, hors-ligne » (`docs` spec, `Requirement: Le HTML est
autonome`, `Scenario: Le HTML est autonome`).

**Alternative écartée B** : générer des SVG statiques via
`mermaid-cli` et les embarquer en `data:` URI dans le HTML. Rejeté —
nécessite un pipeline de build, casse le principe « la doc est
un `.md` unique édité à la main ».

### Décision : trois diagrammes, pas cinq

- Machine à états d'un change — dans §3 (Le cycle).
- Graphe de crates — dans §5 (Concepts), sous-section « Architecture ».
- Cycle de vie d'un delta — dans §5 (Concepts), sous-section
  « Deltas ».

Écartés à ce stade — ne bougent pas suffisamment l'aiguille :

- Diagramme des dépendances entre artefacts d'un change
  (`proposal → design → tasks`) — trop simple, déjà clair dans le
  texte.
- Diagramme du chemin de résolution des sources héritées (K5) —
  intéressant mais niche.
- Diagramme du flux `codev validate --strict` — intéressant pour un
  contributeur, mais hors du sujet « ce que fait codev pour son
  utilisateur ».

### Décision : le tutoriel s'insère en §3.5, pas en tête de document

Le lecteur qui veut « comprendre en 30 secondes » lit la section 1
(Pourquoi codev) et 2 (Installation). Celui qui veut « apprendre en 5
minutes » entre par §3 (Le cycle) puis §3.5 (le tutoriel).

Placer le tutoriel en tête (avant §1) le rendrait plus visible mais
mélangerait deux publics : « je découvre » et « je vais commencer ».
La progression actuelle du document (pourquoi → installer → comprendre
→ pratiquer → référence) est la bonne.

**Alternative écartée** : sortir le tutoriel dans un fichier
`docs/tutorial.md` séparé. Rejeté — casse le principe « une seule
source markdown, embarquée » (§4 de la spec `docs`).

## Risques et compromis

- **Le tutoriel peut vieillir** — un changement de la sortie de
  `codev status` invalide la « sortie attendue » du bloc. →
  **Atténuation** : garder les sorties courtes (une ou deux lignes),
  et ajouter en fin de section une phrase « la sortie exacte peut
  différer d'une version à l'autre — l'important est … ».
- **Mermaid rend mal dans le HTML embarqué** — vécu comme une
  régression par qui lit `codev docs` plutôt que GitHub. →
  **Atténuation** : chaque diagramme est précédé d'une **légende en
  prose** qui contient l'information essentielle. Le diagramme
  enrichit, il ne remplace pas.
- **Le tutoriel prend de la place** — probablement 150-200 lignes
  ajoutées à un fichier déjà à 693 lignes. → **Compromis assumé** :
  la longueur totale de la doc reste raisonnable (~900 lignes), et
  le tutoriel est un point d'entrée, pas une lecture séquentielle.

## Plan de migration

Aucune. Évolution additive du contenu de `docs/codev.md`.
