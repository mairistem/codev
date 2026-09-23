# Design : `codev validate --strict`

## Contexte

Voir `proposal.md`. Petit change à haute valeur pour l'automation — le
noyau change à peine, l'API publique gagne un flag et un champ.

## Objectifs / Hors objectifs

Ce design cadre le comportement du flag, la logique d'exit code, la
signature JSON et le rendu humain. Il ne cadre pas `--archived` (E6),
`--strict-level` (paramétrable), ni un mode `--fix`.

## Décisions

### Décision : le mode strict change l'exit code, pas les sévérités

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. Promouvoir les warnings en erreurs dans le rapport** | Rendu humain et JSON reflètent directement le verdict | Trompeur pour un lecteur qui verrait « error » sur un `decision_unsealed` juste parce que la CI a passé `--strict` |
| **B. Changer uniquement l'exit code** | Le rapport dit ce qui est, le mode strict dit ce que ça déclenche | Il faut expliquer la nuance dans la doc |

**Choisi : B.** Un finding a une sévérité intrinsèque — le fait qu'un
caller le prenne en erreur relève de son contrat, pas de la nature du
finding. Aligne aussi le comportement avec les linters classiques
(clippy `--deny warnings`) qui laissent les warnings warnings dans la
sortie et changent juste le retour.

### Décision : `has_warnings()` méthode sur `ValidateReport`, comme `has_errors()`

Cohérent avec la méthode existante. Le CLI compose :

```rust
let exit_code = if strict {
    if report.has_errors() || report.has_warnings() { 1 } else { 0 }
} else if report.has_errors() {
    1
} else {
    0
};
```

Ou plus court, une méthode dédiée `is_fail(strict: bool)` — mais l'expression
littérale reste plus lisible dans une CLI qui a déjà six branches similaires.
Je garde le calcul inline.

### Décision : le champ JSON s'appelle `hasWarnings`, additif

Nommage cohérent avec le reste du contrat (`hasErrors` implicite via
`has_errors()` — mais **pas** dans le contrat JSON aujourd'hui : `items[]`
et `findings[]` suffisent). Ajouter `hasWarnings` uniquement — le champ
`hasErrors` serait redondant (le consommateur qui parse `findings[]`
sait déjà).

**Alternative écartée** : ajouter les deux, `hasErrors` et `hasWarnings`,
pour la symétrie. Coût : un champ de plus, potentiellement source de
divergence si `has_errors()` et le contenu de `findings` disent des
choses différentes. Refusé.

### Décision : `--strict` s'applique à toutes les formes de `validate`

Le flag est global à la sous-commande, pas restreint à `--all`. Un
utilisateur qui valide **un** item avec `--strict` doit avoir la même
règle de sortie. La CLI place `--strict` sur `Command::Validate` (pas
sur une variante), pas d'aiguillage.

## Risques et compromis

- **Un caller migre de « exit 0 → OK » à « exit 0 → OK sauf si strict »**
  — pas un risque : les callers actuels ne passent pas `--strict`, donc
  leur exit code ne bouge pas. Le flag est opt-in.
- **Un consommateur JSON confond `hasWarnings` avec le verdict** — le
  champ est documenté comme informatif. Le vrai verdict, c'est l'exit
  code. C'est expliqué dans la spec et rappelé dans la doc CLI.

## Plan de migration

Aucune. Le flag est opt-in ; le champ JSON est additif.
