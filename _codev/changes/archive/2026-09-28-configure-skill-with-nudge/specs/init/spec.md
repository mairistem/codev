## ADDED Requirements

### Requirement: `codev init` incite à `/codev-configure` quand la config générée est thin

À la sortie de `codev init`, la sortie **humaine** SHALL évaluer si le
`_codev/config.yaml` fraîchement écrit ou déjà présent est **thin** —
c'est-à-dire dont le `context:` fait moins de 200 caractères et dont
la clé `rules:` est vide.

Si la config est thin, la dernière ligne de la sortie humaine MUST
inviter l'utilisateur à lancer `/codev-configure` :

```
→ Prochaine étape recommandée : dans Claude Code, tape /codev-configure.
  Claude analysera ton projet et enrichira _codev/config.yaml
  (contexte, règles par artefact) — ~30 secondes.

Ou saute cette étape et tape /codev-propose <une-idée> directement.
```

Si la config n'est pas thin (l'utilisateur avait déjà rempli le YAML,
ou un `codev-configure` a déjà tourné), la sortie garde sa forme
courte actuelle : « Redémarre Claude Code puis tape /codev-propose. »

La sortie **JSON** MUST rester inchangée — aucun champ ajouté, aucune
promesse cassée. L'incitation est réservée à la sortie humaine, où
elle n'affecte pas les scripts qui consomment le rapport machine.

#### Scenario: Config thin déclenche la nudge

- **GIVEN** un projet neuf avec un `Cargo.toml` minimal (donc un
  `context:` détecté court, `rules:` vides)
- **WHEN** l'utilisateur lance `codev init --yes`
- **THEN** la sortie humaine contient le mot-clé `/codev-configure`
- **AND** la sortie JSON (`--json`) ne le contient pas

#### Scenario: Config non-thin ne déclenche pas la nudge

- **GIVEN** un projet dont le `_codev/config.yaml` existe déjà avec
  un `context:` de plus de 200 caractères
- **WHEN** l'utilisateur lance `codev init --yes` (idempotence)
- **THEN** la sortie humaine ne contient pas `/codev-configure`
