# Proposal : skill `/codev-configure` + nudge de première prise en main

## Pourquoi

Le lot précédent (`init-interactive-with-detection`) a bien avancé sur
la première invocation : `codev init` sonde l'environnement, pose deux
questions, écrit un `_codev/config.yaml` prérempli. Mais dans la pratique,
le fichier reste souvent **maigre** : sur un projet frais avec juste un
`Cargo.toml` sans commentaires, on obtient un `context: | Projet Rust,
2024.` — utile mais très pauvre pour orienter les skills.

Deux limites que le CLI ne peut pas franchir seul :

- **Il ne parle pas à un LLM** — c'est un principe (`_codev/decisions/0001`).
  Le CLI ne peut donc pas lire le README, deviner le style de code du
  projet, ou synthétiser des règles.
- **Il ignore le contexte non structuré** — les conventions vivent dans
  `CONTRIBUTING.md`, dans les docs, dans les commentaires du code. Le
  CLI en est aveugle.

C'est exactement là que les **skills** rentrent : elles ont accès à
Claude, elles peuvent lire un projet entier et en tirer un contexte
riche. Aujourd'hui aucune skill n'a ce rôle — `onboard` présente
codev, `propose` planifie un change, mais rien ne dit à Claude
« regarde ce projet et enrichis sa config ».

## Ce qui change

### Nouvelle skill `/codev-configure`

Une **8ème skill**, opt-out comme les 7 autres, qui :

1. Lit le `_codev/config.yaml` existant.
2. Explore le projet en lecture pure — `README.md`, `CONTRIBUTING.md`,
   `docs/`, un échantillon des fichiers source les plus édités selon
   `git log`, la structure des dossiers.
3. Rédige une proposition d'enrichissement du `context:` (2-5 lignes
   ciblées sur ce qui compte pour piloter les skills — stack au-delà
   du langage, conventions d'API, style d'erreur, ton des commentaires,
   choix de bibliothèques structurants) et des `rules:` par artefact
   (`specs:`, `design:`, `tasks:` — 1-2 règles chacune, ancrées dans
   ce que le projet fait).
4. Affiche le diff à l'utilisateur.
5. Écrit le nouveau `_codev/config.yaml` sur confirmation — en
   **préservant** le schéma, les workflows, la clé `mcp:` et les
   commentaires de provenance déjà présents.

La skill est explicite sur ce qu'elle ne fait pas : elle ne modifie
ni les workflows, ni les MCPs détectés, ni le schéma. Elle ne touche
qu'aux champs libres (`context`, `rules`).

### Fonction pure `is_config_thin` dans `codev-core`

```
pub fn is_config_thin(cfg: &ProjectConfig) -> bool {
    cfg.context.as_deref().map(str::len).unwrap_or(0) < 200
        && cfg.rules.is_empty()
}
```

Réutilisée par les trois lieux d'incitation ci-dessous. Testable seule,
change trivial dans le futur si le seuil doit bouger.

### Nudge dans `codev init`

Aujourd'hui la sortie de `codev init` termine par :

```
Redémarre Claude Code pour qu'il découvre les skills, puis tape /codev-propose.
```

Elle devient une vraie call-to-action **conditionnelle** — seulement
quand `is_config_thin(&resolved_config)` est vrai :

```
✓ codev initialisé — 7 skills prêtes.

→ Prochaine étape recommandée : dans Claude Code, tape /codev-configure.
  Claude analysera ton projet et enrichira _codev/config.yaml
  (contexte, règles par artefact) — ~30 secondes.

Ou saute cette étape et tape /codev-propose <une-idée> directement.
```

Si la config n'est pas thin (l'utilisateur l'a remplie à la main, ou
un `codev-configure` a déjà tourné), on garde la sortie actuelle
courte.

### Nudge dans `codev-onboard`

La skill `onboard` (spec `skills`, requirement « présente codev et
recommande la prochaine action ») acquiert une nouvelle branche dans
sa table de décision : **si `_codev/config.yaml` a un contexte thin,
la recommandation devient `/codev-configure` avant `/codev-propose`**.

