//! Update notice and `codev self-update`: every decision, none of the
//! effects.
//!
//! The shell gathers facts — the cache file, the environment, the clock,
//! the release API's answer, the downloaded bytes, the binary's path — and
//! these functions decide: whether to check, what to announce, what to
//! snooze, which asset to download, whether its checksum holds, whether the
//! binary may be replaced, and with which version. See
//! `_codev/decisions/0001-functional-core-imperative-shell.md`.

use std::path::{Path, PathBuf};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// The latest-release endpoint of GitHub's public API — the only URL the
/// background check ever requests.
pub const RELEASES_API: &str = "https://api.github.com/repos/mairistem/codev/releases/latest";

const DOWNLOAD_BASE: &str = "https://github.com/mairistem/codev/releases/download";
const RELEASE_PAGE_BASE: &str = "https://github.com/mairistem/codev/releases/tag";

/// A check younger than this is reused as is.
pub const CHECK_INTERVAL_SECS: u64 = 24 * 60 * 60;

/// How long "Later" silences the notice for one version.
pub const SNOOZE_SECS: u64 = 7 * 24 * 60 * 60;

// ─────────────────────────────── versions ───────────────────────────────

/// A release version, with or without the leading `v` of its tag.
pub fn parse_version(raw: &str) -> Option<Version> {
    let raw = raw.trim();
    Version::parse(raw.strip_prefix('v').unwrap_or(raw)).ok()
}

/// A `codev_version` requirement, in Cargo's syntax (`">=0.5"`, `"^0.6"`,
/// `">=0.5, <0.8"`). The error is the message to show.
pub fn parse_requirement(raw: &str) -> Result<VersionReq, String> {
    VersionReq::parse(raw.trim()).map_err(|e| {
        format!(
            "`codev_version: {raw}` is not a version requirement ({e}); \
             use Cargo's syntax, for example \">=0.5\""
        )
    })
}

/// Whether `current` satisfies `requirement`. `None` when either does not
/// parse — the caller has already reported an invalid requirement.
pub fn satisfies(requirement: &str, current: &str) -> Option<bool> {
    let requirement = parse_requirement(requirement).ok()?;
    let current = parse_version(current)?;
    Some(requirement.matches(&current))
}

// ─────────────────────────────── check policy ───────────────────────────────

/// What the environment says about the background check.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CheckEnv {
    /// `CI` is set, to any value.
    pub ci: bool,
    /// `CODEV_NO_UPDATE_CHECK=1`.
    pub opt_out: bool,
}

impl CheckEnv {
    pub fn from_vars(ci: Option<&str>, no_update_check: Option<&str>) -> Self {
        Self {
            ci: ci.is_some(),
            opt_out: no_update_check.map(str::trim) == Some("1"),
        }
    }

    pub fn disabled(self) -> bool {
        self.ci || self.opt_out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckDecision {
    /// No check, no cache read, no notice.
    Disabled,
    /// The cache is recent enough.
    UseCache,
    /// One request, then record its result.
    Refresh,
}

pub fn should_check(env: CheckEnv, cache: Option<&UpdateCache>, now: u64) -> CheckDecision {
    if env.disabled() {
        return CheckDecision::Disabled;
    }
    match cache {
        // A clock that went backwards counts as recent: never a retry storm.
        Some(cache) if now.saturating_sub(cache.checked_at) < CHECK_INTERVAL_SECS => {
            CheckDecision::UseCache
        }
        _ => CheckDecision::Refresh,
    }
}

// ─────────────────────────────── cache ───────────────────────────────

/// `update-check.json` in the codev cache directory.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCache {
    /// Unix seconds of the last attempt, successful or not.
    pub checked_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snoozed_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snoozed_until: Option<u64>,
}

impl UpdateCache {
    /// An unreadable cache is no cache: the check simply runs again.
    pub fn parse(raw: &str) -> Option<Self> {
        serde_json::from_str(raw).ok()
    }

    pub fn render(&self) -> String {
        let mut out = serde_json::to_string_pretty(self).expect("the cache is serializable");
        out.push('\n');
        out
    }
}

/// A published release.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Release {
    /// Without the leading `v`.
    pub version: String,
    pub url: String,
}

