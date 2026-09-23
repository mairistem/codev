use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

/// L'accès au système de fichiers, derrière un port.
///
/// L'intérêt n'est pas de pouvoir « changer de système de fichiers » — on n'en
/// changera pas. C'est que `init`, `new change` et `archive` deviennent
/// testables en mémoire, sans répertoire temporaire ni tests sérialisés.
///
/// Le trait reste compatible `dyn` : `codev-agents` en tient une référence
/// dynamique pour rester lui-même utilisable via `dyn AgentTarget`.
pub trait FileSystem {
    fn exists(&self, path: &Path) -> bool;

    fn read_to_string(&self, path: &Path) -> io::Result<String>;

    /// Écrit le fichier, en créant ses dossiers parents si besoin.
    fn write(&self, path: &Path, contents: &str) -> io::Result<()>;

    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Les fichiers sous `dir`, récursivement, en chemins **relatifs à `dir`**
    /// et toujours séparés par `/`.
    ///
    /// Ce format n'est pas un détail : c'est ce que consomme
    /// `codev_core::outputs::pattern_matches_any`, et il garde les séparateurs
    /// propres à la plateforme hors du cœur pur.
    ///
    /// Un `dir` absent rend une liste vide, pas une erreur : « ce change n'a
    /// encore aucun fichier » est un état normal, pas une panne.
    fn walk_files(&self, dir: &Path) -> io::Result<Vec<String>>;

    /// Les noms des entrées directes de `dir`. Vide si `dir` est absent.
    fn list_dir(&self, dir: &Path) -> io::Result<Vec<String>>;

    /// Déplace `from` vers `to`. Crée les dossiers parents de `to` si besoin.
    ///
    /// L'implémentation réelle tente d'abord `std::fs::rename` (atomique
    /// intra-volume) et retombe sur copy+remove en cas d'erreur cross-device.
    /// Toutes les implémentations doivent traiter aussi bien un fichier
    /// qu'un dossier — c'est ce que fait `codev archive` avec le dossier du
    /// change en entier.
    fn rename(&self, from: &Path, to: &Path) -> io::Result<()>;

    /// Supprime un seul fichier. Un fichier absent renvoie
    /// [`io::ErrorKind::NotFound`] — l'exécuteur (`apply::execute`) décide
    /// s'il ignore ou remonte. Le rôle du port est simplement d'exposer
    /// le geste destructeur ; toute politique se joue en amont.
    fn remove_file(&self, path: &Path) -> io::Result<()>;
}

/// L'horloge, derrière un port.
///
/// Volontairement étroite : ces dates ne servent qu'à nommer un dossier
/// d'archive et à horodater une décision. Une [`FixedClock`] rend les tests
/// déterministes en une ligne.
pub trait Clock {
    /// La date du jour, au format `AAAA-MM-JJ`.
    fn today(&self) -> String;
}

/// Un processus externe lancé, avec sa sortie capturée.
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

/// Le pilotage d'un binaire externe, derrière un port.
///
/// Utilisé pour `git` par `codev sources update` — la seule opération de
/// codev qui contacte un service distant. Le port permet à la fois de
/// piloter le vrai `git` et d'écrire des tests déterministes qui ne
/// touchent jamais au réseau.
pub trait ProcessRunner {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        cwd: Option<&Path>,
    ) -> io::Result<ProcessOutput>;
}

pub struct RealProcessRunner;

impl ProcessRunner for RealProcessRunner {
    fn run(
        &self,
        program: &str,
        args: &[&str],
        cwd: Option<&Path>,
    ) -> io::Result<ProcessOutput> {
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

/// Impl de test qui rend une réponse prédéfinie par tuple `(programme,
/// premier argument)`. Suffit pour les scénarios d'`update` — pilotage
/// séquentiel de `git ls-remote`, `git fetch`, `git worktree add`.
#[derive(Debug, Default)]
pub struct MockProcessRunner {
    /// Réponses associées à un préfixe `(program, args_prefix)`.
    responses: RefCell<Vec<(String, Vec<String>, ProcessOutput)>>,
    /// Log des appels effectifs — pour affirmer côté test.
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
    fn run(
        &self,
        program: &str,
        args: &[&str],
        _cwd: Option<&Path>,
    ) -> io::Result<ProcessOutput> {
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
                format!("aucune réponse mock pour {program} {args:?}"),
            )),
        }
    }
}

