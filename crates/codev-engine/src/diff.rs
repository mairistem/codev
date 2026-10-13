//! The change's diff, read from git: what the branch changed since it left
//! the default branch, committed or not, plus the untracked files — the
//! material the experts' triggers are matched against.
//!
//! Git is reached through the [`ProcessRunner`] port only, for local,
//! read-only commands (`rev-parse`, `symbolic-ref`, `merge-base`, `diff`,
//! `ls-files`): codev never fetches. Which files are noise, the caps and the
//! matching are decided in `codev_core::triggers` (decision 0001); this
//! module only reads, and stops reading once the core's caps are reached.
//!
//! Nothing here fails: whatever goes wrong — no repository, no commit, no
//! default branch, a git error — is a [`Diff::Unavailable`] with its reason.

use std::path::Path;

use codev_core::triggers::{
    self, AddedLine, ChangedFile, MAX_FILE_ADDED_LINES, MAX_FILES, MAX_SCAN_BYTES, Skip,
};

use crate::ports::{FileSystem, ProcessRunner};

/// Given to every git command: paths come out unescaped (`core.quotePath`),
/// and a repository unpacked from an archive with its own `.git/config`
/// cannot make codev run its file-system monitor program.
pub const GIT_CONFIG: &[&str] = &["-c", "core.quotePath=false", "-c", "core.fsmonitor=false"];

/// The options of both `git diff` calls: no external diff or text
/// conversion program runs on codev's behalf, renames read as a removal
/// and an addition, submodules are their own repositories, and paths are
/// relative to — and limited to — the project directory.
const DIFF_OPTIONS: &[&str] = &[
    "--no-ext-diff",
    "--no-textconv",
    "--no-renames",
    "--ignore-submodules",
    "--relative",
];

/// A patch batch holds at most this many paths: the command line stays
/// well under Windows' limit.
const BATCH_PATHS: usize = 50;
/// … and at most this many announced added lines: one batch's output stays
/// bounded.
const BATCH_LINES: usize = 20_000;

/// The change's diff, or why there is none.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Unavailable {
        reason: String,
    },
    Available {
        /// The default branch the merge-base was taken with, as found:
        /// `origin/main`, `main` or `master`.
        base: String,
        /// Every changed file, in path order; the lines are read for the
        /// files the patterns will match, within the caps.
        files: Vec<ChangedFile>,
    },
}

/// Reads the diff of the project rooted at `root`.
pub fn read(fs: &dyn FileSystem, runner: &dyn ProcessRunner, root: &Path) -> Diff {
    match read_available(fs, runner, root) {
        Ok((base, files)) => Diff::Available { base, files },
        Err(reason) => Diff::Unavailable { reason },
    }
}

