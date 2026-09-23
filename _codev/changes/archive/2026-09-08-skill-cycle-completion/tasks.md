# Tâches

## 1. Contenu des deux skills

- [x] 1.1 Rédiger `assets/workflows/sync.md` — le corps de la skill `sync`.
      Contenu attendu :
      - entrée : nom de change optionnel, résolution implicite (un seul actif) ;
      - vérification préalable : `codev status --change <nom> --json` pour
        confirmer que la planification est complète (`isPlanningComplete`) ;
      - action : `codev sync <nom> --json` ;
      - lecture du `SyncReportV1` : `changeName`, `created[]`, `updated[]`,
        `unchanged[]` ; rendu structuré (« ✓ N spec(s) créée(s), M mise(s) à
        jour, K inchangée(s) » puis liste par catégorie) ;
      - fin conditionnelle : si `created` ou `updated` est non vide, ajouter
        « Le change est prêt à être archivé si tu veux clore le cycle. » ;
        sinon, ne rien suggérer.
      Vérifié par la présence du fichier et par l'invariant du CATALOG.
- [x] 1.2 Rédiger `assets/workflows/archive.md` — le corps de la skill
      `archive`. Contenu attendu :
      - entrée : nom de change optionnel, résolution implicite ;
      - vérification préalable : `codev status --change <nom> --json` pour
        confirmer que la planification est complète ;
      - action : `codev archive <nom> --json` ;
      - lecture du `ArchiveReportV1` en cas de succès (exit 0) : rendu
        structuré (nombres par catégorie + `movedTo` sur sa propre ligne) ;
      - en cas d'exit non nul : lire le tableau `status` racine du JSON, si
        un `code == "validation_failed"` est trouvé, dire exactement
        « Le change a des erreurs. Lance `codev validate <nom>` pour voir le
        détail. » ; pour tout autre code d'erreur, relayer le champ `message`
        du JSON tel quel — pas de deviner, pas de retenter.
      Vérifié par la présence du fichier et par l'invariant du CATALOG.

## 2. Entrées dans le CATALOG

- [x] 2.1 Ajouter `Workflow { id: "sync", description: "…", allowed-tools:
      "Bash(codev:*), Read", body: include_str!("../../../assets/workflows/sync.md") }`
      au `CATALOG` de `codev-agents::workflows`. Description : « Synchroniser
      les deltas d'un change codev déjà planifié dans les specs principales,
      sans déplacer le change. À utiliser quand une capacité nouvelle doit
      apparaître dans les specs avant d'être consommée par un autre change.
      N'archive pas. »
- [x] 2.2 Ajouter `Workflow { id: "archive", description: "…", allowed-tools:
      "Bash(codev:*), Read", body: include_str!("../../../assets/workflows/archive.md") }`.
      Description : « Clore un change codev : fusionner ses deltas dans les
      specs principales et déplacer le dossier vers l'archive datée. Refuse
      d'agir si la validation remonte des erreurs. »

## 3. Tests d'invariant

- [x] 3.1 Ajouter un test
      `workflows::cycle_completion_skills_present_et_restreintes` qui vérifie :
      `find("sync")` et `find("archive")` rendent `Some` ; leurs
      `allowed_tools` égalent exactement `"Bash(codev:*), Read"` ; ni l'un ni
      l'autre ne contient `Bash` autrement que dans le préfixe `Bash(codev:*)`.
- [x] 3.2 Le test `apply_est_dans_le_catalogue_et_a_les_bons_outils` continue
      de compter « exactement zéro autre workflow » qui aurait le `Bash`
      général. Vérifié en relançant `cargo test -p codev-agents`.
- [x] 3.3 L'invariant `chaque_workflow_a_un_corps_et_une_description_utilisables`
      couvre les deux nouveaux workflows automatiquement (bouclage sur
      `CATALOG`). Vérifié à la même passe.
- [x] 3.4 L'invariant `le_frontmatter_de_chaque_skill_est_du_yaml_valide`
      couvre également les deux nouveaux frontmatter automatiquement. Vérifié
      à la même passe.
- [x] 3.5 Mettre à jour `sans_demande_installe_le_catalogue_par_defaut` pour
      affirmer que `DEFAULT_WORKFLOWS` ne contient ni `"sync"` ni `"archive"` —
      symétriquement à ce qui est fait pour `apply`.
- [x] 3.6 Ajouter dans `assets/workflows/sync.md` et `archive.md` des
      références nominatives aux champs du contrat JSON (`SyncReportV1`,
      `ArchiveReportV1`, `changeName`, `created`, `updated`, `unchanged`,
      `movedTo`, `status[].code`) — pour qu'un `grep` sur ces noms depuis un
      `contract.rs` renommé remonte les skills concernées. Vérifié par un
      test `workflows::sync_et_archive_citent_les_champs_du_contrat` qui
      cherche ces chaînes dans les `body` des deux workflows.

## 4. Config de ce dépôt et dogfooding

- [x] 4.1 Ajouter `- sync` et `- archive` à la liste `workflows` de
      `_codev/config.yaml` de ce projet.
- [x] 4.2 Lancer `cargo install --path crates/codev-cli` puis `codev update`.
      Vérifier que `.claude/skills/codev-sync/SKILL.md` et
      `.claude/skills/codev-archive/SKILL.md` apparaissent avec le bon
      frontmatter.
- [x] 4.3 Vérifier que la liste des skills annoncée par `codev update`
      contient bien les cinq workflows : `codev-propose, codev-explore,
      codev-apply, codev-sync, codev-archive`.

## 5. Intégration workspace

- [x] 5.1 `cargo test --workspace` reste vert et compte au moins 1 test
      supplémentaire (`cycle_completion_skills_present_et_restreintes`).
- [x] 5.2 `cargo clippy --workspace --all-targets` reste sans avertissement.
- [x] 5.3 `codev validate --all` reste vert.
