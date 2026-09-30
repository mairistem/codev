use std::path::PathBuf;

/// What an operation wants to write, computed **before** any write happens.
///
/// This is the pivot of the architecture: the core never touches the disk, it
/// produces a plan; the imperative shell executes it. This buys us `--dry-run`, the
/// `--json` preview, atomicity — the plan is fully validated
/// before the first write — and tests without a temporary directory.
/// See `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    pub dirs: Vec<PathBuf>,
    pub writes: Vec<FileWrite>,
    /// File deletions — applied after the writes and before
    /// the moves. A `Plan` without deletions behaves as before.
    ///
    /// Keeping them apart from writes is not cosmetic: a legitimately
    /// empty file (a `write` with `contents: ""`) must **never** be
    /// mistaken for a deletion. The destructive action deserves its
    /// own field, and the executor can refuse an unconfirmed deletion
    /// in a future `--dry-run` mode without having to
    /// re-parse `contents=""`.
    pub deletions: Vec<PathBuf>,
    /// Directory (or file) moves — applied after the writes.
    ///
    /// Order matters: a `Move` may need a parent directory to
    /// exist at the destination (hence dirs first), and a `Move` may also
    /// "wrap up" files that were just written (hence
    /// writes first). The imperative shell enforces that order; here we
    /// merely collect it.
    pub moves: Vec<Move>,
}

/// A `from → to` move.
///
/// Every disk modification must go through a `Plan` — see decision
/// [0001](_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md).
/// A move living outside the plan would reopen the door to an inconsistent
/// state: main spec written, directory not moved. We do not want that.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub from: PathBuf,
    pub to: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileWrite {
    pub path: PathBuf,
    pub contents: String,
    pub mode: WriteMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    /// Writes only if the file is absent.
    ///
    /// The scaffolding mode: neither `init` nor `new change` may ever
    /// overwrite what the user wrote. Re-running `init` on an already
    /// initialized project must therefore be a harmless no-op.
    CreateOnly,
    /// Overwrites the existing file.
    ///
    /// Reserved for files codev owns — the generated skills.
    /// Anything written in this mode is regenerable, hence expendable.
    Overwrite,
}

impl Plan {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dir(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path = path.into();
        if !self.dirs.contains(&path) {
            self.dirs.push(path);
        }
        self
    }

    pub fn write(
        &mut self,
        path: impl Into<PathBuf>,
        contents: impl Into<String>,
        mode: WriteMode,
    ) -> &mut Self {
        self.writes.push(FileWrite {
            path: path.into(),
            contents: contents.into(),
            mode,
        });
        self
    }

    /// Schedules a file deletion. A given path is added only
    /// once — two identical deletions would hide a composition bug.
    pub fn delete(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path = path.into();
        if !self.deletions.contains(&path) {
            self.deletions.push(path);
        }
        self
    }

    /// Schedules a move. A given `(from, to)` is added only once —
    /// two identical moves make no sense and would hide a composition
    /// bug.
    pub fn move_dir(&mut self, from: impl Into<PathBuf>, to: impl Into<PathBuf>) -> &mut Self {
        let mv = Move {
            from: from.into(),
            to: to.into(),
        };
        if !self.moves.contains(&mv) {
            self.moves.push(mv);
        }
        self
    }

    /// Absorbs another plan. Used for composition: the CLI combines the engine's
    /// scaffolding plan with the `codev-agents` skills plan, then
    /// executes the whole thing in a single pass.
    pub fn merge(&mut self, other: Plan) {
        for dir in other.dirs {
            self.dir(dir);
        }
        self.writes.extend(other.writes);
        for path in other.deletions {
            self.delete(path);
        }
        for mv in other.moves {
            self.move_dir(mv.from, mv.to);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.dirs.is_empty()
            && self.writes.is_empty()
            && self.deletions.is_empty()
            && self.moves.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduplicates_directories() {
        let mut plan = Plan::new();
        plan.dir("/a").dir("/b").dir("/a");
        assert_eq!(plan.dirs, [PathBuf::from("/a"), PathBuf::from("/b")]);
    }

    #[test]
    fn merges_two_plans() {
        let mut first = Plan::new();
        first.dir("/a").write("/a/x", "x", WriteMode::CreateOnly);

        let mut second = Plan::new();
        second.dir("/a").write("/a/y", "y", WriteMode::Overwrite);

        first.merge(second);
        assert_eq!(first.dirs, [PathBuf::from("/a")]);
        assert_eq!(first.writes.len(), 2);
    }

    #[test]
    fn move_is_plannable_and_deduplicated() {
        let mut plan = Plan::new();
        plan.move_dir("/a/x", "/a/y")
            .move_dir("/b/x", "/b/y")
            .move_dir("/a/x", "/a/y"); // duplicate

        assert_eq!(
            plan.moves,
            [
                Move {
                    from: "/a/x".into(),
                    to: "/a/y".into()
                },
                Move {
                    from: "/b/x".into(),
                    to: "/b/y".into()
                },
            ]
        );
    }

    #[test]
    fn merge_absorbs_moves() {
        let mut first = Plan::new();
        first.move_dir("/a", "/z/a");

        let mut second = Plan::new();
        second.move_dir("/b", "/z/b");

        first.merge(second);
        assert_eq!(first.moves.len(), 2);
    }

    #[test]
    fn is_empty_also_covers_moves() {
        let mut plan = Plan::new();
        plan.move_dir("/from", "/to");
        assert!(!plan.is_empty());
    }

    #[test]
    fn delete_is_plannable_and_deduplicated() {
        let mut plan = Plan::new();
        plan.delete("/a/x").delete("/b/y").delete("/a/x"); // duplicate
        assert_eq!(
            plan.deletions,
            [PathBuf::from("/a/x"), PathBuf::from("/b/y")]
        );
    }

    #[test]
    fn merge_absorbs_deletions() {
        let mut first = Plan::new();
        first.delete("/a");

        let mut second = Plan::new();
        second.delete("/b").delete("/a"); // /a is already there

        first.merge(second);
        assert_eq!(first.deletions, [PathBuf::from("/a"), PathBuf::from("/b")]);
    }

    #[test]
    fn is_empty_also_covers_deletions() {
        let mut plan = Plan::new();
        plan.delete("/a");
        assert!(!plan.is_empty());
    }
}
