# Proposal : polir codev pour le lancement public

## Pourquoi

Le repo `mairistem/codev` vient de passer en public. Un visiteur qui
tombe dessus doit **comprendre en dix secondes** :

- ce que fait codev ;
- comment l'installer ;
- comment contribuer ;
- que le projet est vivant, maintenu, et respecte les conventions
  open source de base.

Aujourd'hui le README est technique et minimal, il n'y a pas de
`CONTRIBUTING`, pas de `CHANGELOG`, pas de `CODE_OF_CONDUCT`, pas de
`SECURITY`, pas de templates d'issue/PR, et **rien ne crédite
OpenSpec** — dont on s'est ouvertement inspiré au début du projet
(exploration, listing des features, décisions d'écart).

Ce lot n'ajoute **aucune capacité fonctionnelle** — c'est un change
de **méta et documentation**, prévu explicitement par le schéma
spec-driven via le marqueur `skip_specs: true`.

## Ce qui change

**Fichiers ajoutés à la racine** :

- `CONTRIBUTING.md` — comment contribuer via le cycle codev
  lui-même (fork, `codev init` si nécessaire, `/codev-propose`,
  `/codev-apply`, PR). Meta et cohérent avec l'outil.
- `CHANGELOG.md` — versions publiées avec leurs changes archivés,
  format Keep-a-Changelog. Renvoie vers les GitHub Releases pour
  les notes complètes.
- `CODE_OF_CONDUCT.md` — Contributor Covenant 2.1 standard,
  contact `mairistem` par défaut.
- `SECURITY.md` — procédure de report de faille : email privé
  (`security@…` ou GitHub Security Advisories), délai de réponse
  cible 72h.

**Fichiers ajoutés sous `.github/`** :

- `.github/ISSUE_TEMPLATE/bug_report.md` — template bug.
- `.github/ISSUE_TEMPLATE/feature_request.md` — template feature.
- `.github/ISSUE_TEMPLATE/config.yml` — désactive les issues vides,
  pointe vers CONTRIBUTING et Discussions.
- `.github/PULL_REQUEST_TEMPLATE.md` — checklist PR : change codev
  associé, tests verts, doc à jour.

**Fichiers modifiés** :

- `README.md` — refonte :
  - En-tête avec badges (build status, latest release, license MIT,
    plateformes supportées).
  - Bloc « Inspiré par [OpenSpec] » dans les premières lignes.
  - Table des matières explicite pour un lecteur qui scroll.
  - Un exemple visuel du cycle (`ASCII art` du cycle propose →
    apply → archive).
  - Lien vers `docs/codev.md` pour le manuel complet.
- `docs/codev.md` — nouvelle sous-section **« Origines »** dans la
  section 1 (Pourquoi codev), qui crédite OpenSpec explicitement,
  liste ce qu'on a repris à l'idée et ce qu'on a écarté.

## Capacités

### Nouvelles capacités

Aucune.

### Capacités modifiées

Aucune.

### Capacités retirées

Aucune.

*(Change marqué `skip_specs: true` — pure méta / documentation, pas
d'exigence observable nouvelle sur le comportement de codev.)*

## Impact

- **Code** : rien dans les crates Rust ni dans les scripts d'install.
  Uniquement des fichiers markdown à la racine et sous `.github/`.
- **Contrat JSON** : rien.
- **Fichier écrit** : 8 fichiers nouveaux, 2 fichiers modifiés (README,
  docs/codev.md). Aucun test à écrire — la doc n'a pas de tests
  automatiques.
- **Migration** : aucune.
- **Hors périmètre** :
  - **`docs.rs`** — nécessite `crates.io`, potentiellement plus tard.
  - **Site web dédié** — `codev docs` suffit pour l'instant.
  - **Traduction anglaise du README** — reportable au premier
    utilisateur non-francophone qui se manifeste.
  - **Badges sur nombre de downloads / stars** — trop précoce ; on
    ajoutera quand il y aura des chiffres à montrer.
  - **Automatisation du CHANGELOG à chaque tag** — la version V1
    est éditée à la main lors du bump ; on automatisera à partir de
    `_codev/changes/archive/` dans un cycle dédié si le besoin est là.
