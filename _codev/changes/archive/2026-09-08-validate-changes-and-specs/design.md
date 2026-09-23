# Design : validation des changes et des specs

## Contexte

Voir `proposal.md` pour la motivation. Le parseur émet déjà des `Finding` par
fichier — la validation les complète, coordonne la lecture disque, et produit
un rapport que le CLI rend en deux formes (humain et JSON à contrat stable).

## Objectifs / Hors objectifs

Ce design cadre :

- où vit chaque règle (cœur pur vs coquille), et pourquoi ;
- la forme du `Finding` étendu par un `path`, sans perdre le contrat déjà
  utilisé par le parseur ;
- l'orchestration côté engine et la forme JSON côté CLI.

Il ne cadre **pas** `--strict` ni `--archived` ni `--concurrency` (reportés,
cf. proposal), ni la fusion sémantique qu'exécutera `sync` (change à venir).

## Décisions

### Décision : règles pures dans `codev-core`, coordination dans `codev-engine`

Les règles supplémentaires (SHALL/MUST manquant, exigence sans scénario,
cohérences cross-sections) sont des fonctions pures sur l'AST — mêmes entrées,
mêmes sorties, sans horloge ni disque. Elles vivent donc dans
`codev-core::validate`. La coordination — trouver les fichiers, les lire, les
grouper, calculer un exit code — vit dans `codev-engine::validate` derrière
les ports `FileSystem` et `Env`.

Cela suit directement la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) : décider
n'est pas exécuter. Les règles décrivent, l'engine exécute la lecture, le CLI
formate.

**Alternatives considérées** :

- **Tout dans `codev-engine`.** Rend les règles opaques aux tests unitaires
  purs — il faut construire un `FileSystem` en mémoire pour tester
  « exigence sans scénario », alors qu'on ne fait qu'inspecter un `Requirement`.
- **Tout dans `codev-core`.** Impose à `codev-core` de savoir marcher un
  dossier, ce qui contredit le contrat « aucune I/O » de
  [0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md).

### Décision : `trait Rule` avec un registre statique

```rust
pub trait Rule {
    fn code(&self) -> &'static str;
    fn check_spec(&self, spec: &Spec) -> Vec<Finding> { Vec::new() }
    fn check_delta(&self, delta: &Delta, change: &ChangeMetadata) -> Vec<Finding> {
        Vec::new()
    }
}
```

Chaque règle est une implémentation, enregistrée dans un `pub static
RULES: &[&dyn Rule]`. Le validateur les applique toutes et concatène les
findings. Ajouter une règle E4 (warnings du lot 2) sera une struct de plus dans
le registre, sans toucher au reste.

**Alternatives considérées** :

- **Un `enum RuleId` + un `match` géant.** Plus concis au début, ingérable
  à quinze règles. Perd aussi le point d'extension annoncé dans
  [0002](../../decisions/0002-graphe-de-crates-comme-regle-de-dependance.md).
- **Des fonctions libres, sans trait.** Perd le point unique d'enregistrement,
  et donc la garantie qu'une règle nouvelle est bien intégrée à la commande.

### Décision : `Finding` conservé, `LocatedFinding` ajouté

Le `Finding` du parseur reste tel qu'il est (code stable côté API publique du
parseur, `code + line + severity + message`). L'engine l'enrichit d'un `path`
relatif au projet et d'un `item_kind`, produisant un `LocatedFinding` — c'est
ce type qui traverse le contrat JSON.

**Rationale** : casser le `Finding` existant pour y ajouter un `path` casserait
sa présence dans le contrat des tests golden du parseur, et forcerait chaque
call site du parseur à porter un chemin qui n'a de sens qu'à l'échelle d'un
projet. La séparation coûte une conversion triviale et évite ce couplage.

### Décision : un seul appel de parsing par fichier

L'engine ouvre chaque fichier une seule fois, produit un `Parsed<Spec>` ou
`Parsed<Delta>`, en tire à la fois les findings du parseur et les findings des
règles pures. C'est ce qui rend la validation en lot linéaire en nombre de
fichiers, et non quadratique.

### Décision : rapport `ValidateReport` sans imbrication profonde

```
ValidateReport
├── root: PathBuf
├── items: Vec<ItemReport>       // un par fichier ou change
│    ├── kind: "change" | "spec"
│    ├── name: String            // "add-auth" ou "user-auth"
│    ├── path: String            // relatif au projet
│    └── findings: Vec<LocatedFinding>
└── status: Vec<StatusEntry>     // erreurs d'exécution seulement
```

La CLI itère `items` pour le rendu humain, et sérialise tel quel pour le
JSON — même règle « exactement un document sur stdout » que le contrat
existant.

**Rationale** : hiérarchiser par change → fichier → finding paraît propre,
mais un `_codev/specs/x/spec.md` n'a pas de « change » parent. Un modèle plat,
avec un `kind` explicite, sert les deux cas sans branche conditionnelle dans
le consommateur.

### Décision : exit code binaire, jamais confondu avec un `sh` mal configuré

`0` sans erreur, `1` avec au moins un `Finding` de sévérité `Error` **ou**
avec une erreur d'exécution. Un code plus riche (2 pour warnings, 3 pour usage,
…) est tentant, mais le lot 2 introduira `--strict` qui promeut les warnings
en erreurs ; réserver plusieurs codes maintenant serait un choix qu'il faudrait
défaire.

## Risques et compromis

- **Divergence code/message entre le parseur et les règles.** Deux endroits
  produisent des findings — un contributeur pourrait dupliquer un code, ou en
  choisir un incompatible. → **Atténuation** : un test qui itère `RULES` et le
  parseur, vérifie que chaque `code` est unique, et refuse une intersection.
- **Détection de doublon inter-section coûteuse en cas d'énorme delta.**
  Théoriquement O(n²) sur le nombre d'exigences. → **Compromis assumé** : le
  seuil pratique est très bas (<50 exigences par delta) ; on repassera dessus
  quand un cas réel montrera le contraire.
- **Sortie humaine sensible aux terminaux étroits.** Un chemin long + un
  message long tient mal en 80 colonnes. → **Atténuation** : le message va sur
  sa propre ligne, préfixé de sa localisation ; pas de tableau ASCII fragile.

## Plan de migration

Sans objet : nouvelle capacité, aucun consommateur existant à migrer. Le champ
`status` du contrat JSON réutilise la forme déjà tenue par `codev status` et
`codev instructions`, donc rien de nouveau pour un agent qui connaissait déjà
le contrat.
