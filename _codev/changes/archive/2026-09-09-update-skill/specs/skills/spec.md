## ADDED Requirements

### Requirement: Skill `update` révise un artefact de planification

Le catalogue de codev SHALL exposer un workflow `update` — installé sous
`.claude/skills/codev-update/SKILL.md`, invocable `/codev-update` — dont le
rôle est de réviser un artefact de planification déjà écrit (proposal,
specs, design ou tasks) d'un change actif, à partir d'une description libre
donnée par l'utilisateur.

#### Scenario: Révision d'un design d'après une nouvelle contrainte

- **GIVEN** un change actif dont `design.md` cite une décision technique X
- **WHEN** l'utilisateur tape `/codev-update design "remplacer X par Y à
  cause de la contrainte Z"`
- **THEN** la skill lit `design.md`, applique la révision demandée, et
  écrit la nouvelle version
- **AND** relance `codev validate <change>` en fin de traitement

#### Scenario: Résolution implicite quand un seul change est actif

- **GIVEN** un projet avec un seul change actif
- **WHEN** l'utilisateur tape `/codev-update proposal "réduire le
  périmètre"`
- **THEN** la skill résout implicitement le change actif
- **AND** applique la révision au `proposal.md` de ce change

#### Scenario: Ambiguïté sur le change à réviser

- **GIVEN** deux changes actifs
- **WHEN** l'utilisateur tape `/codev-update tasks "…"` sans nommer de
  change
- **THEN** la skill demande à l'utilisateur lequel réviser, en listant les
  deux noms
- **AND** n'écrit rien avant d'avoir la réponse

### Requirement: Skill `update` annonce la ripple avant d'agir

Quand la révision demandée sur un artefact rend un autre incohérent, la
skill MUST le signaler à l'utilisateur et proposer la correction avant de
l'écrire, plutôt que de laisser la spec principale, le design ou la liste
de tâches en désaccord silencieux.

#### Scenario: Retirer une capacité du proposal ripple sur specs

- **GIVEN** un `proposal.md` déclarant deux capacités nouvelles `a` et
  `b`, et un fichier `specs/b/spec.md` déjà écrit
- **WHEN** l'utilisateur tape `/codev-update proposal "retirer la
  capacité b — hors périmètre finalement"`
- **THEN** la skill applique la révision au `proposal.md`
- **AND** signale à l'utilisateur que `specs/b/spec.md` devient orphelin
- **AND** propose de supprimer ce fichier ou d'appeler
  `/codev-update specs …` pour l'ajuster
- **AND** n'écrit pas cette seconde modification sans confirmation

#### Scenario: Une révision sans ripple s'applique sans confirmation supplémentaire

- **GIVEN** une révision qui ne touche qu'à `design.md` sans conséquence
  sur les autres artefacts
- **WHEN** l'utilisateur tape `/codev-update design "…"`
- **THEN** la skill applique la révision sans demander de confirmation
  additionnelle

### Requirement: Skill `update` reste dans la frontière planning

Le workflow `update` MUST se limiter aux fichiers sous
`_codev/changes/<nom>/` et MUST NOT :

- modifier du code du projet ;
- créer un artefact manquant (proposal, specs, design, tasks) — c'est
  `/codev-propose` qui le fait ;
- toucher à un change déjà archivé sous `changes/archive/`.

#### Scenario: Refus d'écrire un artefact manquant

- **GIVEN** un change dont `design.md` n'existe pas encore
- **WHEN** l'utilisateur tape `/codev-update design "ajouter la décision
  Z"`
- **THEN** la skill refuse
- **AND** invite explicitement à `/codev-propose` pour créer l'artefact

#### Scenario: Refus d'un change archivé

- **GIVEN** un change qui vit sous `changes/archive/2026-09-09-<nom>/`
- **WHEN** l'utilisateur tape `/codev-update proposal --change
  <archived-nom>`
- **THEN** la skill refuse
- **AND** rappelle qu'un change archivé est de l'histoire ; corriger
  demande de le dé-archiver à la main
