# Tâches

## 1. Dépendance

- [x] 1.1 Ajouter `clap_complete = "4"` aux dépendances de
      `crates/codev-cli/Cargo.toml`. Vérifier que la version majeure
      correspond à celle de `clap` déjà utilisé (aujourd'hui `4`).

## 2. Sous-commande CLI

- [x] 2.1 Ajouter la variante `Command::Completions { shell:
      clap_complete::Shell }` à l'enum `Command` dans
      `crates/codev-cli/src/main.rs`. Documentation clap : « Génère
      un script de complétion shell pour l'installation locale. »
- [x] 2.2 Documenter en tête du variant les commandes typiques
      d'installation par shell (bash, zsh, fish, powershell,
      elvish) — le texte apparaît dans `codev completions --help`.
- [x] 2.3 Ajouter le bras du `match` dans `run(cli)` :
      ```rust
      Command::Completions { shell } => {
          use clap::CommandFactory;
          let mut cmd = Cli::command();
          clap_complete::generate(shell, &mut cmd, "codev", &mut std::io::stdout());
          0
      }
      ```

## 3. Tests

- [x] 3.1 Test dans `main.rs` (ou module dédié) qui, pour chacun des
      cinq shells, appelle la génération via `clap_complete::generate`
      dans un buffer et vérifie :
      - la sortie est non vide (> 200 octets, la borne inférieure
        typique) ;
      - la sortie contient au moins une fois `codev`.
- [x] 3.2 Test de refus d'un shell inconnu — vérifié
      structurellement : `clap` refuse à l'analyse d'arguments (via
      `ValueEnum` sur `clap_complete::Shell`), on n'a rien à écrire
      côté nous. Un test « `codev completions nushell` remonte une
      erreur clap » est facultatif.

## 4. Documentation

- [x] 4.1 Le message d'aide de `codev completions --help` cite les
      procédures d'installation par shell (bash, zsh, fish,
      powershell). Placé dans le doc-comment de la variante.
- [x] 4.2 Le `README.md` du dépôt (ou une section dédiée) mentionne
      brièvement `codev completions` et pointe vers `--help` pour le
      détail.

## 5. Dogfooding et intégration workspace

- [x] 5.1 Après `cargo install --path crates/codev-cli`, lancer
      `codev completions zsh | head -20` — vérifier de visu que la
      sortie est un script zsh cohérent.
- [x] 5.2 Installer réellement dans le shell (`codev completions zsh
      > "${fpath[1]}/_codev"` puis `compinit`), taper `codev de<TAB>`
      dans un nouveau shell — la complétion doit lister `decision`,
      `deviate`, etc.
- [x] 5.3 `cargo test --workspace` reste vert, gagne au moins 1 test
      (génération des cinq shells).
- [x] 5.4 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 5.5 `codev validate --strict` sur ce dépôt reste vert.