/// Reads the answer of [`RELEASES_API`]: `tag_name` and `html_url`.
pub fn parse_latest_release(json: &str) -> Option<Release> {
    let value: serde_json::Value = serde_json::from_str(json).ok()?;
    let tag = value.get("tag_name")?.as_str()?;
    let version = parse_version(tag)?.to_string();
    let url = value
        .get("html_url")
        .and_then(|u| u.as_str())
        .map(str::to_string)
        .unwrap_or_else(|| release_page(&version));
    Some(Release { version, url })
}

/// The release page of a version.
pub fn release_page(version: &str) -> String {
    format!("{RELEASE_PAGE_BASE}/v{version}")
}

/// The cache after an attempt at `now`. A failed attempt (`fetched` is
/// `None`) keeps the release previously known and the snooze: the time is
/// what stops the next commands from retrying.
pub fn record_check(
    previous: Option<&UpdateCache>,
    fetched: Option<&Release>,
    now: u64,
) -> UpdateCache {
    let mut cache = previous.cloned().unwrap_or_default();
    cache.checked_at = now;
    if let Some(release) = fetched {
        cache.latest = Some(release.version.clone());
        cache.url = Some(release.url.clone());
    }
    cache
}

// ─────────────────────────────── notice and snooze ───────────────────────────────

/// What `codev status` reports about updates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateNotice {
    pub current: String,
    pub latest: String,
    pub url: String,
    pub available: bool,
    pub snoozed: bool,
}

impl UpdateNotice {
    /// Whether the human output prints the line.
    pub fn should_announce(&self) -> bool {
        self.available && !self.snoozed
    }

    pub fn line(&self) -> String {
        format!(
            "codev {} is available (you have {}) — run `codev self-update`",
            self.latest, self.current
        )
    }
}

/// The notice the cache supports; `None` when the latest release is
/// unknown.
pub fn notice(cache: &UpdateCache, current: &str, now: u64) -> Option<UpdateNotice> {
    let latest_raw = cache.latest.as_deref()?;
    let latest = parse_version(latest_raw)?;
    let running = parse_version(current)?;
    let snoozed = cache.snoozed_version.as_deref().and_then(parse_version) == Some(latest.clone())
        && cache.snoozed_until.is_some_and(|until| now < until);
    Some(UpdateNotice {
        current: running.to_string(),
        latest: latest.to_string(),
        url: cache
            .url
            .clone()
            .unwrap_or_else(|| release_page(&latest.to_string())),
        available: latest > running,
        snoozed,
    })
}

/// The cache with the latest known version snoozed for [`SNOOZE_SECS`];
/// `None` when no update is available, so there is nothing to snooze.
pub fn snooze(cache: &UpdateCache, current: &str, now: u64) -> Option<UpdateCache> {
    let notice = notice(cache, current, now)?;
    if !notice.available {
        return None;
    }
    let mut snoozed = cache.clone();
    snoozed.snoozed_version = Some(notice.latest);
    snoozed.snoozed_until = Some(now + SNOOZE_SECS);
    Some(snoozed)
}

// ─────────────────────────────── release assets ───────────────────────────────

/// The release target serving a build target. A glibc Linux build gets the
/// static musl binary, which runs there too.
pub fn asset_target(target: &str) -> Option<&'static str> {
    match target {
        "aarch64-apple-darwin" => Some("aarch64-apple-darwin"),
        "x86_64-apple-darwin" => Some("x86_64-apple-darwin"),
        "x86_64-unknown-linux-musl" | "x86_64-unknown-linux-gnu" => {
            Some("x86_64-unknown-linux-musl")
        }
        "x86_64-pc-windows-msvc" => Some("x86_64-pc-windows-msvc"),
        _ => None,
    }
}

fn is_windows(target: &str) -> bool {
    target.contains("windows")
}

/// The archive name, as `.github/workflows/release.yml` publishes it.
pub fn asset_name(version: &str, target: &str) -> Option<String> {
    let target = asset_target(target)?;
    let ext = if is_windows(target) { "zip" } else { "tar.gz" };
    Some(format!("codev-{version}-{target}.{ext}"))
}

/// The folder the archive extracts to.
pub fn archive_root(version: &str, target: &str) -> Option<String> {
    Some(format!("codev-{version}-{}", asset_target(target)?))
}

