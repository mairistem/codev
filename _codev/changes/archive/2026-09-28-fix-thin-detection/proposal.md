# Proposal : la détection « thin » regarde uniquement les règles

## Pourquoi

Le lot `configure-skill-with-nudge` a introduit la fonction
`is_config_thin(context, rules_empty) -> bool` avec un seuil sur la
longueur du `context:` (200 caractères). L'intention était :
« un contexte long OU des règles présentes signifient que
l'utilisateur a rempli sa config ».

Testé sur un vrai projet **mira** (TypeScript, ~10 dépendances), le
comportement observé casse cette intention :

- `codev init` détecte la stack et écrit un contexte de **280
  caractères** — uniquement composé de la liste des dépendances
  détectées et de « CI GitHub Actions active ».
- `is_config_thin` retourne `false` — le seuil de 200 est franchi.
- **Aucune nudge n'apparaît**, alors que la config n'a en réalité
  reçu aucune contribution humaine et que `/codev-configure` a
  exactement quelque chose à offrir.

Le seuil sur le contexte est le mauvais critère. Il ne distingue pas
« contexte auto-détecté par la sonde » de « contexte rédigé par
l'utilisateur ». Or seul le second traduit une intention.

## Ce qui change

**Simplification** : `is_config_thin` prend **une seule primitive** —
`rules_empty: bool` — et retourne exactement `rules_empty`. La
signature devient :

```rust
pub fn is_config_thin(rules_empty: bool) -> bool
```

**Justification** : les `rules:` par artefact sont toujours un choix
utilisateur — jamais auto-détectées, jamais renseignées par la
sonde. Leur présence est un indicateur fiable et binaire :

- `rules:` vide → l'utilisateur n'a pas encore rempli sa config → la
  nudge est utile → thin.
- `rules:` non vide → l'utilisateur a pris la peine d'écrire au
  moins une règle → sa config n'est plus vierge → pas thin.

Le champ `context:` est ignoré parce qu'on ne peut pas différencier
sans marqueur ce qui vient de la sonde de ce qui vient de
l'utilisateur — et on ne veut pas ajouter un marqueur pour un signal
qui reste secondaire.

**Effet observable** : sur un projet qui n'a pas encore été enrichi
par `/codev-configure` (ou à la main), la nudge apparaît
**systématiquement** — dans `codev init`, dans `codev status`, dans
`/codev-onboard`. Dès qu'une règle est écrite (par `configure` ou à
la main), la nudge disparaît.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- **`configure`** — une phrase du `## Purpose` mentionnait le seuil
  de contexte ; à supprimer dans le body de la skill uniquement (pas
  dans la spec, qui ne le mentionne pas).
- **`init`** — la Requirement « incite à `/codev-configure` quand la
  config générée est thin » et son scenario évoquent « context < 200
  caractères ET rules vides ». À réécrire pour ne parler que de la
  clé `rules:` vide.
- **`skills`** — la Requirement « Skill `onboard` présente codev »
  évoque « context < 200 chars, rules vides ». À réécrire.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - `codev-core::config::is_config_thin` — signature simplifiée.
  - Tests unitaires — 3 cas ré-écrits.
  - Trois lieux d'appel simplifiés :
    - `codev-cli::commands::install_skills` (dans le calcul de
      `SetupOutcome.config_thin`).
    - `codev-cli::main::config_is_thin` (helper pour le nudge de
      `codev status`).
    - Aucun autre — la skill `onboard` lit directement le YAML, pas
      la fonction Rust.
  - `assets/workflows/onboard.md` — la mention du seuil « context <
    200 chars » retirée ; ne reste que la vérification `rules:`
    vides.
  - `assets/workflows/configure.md` — n'évoque pas le seuil, mais
    vérifier que rien ne dépend implicitement de la nouvelle
    définition.
- **Contrat JSON** : rien ne change. `is_config_thin` ne sort pas
  dans le contrat.
- **Backward compat** : parfait. Les configs existantes voient
  simplement une évaluation plus stricte de « thin » — dans le sens
  où « une config avec des règles n'est plus thin ». Une config
  sans règles reste thin (comme avant). Ce qui change : une config
  sans règles mais avec long contexte auto-détecté redevient thin
  (c'était le bug).
- **Migration** : aucune.
- **Hors périmètre** :
  - Marqueur `# rédigé par /codev-configure` — reporté ; la
    simplification `rules_empty` seule suffit à corriger le
    symptôme observé.
  - Introduction d'un mode `--force-nudge` en flag — pas utile,
    l'utilisateur peut toujours ignorer la nudge.
