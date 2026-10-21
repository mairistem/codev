//! The commands the runner starts itself on an implementation case's
//! directory: the build preparation before the agent, then the checks, the
//! binary and the acceptance script after it. They run code the agent
//! wrote — `build.rs`, proc macros, tests, the binary itself — outside the
//! sandbox, so each one gets a cleared environment and a timeout. The pure
//! parts — the environment, the commands, the reading of the script's
//! output and of the agent's reads — come first and are unit-tested.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

/// The Cargo home of an implementation case, inside its directory: the
/// sandbox lets the agent's builds write there, and git ignores it.
pub const CARGO_HOME_DIR: &str = ".cargo-home";

/// How long each command of the runner may take, the script's aside: a
/// test the agent wrote that never ends must fail its check, not block a
/// job forever.
pub const COMMAND_TIMEOUT: Duration = Duration::from_secs(1200);
pub const ACCEPTANCE_TIMEOUT: Duration = Duration::from_secs(300);

/// The lines of a command's output kept in the observation.
pub const TAIL_LINES: usize = 40;

/// The variables of the operator's environment a command keeps: what the
/// toolchain needs to run, nothing that could hold a secret. `CARGO_*` is
/// not kept — a registry token lives there —: the runner sets its own.
pub(crate) fn kept(name: &str) -> bool {
    matches!(
        name,
        "PATH" | "HOME" | "TMPDIR" | "TMP" | "TEMP" | "LANG" | "LC_ALL" | "LC_CTYPE"
            // What a process needs to start on Windows.
            | "SYSTEMROOT" | "PATHEXT" | "COMSPEC" | "USERPROFILE"
    ) || name.starts_with("RUSTUP_")
}

/// The environment of a command on the case's directory `dir`: the kept
/// variables of `operator`, the case's Cargo home, and Cargo offline unless
/// the command fetches.
pub fn command_env(
    operator: &[(String, String)],
    dir: &Path,
    offline: bool,
) -> Vec<(String, String)> {
    let mut env: Vec<(String, String)> = operator
        .iter()
        .filter(|(name, _)| kept(name))
        .cloned()
        .collect();
    env.push((
        "CARGO_HOME".into(),
        dir.join(CARGO_HOME_DIR).to_string_lossy().into_owned(),
    ));
    if offline {
        env.push(("CARGO_NET_OFFLINE".into(), "true".into()));
    }
    env.push(("CODEV_NO_UPDATE_CHECK".into(), "1".into()));
    env
}

/// The variables Claude Code is given besides its own, so that the agent's
/// builds use the prepared Cargo home and stay offline.
pub fn agent_env(dir: &Path) -> Vec<(String, String)> {
    vec![
        (
            "CARGO_HOME".into(),
            dir.join(CARGO_HOME_DIR).to_string_lossy().into_owned(),
        ),
        ("CARGO_NET_OFFLINE".into(), "true".into()),
    ]
}

/// One command of the runner: its name in the observation, and `cargo`'s
/// arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CargoCommand {
    pub name: &'static str,
    pub args: &'static [&'static str],
    /// Whether it may reach the network: only the fetch does.
    pub online: bool,
}

/// Before the agent: fetch what `Cargo.lock` pins, then build the
/// workspace and its tests, so that the agent's builds need no network.
pub const PREPARATION: [CargoCommand; 2] = [
    CargoCommand {
        name: "fetch",
        args: &["fetch", "--locked"],
        online: true,
    },
    CargoCommand {
        name: "prebuild",
        args: &["build", "--workspace", "--all-targets", "--locked"],
        online: false,
    },
];

/// The build of the binary the acceptance script drives.
pub const BUILD: CargoCommand = CargoCommand {
    name: "build",
    args: &["build", "-p", "codev-cli"],
    online: false,
};

/// After the agent: the project's checks, then the binary the acceptance
/// script drives.
pub const CHECKS: [CargoCommand; 4] = [
    CargoCommand {
        name: "format",
        args: &["fmt", "--all", "--check"],
        online: false,
    },
    CargoCommand {
        name: "lints",
        args: &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        online: false,
    },
    CargoCommand {
        name: "tests",
        args: &["test", "--workspace"],
        online: false,
    },
    BUILD,
];

/// The binary [`CHECKS`]' build leaves, relative to the case's directory.
pub fn built_binary(dir: &Path) -> PathBuf {
    let name = if cfg!(windows) { "codev.exe" } else { "codev" };
    dir.join("target").join("debug").join(name)
}

