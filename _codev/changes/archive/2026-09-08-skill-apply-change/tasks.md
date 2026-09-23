# Tâches

## 1. Contenu de la skill

- [x] 1.1 Rédiger `assets/workflows/apply.md` — le corps de la skill. Contenu
      attendu, en français, cadré par les scénarios de la spec :
      - entrée : nom de change optionnel ; résolution implicite s'il n'y en a
        qu'un actif ; question à l'utilisateur si ambigu (liste des candidats) ;
      - garde-fous d'ouverture : c'est un workflow d'implémentation, il modifie
        du code du projet ; il ne modifie pas d'autres changes ; il n'archive
        pas ni ne sync — c'est un pas suivant explicite ;
      - lecture : `codev status --change <nom> --json` d'abord, pour vérifier
        que la planification est complète ; puis `Read` sur `tasks.md` ;
      - boucle : pour chaque tâche `- [ ]` dans l'ordre du fichier, la lire à
        voix haute, la faire, cocher avec `Edit` (`- [ ]` → `- [x]` sur la
        ligne exacte), courte annonce ;
      - arrêt en cas d'ambiguïté matérielle ou de blocage ; ne pas cocher tant
        que la clarification n'est pas obtenue ; suggérer un découpage en
        `X.Y.a`/`X.Y.b` sans imposer ;
      - fin : quand tout est coché, résumer et inviter à `/codev-archive` ou
        `codev archive` en pas suivant explicite.
      Vérifié par la présence du fichier et par les invariants du CATALOG
      (tâches 2.x).

## 2. Entrée dans le catalogue

- [x] 2.1 Ajouter un `Workflow { id: "apply", description: "…", allowed-tools:
      "Bash(codev:*), Read, Write, Edit, Glob, Grep, Bash", body:
      include_str!("../../../assets/workflows/apply.md") }` dans le `CATALOG`
      de `codev-agents::workflows`. La description fait plus de 60 caractères,
      décrit clairement quand invoquer la skill ("Implémenter les tâches d'un
      change… ne modifie ni les specs, ni les autres changes"). Vérifié par
      les invariants existants du CATALOG (`chaque_workflow_a_un_corps_et_une_description_utilisables`).
- [x] 2.2 Ajouter un test dédié
      `workflows::apply_est_dans_le_catalogue_et_a_les_bons_outils` qui
      vérifie : `find("apply")` rend `Some`, `allowed_tools` contient à la fois
      `Bash(codev:*)` et le `Bash` général (le seul workflow du catalogue à
      demander le second).
- [x] 2.3 Test d'invariant du frontmatter :
      `claude::le_frontmatter_de_chaque_skill_est_du_yaml_valide` (existant)
      doit passer sans modification — sa boucle itère `CATALOG` et le nouveau
      workflow y sera couvert automatiquement. Vérifié par `cargo test -p
      codev-agents`.

## 3. Config et catalogue par défaut

- [x] 3.1 Ne PAS ajouter `apply` à `DEFAULT_WORKFLOWS` — cf. décision du
      design. Vérifié par `workflows::sans_demande_installe_le_catalogue_par_defaut`
      qui reste inchangé.
- [x] 3.2 Mettre à jour le commentaire d'exemple dans
      `crates/codev-engine/src/scaffold.rs::DEFAULT_CONFIG` pour lister aussi
      `apply` dans le bloc commenté `# workflows:`, sans le décocher.
      Vérifié par `scaffold::la_config_par_defaut_est_valide_et_relisible`
      qui doit rester au vert (le bloc est en commentaire, YAML l'ignore).

## 4. Config de ce dépôt et dogfooding

- [x] 4.1 Ajouter `- apply` à la liste `workflows` de `_codev/config.yaml` de
      ce projet, pour que `codev init` / `codev update` l'installe.
- [x] 4.2 Lancer `cargo install --path crates/codev-cli` puis `codev update`
      dans ce dépôt. Vérifier que `.claude/skills/codev-apply/SKILL.md`
      apparaît, avec le bon frontmatter et le corps attendu.
- [x] 4.3 Vérifier que le frontmatter généré est du YAML valide via
      `python3 -c "import yaml; yaml.safe_load(open('.claude/skills/codev-apply/SKILL.md').read().split('---')[1])"`
      (ou équivalent — le test unitaire couvre déjà l'invariant, celle-ci sert
      de confirmation sur le vrai fichier écrit).

## 5. Intégration workspace

- [x] 5.1 `cargo test --workspace` reste vert et compte au moins 2 tests
      supplémentaires (invariants ajoutés).
- [x] 5.2 `cargo clippy --workspace --all-targets` reste sans avertissement.
- [x] 5.3 `codev validate --all` reste vert (le change encore actif est ce
      change lui-même, plus deux specs principales).
