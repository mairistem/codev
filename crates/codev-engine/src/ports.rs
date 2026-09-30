use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// File system access, behind a port.
///
/// The point is not to be able to "swap file systems" — that will never
/// happen. It is that `init`, `new change` and `archive` become testable in
/// memory, with no temporary directory and no serialized tests.
///
/// The trait stays `dyn`-compatible: `codev-agents` holds a dynamic reference
/// to it so that it can itself remain usable through `dyn AgentTarget`.
pub trait FileSystem {
    fn exists(&self, path: &Path) -> bool;

    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Writes the file, creating its parent directories if needed.
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;

    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// The files under `dir`, recursively, as paths **relative to `dir`**,
    /// always separated by `/`.
    ///
    /// This format is not a detail: it is what
    /// `codev_core::outputs::pattern_matches_any` consumes, and it keeps
    /// platform-specific separators out of the pure core.
    ///
    /// A missing `dir` yields an empty list, not an error: "this change has
    /// no files yet" is a normal state, not a failure.
    fn walk_files(&self, dir: &Path) -> io::Result<Vec<String>>;

    /// The names of the direct entries of `dir`. Empty if `dir` is missing.
    fn list_dir(&self, dir: &Path) -> io::Result<Vec<String>>;

    /// Moves `from` to `to`. Creates the parent directories of `to` if needed.
    ///
    /// The real implementation first tries `std::fs::rename` (atomic within a
    /// volume) and falls back to copy+remove on a cross-device error.
    /// Every implementation must handle a file as well as a directory —
    /// that is what `codev archive` does with the whole change directory.
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;

    /// Removes a single file. A missing file returns
    /// [`io::ErrorKind::NotFound`] — the executor (`apply::execute`) decides
    /// whether to ignore or propagate it. The port's only role is to expose
    /// the destructive operation; any policy is decided upstream.
    fn remove_file(&self, path: &Path) -> io::Result<()>;
}

/// The clock, behind a port.
///
/// Deliberately narrow: these dates are only used to name an archive
/// directory and to date a decision. A [`FixedClock`] makes tests
/// deterministic in one line.
pub trait Clock {
    /// Today's date, in `YYYY-MM-DD` format.
    fn today(&self) -> String;
}

/// An external process that was run, with its output captured.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub exit_code: i32,
}

impl ProcessOutput {
    pub fn stdout_str(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
    pub fn stderr_str(&self) -> String {
        String::from_utf8_lossy(&self.stderr).into_owned()
    }
    pub fn is_ok(&self) -> bool {
        self.exit_code == 0
    }
}

/// Driving an external binary, behind a port.
///
/// Used for `git` by `codev sources update` — the only codev operation that
/// contacts a remote service. The port makes it possible both to drive the
/// real `git` and to write deterministic tests that never touch the
/// network.
pub trait ProcessRunner {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>) -> io::Result<ProcessOutput>;
}

pub struct RealProcessRunner;

impl ProcessRunner for RealProcessRunner {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>) -> io::Result<ProcessOutput> {
        let mut command = Command::new(program);
        command.args(args);
        if let Some(cwd) = cwd {
            command.current_dir(cwd);
        }
        match command.output() {
            Ok(output) => Ok(ProcessOutput {
                stdout: output.stdout,
                stderr: output.stderr,
                exit_code: output.status.code().unwrap_or(-1),
            }),
            Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(ProcessOutput {
                stdout: Vec::new(),
                stderr: format!("{program}: command not found").into_bytes(),
                exit_code: 127,
            }),
            Err(err) => Err(err),
        }
    }
}

/// Test implementation that returns a predefined response per `(program,
/// first argument)` tuple. Enough for the `update` scenarios — sequential
/// driving of `git ls-remote`, `git fetch`, `git worktree add`.
#[derive(Debug, Default)]
pub struct MockProcessRunner {
    /// Responses associated with a `(program, args_prefix)` prefix.
    responses: RefCell<Vec<(String, Vec<String>, ProcessOutput)>>,
    /// Log of the actual calls — for assertions in tests.
    calls: RefCell<Vec<(String, Vec<String>)>>,
}