Sur un projet dont l'utilisateur a rempli le contexte, `onboard` se
comporte comme aujourd'hui — il recommande `/codev-propose` en priorité.

### Nudge dans `codev status` (projet neuf)

Sur un projet initialisé mais sans change actif (le cas typique après
un `codev init`), `codev status` sort aujourd'hui :

```
Erreur : aucun change actif dans ce projet
Correction : crée-en un avec `codev new change <nom>`
```

Quand la config est thin, ajouter une **troisième ligne** :

```
Astuce : config peu remplie — /codev-configure peut l'enrichir.
```

Cette astuce n'apparaît qu'en sortie humaine, jamais dans le JSON.

### Retour de `DEFAULT_WORKFLOWS` — 7 → 8

`configure` rejoint les workflows par défaut : un utilisateur nouveau
doit pouvoir l'invoquer sans re-configurer. Le préset **Minimal**
(actuellement 3) devient 4 avec `configure` — c'est une skill à usage
rare mais son absence forcerait à ré-installer plus tard.

## Capacités

### Nouvelles capacités

- **`configure`** — nouvelle capacité qui décrit le contrat de la skill
  `/codev-configure` : ce qu'elle lit, ce qu'elle écrit, ce qu'elle
  préserve, comment elle demande confirmation.

### Capacités modifiées

- **`skills`** — trois Requirements touchés :
  - Le Requirement « `onboard` fait partie du catalogue par défaut »
    (déjà réécrit dans le lot précédent) devient explicitement « les
    **8 workflows** — `configure` inclus — sont dans le défaut ».
  - Un ADDED : « Skill `configure` enrichit `_codev/config.yaml` en
    analysant le projet ».
  - Un MODIFIED sur le Requirement `onboard` : ajout de la branche
    « si la config est thin, recommande `/codev-configure` avant
    `/codev-propose` ».
- **`init`** — Requirement ajouté : « `codev init` incite à
  `/codev-configure` en sortie humaine quand la config générée est
  thin ». La sortie JSON reste inchangée pour préserver le contrat.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - Fonction pure `is_config_thin` dans `codev-core::config`. Test
    unitaire.
  - Nouvelle entrée dans `CATALOG` de `codev-agents::workflows` pour
    `configure`. Test d'invariant.
  - Body de skill dans `assets/workflows/configure.md`.
  - Sortie humaine de `codev init` dans `codev-cli::render` : branche
    conditionnelle sur `is_config_thin`.
  - Sortie humaine de `codev status` (cas « aucun change actif ») :
    même branche.
  - Body de `assets/workflows/onboard.md` : nouvelle branche dans la
    table de décision.
  - Mise à jour de `DEFAULT_WORKFLOWS` et des tests associés.
- **Contrat JSON** : rien ne change. Les nudges vivent dans la sortie
  humaine uniquement.
- **Backward compat** : sur un projet existant avec `_codev/config.yaml`
  déjà bien rempli, aucun nudge ne s'affiche — l'expérience de
  l'utilisateur existant est identique. Sur un projet dont l'utilisateur
  n'a rempli ni `context:` ni `rules:`, un nudge apparaît la prochaine
  fois qu'il lance `codev status` ou `codev init` (idempotent, non
  intrusif — juste une ligne).
- **Fichiers écrits** : ~3 fichiers nouveaux (skill body configure,
  section spec configure, fonction is_config_thin) + ~5 modifiés
  (workflows, onboard body, init render, status render, tests). Delta
  de spec sur `skills` + `init` + nouvelle capacité `configure`.
- **Hors périmètre** :
  - **Skill qui enrichit `mcp:`** — la détection MCP marche déjà, pas
    besoin d'un enrichissement LLM.
  - **Skill qui propose des `inherits:`** — trop conjectural, un
    utilisateur qui hérite d'un dépôt le sait.
  - **Auto-configure au premier lancement** — la skill reste à la
    main de l'utilisateur, on ne l'invoque jamais sans son
    consentement.
  - **Nudge dans `codev list`** — trop marginal, `list` est appelé par
    des scripts, éviter le bruit.
