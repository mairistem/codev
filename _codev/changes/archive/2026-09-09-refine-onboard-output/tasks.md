# Tâches

## 1. Body de la skill

- [x] 1.1 Éditer `assets/workflows/onboard.md`, bloc 2 (« ici, tu
      as ») : après la liste des changes actifs, ajouter une puce
      « Q change(s) archivé(s) » affichée **uniquement si Q > 0**.
      La skill compte les dossiers sous `_codev/changes/archive/` via
      `ls` ou une commande équivalente (Glob). Un projet neuf ou sans
      archive n'ajoute rien.
- [x] 1.2 Éditer le bloc 3 (« la suite »), branche « projet initialisé,
      aucun change actif » : nouvelle formulation qui invite à lire
      `README.md` d'abord, puis propose `/codev-propose <une-idée>`,
      et mentionne `/codev-explore <sujet>` comme alternative. Exemple
      de sortie attendue :
      > **La suite** : commence par lire `README.md` pour prendre le
      > pouls du projet. Puis, quand une idée émerge, tape
      > `/codev-propose <une-idée>`. Alternative si tu as une question
      > mais pas encore d'idée d'action : `/codev-explore <sujet>`.
- [x] 1.3 Les autres branches (change actif, plusieurs changes,
      `codev init` absent) restent inchangées.

## 2. Vérifications

- [x] 2.1 L'invariant `onboard_cite_ses_trois_blocs` continue de
      passer — les mots-clés `codev, c'est` / `ici, tu as` / `la
      suite` restent présents dans le body. Vérifié par
      `cargo test -p codev-agents onboard_cite_ses_trois_blocs`.
- [x] 2.2 `cargo test --workspace` reste vert.
- [x] 2.3 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 2.4 `codev validate --strict` reste vert.

## 3. Dogfooding

- [x] 3.1 Après `cargo install --path crates/codev-cli` puis
      `codev update`, relancer `/codev-onboard` sur ce dépôt et
      vérifier de visu :
      - la ligne « 11 change(s) archivé(s) » apparaît dans « ici, tu
        as » ;
      - le bloc « la suite » cite `README.md` avant `/codev-propose`.
