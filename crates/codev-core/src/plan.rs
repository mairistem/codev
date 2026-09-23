use std::path::PathBuf;

/// Ce qu'une opération veut écrire, calculé **avant** la moindre écriture.
///
/// C'est le pivot de l'architecture : le cœur ne touche jamais au disque, il
/// produit un plan ; la coquille l'exécute. On y gagne `--dry-run`, la
/// prévisualisation `--json`, l'atomicité — le plan est validé entièrement
/// avant la première écriture — et des tests sans répertoire temporaire.
/// Voir `_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Plan {
    pub dirs: Vec<PathBuf>,
    pub writes: Vec<FileWrite>,
    /// Suppressions de fichiers — appliquées après les writes et avant
    /// les moves. Un `Plan` sans deletion se comporte comme avant.
    ///
    /// Séparer des writes n'est pas cosmétique : un fichier légitimement
    /// vide (un `write` avec `contents: ""`) ne doit **jamais** être
    /// confondu avec une suppression. Le geste destructeur mérite son
    /// champ propre, et l'exécuteur peut refuser une deletion sans
    /// confirmation dans un futur mode `--dry-run` sans avoir à
    /// reparser des `contents=""`.
    pub deletions: Vec<PathBuf>,
    /// Déplacements de dossiers (ou fichiers) — appliqués après les écritures.
    ///
    /// L'ordre importe : un `Move` peut avoir besoin qu'un dossier parent
    /// existe côté destination (donc dirs d'abord), et un `Move` peut aussi
    /// venir « emballer » des fichiers qui viennent d'être écrits (donc
    /// writes d'abord). C'est la coquille qui joue cet ordre ; ici on ne
    /// fait que le collecter.
    pub moves: Vec<Move>,
}

/// Un déplacement `from → to`.
///
/// Toute modification du disque doit passer par un `Plan` — voir la décision
/// [0001](_codev/decisions/0001-coeur-fonctionnel-coquille-imperative.md).
/// Un déplacement qui vivrait à côté du plan rouvrirait la porte à un état
/// incohérent : main spec écrite, dossier non déplacé. On ne veut pas ça.
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
    /// N'écrit que si le fichier est absent.
    ///
    /// Le mode du scaffolding : ni `init` ni `new change` ne doivent jamais
    /// écraser ce que l'utilisateur a écrit. Relancer `init` sur un projet déjà
    /// initialisé doit donc être sans effet et sans danger.
    CreateOnly,
    /// Écrase le fichier existant.
    ///
    /// Réservé aux fichiers dont codev est propriétaire — les skills générées.
    /// Tout ce qui porte ce mode est régénérable, donc perdable.
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

    /// Planifie la suppression d'un fichier. Un même chemin n'entre qu'une
    /// fois — deux deletions identiques masqueraient un bug de composition.
    pub fn delete(&mut self, path: impl Into<PathBuf>) -> &mut Self {
        let path = path.into();
        if !self.deletions.contains(&path) {
            self.deletions.push(path);
        }
        self
    }

    /// Planifie un déplacement. Un même `(from, to)` n'entre qu'une fois —
    /// deux moves identiques n'ont aucun sens et masqueraient un bug de
    /// composition.
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

    /// Absorbe un autre plan. Sert à composer : le CLI réunit le plan de
    /// scaffolding de l'engine et celui des skills de `codev-agents`, puis
    /// exécute l'ensemble en une passe.
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
    fn deduplique_les_dossiers() {
        let mut plan = Plan::new();
        plan.dir("/a").dir("/b").dir("/a");
        assert_eq!(plan.dirs, [PathBuf::from("/a"), PathBuf::from("/b")]);
    }

    #[test]
    fn fusionne_deux_plans() {
        let mut premier = Plan::new();
        premier.dir("/a").write("/a/x", "x", WriteMode::CreateOnly);

        let mut second = Plan::new();
        second.dir("/a").write("/a/y", "y", WriteMode::Overwrite);

        premier.merge(second);
        assert_eq!(premier.dirs, [PathBuf::from("/a")]);
        assert_eq!(premier.writes.len(), 2);
    }

    #[test]
    fn move_est_planifiable_et_deduplicable() {
        let mut plan = Plan::new();
        plan.move_dir("/a/x", "/a/y")
            .move_dir("/b/x", "/b/y")
            .move_dir("/a/x", "/a/y"); // doublon

        assert_eq!(
            plan.moves,
            [
                Move { from: "/a/x".into(), to: "/a/y".into() },
                Move { from: "/b/x".into(), to: "/b/y".into() },
            ]
        );
    }

    #[test]
    fn merge_absorbe_les_moves() {
        let mut premier = Plan::new();
        premier.move_dir("/a", "/z/a");

        let mut second = Plan::new();
        second.move_dir("/b", "/z/b");

        premier.merge(second);
        assert_eq!(premier.moves.len(), 2);
    }

    #[test]
    fn is_empty_couvre_aussi_les_moves() {
        let mut plan = Plan::new();
        plan.move_dir("/from", "/to");
        assert!(!plan.is_empty());
    }

    #[test]
    fn delete_est_planifiable_et_dedupliquable() {
        let mut plan = Plan::new();
        plan.delete("/a/x").delete("/b/y").delete("/a/x"); // doublon
        assert_eq!(
            plan.deletions,
            [PathBuf::from("/a/x"), PathBuf::from("/b/y")]
        );
    }

    #[test]
    fn merge_absorbe_les_deletions() {
        let mut premier = Plan::new();
        premier.delete("/a");

        let mut second = Plan::new();
        second.delete("/b").delete("/a"); // /a est déjà là

        premier.merge(second);
        assert_eq!(
            premier.deletions,
            [PathBuf::from("/a"), PathBuf::from("/b")]
        );
    }

    #[test]
    fn is_empty_couvre_aussi_les_deletions() {
        let mut plan = Plan::new();
        plan.delete("/a");
        assert!(!plan.is_empty());
    }
}
