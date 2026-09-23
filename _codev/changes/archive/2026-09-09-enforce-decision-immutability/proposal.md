# Proposal : verrouiller l'immutabilité des décisions par le CLI

## Pourquoi

Une décision `accepted` doit être une trace figée : « voici le choix qu'on
a tranché et sur lequel on s'appuie ». Aujourd'hui, cette immutabilité
n'est que sociale — le fichier `_codev/decisions/0003-*.md` peut être
édité en place, silencieusement, sans que ni le CLI ni `validate` ne
s'en rendent compte. Un `git blame` finira par le voir, mais tout le
raisonnement de l'outil qui s'appuie sur les décisions en vigueur (injection
dans les instructions de `design`, index, résolution des supersessions)
tourne à partir du contenu **courant** du fichier, pas de celui qui avait
été accepté.

C'est le prérequis de K6 (dérives permises des décisions héritées) : on ne
peut raisonner sur une dérive que par rapport à une base immuable.

## Ce qui change

- **Nouveau fichier `_codev/decisions/seal.yaml`** — versionné avec le
  projet, tenu à jour par le CLI. Une entrée par ADR local `accepted` ou
  `superseded`, portant l'`id`, le hash SHA-256 du **corps** de l'ADR (ce
  qui suit le frontmatter), et la date à laquelle le sceau a été apposé.
- **`codev decision new` scelle en même temps qu'il écrit** — un seul plan
  d'effets porte les deux écritures (ADR + entrée du sceau), soit les deux
  réussissent, soit aucune.
- **`codev decision supersede` scelle le nouvel ADR** — le corps de
  l'ancien restant byte-identique (déjà exigé par la spec), son sceau
  reste valide sans manipulation.
- **`codev validate` remonte trois nouveaux findings stables** —
  `decision_unsealed` (warning : ADR sans entrée de sceau, migration
  attendue), `decision_seal_mismatch` (**erreur** : le corps ne correspond
  plus à son sceau, quelqu'un a édité en place), et
  `decision_orphan_seal` (warning : entrée de sceau pour un ADR qui
  n'existe plus).
- **Nouvelle commande `codev decision seal <id>`** — pour la migration
  initiale (les 6 ADRs actuels du dépôt seront flagués `unsealed` au
  premier `validate`) et pour ré-approuver un corps qui a délibérément
  changé (`--force` obligatoire si un sceau différent existait déjà).
- **Rien qui ne casse le contrat JSON existant** — les commandes actuelles
  gagnent au plus un champ additionnel dans leur réponse (le hash quand
  pertinent), aucun champ n'est retiré ni renommé.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

- `decisions` — quatre nouvelles exigences pour le scellement : format et
  emplacement du sceau, écriture par `decision new`, écriture par
  `decision supersede`, findings de `validate`, et commande
  `decision seal`.

## Impact

- **Code** : nouveau module `codev-core::decisions::seal` (calcul du hash,
  parsing/écriture de `seal.yaml`, comparaison), extension du plan
  produit par `plan_new_decision` et `plan_supersede` pour inclure l'écriture
  du sceau, extension de `validate` pour émettre les trois findings.
- **Contrat JSON** : ajout d'un champ `bodySha256` (optionnel) dans
  l'entrée `decision` de `decision new --json` et `decision seal --json`.
  Le tableau `status` gagne les trois nouveaux `code`, respectant le
  format déjà versionné.
- **Fichier écrit** : `_codev/decisions/seal.yaml`, format YAML aligné
  avec `codev.lock` (fichier de vérité tenu par le CLI, éditable en cas
  de besoin mais normalement pas manipulé à la main).
- **Migration** : au premier `validate` après cette livraison, les 6 ADRs
  du dépôt actuel remontent en `decision_unsealed`. Un `codev decision
  seal --all` (ou six `codev decision seal <id>` séquentiels) suffit à
  fermer la migration.
- **Hors périmètre** :
  - Le scellement des décisions **héritées** — c'est le projet source qui
    en est responsable, pas le projet consommateur. K6 dira comment le
    projet consommateur peut *dévier* d'une décision héritée sans
    prétendre à en modifier le contenu.
  - La signature cryptographique (GPG, signify) — le sceau atteste de
    l'intégrité, pas de l'authenticité. Reportable si le besoin apparaît.
  - Un mode `--strict` de `validate` qui transformerait tout warning en
    erreur — c'est le change E5 séparé.
