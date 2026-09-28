# Tâches

## 1. Dépendance `dialoguer`

- [x] 1.1 Ajouter `dialoguer = { version = "0.11", default-features = false, features = ["editor", "fuzzy-select"] }` aux `workspace.dependencies` de `Cargo.toml` (racine). Vérifié par `cargo check --workspace` vert.
- [x] 1.2 Déclarer la dépendance dans `crates/codev-cli/Cargo.toml` via `dialoguer.workspace = true`. Vérifié par `cargo tree -p codev-cli | grep dialoguer` non vide.

## 2. Module `codev-core::detect`

- [x] 2.1 Créer `crates/codev-core/src/detect/mod.rs` — expose la struct `Detected` avec les champs `stack: Option<Stack>`, `project_name: Option<String>`, `license: Option<String>`, `has_ci: bool`, `is_git_repo: bool`, `mcps: Vec<DetectedMcp>`. Ajouter `pub mod detect;` dans `lib.rs`. Vérifié par `cargo build -p codev-core`.
- [x] 2.2 `detect/stack.rs` — fonctions `from_cargo_toml(&[u8]) -> Option<Stack>`, `from_package_json`, `from_pyproject_toml`, `from_go_mod`, `from_pom_xml`. Chaque fonction retourne `Some(Stack { language, edition_or_version, workspace_crate_count, dependencies_summary })`. Tests unitaires golden par fixture (au moins un `Cargo.toml` workspace + un `package.json` React).
- [x] 2.3 `detect/license.rs` — regex sur les noms courants (MIT / Apache-2.0 / BSD-3-Clause / GPL-3.0 / MPL-2.0). Test : chaque fixture connue est reconnue ; un texte quelconque retourne `None`.
- [x] 2.4 `detect/mcp.rs` — `parse_mcp_config(&[u8]) -> Vec<DetectedMcp>` qui lit `mcpServers` d'un JSON, retient nom + command + url + args. `matches_jira(&DetectedMcp) -> bool` avec regex `(?i)jira|atlassian`. `tool_id(name, tool_suffix) -> String` qui applique la normalisation nom → `mcp__<name_normalized>__<tool_suffix>`. Tests : trois fixtures JSON (Atlassian Rovo, Atlassian classic, une non-Jira), plus une chaîne d'espaces + points pour la normalisation.
- [x] 2.5 Fonction publique `detect::run(fs: &dyn FileSystem, root: &Path, home: &Path) -> Detected` qui orchestre les sondes : tente Cargo puis package.json puis…, ouvre `LICENSE`, teste `.github/workflows/`, ouvre `.git/`, lit les 4 fichiers MCP et fusionne (le premier gagne). Test intégration avec un `MemoryFileSystem` prérempli reproduisant le workspace codev — la sortie contient stack Rust workspace + MCP Atlassian. *(Orchestrateur placé dans `codev-engine::detect` — le port `FileSystem` vit dans engine, l'invariant « aucune I/O dans core » est préservé. Parseurs purs dans `codev-core::detect`.)*

## 3. Module `codev-core::config::generate`

- [x] 3.1 Créer `crates/codev-core/src/config/mod.rs` (ou `config.rs` si le module n'existe pas encore) exposant la struct `GeneratedConfig` avec les champs typés portant valeur + commentaire optionnel de provenance. Vérifié : compile.
- [x] 3.2 `render(&GeneratedConfig) -> String` — assemble un YAML manuel, ordre stable des clés, commentaires de provenance imprimés sur la ligne au-dessus. Test golden : un `GeneratedConfig` fixe produit exactement le YAML attendu (octets près).
- [x] 3.3 Test « rendu → relecture » — le YAML produit MUST parser sans erreur via `serde_norway::from_str::<ProjectConfig>()` du module `codev-engine`. Test intégration cross-crate. *(Placé dans `crates/codev-engine/tests/config_generate_roundtrip.rs` — sens de dépendance respecté.)*

## 4. Passerelle Detected → GeneratedConfig

- [x] 4.1 `codev-core::config::from_detected(detected: &Detected, choices: &UserChoices) -> GeneratedConfig` — assemble la config finale à partir de la détection + des choix utilisateur (workflows, contexte libre). Fonction pure. Test unitaire : trois cas — full détection + choix full, détection vide + choix minimal, détection MCP seule + choix full.

## 5. Prompts CLI

- [x] 5.1 Créer `crates/codev-cli/src/init_prompts.rs` — expose `pub fn run(detected: &Detected, opts: &InitOptions) -> Result<UserChoices>`. `InitOptions` porte `--yes`, `--preset`, `--no-detect`. Court-circuite les prompts en mode non-interactif ou sans TTY.
- [x] 5.2 Détection TTY : utiliser `std::io::IsTerminal` (stable depuis 1.70). Test unitaire pour la fonction de décision `should_prompt(opts, is_tty) -> bool`.
- [x] 5.3 Prompt workflows : `dialoguer::Select` avec les trois presets, défaut « Complet (7) ». Le preset `Personnalisé` ouvre un `MultiSelect` sur les 7 workflows.
- [x] 5.4 Prompt contexte : `dialoguer::Input` pour une ligne courte, ou Entrée sur ligne vide bascule vers `dialoguer::Editor` avec un squelette prérempli issu de la stack détectée. Test manuel documenté dans `tasks.md` — les fonctions `dialoguer` ne se testent pas unitairement.
- [x] 5.5 Confirmation MCP : si `detected.mcps` a un candidat non ambigu, affiche `✓ MCP Jira détecté : <tool_id>` sans prompt en `--yes`, avec `Confirm` en interactif. Si plusieurs candidats : `Select` court.

## 6. Refonte de `codev-cli::commands::init`

- [x] 6.1 Ajouter les flags CLI dans `crates/codev-cli/src/main.rs` : `--yes` (`-y`), `--no-detect`, `--preset <complet|minimal|personnalise>`. `--force` inchangé. Vérifié par `codev init --help` qui les liste.
- [x] 6.2 Refondre `commands::init` pour orchestrer : `detect::run` (sauf `--no-detect`) → `init_prompts::run` → `config::from_detected` → `render` → écrire `_codev/config.yaml` **avant** le scaffolding existant, puis appeler `install_skills`. La logique actuelle qui repose sur le template commenté disparaît.
- [x] 6.3 Cas « projet déjà initialisé » : si `_codev/config.yaml` existe, `codev init` **ne re-génère pas** le fichier ; il continue en installation des skills seulement (comportement idempotent actuel). Test unitaire *(couvert par `relancer_init_ne_change_rien` et `init_respecte_les_workflows_deja_configures`)*.
- [x] 6.4 Test intégration bout-en-bout via `MemoryFileSystem` : `codev init --yes` sur un dossier avec `Cargo.toml` workspace + `.mcp.json` Atlassian produit un `_codev/config.yaml` contenant les 7 workflows, la clé `mcp.jira_tool` prérequise, le `context:` avec la stack détectée, et les 7 skills installées.

## 7. Retour de `DEFAULT_WORKFLOWS`

- [x] 7.1 Modifier `crates/codev-agents/src/workflows.rs` — `DEFAULT_WORKFLOWS` passe de `&["propose", "explore", "onboard"]` à `&["propose", "explore", "onboard", "apply", "sync", "archive", "update"]`.
- [x] 7.2 Réécrire le test `sans_demande_installe_le_catalogue_par_defaut` : l'assertion sur les 3 workflows devient une assertion sur les 7 ; la boucle « autres opt-in restent opt-in » disparaît. Ajouter un test complémentaire « restriction opt-out via workflows explicite » qui appelle `select(Some(&["propose"]))` et vérifie qu'une seule skill est retenue.
- [x] 7.3 Vérifier que le test `init_respecte_les_workflows_deja_configures` de `codev-cli/src/commands.rs` reste vert (il fixe un config `workflows: - explore` explicite, ce qui doit continuer à produire une seule skill).

## 8. Fichier config template

- [x] 8.1 `crates/codev-engine/src/scaffold.rs` — `DEFAULT_CONFIG` (le template YAML avec workflows commentés) n'est plus utilisé au moment de `init` (la génération le remplace) mais **reste utilisé** pour `codev update` sur un projet où le fichier a été supprimé à la main. Vérifier qu'aucun test n'en dépend au-delà de ça, et documenter le rôle résiduel dans un commentaire au-dessus de la constante.

## 9. Spec principale — merge des deltas

- [ ] 9.1 Vérifier que `codev validate init-interactive-with-detection` est vert avant l'archive (les deux deltas — `init` ADDED et `skills` MODIFIED — sont bien formés).
- [ ] 9.2 Pas d'action ici : la fusion dans `_codev/specs/init/spec.md` (nouveau) et la mise à jour de `_codev/specs/skills/spec.md` sont faites par `codev archive`, pas dans `apply`.

## 10. Documentation

- [x] 10.1 `docs/codev.md` §2 (Installation) — ajouter une sous-section « L'expérience `codev init` » qui décrit en 15-20 lignes le nouveau flux : sonde, deux questions, provenance dans le YAML, flags `--yes` et `--preset`.
- [x] 10.2 `docs/codev.md` §7 (Configuration) — mettre à jour la mention de `workflows:` pour refléter le nouveau défaut (7 au lieu de 3), en la présentant comme voie **opt-out**.
- [x] 10.3 `CHANGELOG.md` — nouvelle entrée `[Unreleased]` en tête avec section `### Changed` : « Défaut de `codev init` et `codev update` : les 7 workflows sont installés par défaut. Un projet qui veut moins déclare `workflows:` explicite (voie opt-out). » et section `### Added` : « `codev init` interactif avec détection automatique de la stack et des MCPs configurés ; flags `--yes`, `--preset`, `--no-detect`. »

## 11. Vérifications finales

- [x] 11.1 `cargo test --workspace` vert (tous les nouveaux tests + les tests existants adaptés).
- [x] 11.2 `cargo clippy --workspace --all-targets` sans avertissement.
- [x] 11.3 `codev validate --strict` vert.
- [x] 11.4 Test manuel de bout en bout : dans un dossier vierge de scratchpad, `mkdir /tmp/codev-manual-test && cd /tmp/codev-manual-test && cargo init --lib` (crée un Cargo.toml), puis `codev init --yes`, vérifier que le `_codev/config.yaml` produit contient les 7 workflows et le `context:` avec « Rust ». *(Fait — 7 skills, `context: | Projet Rust, 2024.`, `codev status` OK.)*
- [ ] 11.5 Test manuel interactif : dans un TTY, `codev init` sans flag, répondre aux deux prompts, vérifier que le YAML final reflète les choix.

## 12. Livraison

- [ ] 12.1 Bump `Cargo.toml` à `0.3.0` — **minor** parce que le comportement de défaut change (les 7 workflows au lieu de 3 est un changement notable observable). SemVer 0.x autorise ce genre de bascule sur bump minor. Tag `v0.3.0`, `git push origin main --tags`.
- [ ] 12.2 Ajouter l'entrée `[0.3.0] — <date>` dans `CHANGELOG.md` avec le contenu récapitulé de 10.3.