/// How a command of the runner ended.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandCheck {
    pub name: String,
    pub passed: bool,
    #[serde(default)]
    pub timed_out: bool,
    /// The last [`TAIL_LINES`] lines of its output.
    pub tail: String,
}

/// The last `n` lines of `text`.
pub fn tail(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

/// The assertions an acceptance script printed, in order: `ok <name>` and
/// `not ok <name>`. Any other line is the script's own output.
pub fn printed_assertions(stdout: &str) -> Vec<(String, bool)> {
    stdout
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let (ok, name) = if let Some(name) = line.strip_prefix("not ok ") {
                (false, name)
            } else {
                (true, line.strip_prefix("ok ")?)
            };
            let name = name.trim();
            (!name.is_empty()).then(|| (name.to_string(), ok))
        })
        .collect()
}

/// Each assertion of `names`, `ok` only when the script printed `ok` for it
/// and never `not ok`: an assertion it did not print — a timeout, a crash
/// — failed.
pub fn acceptance_results(printed: &[(String, bool)], names: &[String]) -> BTreeMap<String, bool> {
    names
        .iter()
        .map(|name| {
            let lines: Vec<bool> = printed
                .iter()
                .filter(|(n, _)| n == name)
                .map(|(_, ok)| *ok)
                .collect();
            let ok = !lines.is_empty() && lines.iter().all(|ok| *ok);
            (name.clone(), ok)
        })
        .collect()
}

/// The paths a run's tool calls read or searched, and the shell commands
/// it ran, from Claude Code's event stream — the sub-agents' included.
pub fn tool_reads(events: &[serde_json::Value]) -> Vec<String> {
    let mut reads = Vec::new();
    for event in events {
        if event.get("type").and_then(|t| t.as_str()) != Some("assistant") {
            continue;
        }
        for block in event["message"]["content"].as_array().into_iter().flatten() {
            if block.get("type").and_then(|t| t.as_str()) != Some("tool_use") {
                continue;
            }
            let input = &block["input"];
            let field = |k: &str| input.get(k).and_then(|v| v.as_str()).map(str::to_string);
            let read = match block.get("name").and_then(|n| n.as_str()) {
                Some("Read" | "NotebookRead") => field("file_path"),
                Some("Grep" | "Glob") => field("path"),
                Some("Bash") => field("command"),
                _ => None,
            };
            reads.extend(read);
        }
    }
    reads
}

/// The reads that name `root`, a directory outside the case's: an absolute
/// path under it, or a command that names it. Both spellings of `root` are
/// tried — as given and resolved — since a temporary directory is often
/// reached through a link. A relative root names nothing: `.` would match
/// every path.
pub fn reads_under(reads: &[String], roots: &[PathBuf]) -> Vec<String> {
    let names: BTreeSet<String> = roots
        .iter()
        .filter(|r| r.is_absolute())
        .map(|r| r.to_string_lossy().trim_end_matches('/').to_string())
        .filter(|r| !r.is_empty())
        .collect();
    let mut found: Vec<String> = reads
        .iter()
        .filter(|read| {
            names.iter().any(|root| {
                read.match_indices(root.as_str()).any(|(at, _)| {
                    // `/repo-x` is not under `/repo`.
                    let after = &read[at + root.len()..];
                    after.is_empty() || after.starts_with(['/', '\\', ' ', '"', '\''])
                })
            })
        })
        .cloned()
        .collect();
    found.sort();
    found.dedup();
    found
}

/// The paths of a `git status --porcelain -z` output, renames by their new
/// path.
pub fn changed_paths(porcelain: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut entries = porcelain.split('\0').filter(|e| !e.is_empty());
    while let Some(entry) = entries.next() {
        if entry.len() < 4 {
            continue;
        }
        let (status, path) = entry.split_at(3);
        if status.starts_with('R') || status.starts_with('C') {
            entries.next();
        }
        out.push(path.to_string());
    }
    out.sort();
    out
}

