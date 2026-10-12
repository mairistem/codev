//! The update check and `codev self-update`: the effects.
//!
//! Every decision is taken by `codev_core::update`; this module reads and
//! writes the cache, calls the [`Http`] port, stages and swaps the binary
//! through the [`Executable`] port, extracts with `tar` and runs the new
//! binary's `codev update` through the [`ProcessRunner`] port.

use std::path::{Path, PathBuf};

use codev_core::update::{
    self as core, CheckDecision, CheckEnv, Decision, InstallMethod, Release, UpdateCache,
    UpdateNotice,
};
use thiserror::Error;

use crate::error::Warning;
use crate::ports::{Clock, Env, Executable, FileSystem, Http, ProcessRunner};
use crate::{root, sources};

/// The background check must never slow a command down noticeably.
pub const CHECK_TIMEOUT_SECS: u64 = 2;

/// An archive is a few megabytes; a slow connection still gets there.
pub const DOWNLOAD_TIMEOUT_SECS: u64 = 300;

pub const CACHE_FILE: &str = "update-check.json";

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("cannot reach the codev release API: {0}")]
    CheckFailed(String),

    #[error("no published codev binary for this platform ({target}); build it from source")]
    UnsupportedPlatform { target: String },

    #[error("cannot download {url}: {reason}")]
    DownloadFailed { url: String, reason: String },

    #[error("{0}")]
    ChecksumMismatch(String),

    #[error("cannot extract the codev binary from {asset}: {reason}")]
    ExtractFailed { asset: String, reason: String },

    #[error(
        "cannot write {path}: {reason} — reinstall codev where you can write, or run with the needed rights"
    )]
    NotWritable { path: PathBuf, reason: String },

    #[error(
        "codev {target} is older than the running {current}; \
         pass `--version {target}` to downgrade on purpose"
    )]
    DowngradeRefused { current: String, target: String },

    #[error("`{0}` is not a version; use `X.Y.Z`, for example `0.6.0`")]
    InvalidVersion(String),

    #[error("cannot locate the running codev binary: {0}")]
    NoExecutable(String),
}

impl UpdateError {
    /// Stable code, exposed in the `status[]` array of the JSON contract.
    pub fn code(&self) -> &'static str {
        match self {
            Self::CheckFailed(_) => "update_check_failed",
            Self::UnsupportedPlatform { .. } => "update_unsupported_platform",
            Self::DownloadFailed { .. } => "update_download_failed",
            Self::ChecksumMismatch(_) => "update_checksum_mismatch",
            Self::ExtractFailed { .. } => "update_extract_failed",
            Self::NotWritable { .. } => "update_not_writable",
            Self::DowngradeRefused { .. } => "update_downgrade_refused",
            Self::InvalidVersion(_) => "invalid_version",
            Self::NoExecutable(_) => "update_not_writable",
        }
    }
}

// ─────────────────────────────── cache and check ───────────────────────────────

/// `<codev cache root>/update-check.json`, next to the inherited sources'
/// cache.
pub fn cache_path(env: &dyn Env) -> PathBuf {
    sources::cache::root(env).join(CACHE_FILE)
}

pub fn read_cache(fs: &dyn FileSystem, env: &dyn Env) -> Option<UpdateCache> {
    let raw = fs.read_to_string(&cache_path(env)).ok()?;
    UpdateCache::parse(&raw)
}

/// A cache that cannot be written only means the next command checks
/// again: never worth failing for.
fn write_cache(fs: &dyn FileSystem, env: &dyn Env, cache: &UpdateCache) {
    let _ = fs.write(&cache_path(env), &cache.render());
}

pub fn check_env(env: &dyn Env) -> CheckEnv {
    CheckEnv::from_vars(
        env.var("CI").as_deref(),
        env.var("CODEV_NO_UPDATE_CHECK").as_deref(),
    )
}

/// The latest release, asked to the API now.
pub fn fetch_latest(http: &dyn Http, timeout_secs: u64) -> Result<Release, UpdateError> {
    let body = http
        .get(core::RELEASES_API, timeout_secs)
        .map_err(|e| UpdateError::CheckFailed(e.to_string()))?;
    core::parse_latest_release(&String::from_utf8_lossy(&body))
        .ok_or_else(|| UpdateError::CheckFailed("the answer names no release".to_string()))
}

