## MODIFIED Requirements

### Requirement: `codev init` incite à `/codev-configure` quand la config générée est thin

À la sortie de `codev init`, la sortie **humaine** SHALL évaluer si le
`_codev/config.yaml` fraîchement écrit ou déjà présent est **thin** —
c'est-à-dire dont la clé `rules:` est absente ou vide.

Le champ `context:` n'entre plus dans la définition de « thin » — la
sonde de `codev init` le remplit systématiquement à partir des
manifestes détectés, ce qui rend sa longueur inutile pour deviner si
l'utilisateur a vraiment rempli sa config. Les `rules:`, à l'inverse,
sont toujours un choix utilisateur explicite ; leur présence est le
seul indicateur fiable.

Si la config est thin, la dernière ligne de la sortie humaine MUST
inviter l'utilisateur à lancer `/codev-configure` :

```
→ Prochaine étape recommandée : dans Claude Code, tape /codev-configure.
  Claude analysera ton projet et enrichira _codev/config.yaml
  (contexte, règles par artefact) — ~30 secondes.

Ou saute cette étape et tape /codev-propose <une-idée> directement.
```

Si la config n'est pas thin (l'utilisateur avait déjà écrit des
règles, ou un `codev-configure` a déjà tourné), la sortie garde sa
forme courte actuelle : « Redémarre Claude Code puis tape
/codev-propose. »

La sortie **JSON** MUST rester inchangée — aucun champ ajouté, aucune
promesse cassée. L'incitation est réservée à la sortie humaine, où
elle n'affecte pas les scripts qui consomment le rapport machine.

#### Scenario: Config sans règles déclenche la nudge

- **GIVEN** un projet neuf avec un `Cargo.toml` minimal (donc un
  `context:` détecté, mais aucune `rules:` écrite)
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** la sortie humaine contient le mot-clé `/codev-configure`
- **AND** la sortie JSON (`--json`) ne le contient pas

#### Scenario: Config avec règles ne déclenche pas la nudge

- **GIVEN** un projet dont le `_codev/config.yaml` existe déjà et
  porte au moins une entrée dans `rules:` (par exemple
  `rules: { specs: [...] }`)
- **WHEN** l'utilisateur lance `codev init --yes` (idempotence)
- **THEN** la sortie humaine ne contient pas `/codev-configure`

#### Scenario: Long contexte auto-détecté n'inhibe pas la nudge

- **GIVEN** un projet TypeScript avec beaucoup de dépendances
  (contexte auto-détecté de plus de 200 caractères), sans `rules:`
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** la sortie humaine contient bien `/codev-configure` — la
  longueur du contexte auto-détecté n'inhibe plus la nudge
