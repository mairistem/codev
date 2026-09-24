# Design : polish public launch

## Contexte

Voir `proposal.md`. Change **méta / documentation** — 8 fichiers
nouveaux, 2 modifiés, zéro Rust. `skip_specs: true` assumé.

## Décisions

### Décision : crédit **OpenSpec** en tête de README + section « Origines » dans `docs/codev.md`

Deux emplacements :

- **README** — ligne visible dans les premières lignes du visiteur
  qui découvre le repo : `> Inspiré par [OpenSpec](https://github.com/tobyhs/openspec), reconstruit en Rust avec ses propres choix (voir `docs/codev.md` § Origines).`
- **`docs/codev.md`** — nouvelle sous-section **« Origines »** sous
  la section 1 (Pourquoi codev), qui détaille :
  - Ce qui vient d'OpenSpec — cycle propose/apply/archive, notion
    de deltas de spec, capacités, ADR.
  - Ce qui a été écarté — TypeScript (Rust choisi), `.openspec/`
    caché (`_codev/` visible), un binaire `openspec` de plusieurs
    milliers de lignes (cœur pur + coquille impérative).
  - Ce qui est propre à codev — sceau K3, dérives K6, promotion K7,
    intégration MCP configurable, `codev docs`, distribution
    précompilée.

**Alternative écartée** : une simple mention `Prior art: OpenSpec`
en pied de README. Rejeté — trop discret, insuffisant si on veut
vraiment reconnaître la dette intellectuelle.

### Décision : `CHANGELOG.md` alimenté à la main, pas auto-généré

Le format **Keep-a-Changelog** est adopté. À chaque tag, le mainteneur
ajoute une entrée V1.x avec les changes archivés depuis la version
précédente. Le CHANGELOG renvoie systématiquement à la Release GitHub
pour les notes détaillées (elles sont auto-générées par
`generate_release_notes: true` dans le workflow).

**Alternative écartée** : script `codev changelog` qui parcourt
`_codev/changes/archive/` pour construire le fichier. Utile un jour,
mais reportable — la version manuelle prend 2 minutes par tag.

### Décision : `CODE_OF_CONDUCT.md` = Contributor Covenant 2.1 sans modification

Standard bien connu, adopté par des milliers de projets (Rust
lui-même, Node.js, la plupart des projets Apache). Reprendre le
texte officiel évite d'inventer une charte incomplète. Contact :
`mairistem` (ou le mail de Ludovic à choisir).

**Alternative écartée** : version maison. Rejeté — ne rassure
personne, coûte cher à maintenir.

### Décision : `SECURITY.md` = GitHub Security Advisories, pas de mail

Le repo est sur GitHub → la voie officielle pour signaler une faille
est **GitHub Security Advisories** (Repo → Security → Advisories →
Report a vulnerability). C'est privé, versionné, avec une CVE si
besoin.

Un mail générique (`security@…`) est une deuxième option pour ceux
qui préfèrent, mais on ne l'inventera pas si Ludovic n'a pas déjà
un domaine `mairistem.com` prêt.

**Alternative écartée** : un fichier `SECURITY.md` vide qui promet
un traitement futur. Refusé — mieux vaut ne rien promettre que
promettre à vide.

### Décision : `CONTRIBUTING.md` guide via **le cycle codev lui-même**

Un contributeur qui découvre codev voit dans `CONTRIBUTING.md` :

1. Fork + clone.
2. `cargo install --path crates/codev-cli` pour construire depuis
   la source.
3. `codev init` sur le fork (si pas déjà initialisé).
4. Créer un change : `codev new change <mon-idée>` ou
   `/codev-propose <mon-idée>` dans Claude Code.
5. Implémenter, tester (`cargo test --workspace`), archiver
   (`codev archive <mon-idée>`).
6. Push, ouvrir une PR.

**Meta** : codev **utilise codev pour se documenter, se maintenir,
et se contribuer**. Cohérence totale, et démonstration vivante que
le cycle marche pour un vrai projet.

### Décision : Templates `.github/ISSUE_TEMPLATE/` — deux formulaires, pas trois

- `bug_report.md` — Reproduction, comportement attendu, comportement
  observé, environnement (`codev --version`, OS).
- `feature_request.md` — Problème que ça résout, alternative
  considérée, périmètre.
- `config.yml` — désactive les blank issues (`blank_issues_enabled:
  false`), redirige vers Discussions et CONTRIBUTING.

Trois formulaires (bug / feature / question) créent du friction pour
l'utilisateur. Question → Discussions.

### Décision : Badges du README — 4 badges, pas 10

- **Build status** (GitHub Actions release workflow) — signal santé.
- **Latest release** (dernière version taguée) — utile pour installer.
- **License MIT** — clarifie la réutilisation.
- **Plateformes supportées** (macOS, Linux, Windows) — cadre l'usage.

Éviter les badges cosmétiques (rustdoc, code coverage, stars). Ils
peuvent venir plus tard s'il y a une raison.

## Risques et compromis

- **`CHANGELOG.md` peut dériver** de la vraie histoire si un
  mainteneur oublie de le mettre à jour à un tag. → **Atténuation** :
  ajouter un rappel dans `CONTRIBUTING.md` section « Release », et
  la Release GitHub reste la source de vérité de secours.
- **`CODE_OF_CONDUCT.md` peut être perçu comme performatif** —
  personne ne le lit avant d'avoir un vrai conflit. → **Compromis
  assumé** : c'est le standard de fait open source, l'absence
  serait plus remarquable que la présence.
- **Le crédit OpenSpec peut sembler défensif** (« on n'a rien
  volé »). → **Cadré positivement** : « inspiré par, reconstruit
  en Rust avec ses choix », ton neutre, section Origines détaille.

## Plan de migration

Aucune. Feature additive, opt-in visuel (le visiteur voit du
polish supplémentaire, l'utilisateur existant ne remarque rien).