pub fn binary_name(target: &str) -> &'static str {
    if is_windows(target) {
        "codev.exe"
    } else {
        "codev"
    }
}

pub fn download_url(version: &str, file: &str) -> String {
    format!("{DOWNLOAD_BASE}/v{version}/{file}")
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChecksumError {
    /// `SHA256SUMS` lists no such archive.
    Missing {
        asset: String,
    },
    Mismatch {
        expected: String,
        actual: String,
    },
}

impl ChecksumError {
    pub fn message(&self) -> String {
        match self {
            Self::Missing { asset } => {
                format!("SHA256SUMS has no entry for {asset} — refusing to install")
            }
            Self::Mismatch { expected, actual } => format!(
                "SHA-256 mismatch (expected {expected}, got {actual}) — refusing to install"
            ),
        }
    }
}

/// Checks `archive` against its line of `SHA256SUMS` (`<hex>  <name>`, as
/// `sha256sum` writes it, a `*` binary marker tolerated).
pub fn verify_archive(archive: &[u8], sums: &str, asset: &str) -> Result<(), ChecksumError> {
    let expected = sums
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            Some((parts.next()?, parts.next()?))
        })
        .find(|(_, name)| name.trim_start_matches('*') == asset)
        .map(|(hash, _)| hash.to_ascii_lowercase())
        .ok_or_else(|| ChecksumError::Missing {
            asset: asset.to_string(),
        })?;
    let actual = sha256_hex(archive);
    if actual == expected {
        Ok(())
    } else {
        Err(ChecksumError::Mismatch { expected, actual })
    }
}

// ─────────────────────────────── installation method ───────────────────────────────

/// How the running binary was installed — only `Standalone` is replaced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallMethod {
    /// The installers, or a manual copy.
    Standalone,
    /// `cargo install`.
    Cargo,
    /// A cargo build folder (`target/debug`, `target/release`).
    Build,
    /// A package manager's tree; the name is for the message.
    PackageManager(&'static str),
}

impl InstallMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standalone => "standalone",
            Self::Cargo => "cargo",
            Self::Build => "build",
            Self::PackageManager(_) => "package_manager",
        }
    }

    pub fn label(self) -> String {
        match self {
            Self::Standalone => "the codev installer".to_string(),
            Self::Cargo => "cargo install".to_string(),
            Self::Build => "a cargo build of the sources".to_string(),
            Self::PackageManager(name) => name.to_string(),
        }
    }

    /// The command that upgrades this installation; `None` for the one
    /// `codev self-update` handles itself.
    pub fn upgrade_hint(self) -> Option<String> {
        match self {
            Self::Standalone => None,
            Self::Cargo => Some(
                "cargo install --path crates/codev-cli, from an up-to-date clone of \
                 https://github.com/mairistem/codev"
                    .to_string(),
            ),
            Self::Build => {
                Some("git pull, then cargo build --release --bin codev in your clone".to_string())
            }
            Self::PackageManager(name) => Some(format!("upgrade codev with {name}")),
        }
    }
}

fn normalize(path: &str) -> String {
    let slashed = path.replace('\\', "/").to_ascii_lowercase();
    // A canonical Windows path is verbatim (`\\?\C:\…`), `CARGO_HOME` and the
    // home folder are not: compare them without the prefix.
    let mut normalized = match slashed.strip_prefix("//?/") {
        Some(rest) => rest.to_string(),
        None => slashed,
    };
    if !normalized.ends_with('/') {
        normalized.push('/');
    }
    normalized
}