/// What `codev status` reports: refreshes the cache when it is older than
/// a day, with one request; a failure is recorded and stays silent.
pub fn status_notice(
    fs: &dyn FileSystem,
    env: &dyn Env,
    clock: &dyn Clock,
    http: &dyn Http,
    current: &str,
) -> Option<UpdateNotice> {
    let now = clock.now_unix();
    let cached = read_cache(fs, env);
    let cache = match core::should_check(check_env(env), cached.as_ref(), now) {
        CheckDecision::Disabled => return None,
        CheckDecision::UseCache => cached?,
        CheckDecision::Refresh => {
            let fetched = fetch_latest(http, CHECK_TIMEOUT_SECS).ok();
            let cache = core::record_check(cached.as_ref(), fetched.as_ref(), now);
            write_cache(fs, env, &cache);
            cache
        }
    };
    core::notice(&cache, current, now)
}

/// What `codev doctor` reads: the cache only, never a request.
pub fn cached_notice(
    fs: &dyn FileSystem,
    env: &dyn Env,
    clock: &dyn Clock,
    current: &str,
) -> Option<UpdateNotice> {
    if check_env(env).disabled() {
        return None;
    }
    core::notice(&read_cache(fs, env)?, current, clock.now_unix())
}

/// Removes the binary a Windows update moved aside. Best effort: a
/// leftover file is harmless, a failing command is not.
pub fn cleanup_previous_binary(exe: &dyn Executable) {
    if let Ok(current) = exe.current_exe() {
        let (_, aside) = core::staging_paths(&current, "");
        let _ = exe.remove_file(&aside);
    }
}

// ─────────────────────────────── self-update ───────────────────────────────

/// The ports `codev self-update` needs.
pub struct Ports<'a> {
    pub fs: &'a dyn FileSystem,
    pub env: &'a dyn Env,
    pub clock: &'a dyn Clock,
    pub http: &'a dyn Http,
    pub runner: &'a dyn ProcessRunner,
    pub exe: &'a dyn Executable,
}

#[derive(Debug, Clone, Default)]
pub struct Request {
    /// The running version.
    pub current: String,
    /// The build target triple of the running binary.
    pub target: String,
    /// `--version`.
    pub version: Option<String>,
    /// `--check`.
    pub check_only: bool,
    /// `--snooze`.
    pub snooze: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Updated,
    UpToDate,
    /// `--check` found a newer version.
    Available,
    Snoozed,
    NothingToSnooze,
    /// Installed by cargo, a build or a package manager.
    ManagedElsewhere,
    /// The user declined the confirmation.
    Cancelled,
}

