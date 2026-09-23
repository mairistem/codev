# Design : parseur de specs et de deltas

## Contexte

Voir `proposal.md` pour la motivation. Le parseur vit dans `codev-core`, donc
en pur — aucune `std::fs`, aucune horloge — conformément à la décision
[0001 — Cœur fonctionnel, coquille impérative](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md).

Il sera consommé par `codev-engine` (via des modules à venir : `validate`,
`sync`, `archive`), pas par le CLI directement. Le graphe de crates suffit à
faire respecter le sens du flux, cf.
[0002 — Le graphe de crates applique la règle de dépendance](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).

## Objectifs / Hors objectifs

Ce design cadre uniquement le parseur — sa forme d'AST, ses frontières
d'erreur, sa gestion des zones littérales. La fusion sémantique
(`sync`/`archive`) et les règles de validation cross-fichier restent hors
périmètre : elles habiteront leurs propres modules dans `codev-engine`, chacun
avec son design.

## Décisions

### Décision : parseur maison ligne à ligne, sans dépendance markdown

Le format est un sous-ensemble strict de CommonMark qu'on contrôle : quelques
en-têtes de niveau 2, 3 et 4, plus un traitement particulier des blocs de code
et des commentaires HTML. La référence OpenSpec fait tout son parsing en
~1 200 lignes de TypeScript sans dépendance markdown.

Une passe ligne à ligne, avec un masque de fences précalculé, couvre 100 % du
besoin et garde `codev-core` sans nouvelle dépendance.

**Alternatives considérées** :

- **`pulldown-cmark`** — pull parser CommonMark, léger, très utilisé. Écarté :
  il produit des événements CommonMark génériques dont il faut extraire nos
  concepts, et sa notion de « position » est un `Range<usize>` par événement
  qu'il faut regrouper à la main. On paierait la dépendance sans gagner de
  code.
- **`comrak`** — AST GitHub-Flavored Markdown complet. Écarté : plus lourd,
  arbre non nécessaire pour cette structure plate.
- **`markdown` 1.0** — CommonMark en Rust avec AST. Mêmes objections que
  `comrak`, moins connu.

### Décision : deux types de sortie distincts, `Spec` et `Delta`

Une spec principale et un delta se ressemblent en surface, mais un `MODIFIED`
qui apparaîtrait dans une spec principale est une erreur, et un `Purpose` dans
un delta n'a de sens que pour une nouvelle capacité. Deux types produisent une
API où le mauvais mélange ne compile pas, plutôt qu'un type commun où chaque
consommateur re-vérifie ce qu'il tient.

Signature envisagée :

```rust
pub fn parse_spec(source: &str) -> Parsed<Spec>;
pub fn parse_delta(source: &str) -> Parsed<Delta>;
```

Aucun `Result` en sortie : un fichier catastrophiquement illisible se
distingue mal d'un fichier partiellement récupérable, et le second cas est le
plus fréquent. La distinction se fait via `Parsed`.

### Décision : rapport de défauts porté par le résultat, jamais par une erreur

`Parsed<T>` porte à la fois l'AST reconstruit — même partiel — et une liste de
`Finding` structurels typés `{ line, column, code, message }`. Un consommateur
comme `validate` remonte les findings à l'utilisateur ; `sync` et `archive`
refusent d'écrire dès qu'il y en a un de sévérité `Error`.

C'est la même logique que le contrat JSON du CLI : le message peut être
reformulé sans préavis, le `code` est stable et testable. Aligne avec
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) — décider
n'est pas exécuter — puisque le parseur *décrit* les défauts, et laisse au
consommateur de *décider* quoi en faire.

### Décision : spans en octets **et** en lignes, sans emprunter la source

Chaque nœud de l'AST porte deux vues du même intervalle : `byte_range:
Range<usize>` pour une réécriture au caractère près, `line_range: Range<u32>`
pour un message d'erreur lisible.

L'AST **ne possède pas** de `&str` empruntés au source : ses champs sont
des `String` ou des `Range<usize>`. Le consommateur qui veut réécrire tient
le source original et le combine avec les spans. Cela permet à un `Parsed`
d'être `Send + 'static` — condition nécessaire pour traverser une frontière
de fonction sans lifetime dans les signatures publiques.

**Alternative** : emprunter le source (`Spec<'a>`). Zéro copie, mais impose
des lifetimes dans tout ce qui manipule un AST — y compris `codev-engine`,
qui deviendrait générique sur des durées de vie pour un gain négligeable
puisqu'un `Requirement` fait quelques centaines d'octets et qu'on en a
quelques dizaines par fichier.

### Décision : les fences suivent la règle « premier ouvert, premier fermé »

Une fence ouverte par ` ``` ` ferme sur ` ``` ` (ou plus long, du même
caractère) et n'accepte pas un `~~~` comme fermeture. Une fence ouverte par
`~~~` ferme symétriquement. Le contenu à l'intérieur — y compris d'autres
fences apparentes du **même** marqueur mais plus courtes — est traité comme
littéral. Ce comportement suit CommonMark et OpenSpec.

Les commentaires HTML sont détectés de leur `<!--` à leur `-->`, sur plusieurs
lignes si besoin, en dehors des fences uniquement (à l'intérieur, ils sont
déjà littéraux).

## Risques et compromis

- **Fins de ligne mixtes** — Un fichier CRLF donne des positions de span
  faussées si on convertit en interne. → **Atténuation** : le parseur opère
  sur `&str` sans normalisation, et les spans sont en octets sur le source
  d'entrée tel qu'il est.
- **Fenêtre de bug entre spec parseur et fusion sémantique** — Un `Finding`
  raté ici devient une réécriture destructive dans `archive`. →
  **Atténuation** : golden tests dès ce change (fichiers d'entrée + AST
  attendu sérialisé), et un test d'invariant qui vérifie qu'écrire chaque
  bloc via son span puis reconcaténer reproduit la source à l'octet près.
- **Extensibilité du delta** — Ajouter une cinquième opération demanderait de
  modifier l'énumération. → **Compromis assumé** : les quatre opérations sont
  gravées dans le format, une cinquième mériterait une décision au sens
  `_codev/decisions/`, pas un simple ajout d'`enum` variant.

## Plan de migration

Sans objet : capacité nouvelle, aucun code à faire évoluer.