impl MockProcessRunner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_response(
        self,
        program: &str,
        args_prefix: &[&str],
        response: ProcessOutput,
    ) -> Self {
        self.responses.borrow_mut().push((
            program.to_string(),
            args_prefix.iter().map(|s| s.to_string()).collect(),
            response,
        ));
        self
    }

    pub fn calls(&self) -> Vec<(String, Vec<String>)> {
        self.calls.borrow().clone()
    }
}

impl ProcessRunner for MockProcessRunner {
    fn run(&self, program: &str, args: &[&str], _cwd: Option<&Path>) -> io::Result<ProcessOutput> {
        self.calls.borrow_mut().push((
            program.to_string(),
            args.iter().map(|s| s.to_string()).collect(),
        ));
        let responses = self.responses.borrow();
        let matched = responses.iter().find(|(prog, prefix, _)| {
            prog == program
                && prefix
                    .iter()
                    .zip(args.iter())
                    .all(|(expected, actual)| expected == actual)
                && prefix.len() <= args.len()
        });
        match matched {
            Some((_, _, resp)) => Ok(resp.clone()),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("no mock response for {program} {args:?}"),
            )),
        }
    }
}

/// The execution environment, behind a port.
pub trait Env {
    fn current_dir(&self) -> io::Result<PathBuf>;
    fn var(&self, key: &str) -> Option<String>;

    /// The home directory, used to expand a `~` in a declared path.
    ///
    /// Read from the environment rather than through `std::env::home_dir`,
    /// whose deprecation history is not worth following here.
    fn home_dir(&self) -> Option<PathBuf> {
        self.var("HOME")
            .or_else(|| self.var("USERPROFILE"))
            .map(PathBuf::from)
    }
}

// ─────────────────────────── real implementations ───────────────────────────

pub struct RealFileSystem;

impl FileSystem for RealFileSystem {
    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        std::fs::read_to_string(path)
    }

    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, contents)
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        std::fs::create_dir_all(path)
    }

    fn walk_files(&self, dir: &Path) -> io::Result<Vec<String>> {
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut files = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(current) = stack.pop() {
            for entry in std::fs::read_dir(&current)? {
                let entry = entry?;
                let path = entry.path();
                if entry.file_type()?.is_dir() {
                    stack.push(path);
                } else if let Ok(relative) = path.strip_prefix(dir) {
                    files.push(to_slash(relative));
                }
            }
        }
        files.sort();
        Ok(files)
    }

    fn list_dir(&self, dir: &Path) -> io::Result<Vec<String>> {
        if !dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut names: Vec<String> = std::fs::read_dir(dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        Ok(names)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        // `create_dir_all` on the destination's parent, otherwise a move to
        // `archive/YYYY-MM-DD-name/` would fail whenever `archive/` does not
        // exist yet.
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        match std::fs::rename(from, to) {
            Ok(()) => Ok(()),
            Err(err) if is_cross_device(&err) => copy_then_remove(from, to),
            Err(err) => Err(err),
        }
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        std::fs::remove_file(path)
    }
}

/// `rename(2)` fails with `EXDEV` when `from` and `to` are on different file
/// systems. The fallback is kept silent rather than fatal: unlikely in
/// practice (the planning directory and the code share the same volume), but
/// the fallback avoids an unreadable error message in the rare cases where
/// it happens.
fn is_cross_device(err: &io::Error) -> bool {
    err.raw_os_error() == Some(18) // EXDEV on Linux/macOS
}

fn copy_then_remove(from: &Path, to: &Path) -> io::Result<()> {
    if from.is_dir() {
        copy_dir_recursive(from, to)?;
        std::fs::remove_dir_all(from)
    } else {
        std::fs::copy(from, to)?;
        std::fs::remove_file(from)
    }
}

fn copy_dir_recursive(from: &Path, to: &Path) -> io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

fn to_slash(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn today(&self) -> String {
        // The local date is the one users read on their clock, hence the one
        // they expect in an archive directory name. It fails on some system
        // configurations: we then fall back to UTC, which beats refusing to
        // archive.
        let now =
            time::OffsetDateTime::now_local().unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        format!(
            "{:04}-{:02}-{:02}",
            now.year(),
            u8::from(now.month()),
            now.day()
        )
    }
}

/// Frozen clock, for tests and reproducible runs.
pub struct FixedClock(pub String);

