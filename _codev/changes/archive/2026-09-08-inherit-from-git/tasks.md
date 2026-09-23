# Tâches

## 1. Port `ProcessRunner` et impl en mémoire

- [x] 1.1 Ajouter `pub trait ProcessRunner` dans `codev-engine::ports`
      avec `fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>)
      -> io::Result<ProcessOutput>`. Vérifié par
      `ports::process_runner_trait_est_defini`.
- [x] 1.2 Impl réelle `RealProcessRunner` qui appelle `std::process::Command`,
      convertit `ExitStatus` en `exit_code: i32`. Vérifié par un test
      d'intégration qui appelle `echo` et parse la sortie.
- [x] 1.3 Impl en mémoire `MockProcessRunner` qui prend une liste de
      `(program, args_prefix, response)` et rend la première réponse
      correspondante. Vérifié par
      `ports::mock_process_runner_repond_aux_commandes_attendues`.
- [x] 1.4 Traiter le cas « binaire introuvable » : `io::Error` avec kind
      `NotFound` sur un `program` inconnu → `ProcessOutput` avec
      `exit_code = 127` et stderr descriptif — cohérent avec le shell.
      Vérifié par
      `ports::real_process_runner_signale_un_binaire_absent`.

## 2. Cache et disposition disque

- [x] 2.1 `codev-engine::sources::cache::root(env, fs) -> PathBuf` — rend
      `$XDG_CACHE_HOME/codev/` ou `~/.cache/codev/`. Vérifié par
      `cache::respecte_xdg_cache_home` et `cache::retombe_sur_home_cache`.
- [x] 2.2 `url_hash(url: &str) -> String` — sha256 hex tronqué aux 16
      premiers caractères. Vérifié par
      `cache::hash_est_stable_pour_une_meme_url` et
      `cache::hash_differe_pour_des_url_distinctes`.
- [x] 2.3 Chemins dérivés : `bare_repo_dir(cache_root, url_hash)` et
      `content_dir(cache_root, sha)`. Vérifié par
      `cache::chemins_calcules_correctement`.

## 3. Fonction pure `plan_sources_update`

- [x] 3.1 Créer `codev-engine::sources::update` avec `pub struct
      SourcesUpdatePlan { fetches: Vec<Fetch>, worktrees: Vec<Worktree>,
      lock_entries: Vec<LockEntry>, diff: Vec<PinChange> }`. Vérifié par
      un test compilant `update::plan_types_sont_constructibles`.
- [x] 3.2 `plan_sources_update(inherits_git: &[GitSource], current_lock:
      Option<&Lockfile>, resolutions: &[(url, ref, sha)]) ->
      SourcesUpdatePlan` — pure, aucune I/O. Vérifié par
      `update::plan_produit_un_fetch_par_nouvelle_source`.
- [x] 3.3 Calcul du diff : nouveau pin → `PinChange::Added`, SHA changé
      → `PinChange::Moved { from, to }`, SHA identique →
      `PinChange::Unchanged`. Vérifié par
      `update::diff_distingue_add_move_unchanged`.

## 4. Exécution `run_sources_update`

- [x] 4.1 `pub fn run_sources_update(fs, env, runner, layout, config) ->
      Result<UpdateOutcome, EngineError>` : (a) collecte les sources
      `git:` de la config, (b) pour chacune appelle `git ls-remote <url>
      <ref>` via `runner` pour obtenir le SHA, (c) construit le plan via
      `plan_sources_update`, (d) exécute les fetches (`git fetch --depth
      1 --filter=blob:none`), (e) crée les worktrees (`git worktree add
      <content-dir> <sha>`), (f) écrit `codev.lock`. Vérifié par
      `update::run_avec_mock_runner_ecrit_le_lock`.
- [x] 4.2 Idempotence : deux `run_sources_update` successifs avec le
      même serveur laissent le lock inchangé (comparaison
      contenu-à-contenu avant écriture). Vérifié par
      `update::deuxieme_update_ne_reecrit_pas_le_lock`.
- [x] 4.3 `git` absent → `EngineError` de code `git_not_found` avec
      message explicite. Vérifié par `update::git_absent_est_signale`.

## 5. Format lock TOML

- [x] 5.1 Ajouter `toml = "0.8"` aux dépendances du workspace. Vérifié
      par `cargo build`.
- [x] 5.2 `codev-engine::sources::lockfile` avec `pub struct Lockfile {
      version: u32, sources: Vec<LockEntry> }`, `pub struct LockEntry {
      git: String, ref_: String, subpath: Option<String>, commit: String,
      resolved_at: String }`. Sérialisation TOML testée par
      `lockfile::serialise_puis_reparse_est_identite`.