impl Action {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Updated => "updated",
            Self::UpToDate => "up_to_date",
            Self::Available => "available",
            Self::Snoozed => "snoozed",
            Self::NothingToSnooze => "nothing_to_snooze",
            Self::ManagedElsewhere => "managed_elsewhere",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Outcome {
    pub current: String,
    /// The version targeted, when known.
    pub latest: Option<String>,
    pub url: Option<String>,
    pub action: Action,
    pub method: InstallMethod,
    pub path: PathBuf,
    /// The project root where the new binary ran `codev update`.
    pub project_updated: Option<PathBuf>,
    pub warnings: Vec<Warning>,
}

/// `codev self-update`. `confirm` is asked once, right before the
/// download, with `(current, target)`; returning `false` cancels.
pub fn self_update(
    ports: &Ports,
    request: &Request,
    confirm: &mut dyn FnMut(&str, &str) -> bool,
) -> Result<Outcome, UpdateError> {
    let path = ports
        .exe
        .current_exe()
        .map_err(|e| UpdateError::NoExecutable(e.to_string()))?;
    let method = core::install_method(
        &path.to_string_lossy(),
        ports.env.var("CARGO_HOME").as_deref(),
        ports.env.home_dir().as_deref().and_then(Path::to_str),
    );
    let current = core::parse_version(&request.current)
        .ok_or_else(|| UpdateError::InvalidVersion(request.current.clone()))?;
    let mut outcome = Outcome {
        current: current.to_string(),
        latest: None,
        url: None,
        action: Action::UpToDate,
        method,
        path: path.clone(),
        project_updated: None,
        warnings: Vec::new(),
    };

    if request.snooze {
        return Ok(snooze(ports, &request.current, outcome));
    }

    if method != InstallMethod::Standalone && !request.check_only {
        outcome.action = Action::ManagedElsewhere;
        return Ok(outcome);
    }

    // The target: `--version`, or the latest release, asked now whatever
    // the cache, `CI` or the opt-out — the user asked for it.
    let explicit = request.version.is_some();
    let (target, url) = match &request.version {
        Some(raw) => {
            let version = core::parse_version(raw)
                .filter(|v| v.pre.is_empty() && v.build.is_empty())
                .ok_or_else(|| UpdateError::InvalidVersion(raw.clone()))?;
            let url = core::release_page(&version.to_string());
            (version, url)
        }
        None => {
            let release = fetch_latest(ports.http, CHECK_TIMEOUT_SECS * 5)?;
            let now = ports.clock.now_unix();
            let cache = core::record_check(
                read_cache(ports.fs, ports.env).as_ref(),
                Some(&release),
                now,
            );
            write_cache(ports.fs, ports.env, &cache);
            let version = core::parse_version(&release.version)
                .ok_or_else(|| UpdateError::CheckFailed("unreadable release version".into()))?;
            (version, release.url)
        }
    };
    outcome.latest = Some(target.to_string());
    outcome.url = Some(url);

    match core::decide(&current, &target, explicit) {
        Decision::UpToDate => return Ok(outcome),
        Decision::RefuseDowngrade if !request.check_only => {
            return Err(UpdateError::DowngradeRefused {
                current: current.to_string(),
                target: target.to_string(),
            });
        }
        Decision::RefuseDowngrade => return Ok(outcome),
        Decision::Install => {}
    }
    if request.check_only {
        outcome.action = if target > current {
            Action::Available
        } else {
            Action::UpToDate
        };
        return Ok(outcome);
    }

    let version = target.to_string();
    let asset = core::asset_name(&version, &request.target).ok_or_else(|| {
        UpdateError::UnsupportedPlatform {
            target: request.target.clone(),
        }
    })?;
    if !confirm(&outcome.current, &version) {
        outcome.action = Action::Cancelled;
        return Ok(outcome);
    }

    let new_binary = download_and_extract(ports, &version, &request.target, &asset)?;
    install_binary(ports.exe, &path, &version, &new_binary)?;
    outcome.action = Action::Updated;

    // The installed release is the latest known one now: the notice must
    // not offer it again.
    if let Some(mut cache) = read_cache(ports.fs, ports.env)
        && cache
            .latest
            .as_deref()
            .and_then(core::parse_version)
            .is_none_or(|latest| latest <= target)
    {
        cache.latest = Some(version.clone());
        cache.url = outcome.url.clone();
        write_cache(ports.fs, ports.env, &cache);
    }

    refresh_project(ports, &path, &mut outcome);
    Ok(outcome)
}

fn snooze(ports: &Ports, current: &str, mut outcome: Outcome) -> Outcome {
    let now = ports.clock.now_unix();
    let cache = read_cache(ports.fs, ports.env).unwrap_or_default();
    match core::snooze(&cache, current, now) {
        Some(snoozed) => {
            outcome.latest = snoozed.snoozed_version.clone();
            outcome.url = snoozed.url.clone();
            write_cache(ports.fs, ports.env, &snoozed);
            outcome.action = Action::Snoozed;
        }
        None => outcome.action = Action::NothingToSnooze,
    }
    outcome
}

/// Downloads the archive and `SHA256SUMS`, verifies, extracts, and returns
/// the new binary's bytes. The temporary folder is removed in every case.
fn download_and_extract(
    ports: &Ports,
    version: &str,
    target: &str,
    asset: &str,
) -> Result<Vec<u8>, UpdateError> {
    let get = |file: &str| {
        let url = core::download_url(version, file);
        ports
            .http
            .get(&url, DOWNLOAD_TIMEOUT_SECS)
            .map_err(|e| UpdateError::DownloadFailed {
                url,
                reason: e.to_string(),
            })
    };
    let sums = get("SHA256SUMS")?;
    let archive = get(asset)?;
    core::verify_archive(&archive, &String::from_utf8_lossy(&sums), asset)
        .map_err(|e| UpdateError::ChecksumMismatch(e.message()))?;

    let extract_failed = |reason: String| UpdateError::ExtractFailed {
        asset: asset.to_string(),
        reason,
    };
    let tmp = ports
        .exe
        .temp_dir()
        .map_err(|e| extract_failed(e.to_string()))?;
    let result = (|| {
        let archive_path = tmp.join(asset);
        ports
            .exe
            .write_bytes(&archive_path, &archive)
            .map_err(|e| extract_failed(e.to_string()))?;
        // `tar` reads `.tar.gz` everywhere, and `.zip` with the bsdtar of
        // macOS and Windows 10+, the only platform shipping zips.
        let flags = if asset.ends_with(".zip") {
            "-xf"
        } else {
            "-xzf"
        };
        let archive_arg = archive_path.to_string_lossy().into_owned();
        let tmp_arg = tmp.to_string_lossy().into_owned();
        let output = ports
            .runner
            .run("tar", &[flags, &archive_arg, "-C", &tmp_arg], None)
            .map_err(|e| extract_failed(e.to_string()))?;
        if !output.is_ok() {
            return Err(extract_failed(output.stderr_str().trim().to_string()));
        }
        let root = core::archive_root(version, target).unwrap_or_default();
        let binary = tmp.join(root).join(core::binary_name(target));
        ports
            .exe
            .read_bytes(&binary)
            .map_err(|_| extract_failed(format!("no {} in the archive", binary.display())))
    })();
    let _ = ports.exe.remove_dir_all(&tmp);
    result
}

/// Writes the new binary next to the running one, then swaps.
fn install_binary(
    exe: &dyn Executable,
    path: &Path,
    version: &str,
    bytes: &[u8],
) -> Result<(), UpdateError> {
    let (staged, aside) = core::staging_paths(path, version);
    let not_writable = |at: &Path, e: std::io::Error| UpdateError::NotWritable {
        path: at.to_path_buf(),
        reason: e.to_string(),
    };
    exe.write_bytes(&staged, bytes)
        .map_err(|e| not_writable(&staged, e))?;
    if let Err(e) = exe.install(&staged, path, &aside) {
        let _ = exe.remove_file(&staged);
        return Err(not_writable(path, e));
    }
    Ok(())
}

/// Runs the new binary's `codev update` in the current project, if any:
/// the running process is still the old version and would stamp the
/// skills with it.
fn refresh_project(ports: &Ports, path: &Path, outcome: &mut Outcome) {
    let Ok(layout) = root::discover_from_cwd(ports.fs, ports.env) else {
        return;
    };
    let project = layout.project_root().to_path_buf();
    let program = path.to_string_lossy().into_owned();
    let failure = match ports.runner.run(&program, &["update"], Some(&project)) {
        Ok(output) if output.is_ok() => {
            outcome.project_updated = Some(project);
            return;
        }
        Ok(output) => output.stderr_str().trim().to_string(),
        Err(e) => e.to_string(),
    };
    outcome.warnings.push(Warning::new(
        "update_project_refresh_failed",
        format!(
            "codev was updated, but `codev update` failed in {}: {failure} — run `codev update` there",
            project.display()
        ),
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ports::{
        FixedClock, FixedEnv, MEMORY_TEMP_DIR, MemoryExecutable, MemoryFileSystem, MemoryHttp,
        MockProcessRunner, ProcessOutput,
    };
    use codev_core::update::{RELEASES_API, download_url, sha256_hex};

    const TARGET: &str = "aarch64-apple-darwin";
    const EXE: &str = "/home/me/.local/bin/codev";
    const DAY: u64 = 86_400;

    fn ok_output() -> ProcessOutput {
        ProcessOutput {
            stdout: Vec::new(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    fn failed_output(stderr: &str) -> ProcessOutput {
        ProcessOutput {
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
            exit_code: 1,
        }
    }

    fn release_json(version: &str) -> String {
        format!(
            r#"{{"tag_name": "v{version}", "html_url": "https://github.com/mairistem/codev/releases/tag/v{version}"}}"#
        )
    }

    fn env_at(cwd: &str) -> FixedEnv {
        let mut env = FixedEnv::at(cwd);
        env.vars.insert("HOME".into(), "/home/me".into());
        env
    }

    /// A published release: its API answer, `SHA256SUMS` and archive.
    fn published(http: MemoryHttp, version: &str, archive: &[u8]) -> MemoryHttp {
        let asset = core::asset_name(version, TARGET).unwrap();
        http.with_response(
            &download_url(version, "SHA256SUMS"),
            format!("{}  {asset}\n", sha256_hex(archive)),
        )
        .with_response(&download_url(version, &asset), archive.to_vec())
    }

    /// What `tar` would have extracted for `version`.
    fn extracted(exe: MemoryExecutable, version: &str, binary: &str) -> MemoryExecutable {
        exe.with_file(
            format!("{MEMORY_TEMP_DIR}/codev-{version}-{TARGET}/codev"),
            binary,
        )
    }

    fn request(current: &str) -> Request {
        Request {
            current: current.into(),
            target: TARGET.into(),
            ..Request::default()
        }
    }

    struct World {
        fs: MemoryFileSystem,
        env: FixedEnv,
        clock: FixedClock,
        http: MemoryHttp,
        runner: MockProcessRunner,
        exe: MemoryExecutable,
    }

    impl World {
        fn new(http: MemoryHttp, exe: MemoryExecutable) -> Self {
            Self {
                fs: MemoryFileSystem::new()
                    .with_file("/p/_codev/config.yaml", "schema: spec-driven\n"),
                env: env_at("/p"),
                clock: FixedClock("2026-10-12".into()),
                http,
                runner: MockProcessRunner::new()
                    .with_response("tar", &[], ok_output())
                    .with_response(EXE, &["update"], ok_output()),
                exe,
            }
        }

        fn ports(&self) -> Ports<'_> {
            Ports {
                fs: &self.fs,
                env: &self.env,
                clock: &self.clock,
                http: &self.http,
                runner: &self.runner,
                exe: &self.exe,
            }
        }

        fn run(&self, request: &Request) -> Result<Outcome, UpdateError> {
            self_update(&self.ports(), request, &mut |_, _| true)
        }
    }

    fn latest_world(current_binary: &str, latest: &str) -> World {
        let http = published(
            MemoryHttp::new().with_response(RELEASES_API, release_json(latest)),
            latest,
            b"archive",
        );
        let exe = extracted(
            MemoryExecutable::at(EXE, current_binary),
            latest,
            "new binary",
        );
        World::new(http, exe)
    }

    // ─────────────── status_notice / cached_notice ───────────────

    fn cache_at(checked_at: u64, latest: &str) -> String {
        UpdateCache {
            checked_at,
            latest: Some(latest.into()),
            url: Some(core::release_page(latest)),
            ..UpdateCache::default()
        }
        .render()
    }

    const CACHE: &str = "/home/me/.cache/codev/update-check.json";

    #[test]
    fn status_notice_reuses_a_recent_cache_without_a_request() {
        let clock = FixedClock("2026-10-12".into());
        let now = clock.now_unix();
        let fs = MemoryFileSystem::new().with_file(CACHE, cache_at(now - 2 * 3600, "0.6.0"));
        let http = MemoryHttp::new();
        let notice = status_notice(&fs, &env_at("/p"), &clock, &http, "0.5.0").unwrap();
        assert!(http.calls().is_empty());
        assert_eq!(notice.latest, "0.6.0");
        assert!(notice.available);
    }

    #[test]
    fn status_notice_refreshes_a_stale_cache_once() {
        let clock = FixedClock("2026-10-12".into());
        let now = clock.now_unix();
        let fs = MemoryFileSystem::new().with_file(CACHE, cache_at(now - 25 * 3600, "0.6.0"));
        let http = MemoryHttp::new().with_response(RELEASES_API, release_json("0.7.0"));
        let notice = status_notice(&fs, &env_at("/p"), &clock, &http, "0.5.0").unwrap();
        assert_eq!(http.calls(), [RELEASES_API]);
        assert_eq!(notice.latest, "0.7.0");
        let cache = UpdateCache::parse(&fs.read(CACHE).unwrap()).unwrap();
        assert_eq!(cache.checked_at, now);
        assert_eq!(cache.latest.as_deref(), Some("0.7.0"));
        // Right after, the fresh cache is reused.
        status_notice(&fs, &env_at("/p"), &clock, &http, "0.5.0");
        assert_eq!(http.calls().len(), 1);
    }

    #[test]
    fn status_notice_offline_records_the_attempt_and_stays_silent() {
        let clock = FixedClock("2026-10-12".into());
        let fs = MemoryFileSystem::new();
        let http = MemoryHttp::new();
        assert!(status_notice(&fs, &env_at("/p"), &clock, &http, "0.5.0").is_none());
        assert!(status_notice(&fs, &env_at("/p"), &clock, &http, "0.5.0").is_none());
        assert_eq!(http.calls().len(), 1, "one attempt per day");
        let cache = UpdateCache::parse(&fs.read(CACHE).unwrap()).unwrap();
        assert_eq!(cache.checked_at, clock.now_unix());
        assert!(cache.latest.is_none());
    }

    #[test]
    fn status_notice_is_disabled_in_ci_and_on_opt_out() {
        let clock = FixedClock("2026-10-12".into());
        for (key, value) in [("CI", "true"), ("CODEV_NO_UPDATE_CHECK", "1")] {
            let mut env = env_at("/p");
            env.vars.insert(key.into(), value.into());
            let fs = MemoryFileSystem::new().with_file(CACHE, cache_at(0, "0.6.0"));
            let http = MemoryHttp::new().with_response(RELEASES_API, release_json("0.7.0"));
            assert!(status_notice(&fs, &env, &clock, &http, "0.5.0").is_none());
            assert!(http.calls().is_empty(), "{key}");
            assert_eq!(fs.read(CACHE).unwrap(), cache_at(0, "0.6.0"), "untouched");
        }
    }

    #[test]
    fn cached_notice_never_requests() {
        let clock = FixedClock("2026-10-12".into());
        let fs = MemoryFileSystem::new().with_file(CACHE, cache_at(0, "0.6.0"));
        let notice = cached_notice(&fs, &env_at("/p"), &clock, "0.5.0").unwrap();
        assert!(notice.available, "a stale cache still says what it knows");
        assert!(cached_notice(&MemoryFileSystem::new(), &env_at("/p"), &clock, "0.5.0").is_none());
    }

    #[test]
    fn cleanup_removes_the_binary_moved_aside() {
        let exe = MemoryExecutable::at("/w/codev.exe", "new").with_file("/w/codev.old.exe", "old");
        cleanup_previous_binary(&exe);
        assert!(exe.read("/w/codev.old.exe").is_none());
        assert!(exe.read("/w/codev.exe").is_some());
        // Nothing to clean: no error.
        cleanup_previous_binary(&MemoryExecutable::at("/u/codev", "x"));
    }

    // ─────────────── self_update ───────────────

    #[test]
    fn self_update_installs_a_verified_release_and_refreshes_the_project() {
        let world = latest_world("old binary", "0.6.0");
        let outcome = world.run(&request("0.5.0")).unwrap();
        assert_eq!(outcome.action, Action::Updated);
        assert_eq!(outcome.latest.as_deref(), Some("0.6.0"));
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"new binary"[..]));
        assert_eq!(outcome.project_updated.as_deref(), Some(Path::new("/p")));
        let calls = world.runner.calls();
        assert_eq!(calls[0].0, "tar");
        assert_eq!(calls[0].1[0], "-xzf");
        assert_eq!(calls[1], (EXE.to_string(), vec!["update".to_string()]));
        // The temporary folder and the staged file are gone.
        assert!(
            world
                .exe
                .paths()
                .iter()
                .all(|p| !p.starts_with(MEMORY_TEMP_DIR) && !p.to_string_lossy().contains(".new")),
            "{:?}",
            world.exe.paths()
        );
        assert!(outcome.warnings.is_empty());
    }

    #[test]
    fn self_update_does_nothing_when_up_to_date() {
        let world = latest_world("current binary", "0.6.0");
        let outcome = world.run(&request("0.6.0")).unwrap();
        assert_eq!(outcome.action, Action::UpToDate);
        assert_eq!(world.http.calls(), [RELEASES_API]);
        assert!(world.runner.calls().is_empty());
    }

    #[test]
    fn self_update_refuses_a_checksum_mismatch_without_touching_the_binary() {
        let asset = core::asset_name("0.6.0", TARGET).unwrap();
        let http = MemoryHttp::new()
            .with_response(RELEASES_API, release_json("0.6.0"))
            .with_response(
                &download_url("0.6.0", "SHA256SUMS"),
                format!("{}  {asset}\n", sha256_hex(b"genuine")),
            )
            .with_response(&download_url("0.6.0", &asset), b"tampered".to_vec());
        let world = World::new(http, MemoryExecutable::at(EXE, "old binary"));
        let err = world.run(&request("0.5.0")).unwrap_err();
        assert_eq!(err.code(), "update_checksum_mismatch");
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"old binary"[..]));
        assert!(world.runner.calls().is_empty(), "nothing extracted");
    }

    #[test]
    fn self_update_reports_a_download_failure() {
        let http = MemoryHttp::new()
            .with_response(RELEASES_API, release_json("0.6.0"))
            .with_failure(&download_url("0.6.0", "SHA256SUMS"), "404 Not Found");
        let world = World::new(http, MemoryExecutable::at(EXE, "old binary"));
        let err = world.run(&request("0.5.0")).unwrap_err();
        assert_eq!(err.code(), "update_download_failed");
        assert!(err.to_string().contains("404"), "{err}");
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"old binary"[..]));
    }

    #[test]
    fn self_update_reports_an_unwritable_folder() {
        let http = published(
            MemoryHttp::new().with_response(RELEASES_API, release_json("0.6.0")),
            "0.6.0",
            b"archive",
        );
        let exe = extracted(MemoryExecutable::at(EXE, "old binary"), "0.6.0", "new")
            .read_only("/home/me/.local/bin");
        let world = World::new(http, exe);
        let err = world.run(&request("0.5.0")).unwrap_err();
        assert_eq!(err.code(), "update_not_writable");
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"old binary"[..]));
    }

    #[test]
    fn self_update_warns_when_the_project_refresh_fails() {
        let mut world = latest_world("old", "0.6.0");
        world.runner = MockProcessRunner::new()
            .with_response("tar", &[], ok_output())
            .with_response(EXE, &["update"], failed_output("error: config is invalid"));
        let outcome = world.run(&request("0.5.0")).unwrap();
        assert_eq!(outcome.action, Action::Updated);
        assert!(outcome.project_updated.is_none());
        assert_eq!(outcome.warnings[0].code, "update_project_refresh_failed");
        assert!(outcome.warnings[0].message.contains("codev update"));
    }

    #[test]
    fn self_update_skips_the_project_refresh_outside_a_project() {
        let mut world = latest_world("old", "0.6.0");
        world.env = env_at("/elsewhere");
        let outcome = world.run(&request("0.5.0")).unwrap();
        assert_eq!(outcome.action, Action::Updated);
        assert!(outcome.project_updated.is_none());
        assert!(world.runner.calls().iter().all(|(p, _)| p == "tar"));
    }

    #[test]
    fn self_update_refuses_an_implicit_downgrade() {
        let world = latest_world("ahead", "0.6.0");
        let err = world.run(&request("0.7.0")).unwrap_err();
        assert_eq!(err.code(), "update_downgrade_refused");
        assert!(err.to_string().contains("--version 0.6.0"), "{err}");
        assert_eq!(world.http.calls(), [RELEASES_API], "nothing downloaded");
    }

    #[test]
    fn self_update_installs_an_explicit_older_version() {
        let http = published(MemoryHttp::new(), "0.5.0", b"archive");
        let exe = extracted(
            MemoryExecutable::at(EXE, "0.6.0 binary"),
            "0.5.0",
            "0.5.0 binary",
        );
        let world = World::new(http, exe);
        let outcome = world
            .run(&Request {
                version: Some("v0.5.0".into()),
                ..request("0.6.0")
            })
            .unwrap();
        assert_eq!(outcome.action, Action::Updated);
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"0.5.0 binary"[..]));
        assert!(!world.http.calls().contains(&RELEASES_API.to_string()));
    }

    #[test]
    fn self_update_leaves_a_cargo_install_alone() {
        let http = MemoryHttp::new().with_response(RELEASES_API, release_json("0.6.0"));
        let world = World::new(http, MemoryExecutable::at("/home/me/.cargo/bin/codev", "x"));
        let outcome = world.run(&request("0.5.0")).unwrap();
        assert_eq!(outcome.action, Action::ManagedElsewhere);
        assert_eq!(outcome.method, InstallMethod::Cargo);
        assert!(world.http.calls().is_empty());
        assert!(
            outcome
                .method
                .upgrade_hint()
                .unwrap()
                .contains("cargo install --path crates/codev-cli")
        );
    }

    #[test]
    fn self_update_check_reports_without_downloading() {
        let world = latest_world("old", "0.6.0");
        let outcome = world
            .run(&Request {
                check_only: true,
                ..request("0.5.0")
            })
            .unwrap();
        assert_eq!(outcome.action, Action::Available);
        assert_eq!(outcome.method, InstallMethod::Standalone);
        assert_eq!(world.http.calls(), [RELEASES_API]);
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"old"[..]));
        // The check refreshed the cache.
        let cache = read_cache(&world.fs, &world.env).unwrap();
        assert_eq!(cache.latest.as_deref(), Some("0.6.0"));
    }

    #[test]
    fn self_update_check_offline_fails_with_update_check_failed() {
        let world = World::new(MemoryHttp::new(), MemoryExecutable::at(EXE, "old"));
        let err = world
            .run(&Request {
                check_only: true,
                ..request("0.5.0")
            })
            .unwrap_err();
        assert_eq!(err.code(), "update_check_failed");
    }

    #[test]
    fn self_update_rejects_a_malformed_version() {
        let world = World::new(MemoryHttp::new(), MemoryExecutable::at(EXE, "old"));
        let err = world
            .run(&Request {
                version: Some("latest".into()),
                ..request("0.5.0")
            })
            .unwrap_err();
        assert_eq!(err.code(), "invalid_version");
        assert!(world.http.calls().is_empty());
    }

    #[test]
    fn self_update_unsupported_platform_downloads_nothing() {
        let world = latest_world("old", "0.6.0");
        let err = world
            .run(&Request {
                target: "riscv64gc-unknown-linux-gnu".into(),
                ..request("0.5.0")
            })
            .unwrap_err();
        assert_eq!(err.code(), "update_unsupported_platform");
        assert_eq!(world.http.calls(), [RELEASES_API]);
    }

    #[test]
    fn self_update_declined_confirmation_changes_nothing() {
        let world = latest_world("old", "0.6.0");
        let outcome = self_update(&world.ports(), &request("0.5.0"), &mut |_, _| false).unwrap();
        assert_eq!(outcome.action, Action::Cancelled);
        assert_eq!(world.exe.read(EXE).as_deref(), Some(&b"old"[..]));
        assert_eq!(world.http.calls(), [RELEASES_API]);
    }

    #[test]
    fn self_update_snooze_records_seven_days_for_the_latest_known_version() {
        let mut world = World::new(MemoryHttp::new(), MemoryExecutable::at(EXE, "old"));
        let now = world.clock.now_unix();
        world.fs = MemoryFileSystem::new().with_file(CACHE, cache_at(now, "0.6.0"));
        let outcome = world
            .run(&Request {
                snooze: true,
                ..request("0.5.0")
            })
            .unwrap();
        assert_eq!(outcome.action, Action::Snoozed);
        let cache = read_cache(&world.fs, &world.env).unwrap();
        assert_eq!(cache.snoozed_version.as_deref(), Some("0.6.0"));
        assert_eq!(cache.snoozed_until, Some(now + 7 * DAY));
        assert!(world.http.calls().is_empty());

        let up_to_date = world
            .run(&Request {
                snooze: true,
                ..request("0.6.0")
            })
            .unwrap();
        assert_eq!(up_to_date.action, Action::NothingToSnooze);
    }

    #[test]
    fn self_update_on_windows_extracts_the_zip_and_keeps_the_old_binary_aside() {
        let target = "x86_64-pc-windows-msvc";
        let exe_path = "C:/Users/me/AppData/Local/Programs/codev/codev.exe";
        let asset = core::asset_name("0.6.0", target).unwrap();
        let http = MemoryHttp::new()
            .with_response(RELEASES_API, release_json("0.6.0"))
            .with_response(
                &download_url("0.6.0", "SHA256SUMS"),
                format!("{}  {asset}\n", sha256_hex(b"zip")),
            )
            .with_response(&download_url("0.6.0", &asset), b"zip".to_vec());
        let exe = MemoryExecutable::at(exe_path, "old").with_file(
            format!("{MEMORY_TEMP_DIR}/codev-0.6.0-{target}/codev.exe"),
            "new",
        );
        let world = World::new(http, exe);
        let outcome = world
            .run(&Request {
                target: target.into(),
                ..request("0.5.0")
            })
            .unwrap();
        assert_eq!(outcome.action, Action::Updated);
        assert_eq!(world.runner.calls()[0].1[0], "-xf");
        assert_eq!(world.exe.read(exe_path).as_deref(), Some(&b"new"[..]));
        assert_eq!(
            world
                .exe
                .read("C:/Users/me/AppData/Local/Programs/codev/codev.old.exe")
                .as_deref(),
            Some(&b"old"[..])
        );
    }
}
