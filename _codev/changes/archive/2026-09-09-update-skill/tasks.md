# Tâches

## 1. Contenu de la skill

- [x] 1.1 Rédiger `assets/workflows/update.md` — le corps de la skill.
      Contenu attendu, en français, cadré par les scénarios de la spec :
      - entrée : `<artefact-id>` optionnel + description libre de la
        révision ; si aucun artefact n'est nommé, la skill demande lequel
        (elle ne devine pas) ;
      - résolution du change : par argument, ou implicite s'il n'y a
        qu'un seul actif ; refus si le nom pointe vers un archivé ;
      - garde-fous d'ouverture : la skill ne modifie **jamais** de code,
        ne crée **jamais** un artefact manquant, ne touche **jamais** à
        un change archivé — chaque cas renvoie explicitement vers la
        skill idoine ;
      - lecture préalable : `codev status --change <nom> --json` pour
        vérifier que l'artefact demandé existe, puis lecture depuis le
        disque (jamais depuis la conversation) ;
      - étude de l'existant : lire les autres artefacts du change avant
        d'écrire, pour repérer un éventuel ripple avant qu'il ne
        surgisse à l'écriture ;
      - application de la révision : `Edit` pour une modification
        ciblée, `Write` pour une réécriture complète, jugement de la
        skill selon l'ampleur ;
      - détection et signalement du ripple : si la révision d'un
        artefact rend un autre incohérent (capacité retirée du proposal
        alors qu'un `specs/<capa>/` existe, décision citée par le design
        qui n'existe plus, tâche qui référence un scénario disparu…),
        la skill **nomme** l'incohérence et **propose** l'action
        (supprimer, ajuster, autre appel `/codev-update`), sans
        l'appliquer sans confirmation ;
      - garde-fou final : `codev validate <change>` et relais du rapport ;
      - fin : résumé — fichier(s) touché(s), verdict de `validate`,
        prochaine action recommandée (souvent `/codev-apply` ou
        `/codev-archive`).
      Vérifié par la présence du fichier et par l'invariant du CATALOG.

## 2. Entrée dans le CATALOG

- [x] 2.1 Ajouter `Workflow { id: "update", description: "…",
      allowed-tools: "Bash(codev:*), Read, Write, Edit, Glob, Grep",
      body: include_str!("../../../assets/workflows/update.md") }` au
      `CATALOG`. Description : « Réviser un artefact de planification
      déjà écrit d'un change codev actif — proposal, specs, design ou
      tasks — en préservant la cohérence avec les autres artefacts. Ne
      modifie aucun code du projet, ne crée aucun artefact manquant, ne
      touche à aucun change archivé. »
- [x] 2.2 Test dédié
      `workflows::update_est_dans_le_catalogue_et_a_les_bons_outils` :
      `find("update")` rend `Some` ; `allowed_tools` égale exactement
      `"Bash(codev:*), Read, Write, Edit, Glob, Grep"` ; ne contient PAS
      le `Bash` général — vérifie que « seule `apply` a le Bash général »
      reste vrai.
- [x] 2.3 Test complémentaire
      `workflows::update_cite_ses_frontieres` : le `body` du workflow
      contient les chaînes « ne modifie » et « archivé » — traceur d'un
      renommage ou d'une suppression accidentelle des garde-fous.
- [x] 2.4 Les invariants existants (`chaque_workflow_a_un_corps_…`,
      `le_frontmatter_de_chaque_skill_est_du_yaml_valide`) couvrent
      automatiquement `update` via la boucle sur `CATALOG`. Vérifié par
      `cargo test -p codev-agents`.

## 3. Config du dépôt et catalogue par défaut

- [x] 3.1 Ne PAS ajouter `update` à `DEFAULT_WORKFLOWS`. Le test
      `sans_demande_installe_le_catalogue_par_defaut` gagne un check
      supplémentaire — un tableau `for opt_in in ["apply", "sync",
      "archive", "update"]`.
- [x] 3.2 Mettre à jour le commentaire d'exemple dans
      `crates/codev-engine/src/scaffold.rs::DEFAULT_CONFIG` pour lister
      aussi `update` dans le bloc commenté `# workflows:`.
- [x] 3.3 Ajouter `- update` à la liste `workflows` de
      `_codev/config.yaml` de ce projet.

## 4. Dogfooding

- [x] 4.1 Après `cargo install --path crates/codev-cli` puis `codev
      update`, vérifier que `.claude/skills/codev-update/SKILL.md`
      apparaît avec le bon frontmatter.
- [x] 4.2 Vérifier que la liste des skills annoncée par `codev update`
      contient bien les six workflows : `codev-propose, codev-explore,
      codev-apply, codev-sync, codev-archive, codev-update`.
- [x] 4.3 Après le change appliqué et archivé, tenter un vrai
      `/codev-update` sur un change futur dès la prochaine session
      Claude Code — la skill sera visible au chargement.

## 5. Intégration workspace

- [x] 5.1 `cargo test --workspace` reste vert et compte au moins 2 tests
      supplémentaires (`update_est_dans_le_catalogue…` et
      `update_cite_ses_frontieres`).
- [x] 5.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 5.3 `codev validate --all` reste vert.