- [x] 5.3 `parse(source: &str) -> Result<Lockfile, LockfileError>` et
      `load(fs, path) -> Result<Option<Lockfile>, EngineError>`. Fichier
      absent = None (pas d'erreur). Vérifié par
      `lockfile::absent_est_none`.
- [x] 5.4 Compatibilité champ `ref` (mot-clé TOML) : sérialiser sous le
      nom `ref` mais lire depuis un champ Rust `ref_`. Vérifié par
      `lockfile::le_champ_ref_est_ecrit_sans_backtick`.

## 6. Intégration à `config::resolve`

- [x] 6.1 Étendre `config::resolve` pour lire `_codev/codev.lock` et
      résoudre chaque `inherits: git:` en chemin `<cache>/content/<sha>/`.
      Vérifié par
      `config::inherits_git_est_resolu_depuis_le_lock`.
- [x] 6.2 Source git non verrouillée → warning `git_source_unlocked` +
      chemin non exposé. Vérifié par
      `config::git_sans_lock_emet_warning_dedie`.
- [x] 6.3 Source git verrouillée mais cache absent → warning
      `git_source_needs_update`. Vérifié par
      `config::git_avec_lock_sans_cache_emet_warning`.
- [x] 6.4 Retirer le warning `inherit_git_unsupported` (obsolète) et
      mettre à jour les tests existants. Vérifié par
      `config::une_source_git_ne_produit_plus_le_warning_de_non_support`.

## 7. Filtrage `.md`/`.yaml` à la lecture

- [x] 7.1 Le loader d'index de décisions filtre déjà par `.md` (déjà en
      place via `walk_files` + filtre). Ajouter un test dédié qui
      vérifie qu'un fichier `_codev/decisions/hook.sh` déposé dans une
      source héritée n'apparaît pas dans l'index. Vérifié par
      `decisions::hook_sh_dans_source_heritee_est_ignore` (scénario
      homonyme de la spec).
- [x] 7.2 Étendre l'index de config héritée pour n'accepter que le
      `config.yaml` — pas de `config.rb`, `config.py`, etc. Vérifié par
      `config::seule_config_yaml_est_lue_dans_une_source_heritee`.

## 8. Commandes CLI

- [x] 8.1 Ajouter le groupe `Command::Sources` avec sous-commandes
      `List`, `Update`, `Show`. Vérifié par
      `cli_sources::sous_commandes_declarees`.
- [x] 8.2 `codev sources list [--json]` — assemble l'état de chaque
      source : `SourceState { type, address, state, path?, sha?, ref? }`.
      Vérifié par `cli_sources::list_montre_letat_de_chaque_source`
      (scénario « List montre l'état de chaque source »).
- [x] 8.3 `codev sources update [--json]` — appelle
      `run_sources_update`, rend le diff en sortie humaine + un rapport
      JSON `SourcesUpdateReportV1 { changes: [PinChange], root, status
      }`. Vérifié par `cli_sources::update_montre_le_diff_avant_ecriture`
      (scénario « Diff avant écriture d'un pin déplacé »).
- [x] 8.4 `codev sources show <ref> [--json]` — résout par URL ou
      chemin, rend `SourceDetailV1 { type, address, ref?, sha?, path,
      files_exposed: [String] }`. Vérifié par
      `cli_sources::show_pointe_vers_le_cache_resolu` (scénario
      homonyme).
- [x] 8.5 Toutes les erreurs des commandes portent un code stable :
      `git_not_found`, `git_source_unlocked`, `git_source_needs_update`,
      `unknown_source`. Vérifié par
      `cli_sources::codes_derreur_sont_stables`.

## 9. Contrat JSON v1

- [x] 9.1 Trois nouveaux types dans `contract::v1`, camelCase :
      `SourceStateV1 { type, address, state, path?, sha?, ref?,
      subpath? }`, `SourcesListReportV1 { root, sources, status }`,
      `SourcesUpdateReportV1 { root, changes, status }`,
      `SourceDetailV1 { type, address, ref?, sha?, path, filesExposed,
      status }`. Vérifié par
      `contract::sources_shapes_stables`.
- [x] 9.2 Champs `origin` étendus au format `git:<url>` dans
      `DecisionV1` (déjà en place via `format!("git:{url}")` — un test
      confirme). Vérifié par
      `contract::decision_origin_git_serialise_correctement`.

## 10. Rendu humain

- [x] 10.1 `render::sources_list` : une ligne par source, format
      `<type> <address> [<state>]`. Vérifié par
      `render::liste_marque_letat_de_chaque_source`.
- [x] 10.2 `render::sources_update` : affiche le diff `<url>: <old> →
      <new>` ligne par ligne, puis un résumé. Vérifié par
      `render::update_montre_le_diff`.
- [x] 10.3 `render::sources_show` : URL, ref, SHA, chemin cache, liste
      des fichiers exposés. Vérifié par
      `render::show_affiche_les_details_dune_source`.

## 11. Dogfooding

- [x] 11.1 Créer un mini dépôt git local dans un dossier temporaire
      avec quelques ADR, faire pointer une source `inherits: git:` de
      ce projet vers ce dépôt via une URL `file://`, lancer
      `codev sources update`, vérifier qu'un `codev.lock` est écrit et
      que `codev decision list` fait apparaître les ADR distants.
- [x] 11.2 Vérifier que `codev sources show file:///…` liste bien les
      fichiers exposés (uniquement `.md` et `.yaml`).
- [x] 11.3 Vérifier que `codev decision list` sur ce dépôt continue à
      afficher les 6 ADR locaux plus les ADR distants (aucun conflit
      d'id attendu si les ADR distants portent des numéros différents).

## 12. Intégration workspace

- [x] 12.1 `cargo test --workspace` reste vert et compte au moins 30
      tests supplémentaires (port + cache + lock + config +
      commandes + rendu).
- [x] 12.2 `cargo clippy --workspace --all-targets` reste sans
      avertissement.
- [x] 12.3 `codev validate --all` reste vert.
