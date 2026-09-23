# Tâches

## 1. Contenu de la skill

- [x] 1.1 Rédiger `assets/workflows/onboard.md` — le corps de la skill.
      Contenu attendu, en français, cadré par les scénarios de la spec :
      - **entrée** : aucune (la skill s'invoque sans argument, avec ou
        sans description) ;
      - **étape 1 — description** : trois phrases sur ce qu'est codev
        (planification versionnée, cycle propose → apply → archive,
        pilotable par Claude Code) ;
      - **étape 2 — état** : lire les sorties humaines de
        `codev list`, `codev list --specs`, `codev decision list` (si
        elle existe déjà via K3 ou K6). Compter, ne pas lister
        exhaustivement.
      - **étape 3 — recommandation** : arbre de cas de la section
        Décisions du design (5 branches, mutuellement exclusives).
      - **garde-fous** : jamais d'écriture, jamais de `codev init` lancé
        à la place de l'utilisateur, jamais de suggestion de skill
        opt-in absente du catalogue installé.
      - **format de sortie** : trois blocs distincts (« codev, c'est »,
        « ici, tu as », « la suite »), séparés par une ligne blanche,
        format markdown lisible en terminal.
      Vérifié par la présence du fichier et par l'invariant du CATALOG.

## 2. Entrée dans le CATALOG

- [x] 2.1 Ajouter `Workflow { id: "onboard", description: "…",
      allowed-tools: "Bash(codev:*), Read, Glob", body:
      include_str!("../../../assets/workflows/onboard.md") }` au
      `CATALOG`. Description : « Présenter codev à un utilisateur qui
      le découvre : ce que fait l'outil, l'état actuel du projet, et
      la prochaine action recommandée. Strictement en lecture — ne
      modifie ni ne crée rien. »
- [x] 2.2 Ajouter `"onboard"` à `DEFAULT_WORKFLOWS`. La liste passe
      de `["propose", "explore"]` à `["propose", "explore",
      "onboard"]`.
- [x] 2.3 Test dédié
      `workflows::onboard_est_dans_le_catalogue_et_a_les_bons_outils` :
      `find("onboard")` rend `Some` ; `allowed_tools` égale exactement
      `"Bash(codev:*), Read, Glob"` ; ne contient PAS le `Bash`
      général.
- [x] 2.4 Test complémentaire
      `workflows::onboard_est_dans_le_catalogue_par_defaut` :
      `DEFAULT_WORKFLOWS.contains(&"onboard")` est `true`.
- [x] 2.5 Adapter le test existant
      `sans_demande_installe_le_catalogue_par_defaut` — attend une
      liste de trois entrées `["propose", "explore", "onboard"]` ; le
      tableau `for opt_in` retire `"onboard"` (`onboard` est
      désormais dans le default, pas opt-in).
- [x] 2.6 Test complémentaire `onboard_cite_ses_trois_blocs` : le body
      contient les chaînes « codev » (description), « ici » (état) et
      « suite » ou « recommand » (action) — traceur d'une refonte
      accidentelle du body.
- [x] 2.7 Les invariants existants (`chaque_workflow_a_un_corps_…`,
      `le_frontmatter_de_chaque_skill_est_du_yaml_valide`) couvrent
      automatiquement `onboard`. Vérifié par
      `cargo test -p codev-agents`.

## 3. Config du dépôt

- [x] 3.1 Ajouter `- onboard` à la liste `workflows` de
      `_codev/config.yaml` de ce projet — pour dogfooder la skill.
- [x] 3.2 Actualiser le commentaire d'exemple `# workflows:` du
      `DEFAULT_CONFIG` dans `crates/codev-engine/src/scaffold.rs` pour
      lister aussi `- onboard` (les six workflows opt-in + onboard) —
      cohérence pédagogique avec ce que les nouveaux projets voient.

## 4. Dogfooding et intégration workspace

- [x] 4.1 `cargo test --workspace` reste vert, gagne au moins 3 tests
      dédiés (`onboard_est_dans_le_catalogue…`,
      `onboard_est_dans_le_catalogue_par_defaut`,
      `onboard_cite_ses_trois_blocs`).
- [x] 4.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 4.3 Après `cargo install --path crates/codev-cli` puis `codev
      update`, vérifier que `.claude/skills/codev-onboard/SKILL.md`
      apparaît avec le bon frontmatter.
- [x] 4.4 Vérifier que la liste des skills annoncée par `codev update`
      contient bien les sept workflows : `codev-propose,
      codev-explore, codev-apply, codev-sync, codev-archive,
      codev-update, codev-onboard`.
- [x] 4.5 `codev validate --strict --all` reste vert sur ce dépôt.
