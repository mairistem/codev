# Proposal : promouvoir une décision de `design.md` en ADR

## Pourquoi

Chaque `design.md` de change contient une section `## Décisions` avec des
blocs `### Décision : <titre>` qui décrivent les arbitrages techniques
retenus pour l'implémentation. Aujourd'hui, ces décisions **restent
locales** au change : elles vivent dans le design pendant que le change
est actif, et migrent vers `_codev/changes/archive/<date>-<nom>/` au
moment de l'archive. **L'index de décisions ne les voit jamais** —
`codev decision list` ne les liste pas, `codev instructions design` ne
les injecte pas aux changes suivants, et l'ADR équivalent n'existe pas
dans `_codev/decisions/`.

Résultat aujourd'hui : soit l'auteur du change crée à la main un ADR
avec `codev decision new`, en dupliquant à la main la prose du design
(risque de divergence) ; soit la décision reste enterrée dans le
design archivé et n'est plus consultable comme les autres.

K7 ferme ce trou avec un geste explicite : `codev decision promote
<change> <titre>` extrait un bloc `### Décision : <titre>` du design,
en fait un vrai ADR dans `_codev/decisions/`, scellé par K3, et
remplace le bloc d'origine par une référence traçable.

Complète le trio K3 (immutabilité) / K6 (dérives héritées) / K7
(promotion) — après ça, le cycle de vie d'une décision est complet dans
codev.

## Ce qui change

- **Nouvelle commande `codev decision promote <change> <titre>`** —
  extrait un bloc `### Décision : <titre>` du `design.md` du change,
  crée un ADR local scellé (`plan_new_decision` de K3), et met à jour
  le design.
- **Le corps du nouvel ADR** reproduit le contenu du bloc verbatim, sous
  une seule section `## Décision`. Les sections `## Contexte`,
  `## Conséquences`, `## Alternatives écartées` de l'ADR-standard sont
  générées avec un `<!-- placeholder -->` invitant l'auteur à ventiler.
- **Le bloc du design est remplacé** par une note textuelle courte :
  `> Promue en ADR **NNNN** — voir `_codev/decisions/NNNN-<slug>.md`.`.
  Volontairement du texte, pas un lien markdown : préserve la traçabilité
  même quand le change part en archive (où un `../../decisions/` casse).
- **Le titre du bloc `### Décision : <titre>` reste** — permet un
  `promote` ultérieur si l'auteur écrit plusieurs révisions ; permet
  aussi de retrouver rapidement d'où venait la décision.
- **Refus explicites** :
  - Change absent ou archivé (introuvable dans `codev list`) → code
    stable `unknown_change` ou `cannot_promote_from_archived`.
  - Design absent → code `design_missing`.
  - Titre introuvable dans `## Décisions` → `decision_heading_not_found`.
  - Titre ambigu (deux blocs même titre) → `ambiguous_decision_heading`.
- **Contrat JSON** — nouveau `DecisionPromotedV1` (proche de
  `DecisionCreatedV1`), avec le champ additif `sourceChange:
  Option<String>` pour tracer d'où vient la promotion.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `decisions` — deux nouvelles exigences ADDED : la commande
  `decision promote` et le traitement du design après promotion.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - Nouveau parseur léger de blocs `### Décision : ...` dans un
    `design.md` — vit dans `codev-engine::design` (nouveau module —
    reste côté engine puisque parser du markdown à ce niveau n'a pas de
    sens sans I/O).
  - Nouveau `plan_promote(change_id, heading, …) -> DeviatePlan-like`
    dans `codev-engine::decisions_actions` : réutilise
    `plan_new_decision` pour l'ADR + un `write` sur `design.md` pour la
    substitution du bloc.
  - Nouvelle sous-commande `codev decision promote` dans le CLI.
- **Contrat JSON** — `DecisionPromotedV1` ajouté, aucun champ retiré ni
  renommé.
- **Fichiers écrits** : nouvel ADR sous `_codev/decisions/`, entrée dans
  `seal.yaml`, design.md du change en question réécrit.
- **Migration** — aucune. La commande est opt-in ; les designs
  existants ne changent pas.
- **Hors périmètre** :
  - **Promouvoir plusieurs décisions en une seule commande** — un
    appel par décision, comme `decision new` ou `deviate`. Reportable.
  - **Splitter automatiquement en Contexte/Décision/Conséquences** — la
    prose libre d'un design n'a pas de structure exploitable ; on livre
    le corps verbatim et l'auteur ventile à la main.
  - **Ré-importer une décision promue en cas d'annulation** — un
    `codev decision supersede` ou une édition manuelle suffit ; pas de
    « unpromote » dédié.