fn read_available(
    fs: &dyn FileSystem,
    runner: &dyn ProcessRunner,
    root: &Path,
) -> Result<(String, Vec<ChangedFile>), String> {
    let git = Git { runner, root };
    match git.run(&["rev-parse", "--is-inside-work-tree"]) {
        Ok(out) if out.trim() == "true" => {}
        Err(GitError::Missing) => return Err(GitError::Missing.to_string()),
        _ => return Err("the project is not in a git repository".into()),
    }
    git.run(&["rev-parse", "--verify", "--quiet", "HEAD"])
        .map_err(|_| "the repository has no commit yet".to_string())?;
    let base = default_branch(&git)
        .ok_or("no default branch: looked for origin/HEAD, main and master".to_string())?;
    let merge_base = git
        .run(&["merge-base", "HEAD", &base])
        .map(|out| out.trim().to_string())
        .map_err(|_| format!("no common ancestor of HEAD and {base} (a shallow clone has none)"))?;

    let mut numstat = vec!["diff"];
    numstat.extend(DIFF_OPTIONS);
    numstat.extend(["--numstat", "-z", merge_base.as_str()]);
    let numstat = git.run(&numstat).map_err(|e| e.to_string())?;
    let untracked = git
        .run(&["ls-files", "--others", "--exclude-standard", "-z"])
        .map_err(|e| e.to_string())?;

    let mut files = parse_numstat(&numstat);
    for path in untracked.split('\0').filter(|p| !p.is_empty()) {
        let mut file = ChangedFile {
            path: path.to_string(),
            ..ChangedFile::default()
        };
        // A path the core leaves out of every trigger — an unignored
        // `node_modules` lists thousands — needs no metadata.
        if !triggers::classify(path).is_some_and(Skip::excludes_path) {
            let full = root.join(path);
            // Never read: a link may point anywhere, a device never ends.
            file.binary = !fs.is_regular_file(&full);
            if !file.binary {
                file.bytes = fs.len(&full).ok();
            }
        }
        files.push(file);
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files.dedup_by(|a, b| a.path == b.path);
    read_lines(fs, &git, root, &merge_base, &mut files);
    Ok((base, files))
}

/// `origin/HEAD`'s target, else the local `main`, else `master`.
fn default_branch(git: &Git) -> Option<String> {
    if let Ok(out) = git.run(&[
        "symbolic-ref",
        "--quiet",
        "--short",
        "refs/remotes/origin/HEAD",
    ]) {
        let name = out.trim();
        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    ["main", "master"].into_iter().find_map(|branch| {
        git.run(&[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("refs/heads/{branch}"),
        ])
        .ok()
        .map(|_| branch.to_string())
    })
}

/// `added\tremoved\tpath\0` records; a binary file counts `-`.
fn parse_numstat(out: &str) -> Vec<ChangedFile> {
    out.split('\0')
        .filter_map(|record| {
            let mut fields = record.splitn(3, '\t');
            let added = fields.next()?.trim_start_matches('\n');
            let _removed = fields.next()?;
            let path = fields.next()?;
            if path.is_empty() {
                return None;
            }
            Some(ChangedFile {
                path: path.to_string(),
                binary: added == "-",
                added_lines: added.parse().ok(),
                ..ChangedFile::default()
            })
        })
        .collect()
}

/// Reads the added lines of the files the patterns will match — the first
/// [`MAX_FILES`] in path order that the core does not skip — until
/// [`MAX_SCAN_BYTES`] are read. Tracked files come from the patch, in
/// bounded batches; untracked files from the disk.
fn read_lines(
    fs: &dyn FileSystem,
    git: &Git,
    root: &Path,
    merge_base: &str,
    files: &mut [ChangedFile],
) {
    let mut read_bytes = 0usize;
    let mut batch: Vec<usize> = Vec::new();
    let mut batch_lines = 0usize;
    let limit = files.len().min(MAX_FILES);
    for index in 0..limit {
        if read_bytes > MAX_SCAN_BYTES {
            break;
        }
        if triggers::skip_of(&files[index]).is_some() {
            continue;
        }
        match files[index].added_lines {
            // Tracked: its lines come from the patch.
            Some(added) => {
                batch.push(index);
                batch_lines += added.min(MAX_FILE_ADDED_LINES);
                if batch.len() >= BATCH_PATHS || batch_lines >= BATCH_LINES {
                    read_bytes += read_batch(git, merge_base, files, &batch);
                    batch.clear();
                    batch_lines = 0;
                }
            }
            // Untracked: the whole file is added.
            None => match fs.read_to_string(&root.join(&files[index].path)) {
                Ok(text) => {
                    let lines = numbered(&text);
                    read_bytes += lines.iter().map(|l| l.text.len()).sum::<usize>();
                    files[index].lines = Some(lines);
                }
                // Not UTF-8 text.
                Err(_) => files[index].binary = true,
            },
        }
    }
    if !batch.is_empty() && read_bytes <= MAX_SCAN_BYTES {
        read_batch(git, merge_base, files, &batch);
    }
}

/// Runs one patch for the files at `indexes` and stores their added lines;
/// returns the bytes read. A failed batch leaves its files unread.
fn read_batch(git: &Git, merge_base: &str, files: &mut [ChangedFile], indexes: &[usize]) -> usize {
    let pathspecs: Vec<String> = indexes
        .iter()
        .map(|&i| format!(":(literal){}", files[i].path))
        .collect();
    let mut args = vec!["diff"];
    args.extend(DIFF_OPTIONS);
    args.extend([
        "--unified=0",
        "--no-color",
        "--src-prefix=a/",
        "--dst-prefix=b/",
        merge_base,
        "--",
    ]);
    args.extend(pathspecs.iter().map(String::as_str));
    let Ok(patch) = git.run(&args) else {
        return 0;
    };
    let mut bytes = 0;
    for (path, lines) in parse_patch(&patch) {
        if let Some(&index) = indexes.iter().find(|&&i| files[i].path == path) {
            bytes += lines.iter().map(|l| l.text.len()).sum::<usize>();
            files[index].lines = Some(lines);
        }
    }
    // A file of the batch the patch says nothing about (a mode change
    // only) has no added line.
    for &index in indexes {
        files[index].lines.get_or_insert_with(Vec::new);
    }
    bytes
}

/// The added lines of each file of a `--unified=0` patch, numbered in the
/// new file. Hunk sizes are counted down, so an added line that starts
/// with `++` is never read as a header.
fn parse_patch(patch: &str) -> Vec<(String, Vec<AddedLine>)> {
    let mut files: Vec<(String, Vec<AddedLine>)> = Vec::new();
    let mut old_left = 0usize;
    let mut new_left = 0usize;
    let mut next_line = 0u32;
    for raw in patch.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if old_left > 0 || new_left > 0 {
            if let Some(added) = line.strip_prefix('+') {
                new_left = new_left.saturating_sub(1);
                if let Some((_, lines)) = files.last_mut() {
                    lines.push(AddedLine {
                        number: next_line,
                        text: added.to_string(),
                    });
                }
                next_line += 1;
            } else if line.starts_with('-') {
                old_left = old_left.saturating_sub(1);
            } else if line.starts_with(' ') {
                old_left = old_left.saturating_sub(1);
                new_left = new_left.saturating_sub(1);
                next_line += 1;
            }
            // `\ No newline at end of file` counts for nothing.
            continue;
        }
        if let Some(path) = line.strip_prefix("+++ ") {
            if let Some(path) = unquote(path).strip_prefix("b/") {
                files.push((path.to_string(), Vec::new()));
            }
        } else if let Some(header) = line.strip_prefix("@@ ")
            && let Some((old, new)) = hunk_sizes(header)
        {
            (old_left, new_left) = (old.1, new.1);
            next_line = new.0;
        }
    }
    files
}

/// `-12,3 +14,5 @@ …` → `((12, 3), (14, 5))`; a missing count is 1.
fn hunk_sizes(header: &str) -> Option<((u32, usize), (u32, usize))> {
    let mut parts = header.split_whitespace();
    let old = range(parts.next()?.strip_prefix('-')?)?;
    let new = range(parts.next()?.strip_prefix('+')?)?;
    Some((old, new))
}

fn range(text: &str) -> Option<(u32, usize)> {
    match text.split_once(',') {
        Some((start, count)) => Some((start.parse().ok()?, count.parse().ok()?)),
        None => Some((text.parse().ok()?, 1)),
    }
}

/// A path git quoted because it holds a quote, a backslash or a control
/// character: `"b/a\"b"` → `b/a"b`. Octal escapes are UTF-8 bytes.
fn unquote(path: &str) -> String {
    let Some(inner) = path.strip_prefix('"').and_then(|p| p.strip_suffix('"')) else {
        return path.to_string();
    };
    let mut bytes = Vec::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            let mut buffer = [0; 4];
            bytes.extend_from_slice(c.encode_utf8(&mut buffer).as_bytes());
            continue;
        }
        match chars.next() {
            Some('n') => bytes.push(b'\n'),
            Some('t') => bytes.push(b'\t'),
            Some('r') => bytes.push(b'\r'),
            Some(digit @ '0'..='7') => {
                let mut value = digit.to_digit(8).unwrap_or(0);
                for _ in 0..2 {
                    if let Some(d) = chars.peek().and_then(|d| d.to_digit(8)) {
                        value = value * 8 + d;
                        chars.next();
                    }
                }
                bytes.push(value as u8);
            }
            Some(other) => {
                let mut buffer = [0; 4];
                bytes.extend_from_slice(other.encode_utf8(&mut buffer).as_bytes());
            }
            None => {}
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Every line of an untracked file, numbered from 1, without `\r`.
fn numbered(text: &str) -> Vec<AddedLine> {
    text.lines()
        .enumerate()
        .map(|(index, line)| AddedLine {
            number: index as u32 + 1,
            text: line.strip_suffix('\r').unwrap_or(line).to_string(),
        })
        .collect()
}

/// Why a git command gave no output.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GitError {
    /// The `git` program cannot be found.
    Missing,
    /// It ran and failed, or could not be started: the reason.
    Failed(String),
}

impl std::fmt::Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing => f.write_str("git is not installed"),
            Self::Failed(reason) => f.write_str(reason),
        }
    }
}