/// Runs `program` with `args` in `dir`, with exactly `env`, in its own
/// process group, under `timeout`; its output, merged, goes to `log`. The
/// outcome is the check `name`.
pub fn run_command(
    name: &str,
    program: &Path,
    args: &[&str],
    dir: &Path,
    env: &[(String, String)],
    timeout: Duration,
    log: &Path,
) -> Result<CommandCheck, String> {
    let out = fs::File::create(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let err = out.try_clone().map_err(|e| e.to_string())?;
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(dir)
        .env_clear()
        .envs(env.iter().map(|(k, v)| (k, v)))
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    let mut child = command
        .spawn()
        .map_err(|e| format!("cannot start {}: {e}", program.display()))?;
    let started = Instant::now();
    let (passed, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status.success(), false),
            Ok(None) if started.elapsed() > timeout => {
                crate::runner::terminate(&mut child);
                break (false, true);
            }
            Ok(None) => std::thread::sleep(Duration::from_millis(200)),
            Err(e) => return Err(format!("waiting for {}: {e}", program.display())),
        }
    };
    let mut output = tail(&fs::read_to_string(log).unwrap_or_default(), TAIL_LINES);
    if timed_out {
        output.push_str(&format!("\n[timed out after {} s]", timeout.as_secs()));
    }
    Ok(CommandCheck {
        name: name.to_string(),
        passed,
        timed_out,
        tail: output,
    })
}

/// The operator's environment, as the commands filter it.
pub fn operator_env() -> Vec<(String, String)> {
    std::env::vars().collect()
}

/// Runs one cargo command on `dir`, logged as `<log>/<name>.log`.
fn cargo(
    command: &CargoCommand,
    dir: &Path,
    timeout: Duration,
    log: &Path,
) -> Result<CommandCheck, String> {
    let env = command_env(&operator_env(), dir, !command.online);
    run_command(
        command.name,
        Path::new("cargo"),
        command.args,
        dir,
        &env,
        timeout,
        &log.join(format!("{}.log", command.name)),
    )
}

/// Before the agent: hides the Cargo home from git, fetches and builds.
/// A step that fails or times out is an error naming it.
pub fn prepare(dir: &Path, log: &Path) -> Result<(), String> {
    let exclude = dir.join(".git").join("info").join("exclude");
    let mut text = fs::read_to_string(&exclude).unwrap_or_default();
    text.push_str(&format!("\n/{CARGO_HOME_DIR}/\n"));
    if let Some(parent) = exclude.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(&exclude, text).map_err(|e| format!("{}: {e}", exclude.display()))?;
    for command in &PREPARATION {
        let check = cargo(command, dir, COMMAND_TIMEOUT, log)?;
        if !check.passed {
            return Err(format!(
                "preparing the build: `cargo {}` {}: {}",
                command.args.join(" "),
                if check.timed_out {
                    "timed out"
                } else {
                    "failed"
                },
                check.tail
            ));
        }
    }
    Ok(())
}

/// After the agent: every check of [`CHECKS`], each recorded even when an
/// earlier one failed.
pub fn run_checks(dir: &Path, log: &Path) -> Vec<CommandCheck> {
    CHECKS
        .iter()
        .map(|command| {
            cargo(command, dir, COMMAND_TIMEOUT, log).unwrap_or_else(|e| CommandCheck {
                name: command.name.to_string(),
                passed: false,
                timed_out: false,
                tail: e,
            })
        })
        .collect()
}

/// Runs `script` with `bash` in a new empty directory under `log`, with
/// `CODEV` naming `binary`, under [`ACCEPTANCE_TIMEOUT`]; returns what it
/// printed and how it ended.
pub fn run_acceptance(
    script: &str,
    binary: &Path,
    log: &Path,
) -> Result<(Vec<(String, bool)>, CommandCheck), String> {
    // The script runs from its own empty folder: a relative `--out` would
    // leave `bash` looking for it there.
    let log = &fs::canonicalize(log).map_err(|e| format!("{}: {e}", log.display()))?;
    let file = log.join("acceptance.sh");
    fs::write(&file, script).map_err(|e| format!("{}: {e}", file.display()))?;
    let dir = log.join("acceptance-dir");
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut env: Vec<(String, String)> = operator_env()
        .into_iter()
        .filter(|(name, _)| kept(name))
        .collect();
    env.push(("CODEV".into(), binary.to_string_lossy().into_owned()));
    env.push(("CODEV_NO_UPDATE_CHECK".into(), "1".into()));
    let output = log.join("acceptance.log");
    let file_arg = file.to_string_lossy().into_owned();
    let end = run_command(
        "acceptance",
        Path::new("bash"),
        &[&file_arg],
        &dir,
        &env,
        ACCEPTANCE_TIMEOUT,
        &output,
    )?;
    let printed = printed_assertions(&fs::read_to_string(&output).unwrap_or_default());
    Ok((printed, end))
}

