# Tasks

## 1. `ProcessRunner` port and in-memory impl

- [x] 1.1 Add `pub trait ProcessRunner` in `codev-engine::ports`
      with `fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>)
      -> io::Result<ProcessOutput>`. Verified by
      `ports::process_runner_trait_est_defini`.
- [x] 1.2 Real impl `RealProcessRunner` that calls `std::process::Command`,
      converts `ExitStatus` into `exit_code: i32`. Verified by an
      integration test that calls `echo` and parses the output.
- [x] 1.3 In-memory impl `MockProcessRunner` that takes a list of
      `(program, args_prefix, response)` and returns the first matching
      response. Verified by
      `ports::mock_process_runner_repond_aux_commandes_attendues`.
- [x] 1.4 Handle the "binary not found" case: `io::Error` with kind
      `NotFound` on an unknown `program` → `ProcessOutput` with
      `exit_code = 127` and a descriptive stderr — consistent with the
      shell. Verified by
      `ports::real_process_runner_signale_un_binaire_absent`.

## 2. Cache and disk layout

- [x] 2.1 `codev-engine::sources::cache::root(env, fs) -> PathBuf` —
      returns `$XDG_CACHE_HOME/codev/` or `~/.cache/codev/`. Verified by
      `cache::respecte_xdg_cache_home` and `cache::retombe_sur_home_cache`.
- [x] 2.2 `url_hash(url: &str) -> String` — hex sha256 truncated to the
      first 16 characters. Verified by
      `cache::hash_est_stable_pour_une_meme_url` and
      `cache::hash_differe_pour_des_url_distinctes`.
- [x] 2.3 Derived paths: `bare_repo_dir(cache_root, url_hash)` and
      `content_dir(cache_root, sha)`. Verified by
      `cache::chemins_calcules_correctement`.

## 3. Pure function `plan_sources_update`

- [x] 3.1 Create `codev-engine::sources::update` with `pub struct
      SourcesUpdatePlan { fetches: Vec<Fetch>, worktrees: Vec<Worktree>,
      lock_entries: Vec<LockEntry>, diff: Vec<PinChange> }`. Verified by
      a compiling test `update::plan_types_sont_constructibles`.
- [x] 3.2 `plan_sources_update(inherits_git: &[GitSource], current_lock:
      Option<&Lockfile>, resolutions: &[(url, ref, sha)]) ->
      SourcesUpdatePlan` — pure, no I/O. Verified by
      `update::plan_produit_un_fetch_par_nouvelle_source`.
- [x] 3.3 Diff computation: new pin → `PinChange::Added`, changed SHA
      → `PinChange::Moved { from, to }`, identical SHA →
      `PinChange::Unchanged`. Verified by
      `update::diff_distingue_add_move_unchanged`.

## 4. Execution `run_sources_update`

- [x] 4.1 `pub fn run_sources_update(fs, env, runner, layout, config) ->
      Result<UpdateOutcome, EngineError>`: (a) collects the `git:` sources
      from the config, (b) for each one calls `git ls-remote <url>
      <ref>` via `runner` to obtain the SHA, (c) builds the plan via
      `plan_sources_update`, (d) executes the fetches (`git fetch --depth
      1 --filter=blob:none`), (e) creates the worktrees (`git worktree add
      <content-dir> <sha>`), (f) writes `codev.lock`. Verified by
      `update::run_avec_mock_runner_ecrit_le_lock`.
- [x] 4.2 Idempotence: two successive `run_sources_update` runs against
      the same server leave the lock unchanged (content-to-content
      comparison before writing). Verified by
      `update::deuxieme_update_ne_reecrit_pas_le_lock`.
- [x] 4.3 `git` missing → `EngineError` with code `git_not_found` and an
      explicit message. Verified by `update::git_absent_est_signale`.

## 5. TOML lock format

- [x] 5.1 Add `toml = "0.8"` to the workspace dependencies. Verified
      by `cargo build`.
- [x] 5.2 `codev-engine::sources::lockfile` with `pub struct Lockfile {
      version: u32, sources: Vec<LockEntry> }`, `pub struct LockEntry {
      git: String, ref_: String, subpath: Option<String>, commit: String,
      resolved_at: String }`. TOML serialization tested by
      `lockfile::serialise_puis_reparse_est_identite`.
- [x] 5.3 `parse(source: &str) -> Result<Lockfile, LockfileError>` and
      `load(fs, path) -> Result<Option<Lockfile>, EngineError>`. Missing
      file = None (no error). Verified by
      `lockfile::absent_est_none`.
- [x] 5.4 `ref` field compatibility (a TOML keyword): serialize under the
      name `ref` but read into a Rust field `ref_`. Verified by
      `lockfile::le_champ_ref_est_ecrit_sans_backtick`.

## 6. Integration into `config::resolve`