/// L'environnement d'exécution, derrière un port.
pub trait Env {
    fn current_dir(&self) -> io::Result<PathBuf>;
    fn var(&self, key: &str) -> Option<String>;

    /// Le dossier personnel, pour développer un `~` dans un chemin déclaré.
    ///
    /// Lu depuis l'environnement plutôt que via `std::env::home_dir`, dont
    /// l'histoire de dépréciation ne mérite pas d'être suivie ici.
    fn home_dir(&self) -> Option<PathBuf> {
        self.var("HOME")
            .or_else(|| self.var("USERPROFILE"))
            .map(PathBuf::from)
    }
}

// ─────────────────────────── implémentations réelles ───────────────────────────

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
        // `create_dir_all` sur le parent de la destination, sinon un
        // déplacement vers `archive/AAAA-MM-JJ-name/` échouerait dès que
        // `archive/` n'existe pas encore.
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

/// `rename(2)` échoue avec `EXDEV` quand `from` et `to` sont sur des systèmes
/// de fichiers différents. On garde le fallback discret plutôt que fatal :
/// improbable en pratique (dossier de planning et code partagent le même
/// volume), mais le fallback évite un message d'erreur imbuvable dans les
/// rares cas où ça arrive.
fn is_cross_device(err: &io::Error) -> bool {
    err.raw_os_error() == Some(18) // EXDEV sur Linux/macOS
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
        // La date locale est celle que l'utilisateur lit sur son horloge, donc
        // celle qu'il attend dans un nom de dossier d'archive. Elle échoue sur
        // certaines configurations système : on retombe alors sur UTC, ce qui
        // vaut mieux que de refuser d'archiver.
        let now = time::OffsetDateTime::now_local()
            .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
        format!(
            "{:04}-{:02}-{:02}",
            now.year(),
            u8::from(now.month()),
            now.day()
        )
    }
}

/// Horloge figée, pour les tests et les exécutions reproductibles.
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

// ─────────────────────────── doubles en mémoire ───────────────────────────

/// Système de fichiers en mémoire.
///
/// Public et non conditionné aux tests : les crates au-dessus l'utilisent pour
/// tester leurs propres plans sans avoir à redéclarer un double.
#[derive(Default)]
pub struct MemoryFileSystem {
    files: RefCell<BTreeMap<PathBuf, String>>,
    dirs: RefCell<Vec<PathBuf>>,
}

impl MemoryFileSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Dépose un fichier, façon « voilà l'état du disque avant l'opération ».
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
        // Un dossier existe dès qu'un fichier vit dessous, comme sur un vrai
        // disque.
        self.files
            .borrow()
            .keys()
            .any(|f| f.ancestors().any(|a| a == path))
    }

    fn read_to_string(&self, path: &Path) -> io::Result<String> {
        self.files.borrow().get(path).cloned().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} est absent", path.display()),
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
        // On collecte tout ce qui vit sous `from` — fichier isolé ou arbre —
        // avant de muter, pour éviter de lire et écrire dans une même passe
        // sur la même `RefCell`.
        let a_deplacer: Vec<(PathBuf, String)> = self
            .files
            .borrow()
            .iter()
            .filter_map(|(path, contents)| {
                if path == from {
                    // Fichier isolé qui correspond exactement à `from` :
                    // sa nouvelle place est `to`, pas `to.push("")`.
                    Some((to.to_path_buf(), contents.clone()))
                } else if let Ok(relative) = path.strip_prefix(from) {
                    let mut nouveau = to.to_path_buf();
                    nouveau.push(relative);
                    Some((nouveau, contents.clone()))
                } else {
                    None
                }
            })
            .collect();

        if a_deplacer.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} n'existe pas", from.display()),
            ));
        }

        let mut fichiers = self.files.borrow_mut();
        // Purge : tout ce qui est `from` ou dessous disparaît.
        let a_supprimer: Vec<PathBuf> = fichiers
            .keys()
            .filter(|p| p.as_path() == from || p.strip_prefix(from).is_ok())
            .cloned()
            .collect();
        for p in a_supprimer {
            fichiers.remove(&p);
        }
        // Réinsertion sous la nouvelle racine.
        for (nouveau_chemin, contents) in a_deplacer {
            fichiers.insert(nouveau_chemin, contents);
        }
        Ok(())
    }

    fn remove_file(&self, path: &Path) -> io::Result<()> {
        let mut fichiers = self.files.borrow_mut();
        if fichiers.remove(path).is_some() {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("{} est absent", path.display()),
            ))
        }
    }
}