/// Removes what the builds left in the case's directory: about a gigabyte
/// per run. The code and the plan stay, to read the run again.
pub fn remove_build_outputs(dir: &Path) {
    for sub in ["target", CARGO_HOME_DIR] {
        let _ = fs::remove_dir_all(dir.join(sub));
    }
}

/// What `codev-evals calibrate` found on a case: what the script printed
/// on the merged commit and on the commit before it, and the checks on the
/// merged commit.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Calibration {
    pub merged: Vec<(String, bool)>,
    pub before: Vec<(String, bool)>,
    pub checks: Vec<CommandCheck>,
}

/// Why a script is not calibrated; empty when it is: every assertion `ok`
/// on the merged commit and no other name printed, the checks passing
/// there, and at least one assertion not `ok` before the change.
pub fn calibration_causes(names: &[String], found: &Calibration) -> Vec<String> {
    let mut causes = Vec::new();
    for (name, ok) in acceptance_results(&found.merged, names) {
        if !ok {
            causes.push(format!("`{name}` is not ok on the merged commit"));
        }
    }
    let mut extra: Vec<&str> = found
        .merged
        .iter()
        .map(|(n, _)| n.as_str())
        .filter(|n| !names.iter().any(|name| name == n))
        .collect();
    extra.dedup();
    for name in extra {
        causes.push(format!(
            "the script prints `{name}`, which `acceptance` does not name"
        ));
    }
    for check in found.checks.iter().filter(|c| !c.passed) {
        causes.push(format!(
            "`{}` fails on the merged commit{}",
            check.name,
            if check.timed_out { " (timed out)" } else { "" }
        ));
    }
    if acceptance_results(&found.before, names)
        .values()
        .all(|ok| *ok)
    {
        causes.push("no assertion fails before the change".to_string());
    }
    causes
}

/// The tree of `commit` in `repo`, extracted into `dir`, made a git
/// repository so that `git status` and the exclusions work as in a run.
pub fn extract(repo: &Path, commit: &str, dir: &Path) -> Result<(), String> {
    if dir.exists() {
        fs::remove_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let archive = dir.with_extension("tar");
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["archive", "--format=tar", "-o"])
        .arg(&archive)
        .arg(commit)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git archive {commit}: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    let untar = Command::new("tar")
        .arg("-xf")
        .arg(&archive)
        .arg("-C")
        .arg(dir)
        .output()
        .map_err(|e| format!("cannot run tar: {e}"));
    let _ = fs::remove_file(&archive);
    if !untar?.status.success() {
        return Err(format!("extracting {commit} failed"));
    }
    let init = Command::new("git")
        .args(["init", "-q"])
        .current_dir(dir)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !init.status.success() {
        return Err("git init failed".into());
    }
    Ok(())
}

/// Calibrates `script` on `merged` and `before`, each built in its own
/// directory under `work`, logs under `log`. An error is a tree that could
/// not be extracted, prepared or built.
pub fn calibrate(
    repo: &Path,
    merged: &str,
    before: &str,
    script: &str,
    work: &Path,
    log: &Path,
) -> Result<Calibration, String> {
    let (merged_printed, checks) = build_and_accept(
        repo,
        merged,
        &CHECKS,
        script,
        &work.join("merged"),
        &log.join("merged"),
    )?;
    // Before the change, only the script's verdict matters: the binary is
    // enough, and the old tree's checks say nothing about the script.
    let (before_printed, _) = build_and_accept(
        repo,
        before,
        &[BUILD],
        script,
        &work.join("before"),
        &log.join("before"),
    )?;
    Ok(Calibration {
        merged: merged_printed,
        before: before_printed,
        checks,
    })
}

/// What a tree built for calibration gave: the script's lines, and the
/// commands run on it.
type Accepted = (Vec<(String, bool)>, Vec<CommandCheck>);