- [x] 6.1 Extend `config::resolve` to read `_codev/codev.lock` and
      resolve each `inherits: git:` into a `<cache>/content/<sha>/` path.
      Verified by
      `config::inherits_git_est_resolu_depuis_le_lock`.
- [x] 6.2 Unlocked git source → `git_source_unlocked` warning + path not
      exposed. Verified by
      `config::git_sans_lock_emet_warning_dedie`.
- [x] 6.3 Git source locked but cache missing → `git_source_needs_update`
      warning. Verified by
      `config::git_avec_lock_sans_cache_emet_warning`.
- [x] 6.4 Remove the (obsolete) `inherit_git_unsupported` warning and
      update the existing tests. Verified by
      `config::une_source_git_ne_produit_plus_le_warning_de_non_support`.

## 7. `.md`/`.yaml` filtering on read

- [x] 7.1 The decision index loader already filters by `.md` (already in
      place via `walk_files` + filter). Add a dedicated test that checks
      that a `_codev/decisions/hook.sh` file placed in an inherited source
      does not appear in the index. Verified by
      `decisions::hook_sh_dans_source_heritee_est_ignore` (the spec's
      scenario of the same name).
- [x] 7.2 Extend the inherited config index to accept only
      `config.yaml` — no `config.rb`, `config.py`, etc. Verified by
      `config::seule_config_yaml_est_lue_dans_une_source_heritee`.

## 8. CLI commands

- [x] 8.1 Add the `Command::Sources` group with subcommands
      `List`, `Update`, `Show`. Verified by
      `cli_sources::sous_commandes_declarees`.
- [x] 8.2 `codev sources list [--json]` — assembles the state of each
      source: `SourceState { type, address, state, path?, sha?, ref? }`.
      Verified by `cli_sources::list_montre_letat_de_chaque_source`
      (scenario "List shows the state of each source").
- [x] 8.3 `codev sources update [--json]` — calls
      `run_sources_update`, renders the diff in human output + a JSON
      report `SourcesUpdateReportV1 { changes: [PinChange], root, status
      }`. Verified by `cli_sources::update_montre_le_diff_avant_ecriture`
      (scenario "Diff before writing a moved pin").
- [x] 8.4 `codev sources show <ref> [--json]` — resolves by URL or
      path, returns `SourceDetailV1 { type, address, ref?, sha?, path,
      files_exposed: [String] }`. Verified by
      `cli_sources::show_pointe_vers_le_cache_resolu` (scenario of the
      same name).
- [x] 8.5 All command errors carry a stable code:
      `git_not_found`, `git_source_unlocked`, `git_source_needs_update`,
      `unknown_source`. Verified by
      `cli_sources::codes_derreur_sont_stables`.

## 9. JSON contract v1

- [x] 9.1 Three new types in `contract::v1`, camelCase:
      `SourceStateV1 { type, address, state, path?, sha?, ref?,
      subpath? }`, `SourcesListReportV1 { root, sources, status }`,
      `SourcesUpdateReportV1 { root, changes, status }`,
      `SourceDetailV1 { type, address, ref?, sha?, path, filesExposed,
      status }`. Verified by
      `contract::sources_shapes_stables`.
- [x] 9.2 `origin` fields extended to the `git:<url>` format in
      `DecisionV1` (already in place via `format!("git:{url}")` — a test
      confirms it). Verified by
      `contract::decision_origin_git_serialise_correctement`.

## 10. Human rendering

- [x] 10.1 `render::sources_list`: one line per source, format
      `<type> <address> [<state>]`. Verified by
      `render::liste_marque_letat_de_chaque_source`.
- [x] 10.2 `render::sources_update`: shows the diff `<url>: <old> →
      <new>` line by line, then a summary. Verified by
      `render::update_montre_le_diff`.
- [x] 10.3 `render::sources_show`: URL, ref, SHA, cache path, list of
      exposed files. Verified by
      `render::show_affiche_les_details_dune_source`.

## 11. Dogfooding

- [x] 11.1 Create a small local git repository in a temporary folder
      with a few ADRs, point an `inherits: git:` source of this project
      at that repository via a `file://` URL, run
      `codev sources update`, check that a `codev.lock` is written and
      that `codev decision list` shows the remote ADRs.
- [x] 11.2 Check that `codev sources show file:///…` does list the
      exposed files (only `.md` and `.yaml`).
- [x] 11.3 Check that `codev decision list` on this repository still
      shows the 6 local ADRs plus the remote ADRs (no id conflict
      expected if the remote ADRs carry different numbers).

## 12. Workspace integration

- [x] 12.1 `cargo test --workspace` stays green and counts at least 30
      additional tests (port + cache + lock + config +
      commands + rendering).
- [x] 12.2 `cargo clippy --workspace --all-targets` stays
      warning-free.
- [x] 12.3 `codev validate --all` stays green.
