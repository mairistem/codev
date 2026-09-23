## ADDED Requirements

### Requirement: Commande `codev decision promote` extrait un bloc du design en ADR

`codev decision promote <change> <titre>` MUST créer un nouvel ADR local
sous `_codev/decisions/NNNN-<slug>.md`, avec `status: accepted`, scellé
par K3, dont le corps reproduit **verbatim** le contenu du bloc
`### Décision : <titre>` trouvé sous la section `## Décisions` du
`design.md` du change. Le nouvel ADR contient une section `## Décision`
qui porte ce corps, et les sections `## Contexte`, `## Conséquences` et
`## Alternatives écartées` sont émises avec un `<!-- placeholder -->`
inviant l'auteur à ventiler.

Le corps « verbatim » du bloc s'entend comme : tout le texte qui suit la
ligne `### Décision : <titre>` jusqu'à la prochaine ligne commençant par
`### ` ou `## ` (exclu), sans normalisation.

Refus explicites — codes stables :

- `unknown_change` : le change n'est pas dans `codev list`.
- `cannot_promote_from_archived` : la cible pointe vers un dossier sous
  `changes/archive/` — un design archivé est de l'histoire.
- `design_missing` : le change n'a pas de `design.md`.
- `decision_heading_not_found` : aucun bloc `### Décision : <titre>` ne
  correspond dans le design.
- `ambiguous_decision_heading` : plusieurs blocs portent le même titre —
  l'utilisateur précise en éditant.

#### Scenario: Promotion réussie

- **GIVEN** un change `add-auth` dont `design.md` contient sous
  `## Décisions` un bloc `### Décision : Utiliser JWT` avec deux
  paragraphes de rationale
- **WHEN** l'utilisateur lance `codev decision promote add-auth
  "Utiliser JWT"`
- **THEN** un nouvel ADR est créé sous
  `_codev/decisions/NNNN-utiliser-jwt.md` avec `status: accepted`, `date`
  du jour, et un frontmatter valide
- **AND** son corps contient une section `## Décision` avec les deux
  paragraphes de rationale, byte pour byte
- **AND** une entrée est ajoutée à `_codev/decisions/seal.yaml` pour cet
  ADR — cohérence avec K3

#### Scenario: Refus d'un change archivé

- **GIVEN** un change qui vit sous `_codev/changes/archive/…-<nom>/`
- **WHEN** l'utilisateur lance `codev decision promote <nom> "..."`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `cannot_promote_from_archived`

#### Scenario: Titre introuvable

- **GIVEN** un change `add-auth` dont le `design.md` ne mentionne pas de
  bloc « Utiliser Kerberos »
- **WHEN** l'utilisateur lance `codev decision promote add-auth
  "Utiliser Kerberos"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `decision_heading_not_found`

#### Scenario: Titre ambigu

- **GIVEN** un `design.md` contenant **deux** blocs
  `### Décision : Choix de la librairie` (par exemple une version
  révisée du premier bloc pendant la discussion)
- **WHEN** l'utilisateur lance `codev decision promote <c> "Choix de la
  librairie"`
- **THEN** aucun fichier n'est écrit
- **AND** le message d'erreur nomme le code stable
  `ambiguous_decision_heading` et invite à éditer un des deux titres

### Requirement: Le design est mis à jour avec une référence traçable

Après une promotion réussie, le bloc `### Décision : <titre>` du
`design.md` MUST être remplacé par le même titre suivi d'**une seule
ligne** de citation textuelle qui référence le nouvel ADR :

```
### Décision : <titre>

> Promue en ADR **NNNN** — voir `_codev/decisions/NNNN-<slug>.md`.
```

La référence est en texte simple (`` ` `` pour le chemin), pas un lien
markdown — un lien `[..](../..)` casserait au moment de l'archive du
change (où la profondeur des `..` change). Le titre du bloc est
préservé pour permettre à un lecteur du design de comprendre le sujet
qui a été promu.

#### Scenario: Contenu du bloc remplacé par la référence

- **GIVEN** le même contexte que le scénario « Promotion réussie »
- **WHEN** l'utilisateur lance la commande
- **THEN** le `design.md` du change contient désormais, à
  l'emplacement du bloc :
  ```
  ### Décision : Utiliser JWT

  > Promue en ADR **NNNN** — voir `_codev/decisions/NNNN-utiliser-jwt.md`.

  ```
- **AND** le reste du fichier (autres sections, autres blocs `###`,
  espacement) est identique au caractère près
- **AND** l'espacement autour du bloc reste préservé — pas de ligne
  blanche ajoutée ni retirée

#### Scenario: Deux promotions successives sur le même design

- **GIVEN** un design contenant deux blocs distincts :
  `### Décision : A` et `### Décision : B`
- **AND** l'utilisateur a déjà promu `A` en ADR
- **WHEN** l'utilisateur lance `codev decision promote <c> "B"`
- **THEN** le bloc `B` est promu à son tour
- **AND** la ligne de référence de `A` n'est PAS altérée par cette
  seconde promotion — chaque promotion n'agit que sur son propre bloc