/// Extracts `commit` into `dir`, prepares and runs `commands` on it, then
/// runs `script` against the binary they built; the build outputs are
/// removed whether it succeeds or not.
fn build_and_accept(
    repo: &Path,
    commit: &str,
    commands: &[CargoCommand],
    script: &str,
    dir: &Path,
    log: &Path,
) -> Result<Accepted, String> {
    fs::create_dir_all(log).map_err(|e| e.to_string())?;
    extract(repo, commit, dir)?;
    let run = || -> Result<Accepted, String> {
        prepare(dir, log)?;
        let checks: Vec<CommandCheck> = commands
            .iter()
            .map(|command| cargo(command, dir, COMMAND_TIMEOUT, log))
            .collect::<Result<_, _>>()?;
        if !checks.iter().any(|c| c.name == BUILD.name && c.passed) {
            return Err(format!("the binary does not build at {commit}"));
        }
        let (printed, _) = run_acceptance(script, &built_binary(dir), log)?;
        Ok((printed, checks))
    };
    let result = run();
    remove_build_outputs(dir);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operator() -> Vec<(String, String)> {
        [
            ("PATH", "/usr/bin"),
            ("HOME", "/home/op"),
            ("ANTHROPIC_API_KEY", "sk-secret"),
            ("GITHUB_TOKEN", "ghp-secret"),
            ("CARGO_REGISTRY_TOKEN", "cargo-secret"),
            ("RUSTUP_TOOLCHAIN", "stable"),
            ("LANG", "C.UTF-8"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }

    #[test]
    fn the_checks_never_see_the_api_key() {
        let env = command_env(&operator(), Path::new("/w/case"), true);
        let names: Vec<&str> = env.iter().map(|(k, _)| k.as_str()).collect();
        for secret in ["ANTHROPIC_API_KEY", "GITHUB_TOKEN", "CARGO_REGISTRY_TOKEN"] {
            assert!(!names.contains(&secret), "{secret} kept: {names:?}");
        }
        for needed in ["PATH", "HOME", "RUSTUP_TOOLCHAIN", "LANG", "CARGO_HOME"] {
            assert!(names.contains(&needed), "{needed} missing: {names:?}");
        }
    }

    #[test]
    fn an_implementation_run_builds_offline() {
        let dir = Path::new("/w/case");
        let home = dir.join(CARGO_HOME_DIR).to_string_lossy().into_owned();
        let offline = command_env(&operator(), dir, true);
        assert!(offline.contains(&("CARGO_HOME".to_string(), home.clone())));
        assert!(offline.contains(&("CARGO_NET_OFFLINE".to_string(), "true".to_string())));
        let fetch = command_env(&operator(), dir, false);
        assert!(!fetch.iter().any(|(k, _)| k == "CARGO_NET_OFFLINE"));
        assert_eq!(
            agent_env(dir),
            [
                ("CARGO_HOME".to_string(), home),
                ("CARGO_NET_OFFLINE".to_string(), "true".to_string())
            ]
        );
        assert_eq!(PREPARATION[0].args, ["fetch", "--locked"]);
        assert!(PREPARATION[0].online && !PREPARATION[1].online);
        assert!(CHECKS.iter().all(|c| !c.online));
        assert_eq!(
            CHECKS.iter().map(|c| c.name).collect::<Vec<_>>(),
            ["format", "lints", "tests", "build"]
        );
    }

    #[cfg(unix)]
    #[test]
    fn a_command_that_times_out_fails_its_check() {
        let log = std::env::temp_dir().join(format!("codev-evals-timeout-{}", std::process::id()));
        let _ = fs::remove_file(&log);
        let env = vec![("PATH".to_string(), "/bin:/usr/bin".to_string())];
        let check = run_command(
            "sleep",
            Path::new("sleep"),
            &["30"],
            &std::env::temp_dir(),
            &env,
            Duration::from_millis(300),
            &log,
        )
        .unwrap();
        let _ = fs::remove_file(&log);
        assert!(!check.passed && check.timed_out);
        assert!(check.tail.contains("timed out"), "{}", check.tail);
    }

    #[test]
    fn acceptance_lines_are_read_by_name() {
        let printed = printed_assertions(
            "building\nok accept-moves-the-status\nnot ok accept-twice-fails\n  ok  spaced  \nokay\n",
        );
        assert_eq!(
            printed,
            [
                ("accept-moves-the-status".to_string(), true),
                ("accept-twice-fails".to_string(), false),
                ("spaced".to_string(), true)
            ]
        );
    }

    #[test]
    fn an_assertion_not_printed_or_timed_out_fails() {
        let names: Vec<String> = ["a", "b", "c"].iter().map(|s| s.to_string()).collect();
        let printed = vec![
            ("a".to_string(), true),
            ("b".to_string(), true),
            ("b".to_string(), false),
        ];
        let results = acceptance_results(&printed, &names);
        assert!(results["a"]);
        assert!(!results["b"], "an assertion printed `not ok` once failed");
        assert!(!results["c"], "an assertion not printed failed");
    }

    #[test]
    fn a_read_of_the_cases_repository_is_recorded() {
        // A root absolute on every platform: `/src/codev` has no drive on
        // Windows.
        let root = if cfg!(windows) {
            r"C:\src\codev"
        } else {
            "/src/codev"
        };
        let sep = std::path::MAIN_SEPARATOR;
        let script = format!("{root}{sep}evals{sep}cases{sep}x{sep}acceptance.sh");
        let show = format!("git -C {root} show def456");
        let other = format!("{root}-other");
        let events = vec![
            serde_json::json!({"type": "assistant", "message": {"content": [
                {"type": "tool_use", "name": "Read", "input": {"file_path": script}},
                {"type": "tool_use", "name": "Read", "input": {"file_path": "/tmp/w/case/src/lib.rs"}},
                {"type": "tool_use", "name": "Bash", "input": {"command": show}},
                {"type": "tool_use", "name": "Grep", "input": {"pattern": "x", "path": other}},
                {"type": "text", "text": root}
            ]}}),
        ];
        let reads = tool_reads(&events);
        assert_eq!(reads.len(), 4);
        assert_eq!(reads_under(&reads, &[PathBuf::from(root)]), [script, show]);
    }

    #[test]
    fn changed_paths_follow_renames() {
        assert_eq!(
            changed_paths(" M src/a.rs\0R  new.rs\0old.rs\0?? docs/en/x.md\0"),
            ["docs/en/x.md", "new.rs", "src/a.rs"]
        );
    }

    #[test]
    fn the_tail_keeps_the_last_lines() {
        assert_eq!(tail("a\nb\nc\n", 2), "b\nc");
        assert_eq!(tail("", 2), "");
    }

    fn names() -> Vec<String> {
        vec!["moves".into(), "refuses".into()]
    }

    fn passing_checks() -> Vec<CommandCheck> {
        CHECKS
            .iter()
            .map(|c| CommandCheck {
                name: c.name.into(),
                passed: true,
                timed_out: false,
                tail: String::new(),
            })
            .collect()
    }

    #[test]
    fn a_script_that_cannot_tell_the_change_apart() {
        let all_ok = vec![("moves".into(), true), ("refuses".into(), true)];
        let found = Calibration {
            merged: all_ok.clone(),
            before: all_ok,
            checks: passing_checks(),
        };
        assert_eq!(
            calibration_causes(&names(), &found),
            ["no assertion fails before the change"]
        );
    }

    #[test]
    fn a_calibrated_script() {
        let found = Calibration {
            merged: vec![("moves".into(), true), ("refuses".into(), true)],
            before: vec![("moves".into(), false), ("refuses".into(), true)],
            checks: passing_checks(),
        };
        assert!(calibration_causes(&names(), &found).is_empty());
        let mut extra = found.clone();
        extra.merged.push(("unnamed".into(), true));
        assert_eq!(
            calibration_causes(&names(), &extra),
            ["the script prints `unnamed`, which `acceptance` does not name"]
        );
    }

    #[test]
    fn a_merged_commit_the_checks_reject() {
        let mut checks = passing_checks();
        checks[1].passed = false;
        let found = Calibration {
            merged: vec![("moves".into(), true), ("refuses".into(), false)],
            before: vec![("moves".into(), false)],
            checks,
        };
        assert_eq!(
            calibration_causes(&names(), &found),
            [
                "`refuses` is not ok on the merged commit",
                "`lints` fails on the merged commit"
            ]
        );
    }

    #[test]
    fn a_relative_root_names_no_read() {
        let reads = vec![
            "cat /tmp/w/case/src/lib.rs".to_string(),
            "ls ./src".to_string(),
        ];
        assert!(reads_under(&reads, &[PathBuf::from(".")]).is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn the_script_runs_from_a_relative_log_folder() {
        // A relative log folder, as `--out evals/baseline` gives: the
        // script must still be found from its own empty folder.
        let root = std::env::temp_dir()
            .canonicalize()
            .unwrap()
            .join(format!("codev-evals-acceptance-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let cwd = std::env::current_dir().unwrap().canonicalize().unwrap();
        let up: PathBuf = cwd.components().skip(1).map(|_| "..").collect();
        let log = up.join(root.strip_prefix("/").unwrap());
        assert!(log.is_relative() && log.is_dir(), "{}", log.display());
        let (printed, end) =
            run_acceptance("echo ok found\n", Path::new("/bin/true"), &log).unwrap();
        let _ = fs::remove_dir_all(&root);
        assert!(end.passed, "{}", end.tail);
        assert_eq!(printed, [("found".to_string(), true)]);
    }
}