/// Classifies the binary's path. Errs towards "not standalone": printing
/// a command beats replacing a binary another tool owns.
pub fn install_method(exe: &str, cargo_home: Option<&str>, home: Option<&str>) -> InstallMethod {
    // The binary's folder, normalized and ending in `/`.
    let exe_norm = exe.replace('\\', "/");
    let dir = normalize(exe_norm.rsplit_once('/').map_or("", |(dir, _)| dir));

    let cargo_bins = cargo_home
        .filter(|h| !h.is_empty())
        .map(|h| normalize(&format!("{}/bin", h.trim_end_matches(['/', '\\']))))
        .into_iter()
        .chain(
            home.filter(|h| !h.is_empty())
                .map(|h| normalize(&format!("{}/.cargo/bin", h.trim_end_matches(['/', '\\'])))),
        );
    for bin in cargo_bins {
        if dir == bin {
            return InstallMethod::Cargo;
        }
    }

    let segments: Vec<&str> = dir.split('/').filter(|s| !s.is_empty()).collect();
    if let Some(target) = segments.iter().position(|s| *s == "target")
        && segments[target + 1..]
            .iter()
            .any(|s| *s == "debug" || *s == "release")
    {
        return InstallMethod::Build;
    }

    const PACKAGE_MANAGERS: &[(&str, &str)] = &[
        ("/opt/homebrew/", "Homebrew"),
        ("/usr/local/cellar/", "Homebrew"),
        ("/home/linuxbrew/", "Homebrew"),
        ("/nix/store/", "Nix"),
        ("/scoop/apps/", "Scoop"),
    ];
    if let Some((_, name)) = PACKAGE_MANAGERS
        .iter()
        .find(|(marker, _)| dir.contains(marker))
    {
        return InstallMethod::PackageManager(name);
    }
    if dir == "/usr/bin/" {
        return InstallMethod::PackageManager("your system's package manager");
    }
    InstallMethod::Standalone
}

// ─────────────────────────────── decision ───────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    UpToDate,
    /// The target is older and was not asked for explicitly.
    RefuseDowngrade,
    Install,
}

/// `explicit`: the target came from `--version`.
pub fn decide(current: &Version, target: &Version, explicit: bool) -> Decision {
    if target == current {
        Decision::UpToDate
    } else if target < current && !explicit {
        Decision::RefuseDowngrade
    } else {
        Decision::Install
    }
}

