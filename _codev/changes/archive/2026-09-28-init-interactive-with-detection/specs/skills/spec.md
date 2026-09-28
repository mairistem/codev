## MODIFIED Requirements

### Requirement: `onboard` fait partie du catalogue par défaut

Le tableau `DEFAULT_WORKFLOWS` de `codev-agents::workflows` MUST
contenir la **liste complète des 7 workflows** de codev : `propose`,
`explore`, `onboard`, `apply`, `sync`, `archive` et `update`. Un
utilisateur qui lance `codev init --yes` (ou depuis un pipe non
interactif) sur un projet neuf, sans clé `workflows:` dans son
`config.yaml`, obtient donc toutes les skills disponibles
immédiatement.

Un projet qui veut restreindre le catalogue MUST déclarer une clé
`workflows:` explicite avec un sous-ensemble choisi — c'est la voie
opt-out, plutôt que l'ancienne voie opt-in.

Cette bascule règle un problème de découverte : sous l'ancien défaut
(3 workflows), un utilisateur qui tapait `/codev-apply` après
`/codev-propose` ne trouvait pas la skill et croyait qu'elle
n'existait pas.

#### Scenario: Catalogue par défaut inclut les 7 workflows

- **GIVEN** un projet dont le `config.yaml` n'a pas de clé
  `workflows:`
- **WHEN** `select(None)` est appelé sur le catalogue
- **THEN** la liste des `id` retournés est exactement
  `["propose", "explore", "onboard", "apply", "sync", "archive", "update"]`
- **AND** aucun warning n'est émis

#### Scenario: Restriction opt-out via workflows explicite

- **GIVEN** un projet dont le `config.yaml` contient
  `workflows: [propose, explore, onboard]`
- **WHEN** `select` est appelé avec cette liste
- **THEN** seules ces trois skills sont retournées
- **AND** `apply`, `sync`, `archive`, `update` ne sont **pas**
  installés
