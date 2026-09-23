# Design : hériter d'un dépôt git distant

## Contexte

La couche `inherits: path:` livrée précédemment couvre le partage local ;
elle sait résoudre un chemin, lire les décisions et les fusionner avec
provenance. Ce change branche la même infrastructure sur un dépôt distant
en insérant, entre la déclaration et la lecture, une étape « résolution
par lock » qui n'exige jamais le réseau.

## Objectifs / Hors objectifs

Ce design cadre :

- la nouvelle brique de transport (port `ProcessRunner`, appelant `git`) ;
- la disposition du cache et l'algorithme de fetch ;
- le format du `codev.lock` et son intégration à `config::resolve` ;
- les trois commandes `codev sources` et leur contrat JSON.

Il ne cadre **pas** le mode dégradé « sans `git` sur le PATH », le
nettoyage de cache, la parallélisation, ni les autres protocoles
d'authentification. Cf. la liste hors-périmètre du proposal.

## Décisions

### Décision : piloter le binaire `git` via un port `ProcessRunner`

C'est le geste que la décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md) documente
explicitement : « piloter le binaire `git` réutilise l'authentification
existante ». Une bibliothèque pure-Rust comme `gix` demanderait de
recopier la configuration `ssh`, les *credential helpers*, les proxies ;
`git` sait déjà tout ça.

Le port est le quatrième annoncé par la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) :
`FileSystem`, `Clock`, `Env`, et maintenant `ProcessRunner`. Son
interface reste minimale :

```rust
pub trait ProcessRunner {
    fn run(&self, program: &str, args: &[&str], cwd: Option<&Path>)
        -> io::Result<ProcessOutput>;
}
pub struct ProcessOutput { pub stdout: Vec<u8>, pub stderr: Vec<u8>, pub exit_code: i32 }
```

L'implémentation en mémoire enregistre `(program, args, cwd)` et rend un
`Vec<ProcessOutput>` prédéfini — ce qui rend les tests des commandes
`sources update` déterministes sans jamais toucher au réseau.

**Alternative écartée** : `gix`. Beau techniquement, mais coût
d'implémentation et de compatibilité disproportionné pour un besoin
couvert à 100 % par `git`. Reconsidérable si `git` disparaît des postes.

### Décision : cache adressé par SHA, dans un dossier respectant XDG

Racine du cache :

- `$XDG_CACHE_HOME/codev/` si défini ;
- sinon `~/.cache/codev/`.

Sous-arbre :

```
<cache-root>/
├── git/<hash-de-l-url>/          # dépôt bare, cloné une fois par URL
└── content/<sha>/                # contenu extrait pour un SHA donné
```

Le hash de l'URL est un `sha256` tronqué à 16 caractères — juste ce qu'il
faut pour la lisibilité et pour éviter les collisions. Le `content/<sha>/`
est un checkout `git worktree` sur le SHA verrouillé ; deux sources
distinctes qui partagent un même SHA (peu probable en pratique) se
partageraient le contenu.

**Rationale** : suit la convention XDG que respecte déjà l'écosystème
Rust (`cargo`, `rustup`). Reconstituer le cache est bon marché, donc
l'utilisateur peut vider `~/.cache/codev/` en cas de doute.

### Décision : lock TOML, une nouvelle dépendance assumée

`_codev/codev.lock` en TOML :

```toml
version = 1

[[source]]
git = "git@github.com:acme/codev-shared.git"
ref = "main"
subpath = "shared/"                  # optionnel
commit = "9f2c1ab77bd3…"
resolved_at = "2026-09-08T15:22:44Z"
```

Format aligné sur la convention `<outil>.lock` (Cargo.lock, poetry.lock,
uv.lock). Nouvelle dépendance `toml = "0.8"` — coût accepté pour la
convention et pour la clarté du format côté humain.

**Alternative écartée** : YAML pour rester cohérent avec `config.yaml`,
`schema.yaml`, `change.yaml`. Écartée parce qu'un lock est un fichier
généré, pas un fichier écrit à la main — la convention TOML domine dans
ce cas, et le YAML forcerait un `serde_norway` sur un format qu'il n'a
pas d'intérêt à voir.

### Décision : résolution paresseuse, jamais au moment de `config::resolve`

`config::resolve` ne touche ni au réseau ni au cache. Pour un `inherits:
git:`, la résolution consulte le lock :

- entrée trouvée avec un SHA en cache → chemin résolu vers
  `<cache>/content/<sha>/[subpath]` ;
- entrée trouvée mais SHA absent du cache → warning
  `git_source_needs_update` ; le contenu n'est pas exposé ;
- entrée absente du lock → warning `git_source_unlocked` ; le contenu
  n'est pas exposé.

C'est la seule façon de garantir que les commandes courantes restent
déterministes et hors ligne. `codev sources update` est le point unique
d'écriture du lock.

### Décision : `codev sources update` fait tout dans un `Plan`

Comme le reste de l'outil : la fonction pure `plan_sources_update(index,
lock, remote_resolutions)` produit un `SourcesUpdatePlan` avec les fetch
à effectuer, les entrées de lock à écrire, et un diff lisible. La
coquille CLI exécute — ordre : `git fetch` d'abord (dans le cache),
`worktree add` ensuite, écriture du lock en dernier. Si l'utilisateur
interrompt entre deux fetches, le lock reste inchangé — l'atomicité
concerne le lock, pas le cache qui peut être partiellement peuplé sans
conséquence.

### Décision : filtrer par extension `.md` et `.yaml` à la lecture

Le loader de sources héritées SHALL ne présenter que les fichiers
d'extension `.md` et `.yaml`, même si le dépôt en contient d'autres.
C'est la mise en œuvre matérielle du principe « aucun contenu exécutable
hérité » de la décision
[0005](../../decisions/0005-sources-heritees-en-lecture-seule.md). Un
`.sh`, un `.py`, un binaire, un hook : tous ignorés à l'index, jamais
exécutés.

Le filtrage se fait dans `codev-engine::sources::load` — un seul endroit
à auditer.

**Alternative écartée** : liste blanche par nom précis. Trop rigide, et
la validation via extension est déjà utilisée à d'autres endroits du
code.

## Risques et compromis

- **`git ls-remote` sur un serveur lent bloque `codev sources update`**.
  → **Compromis assumé** : la commande est explicitement l'endroit qui
  touche au réseau, l'utilisateur s'attend à un peu de latence.
- **Deux sources qui partagent la même URL avec des `ref` différents
  clonent une seule fois le dépôt bare mais font deux worktrees**.
  → **Correct** : le fetch peuple le bare, les worktrees pointent chacun
  sur son SHA. Test d'invariant qui le vérifie.
- **Cache corrompu** (SHA verrouillé mais dossier `content/<sha>/`
  incomplet parce qu'une commande précédente a été interrompue).
  → **Atténuation** : présence du dossier n'est pas prise comme preuve
  suffisante ; le loader vérifie qu'il contient au moins un `_codev/` ou
  un `.md` — sinon warning `cache_incomplete` invitant à relancer
  `sources update`.
- **URLs SSH avec `~` dans le chemin** ne sont pas concernées : on
  n'expand rien dans une URL git, on la passe telle quelle à `git`. Le
  hash de l'URL est calculé sur la chaîne exacte.

## Plan de migration

Sans objet — c'est la première version. Un projet qui avait déclaré
`inherits: git:` recevait précédemment un warning ; il verra maintenant
soit le contenu (si un `codev sources update` a été fait), soit le
nouveau warning `git_source_unlocked` qui lui dit exactement quoi lancer.