/// Where the new binary is written before the swap, and where Windows
/// moves the running one aside. Both sit next to the target: a rename
/// within one folder is atomic.
pub fn staging_paths(target: &Path, version: &str) -> (PathBuf, PathBuf) {
    let dir = target.parent().unwrap_or_else(|| Path::new("."));
    let windows = target
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("exe"));
    let (staged, aside) = if windows {
        (format!(".codev-{version}.new.exe"), "codev.old.exe")
    } else {
        (format!(".codev-{version}.new"), "codev.old")
    };
    (dir.join(staged), dir.join(aside))
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: u64 = 24 * 60 * 60;
    const NOW: u64 = 1_800_000_000;

    fn cache(latest: &str, checked_at: u64) -> UpdateCache {
        UpdateCache {
            checked_at,
            latest: Some(latest.into()),
            url: Some(release_page(latest)),
            ..UpdateCache::default()
        }
    }

    fn v(raw: &str) -> Version {
        parse_version(raw).unwrap()
    }

    #[test]
    fn versions_parse_with_or_without_a_leading_v() {
        assert_eq!(v("v0.5.0"), v("0.5.0"));
        assert_eq!(v(" 1.2.3 ").to_string(), "1.2.3");
        assert!(parse_version("latest").is_none());
    }

    #[test]
    fn requirements_use_cargo_syntax() {
        assert_eq!(satisfies(">=0.5", "0.6.0"), Some(true));
        assert_eq!(satisfies(">=0.7", "0.6.0"), Some(false));
        assert_eq!(satisfies("^0.6", "0.6.3"), Some(true));
        assert_eq!(satisfies("^0.6", "0.7.0"), Some(false));
        assert_eq!(satisfies(">=0.5, <0.8", "0.8.0"), Some(false));
    }

    #[test]
    fn an_invalid_requirement_is_refused() {
        let err = parse_requirement("latest").unwrap_err();
        assert!(err.contains("latest"), "{err}");
        assert_eq!(satisfies("latest", "0.6.0"), None);
    }

    #[test]
    fn ci_or_opt_out_disables_the_check() {
        let stale = cache("0.6.0", NOW - 2 * DAY);
        for env in [
            CheckEnv::from_vars(Some("true"), None),
            CheckEnv::from_vars(Some(""), None),
            CheckEnv::from_vars(None, Some("1")),
        ] {
            assert_eq!(
                should_check(env, Some(&stale), NOW),
                CheckDecision::Disabled
            );
        }
        // Any other value of the opt-out variable keeps the check.
        assert!(!CheckEnv::from_vars(None, Some("0")).disabled());
    }

    #[test]
    fn a_recent_check_is_reused() {
        let recent = cache("0.6.0", NOW - 2 * 60 * 60);
        assert_eq!(
            should_check(CheckEnv::default(), Some(&recent), NOW),
            CheckDecision::UseCache
        );
        // A clock that went backwards is not a reason to retry.
        let future = cache("0.6.0", NOW + DAY);
        assert_eq!(
            should_check(CheckEnv::default(), Some(&future), NOW),
            CheckDecision::UseCache
        );
    }

    #[test]
    fn a_stale_or_missing_check_is_refreshed() {
        let stale = cache("0.6.0", NOW - 25 * 60 * 60);
        assert_eq!(
            should_check(CheckEnv::default(), Some(&stale), NOW),
            CheckDecision::Refresh
        );
        assert_eq!(
            should_check(CheckEnv::default(), None, NOW),
            CheckDecision::Refresh
        );
    }

    #[test]
    fn a_failed_check_keeps_the_known_release_and_its_time() {
        let mut previous = cache("0.6.0", NOW - 2 * DAY);
        previous.snoozed_version = Some("0.6.0".into());
        previous.snoozed_until = Some(NOW + DAY);
        let after = record_check(Some(&previous), None, NOW);
        assert_eq!(after.checked_at, NOW);
        assert_eq!(after.latest.as_deref(), Some("0.6.0"));
        assert_eq!(after.snoozed_until, Some(NOW + DAY));

        let first = record_check(None, None, NOW);
        assert_eq!(first.checked_at, NOW);
        assert!(first.latest.is_none());

        let fetched = Release {
            version: "0.7.0".into(),
            url: "https://example/0.7.0".into(),
        };
        let refreshed = record_check(Some(&previous), Some(&fetched), NOW);
        assert_eq!(refreshed.latest.as_deref(), Some("0.7.0"));
        assert_eq!(refreshed.url.as_deref(), Some("https://example/0.7.0"));
    }

    #[test]
    fn cache_round_trips_and_garbage_reads_as_none() {
        let mut original = cache("0.6.0", NOW);
        original.snoozed_version = Some("0.6.0".into());
        original.snoozed_until = Some(NOW + SNOOZE_SECS);
        let rendered = original.render();
        assert!(rendered.contains("\"checkedAt\""), "{rendered}");
        assert_eq!(UpdateCache::parse(&rendered), Some(original));
        assert_eq!(UpdateCache::parse("not json"), None);
        assert_eq!(UpdateCache::parse(""), None);
    }

    #[test]
    fn notice_reports_an_available_update() {
        let notice = notice(&cache("0.6.0", NOW), "0.5.0", NOW).unwrap();
        assert_eq!(
            notice,
            UpdateNotice {
                current: "0.5.0".into(),
                latest: "0.6.0".into(),
                url: "https://github.com/mairistem/codev/releases/tag/v0.6.0".into(),
                available: true,
                snoozed: false,
            }
        );
        assert!(notice.should_announce());
        assert_eq!(
            notice.line(),
            "codev 0.6.0 is available (you have 0.5.0) — run `codev self-update`"
        );
    }

    #[test]
    fn notice_is_not_available_when_up_to_date() {
        let notice = notice(&cache("0.6.0", NOW), "0.6.0", NOW).unwrap();
        assert!(!notice.available);
        assert!(!notice.should_announce());
        // Unknown latest release: no notice at all.
        assert!(super::notice(&UpdateCache::default(), "0.6.0", NOW).is_none());
    }

    #[test]
    fn a_snooze_hides_its_version_for_seven_days() {
        let snoozed = snooze(&cache("0.6.0", NOW), "0.5.0", NOW).unwrap();
        assert_eq!(snoozed.snoozed_version.as_deref(), Some("0.6.0"));
        let during = notice(&snoozed, "0.5.0", NOW + 2 * DAY).unwrap();
        assert!(during.snoozed && !during.should_announce());
        let after = notice(&snoozed, "0.5.0", NOW + 8 * DAY).unwrap();
        assert!(!after.snoozed && after.should_announce());
    }

    #[test]
    fn a_newer_release_ends_the_snooze() {
        let mut snoozed = snooze(&cache("0.6.0", NOW), "0.5.0", NOW).unwrap();
        snoozed.latest = Some("0.7.0".into());
        snoozed.url = None;
        let notice = notice(&snoozed, "0.5.0", NOW + DAY).unwrap();
        assert_eq!(notice.latest, "0.7.0");
        assert!(!notice.snoozed);
    }

    #[test]
    fn nothing_to_snooze_when_up_to_date() {
        assert!(snooze(&cache("0.6.0", NOW), "0.6.0", NOW).is_none());
        assert!(snooze(&UpdateCache::default(), "0.6.0", NOW).is_none());
    }

    #[test]
    fn latest_release_reads_tag_and_page() {
        let json = r#"{"tag_name": "v0.6.0", "html_url": "https://github.com/mairistem/codev/releases/tag/v0.6.0", "assets": []}"#;
        assert_eq!(
            parse_latest_release(json),
            Some(Release {
                version: "0.6.0".into(),
                url: "https://github.com/mairistem/codev/releases/tag/v0.6.0".into(),
            })
        );
        assert_eq!(
            parse_latest_release(r#"{"tag_name": "v0.6.0"}"#)
                .unwrap()
                .url,
            "https://github.com/mairistem/codev/releases/tag/v0.6.0"
        );
        assert_eq!(parse_latest_release(r#"{"message": "Not Found"}"#), None);
        assert_eq!(parse_latest_release("<html>"), None);
    }

    #[test]
    fn asset_names_follow_the_release_workflow() {
        assert_eq!(
            asset_name("0.5.0", "aarch64-apple-darwin").as_deref(),
            Some("codev-0.5.0-aarch64-apple-darwin.tar.gz")
        );
        assert_eq!(
            asset_name("0.5.0", "x86_64-unknown-linux-gnu").as_deref(),
            Some("codev-0.5.0-x86_64-unknown-linux-musl.tar.gz")
        );
        assert_eq!(
            asset_name("0.5.0", "x86_64-pc-windows-msvc").as_deref(),
            Some("codev-0.5.0-x86_64-pc-windows-msvc.zip")
        );
        assert_eq!(
            archive_root("0.5.0", "x86_64-apple-darwin").as_deref(),
            Some("codev-0.5.0-x86_64-apple-darwin")
        );
        assert_eq!(binary_name("x86_64-pc-windows-msvc"), "codev.exe");
        assert_eq!(binary_name("aarch64-apple-darwin"), "codev");
        assert_eq!(
            download_url("0.5.0", "SHA256SUMS"),
            "https://github.com/mairistem/codev/releases/download/v0.5.0/SHA256SUMS"
        );
    }

    #[test]
    fn unsupported_targets_have_no_asset() {
        assert_eq!(asset_name("0.5.0", "aarch64-unknown-linux-gnu"), None);
        assert_eq!(asset_name("0.5.0", "aarch64-pc-windows-msvc"), None);
    }

    #[test]
    fn a_matching_checksum_passes() {
        let archive = b"archive bytes";
        let sums = format!(
            "{}  codev-0.6.0-x86_64-pc-windows-msvc.zip\n{} *codev-0.6.0-aarch64-apple-darwin.tar.gz\n",
            sha256_hex(b"other"),
            sha256_hex(archive).to_uppercase()
        );
        assert_eq!(
            verify_archive(archive, &sums, "codev-0.6.0-aarch64-apple-darwin.tar.gz"),
            Ok(())
        );
    }

    #[test]
    fn a_mismatching_or_missing_checksum_fails() {
        let sums = format!(
            "{}  codev-0.6.0-aarch64-apple-darwin.tar.gz\n",
            sha256_hex(b"genuine")
        );
        assert!(matches!(
            verify_archive(
                b"tampered",
                &sums,
                "codev-0.6.0-aarch64-apple-darwin.tar.gz"
            ),
            Err(ChecksumError::Mismatch { .. })
        ));
        let missing = verify_archive(b"x", &sums, "codev-0.6.0-x86_64-apple-darwin.tar.gz");
        assert!(matches!(missing, Err(ChecksumError::Missing { .. })));
        assert!(missing.unwrap_err().message().contains("no entry"));
    }

    #[test]
    fn cargo_installations_are_detected() {
        assert_eq!(
            install_method("/Users/me/.cargo/bin/codev", None, Some("/Users/me")),
            InstallMethod::Cargo
        );
        assert_eq!(
            install_method("/opt/cargo/bin/codev", Some("/opt/cargo/"), Some("/h")),
            InstallMethod::Cargo
        );
        assert!(
            InstallMethod::Cargo
                .upgrade_hint()
                .unwrap()
                .contains("cargo install --path crates/codev-cli")
        );
    }

    #[test]
    fn build_folders_are_detected() {
        for exe in [
            "/src/codev/target/debug/codev",
            "/src/codev/target/release/codev",
            "/src/codev/target/x86_64-unknown-linux-musl/release/codev",
        ] {
            assert_eq!(
                install_method(exe, None, None),
                InstallMethod::Build,
                "{exe}"
            );
        }
    }

    #[test]
    fn package_manager_trees_are_detected() {
        for (exe, name) in [
            ("/opt/homebrew/Cellar/codev/0.5.0/bin/codev", "Homebrew"),
            ("/usr/local/Cellar/codev/0.5.0/bin/codev", "Homebrew"),
            ("/nix/store/abc-codev-0.5.0/bin/codev", "Nix"),
        ] {
            assert_eq!(
                install_method(exe, None, None),
                InstallMethod::PackageManager(name),
                "{exe}"
            );
        }
        assert_eq!(
            install_method("/usr/bin/codev", None, None).as_str(),
            "package_manager"
        );
    }

    #[test]
    fn installer_locations_are_standalone() {
        assert_eq!(
            install_method("/Users/me/.local/bin/codev", None, Some("/Users/me")),
            InstallMethod::Standalone
        );
        assert_eq!(
            install_method("/usr/local/bin/codev", None, Some("/Users/me")),
            InstallMethod::Standalone
        );
        assert!(InstallMethod::Standalone.upgrade_hint().is_none());
    }

    #[test]
    fn windows_paths_are_classified() {
        assert_eq!(
            install_method(
                r"C:\Users\Me\AppData\Local\Programs\codev\codev.exe",
                None,
                Some(r"C:\Users\Me")
            ),
            InstallMethod::Standalone
        );
        assert_eq!(
            install_method(
                r"C:\Users\Me\.cargo\bin\codev.exe",
                None,
                Some(r"C:\Users\Me")
            ),
            InstallMethod::Cargo
        );
        assert_eq!(
            install_method(
                r"C:\Users\Me\scoop\apps\codev\current\codev.exe",
                None,
                None
            ),
            InstallMethod::PackageManager("Scoop")
        );
        // `current_exe` canonicalized on Windows is verbatim; the home folder
        // is not.
        assert_eq!(
            install_method(
                r"\\?\C:\Users\Me\.cargo\bin\codev.exe",
                None,
                Some(r"C:\Users\Me")
            ),
            InstallMethod::Cargo
        );
    }

    #[test]
    fn same_version_is_up_to_date() {
        assert_eq!(decide(&v("0.6.0"), &v("0.6.0"), false), Decision::UpToDate);
        assert_eq!(decide(&v("0.6.0"), &v("0.6.0"), true), Decision::UpToDate);
        assert_eq!(decide(&v("0.5.0"), &v("0.6.0"), false), Decision::Install);
    }

    #[test]
    fn implicit_downgrade_is_refused() {
        assert_eq!(
            decide(&v("0.7.0"), &v("0.6.0"), false),
            Decision::RefuseDowngrade
        );
    }

    #[test]
    fn explicit_downgrade_is_installed() {
        assert_eq!(decide(&v("0.6.0"), &v("0.5.0"), true), Decision::Install);
    }

    #[test]
    fn staged_binary_sits_next_to_the_target() {
        let (staged, aside) = staging_paths(Path::new("/home/me/.local/bin/codev"), "0.6.0");
        assert_eq!(staged, Path::new("/home/me/.local/bin/.codev-0.6.0.new"));
        assert_eq!(aside, Path::new("/home/me/.local/bin/codev.old"));
        let (staged, aside) = staging_paths(Path::new("/p/codev.exe"), "0.6.0");
        assert_eq!(staged, Path::new("/p/.codev-0.6.0.new.exe"));
        assert_eq!(aside, Path::new("/p/codev.old.exe"));
    }
}
