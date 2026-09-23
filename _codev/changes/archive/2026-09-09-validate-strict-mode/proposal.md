# Proposal : mode `--strict` pour `codev validate`

## Pourquoi

Aujourd'hui, `codev validate` sort avec un **exit code binaire** : 0 s'il
n'y a pas d'erreur, 1 sinon. Les **warnings** — `decision_unsealed`,
`decision_dangling_deviation`, `git_source_unlocked`, `decision_id_collision`
et compagnie — passent sans influencer l'exit code. C'est le bon choix par
défaut pour l'humain qui itère (un warning ne bloque pas son flow), mais
c'est le mauvais choix pour un consommateur **automatisé** :

- une CI qui veut refuser tout `push` qui laisserait un `unsealed` ;
- un hook `pre-commit` qui veut interdire une dérive orpheline avant
  qu'elle atteigne `main` ;
- un futur workflow MCP (Jira → codev, Claude Design → codev) qui trancherait
  sur exit-code plutôt que sur un parseur de sortie humaine.

Le mode strict fige la garantie contractuelle « aucun finding, quelle que
soit la sévérité ⇔ exit code 0 » que les callers automatisés attendent.

## Ce qui change

- **Nouveau flag `--strict`** sur `codev validate`, applicable à toutes
  ses formes (`validate <item>`, `validate --all`, `validate --changes`,
  `validate --specs`).
- **Effet** : quand `--strict` est présent, l'exit code passe à 1 dès
  qu'un finding est émis, quelle que soit sa sévérité (Warning inclus).
  Sans le flag, l'exit code reste binaire sur `Error` uniquement —
  compat totale.
- **Aucune sévérité modifiée** dans le rapport : les findings sortent
  avec leur sévérité d'origine. Le mode strict change **le verdict de
  sortie**, pas la nature des findings. Le rendu humain reste identique.
- **JSON contrat** : le champ `hasWarnings: bool` est **ajouté** au
  rapport (`ValidateReportV1`). Additif, sérialisé toujours. Le
  consommateur peut ainsi décider indépendamment de l'exit code —
  l'exit code est le signal, ce champ est la donnée.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `validation` — deux nouvelles exigences ADDED : comportement du flag
  `--strict` sur l'exit code, et exposition de `hasWarnings` dans le
  contrat JSON.

## Impact

- **Code** : ajout du flag `strict: bool` sur `Command::Validate` dans
  `codev-cli::main`, ajout d'une méthode `has_warnings()` sur
  `ValidateReport` dans `codev-engine::validate::report`, et un
  changement de deux lignes dans la logique d'exit-code du CLI.
- **Contrat JSON** : `ValidateReportV1` gagne `hasWarnings: bool`.
  Additif — les consommateurs antérieurs ignorent le champ.
- **Fichier écrit** : aucun. Le mode strict ne change rien sur disque.
- **Migration** : aucune. Sans `--strict`, le comportement est
  bit-identique à aujourd'hui.
- **Hors périmètre** :
  - **Un mode `--strict-level=warning|info`** — pour aujourd'hui, le
    binaire strict/lax suffit ; si le besoin d'un seuil configurable
    apparaît, on l'ajoutera.
  - **Un `--fix` qui corrige automatiquement les warnings** — pas un
    change de validation, mais un change de correction, hors périmètre.
  - **`--archived`** qui valide aussi les changes archivés — c'est un
    change séparé (E6 dans la roadmap noyau).