impl Clock for FixedClock {
    fn today(&self) -> String {
        self.0.clone()
    }
}

pub struct SystemEnv;

impl Env for SystemEnv {
    fn current_dir(&self) -> io::Result<PathBuf> {
        std::env::current_dir()
    }

    fn var(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }
}

// ─────────────────────────── in-memory doubles ───────────────────────────

/// In-memory file system.
///
/// Public and not gated behind `cfg(test)`: the crates above use it to test
/// their own plans without having to redeclare a double.
#[derive(Default)]
pub struct MemoryFileSystem {
    files: RefCell<BTreeMap<PathBuf, String>>,
    dirs: RefCell<Vec<PathBuf>>,
}

impl MemoryFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds a file, as in "here is the state of the disk before the operation".
    pub fn with_file(self, path: impl Into<PathBuf>, contents: impl Into<String>) -> Self {
        self.files.borrow_mut().insert(path.into(), contents.into());
        self
    }

    pub fn read(&self, path: impl AsRef<Path>) -> Option<String> {
        self.files.borrow().get(path.as_ref()).cloned()
    }

    pub fn paths(&self) -> Vec<PathBuf> {
        self.files.borrow().keys().cloned().collect()
    }
}

impl FileSystem for MemoryFileSystem {
    fn exists(&self, path: &Path) -> bool {
        if self.files.borrow().contains_key(path) {
            return true;
        }
        if self.dirs.borrow().iter().any(|d| d == path) {
            return true;
        }
        // A directory exists as soon as a file lives under it, just like on a
        // real disk.
        self.files
            .borrow()
            .keys()
            .any(|f| f.ancestors().any(|a| a == path))
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files.borrow().get(path).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} is missing", path.display()),
            )
        })
    }

    fn write(&self, path: &Path, contents: &str) -> io::Result<()> {
        self.files
            .borrow_mut()
            .insert(path.to_path_buf(), contents.to_string());
        Ok(())
    }

    fn create_dir_all(&self, path: &Path) -> io::Result<()> {
        self.dirs.borrow_mut().push(path.to_path_buf());
        Ok(())
    }

    fn walk_files(&self, dir: &Path) -> io::Result<Vec<String>> {
        let mut files: Vec<String> = self
            .files
            .borrow()
            .keys()
            .filter_map(|path| path.strip_prefix(dir).ok().map(to_slash))
            .filter(|relative| !relative.is_empty())
            .collect();
        files.sort();
        Ok(files)
    }

    fn list_dir(&self, dir: &Path) -> io::Result<Vec<String>> {
        let mut names: Vec<String> = self
            .walk_files(dir)?
            .iter()
            .filter_map(|relative| relative.split('/').next().map(str::to_string))
            .collect();
        names.sort();
        names.dedup();
        Ok(names)
    }

    fn rename(&self, from: &Path, to: &Path) -> io::Result<()> {
        // Collect everything that lives under `from` — single file or tree —
        // before mutating, to avoid reading and writing the same `RefCell`
        // in a single pass.
        let to_move: Vec<(PathBuf, String)> = self
            .files
            .borrow()
            .iter()
            .filter_map(|(path, contents)| {
                if path == from {
                    // Single file matching `from` exactly: its new location
                    // is `to`, not `to.push("")`.
                    Some((to.to_path_buf(), contents.clone()))
                } else if let Ok(relative) = path.strip_prefix(from) {
                    let mut new_path = to.to_path_buf();
                    new_path.push(relative);
                    Some((new_path, contents.clone()))
                } else {
                    None
                }
            })
            .collect();

        if to_move.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} does not exist", from.display()),
            ));
        }

        let mut files = self.files.borrow_mut();
        // Purge: everything that is `from` or below it goes away.
        let to_remove: Vec<PathBuf> = files
            .keys()
            .filter(|p| p.as_path() == from || p.strip_prefix(from).is_ok())
            .cloned()
            .collect();
        for p in to_remove {
            files.remove(&p);
        }
        // Reinsert under the new root.
        for (new_path, contents) in to_move {
            files.insert(new_path, contents);
        }
        Ok(())
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        let mut files = self.files.borrow_mut();
        if files.remove(path).is_some() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} is missing", path.display()),
            ))
        }
    }
}