/// Environnement figé, pour les tests.
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
    fn la_memoire_voit_les_dossiers_impliques_par_ses_fichiers() {
        let fs = MemoryFileSystem::new().with_file("/p/_codev/config.yaml", "schema: spec-driven");
        assert!(fs.exists(Path::new("/p/_codev/config.yaml")));
        assert!(fs.exists(Path::new("/p/_codev")));
        assert!(fs.exists(Path::new("/p")));
        assert!(!fs.exists(Path::new("/p/_codev/specs")));
    }

    #[test]
    fn walk_files_rend_des_chemins_relatifs_en_slash() {
        let fs = MemoryFileSystem::new()
            .with_file("/p/changes/add-auth/proposal.md", "x")
            .with_file("/p/changes/add-auth/specs/user-auth/spec.md", "y");
        let files = fs.walk_files(Path::new("/p/changes/add-auth")).unwrap();
        assert_eq!(files, ["proposal.md", "specs/user-auth/spec.md"]);
    }

    #[test]
    fn walk_files_sur_un_dossier_absent_ne_donne_rien() {
        let fs = MemoryFileSystem::new();
        assert!(fs.walk_files(Path::new("/nulle/part")).unwrap().is_empty());
    }

    #[test]
    fn lhorloge_figee_est_deterministe() {
        assert_eq!(FixedClock("2026-09-08".into()).today(), "2026-09-08");
    }

    #[test]
    fn memory_rename_deplace_un_arbre_entier() {
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
    fn memory_rename_dun_fichier_seul() {
        let fs = MemoryFileSystem::new().with_file("/a/x.md", "hello");
        fs.rename(Path::new("/a/x.md"), Path::new("/b/y.md")).unwrap();
        assert!(!fs.exists(Path::new("/a/x.md")));
        assert_eq!(fs.read("/b/y.md").as_deref(), Some("hello"));
    }

    #[test]
    fn memory_rename_source_absente_est_une_erreur() {
        let fs = MemoryFileSystem::new();
        let err = fs
            .rename(Path::new("/pas/la"), Path::new("/ailleurs"))
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn mock_process_runner_repond_aux_commandes_attendues() {
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
    fn real_process_runner_signale_un_binaire_absent() {
        let runner = RealProcessRunner;
        let out = runner
            .run("codev-binaire-inexistant-abc", &["--help"], None)
            .unwrap();
        assert_eq!(out.exit_code, 127);
        assert!(out.stderr_str().contains("not found"), "{}", out.stderr_str());
    }

    #[test]
    fn le_dossier_personnel_vient_de_lenvironnement() {
        let mut env = FixedEnv::at("/p");
        env.vars.insert("HOME".into(), "/Users/moi".into());
        assert_eq!(env.home_dir(), Some(PathBuf::from("/Users/moi")));
        assert_eq!(FixedEnv::at("/p").home_dir(), None);
    }
}