/// Runs git in the project root with [`GIT_CONFIG`].
struct Git<'a> {
    runner: &'a dyn ProcessRunner,
    root: &'a Path,
}

impl Git<'_> {
    /// Stdout of a successful command; why there is none otherwise.
    fn run(&self, args: &[&str]) -> Result<String, GitError> {
        let mut full: Vec<&str> = GIT_CONFIG.to_vec();
        full.extend(args);
        match self.runner.run("git", &full, Some(self.root)) {
            Ok(out) if out.is_ok() => Ok(out.stdout_str()),
            Ok(out) if out.exit_code == 127 => Err(GitError::Missing),
            Ok(out) => Err(GitError::Failed(format!(
                "`git {}` failed: {}",
                args.first().copied().unwrap_or_default(),
                out.stderr_str().trim()
            ))),
            Err(err) => Err(GitError::Failed(format!("git could not be run: {err}"))),
        }
    }
}

/// Doubles of a git repository, for the tests of this crate and of the
/// crates above — public for the same reason as `MemoryFileSystem`.
pub mod doubles {
    use super::{DIFF_OPTIONS, GIT_CONFIG};
    use crate::ports::{MockProcessRunner, ProcessOutput};

    /// `rest` after [`GIT_CONFIG`]: how every git call of this module
    /// starts.
    pub fn args<'a>(rest: &[&'a str]) -> Vec<&'a str> {
        let mut all: Vec<&str> = GIT_CONFIG.to_vec();
        all.extend(rest);
        all
    }

    pub fn ok(stdout: &str) -> ProcessOutput {
        ProcessOutput {
            stdout: stdout.as_bytes().to_vec(),
            stderr: Vec::new(),
            exit_code: 0,
        }
    }

    pub fn failed() -> ProcessOutput {
        ProcessOutput {
            stdout: Vec::new(),
            stderr: b"fatal".to_vec(),
            exit_code: 128,
        }
    }

    /// A repository on a branch whose default branch is `base`, the
    /// merge-base `abc123`, with the given numstat, untracked files and
    /// patch.
    pub fn repository(
        origin_head: Option<&str>,
        numstat: &str,
        untracked: &str,
        patch: &str,
    ) -> MockProcessRunner {
        let runner = MockProcessRunner::new()
            .with_response(
                "git",
                &args(&["rev-parse", "--is-inside-work-tree"]),
                ok("true\n"),
            )
            .with_response(
                "git",
                &args(&["rev-parse", "--verify", "--quiet", "HEAD"]),
                ok("f00\n"),
            );
        let runner = match origin_head {
            Some(head) => runner.with_response(
                "git",
                &args(&[
                    "symbolic-ref",
                    "--quiet",
                    "--short",
                    "refs/remotes/origin/HEAD",
                ]),
                ok(&format!("{head}\n")),
            ),
            None => runner
                .with_response(
                    "git",
                    &args(&[
                        "symbolic-ref",
                        "--quiet",
                        "--short",
                        "refs/remotes/origin/HEAD",
                    ]),
                    failed(),
                )
                .with_response(
                    "git",
                    &args(&["rev-parse", "--verify", "--quiet", "refs/heads/main"]),
                    ok("e00\n"),
                ),
        };
        let mut numstat_args = vec!["diff"];
        numstat_args.extend(DIFF_OPTIONS);
        numstat_args.extend(["--numstat", "-z", "abc123"]);
        let mut patch_args = vec!["diff"];
        patch_args.extend(DIFF_OPTIONS);
        patch_args.extend(["--unified=0"]);
        runner
            .with_response("git", &args(&["merge-base", "HEAD"]), ok("abc123\n"))
            .with_response("git", &args(&numstat_args), ok(numstat))
            .with_response("git", &args(&patch_args), ok(patch))
            .with_response(
                "git",
                &args(&["ls-files", "--others", "--exclude-standard", "-z"]),
                ok(untracked),
            )
    }

    /// `-z` records: each ends with a NUL.
    pub fn records(lines: &[&str]) -> String {
        lines.iter().map(|line| format!("{line}\u{0}")).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::doubles::{args, failed, ok, records, repository};
    use super::*;
    use crate::ports::{MemoryFileSystem, MockProcessRunner, ProcessOutput};

    fn available(diff: Diff) -> (String, Vec<ChangedFile>) {
        match diff {
            Diff::Available { base, files } => (base, files),
            Diff::Unavailable { reason } => panic!("no diff: {reason}"),
        }
    }

    fn reason(diff: Diff) -> String {
        match diff {
            Diff::Unavailable { reason } => reason,
            Diff::Available { .. } => panic!("a diff was read"),
        }
    }

    const PATCH: &str = "diff --git a/src/auth/login.rs b/src/auth/login.rs\n\
new file mode 100644\n\
--- /dev/null\n\
+++ b/src/auth/login.rs\n\
@@ -0,0 +1,2 @@\n\
+fn login() {\n\
+++counter;\n\
diff --git a/src/db.rs b/src/db.rs\n\
--- a/src/db.rs\n\
+++ b/src/db.rs\n\
@@ -10,2 +10,0 @@ fn old()\n\
-let password = x;\n\
-drop();\n\
@@ -40 +38,2 @@\n\
-old\n\
+sqlx::query(\"ALTER TABLE users DROP COLUMN phone\")\r\n\
+done();\n";

    #[test]
    fn diff_reads_tracked_and_untracked_changes_against_the_merge_base() {
        // Committed since the merge-base, staged and unstaged changes all
        // come out of one `git diff <merge-base>` of the working tree.
        let numstat = records(&[
            "2\t0\tsrc/auth/login.rs",
            "2\t3\tsrc/db.rs",
            "1\t0\tsrc/staged.rs",
            "1\t1\tsrc/unstaged.rs",
        ]);
        let runner = repository(None, &numstat, "notes/todo.txt\0", PATCH);
        let fs =
            MemoryFileSystem::new().with_file("/p/notes/todo.txt", "remember\r\nthe password\n");
        let (base, files) = available(read(&fs, &runner, Path::new("/p")));
        assert_eq!(base, "main");
        let paths: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(
            paths,
            [
                "notes/todo.txt",
                "src/auth/login.rs",
                "src/db.rs",
                "src/staged.rs",
                "src/unstaged.rs"
            ]
        );
        let untracked = &files[0];
        assert_eq!(untracked.bytes, Some(23));
        // Prose: the core skips its patterns, so it is not read.
        assert_eq!(untracked.lines, None);
        let merge_base = runner
            .calls()
            .into_iter()
            .find(|(_, a)| a.contains(&"--numstat".to_string()))
            .unwrap();
        assert_eq!(merge_base.1.last().map(String::as_str), Some("abc123"));
    }

    #[test]
    fn added_lines_are_numbered_from_the_hunk_header() {
        let numstat = records(&["2\t0\tsrc/auth/login.rs", "2\t3\tsrc/db.rs"]);
        let runner = repository(None, &numstat, "", PATCH);
        let (_, files) = available(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        let login = files[0].lines.as_ref().unwrap();
        assert_eq!(login[0].number, 1);
        // An added `++counter;` is a line, not a `+++` header.
        assert_eq!(login[1].text, "++counter;");
        let db = files[1].lines.as_ref().unwrap();
        let numbered: Vec<(u32, &str)> = db.iter().map(|l| (l.number, l.text.as_str())).collect();
        assert_eq!(
            numbered,
            [
                (38, "sqlx::query(\"ALTER TABLE users DROP COLUMN phone\")"),
                (39, "done();")
            ]
        );
    }

    #[test]
    fn removed_lines_are_not_added_lines() {
        let runner = repository(None, "0\t2\tsrc/db.rs\0", "", PATCH);
        let (_, files) = available(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        let lines = files[0].lines.as_ref().unwrap();
        assert!(
            lines.iter().all(|l| !l.text.contains("password")),
            "{lines:?}"
        );
    }

    #[test]
    fn crlf_is_stripped_from_added_lines() {
        let runner = repository(None, "2\t3\tsrc/db.rs\0", "src/new.rs\0", PATCH);
        let fs = MemoryFileSystem::new().with_file("/p/src/new.rs", "let a = 1;\r\nlet b = 2;\r\n");
        let (_, files) = available(read(&fs, &runner, Path::new("/p")));
        let all: Vec<&AddedLine> = files
            .iter()
            .flat_map(|f| f.lines.iter().flatten())
            .collect();
        assert!(all.iter().all(|l| !l.text.ends_with('\r')), "{all:?}");
        assert_eq!(files[1].lines.as_ref().unwrap()[1].text, "let b = 2;");
    }

    #[test]
    fn the_base_is_origin_head_when_set() {
        let runner = repository(Some("origin/trunk"), "", "", "");
        let (base, files) = available(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        assert_eq!(base, "origin/trunk");
        assert!(files.is_empty());
        assert!(runner.calls().iter().any(|(_, a)| a.ends_with(&[
            "merge-base".into(),
            "HEAD".into(),
            "origin/trunk".into()
        ])));
    }

    #[test]
    fn outside_git_there_is_no_diff() {
        let not_a_repo = MockProcessRunner::new().with_response(
            "git",
            &args(&["rev-parse", "--is-inside-work-tree"]),
            failed(),
        );
        let text = reason(read(&MemoryFileSystem::new(), &not_a_repo, Path::new("/p")));
        assert!(text.contains("not in a git repository"), "{text}");
        let no_git = MockProcessRunner::new().with_response(
            "git",
            &args(&["rev-parse"]),
            ProcessOutput {
                stdout: Vec::new(),
                stderr: b"git: command not found".to_vec(),
                exit_code: 127,
            },
        );
        assert_eq!(
            reason(read(&MemoryFileSystem::new(), &no_git, Path::new("/p"))),
            "git is not installed"
        );
        // A runner that cannot even start git.
        assert!(
            !reason(read(
                &MemoryFileSystem::new(),
                &MockProcessRunner::new(),
                Path::new("/p")
            ))
            .is_empty()
        );
    }

    #[test]
    fn without_a_commit_there_is_no_diff() {
        let runner = MockProcessRunner::new()
            .with_response(
                "git",
                &args(&["rev-parse", "--is-inside-work-tree"]),
                ok("true\n"),
            )
            .with_response(
                "git",
                &args(&["rev-parse", "--verify", "--quiet", "HEAD"]),
                failed(),
            );
        let text = reason(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        assert!(text.contains("no commit"), "{text}");
    }

    #[test]
    fn without_a_default_branch_there_is_no_diff() {
        let runner = MockProcessRunner::new()
            .with_response("git", &args(&["rev-parse", "--is-inside-work-tree"]), ok("true\n"))
            .with_response("git", &args(&["rev-parse", "--verify", "--quiet", "HEAD"]), ok("f00\n"))
            .with_response(
                "git",
                &args(&["symbolic-ref", "--quiet", "--short", "refs/remotes/origin/HEAD"]),
                failed(),
            )
            // Neither `main` nor `master`: the mock has no answer for them.
            ;
        let text = reason(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        for name in ["origin/HEAD", "main", "master"] {
            assert!(text.contains(name), "{text}");
        }
    }

    #[test]
    fn the_diff_is_relative_to_the_project_directory() {
        // git limits the diff to the working directory and makes its paths
        // relative to it with `--relative`, and `ls-files` does the same by
        // default: codev runs both in the project root.
        let runner = repository(None, "1\t0\tsrc/auth.rs\0", "", "");
        let _ = read(
            &MemoryFileSystem::new(),
            &runner,
            Path::new("/repo/packages/api"),
        );
        let calls = runner.calls();
        let diffs: Vec<&Vec<String>> = calls
            .iter()
            .map(|(_, a)| a)
            .filter(|a| a.contains(&"diff".to_string()))
            .collect();
        assert!(!diffs.is_empty());
        assert!(diffs.iter().all(|a| a.contains(&"--relative".to_string())));
        for flag in ["--no-ext-diff", "--no-textconv", "core.fsmonitor=false"] {
            assert!(
                diffs.iter().all(|a| a.contains(&flag.to_string())),
                "{flag}"
            );
        }
    }

    /// A runner that records the working directory of each call.
    struct Cwd<'a> {
        inner: &'a MockProcessRunner,
        seen: std::cell::RefCell<Vec<Option<std::path::PathBuf>>>,
    }

    impl ProcessRunner for Cwd<'_> {
        fn run(
            &self,
            program: &str,
            args: &[&str],
            cwd: Option<&Path>,
        ) -> std::io::Result<ProcessOutput> {
            self.seen.borrow_mut().push(cwd.map(Path::to_path_buf));
            self.inner.run(program, args, cwd)
        }
    }

    #[test]
    fn git_runs_in_the_project_root() {
        let inner = repository(None, "1\t0\tsrc/auth.rs\0", "", "");
        let runner = Cwd {
            inner: &inner,
            seen: Default::default(),
        };
        let _ = read(
            &MemoryFileSystem::new(),
            &runner,
            Path::new("/repo/packages/api"),
        );
        let seen = runner.seen.borrow();
        assert!(!seen.is_empty());
        assert!(
            seen.iter()
                .all(|cwd| cwd.as_deref() == Some(Path::new("/repo/packages/api")))
        );
    }

    #[test]
    fn paths_are_passed_as_literal_pathspecs_in_batches() {
        let numstat: String = (0..120)
            .map(|n| format!("1\t0\tsrc/f{n:03}*.rs\0"))
            .collect();
        let runner = repository(None, &numstat, "", "");
        let (_, files) = available(read(&MemoryFileSystem::new(), &runner, Path::new("/p")));
        assert_eq!(files.len(), 120);
        let patches: Vec<Vec<String>> = runner
            .calls()
            .into_iter()
            .map(|(_, a)| a)
            .filter(|a| a.contains(&"--unified=0".to_string()))
            .collect();
        assert_eq!(patches.len(), 3, "120 paths in batches of 50");
        for patch in &patches {
            let after = patch.iter().position(|a| a == "--").unwrap();
            assert!(
                patch[after + 1..]
                    .iter()
                    .all(|p| p.starts_with(":(literal)src/f"))
            );
            assert!(patch[after + 1..].len() <= BATCH_PATHS);
        }
    }

    #[test]
    fn an_untracked_symbolic_link_is_not_read() {
        let runner = repository(None, "", "notes\0src/new.rs\0", "");
        let fs = MemoryFileSystem::new()
            .with_symlink("/p/notes")
            .with_file("/p/src/new.rs", "fn new() {}\n");
        let (_, files) = available(read(&fs, &runner, Path::new("/p")));
        let notes = files.iter().find(|f| f.path == "notes").unwrap();
        assert!(notes.binary);
        assert_eq!(notes.lines, None);
        assert_eq!(fs.reads(), [std::path::PathBuf::from("/p/src/new.rs")]);
    }

    #[test]
    fn untracked_noise_is_neither_stated_nor_read() {
        let runner = repository(None, "", "node_modules/a/index.js\0src/new.rs\0", "");
        let fs = MemoryFileSystem::new()
            .with_file("/p/node_modules/a/index.js", "let password;\n")
            .with_file("/p/src/new.rs", "fn new() {}\n");
        let (_, files) = available(read(&fs, &runner, Path::new("/p")));
        let vendored = &files[0];
        assert_eq!((vendored.bytes, vendored.lines.as_ref()), (None, None));
        assert_eq!(fs.reads(), [std::path::PathBuf::from("/p/src/new.rs")]);
    }

    #[test]
    fn a_quoted_path_is_unquoted() {
        assert_eq!(unquote("\"b/a\\\"b\\tc\""), "b/a\"b\tc");
        assert_eq!(unquote("\"b/caf\\303\\251.rs\""), "b/café.rs");
        assert_eq!(unquote("b/plain.rs"), "b/plain.rs");
    }
}