/// Frozen environment, for tests.
pub struct FixedEnv {
    pub cwd: PathBuf,
    pub vars: BTreeMap<String, String>,
}

impl FixedEnv {
    pub fn at(cwd: impl Into<PathBuf>) -> Self {
        Self {
            cwd: cwd.into(),
            vars: BTreeMap::new(),
        }
    }
}

impl Env for FixedEnv {
    fn current_dir(&self) -> io::Result<PathBuf> {
        Ok(self.cwd.clone())
    }

    fn var(&self, key: &str) -> Option<String> {
        self.vars.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_sees_directories_implied_by_its_files() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "schema: spec-driven");
        assert!(fs.exists(Path::new("/p/_codev/config.yaml")));
        assert!(fs.exists(Path::new("/p/_codev")));
        assert!(fs.exists(Path::new("/p")));
        assert!(!fs.exists(Path::new("/p/_codev/specs")));
    }

    #[test]
    fn walk_files_returns_slash_separated_relative_paths() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/changes/add-auth/proposal.md", "x")
            .with_file("/p/changes/add-auth/specs/user-auth/spec.md", "y");
        let files = fs.walk_files(Path::new("/p/changes/add-auth")).unwrap();
        assert_eq!(files, ["proposal.md", "specs/user-auth/spec.md"]);
    }

    #[test]
    fn walk_files_on_a_missing_directory_yields_nothing() {
        let fs = MemoryFileSystem::new();
        assert!(fs.walk_files(Path::new("/nowhere")).unwrap().is_empty());
    }

    #[test]
    fn fixed_clock_is_deterministic() {
        assert_eq!(FixedClock("2026-09-08".into()).today(), "2026-09-08");
    }

    #[test]
    fn memory_rename_moves_a_whole_tree() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/_codev/changes/x/proposal.md", "p")
            .with_file("/p/_codev/changes/x/specs/y/spec.md", "s");

        fs.rename(
            Path::new("/p/_codev/changes/x"),
            Path::new("/p/_codev/changes/archive/2026-09-08-x"),
        )
        .unwrap();

        assert!(!fs.exists(Path::new("/p/_codev/changes/x")));
        assert_eq!(
            fs.read("/p/_codev/changes/archive/2026-09-08-x/proposal.md")
                .as_deref(),
            Some("p")
        );
        assert_eq!(
            fs.read("/p/_codev/changes/archive/2026-09-08-x/specs/y/spec.md")
                .as_deref(),
            Some("s")
        );
    }

    #[test]
    fn memory_rename_of_a_single_file() {
        let fs = MemoryFileSystem::new().with_file("/a/x.md", "hello");
        fs.rename(Path::new("/a/x.md"), Path::new("/b/y.md"))
            .unwrap();
        assert!(!fs.exists(Path::new("/a/x.md")));
        assert_eq!(fs.read("/b/y.md").as_deref(), Some("hello"));
    }

    #[test]
    fn memory_rename_of_a_missing_source_is_an_error() {
        let fs = MemoryFileSystem::new();
        let err = fs
            .rename(Path::new("/not/here"), Path::new("/elsewhere"))
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn mock_process_runner_answers_expected_commands() {
        let runner = MockProcessRunner::new().with_response(
            "git",
            &["ls-remote"],
            ProcessOutput {
                stdout: b"deadbeef refs/heads/main\n".to_vec(),
                stderr: Vec::new(),
                exit_code: 0,
            },
        );
        let out = runner
            .run("git", &["ls-remote", "url", "main"], None)
            .unwrap();
        assert!(out.is_ok());
        assert!(out.stdout_str().starts_with("deadbeef"));
        assert_eq!(runner.calls().len(), 1);
    }

    #[test]
    fn real_process_runner_reports_a_missing_binary() {
        let runner = RealProcessRunner;
        let out = runner
            .run("codev-nonexistent-binary-abc", &["--help"], None)
            .unwrap();
        assert_eq!(out.exit_code, 127);
        assert!(
            out.stderr_str().contains("not found"),
            "{}",
            out.stderr_str()
        );
    }

    #[test]
    fn home_directory_comes_from_the_environment() {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/Users/me".into());
        assert_eq!(env.home_dir(), Some(PathBuf::from("/Users/me")));
        assert_eq!(FixedEnv::at("/p").home_dir(), None);
    }
}
