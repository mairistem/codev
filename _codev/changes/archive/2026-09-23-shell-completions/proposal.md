# Proposal : générer les completions shell via `codev completions`

## Pourquoi

L'utilisateur qui tape `codev de<Tab>` dans son shell aujourd'hui n'a
rien : pas de complétion sur les sous-commandes (`decision`,
`deviate`, `deviated`…), sur les noms de changes actifs, ni sur les
flags. C'est un petit friction constant sur un outil qu'on utilise
plusieurs dizaines de fois par jour.

`clap` (utilisé par `codev-cli`) expose un compagnon officiel,
`clap_complete`, qui génère des scripts de completion pour bash, zsh,
fish, powershell et elvish à partir de la déclaration `#[derive(Parser)]`
existante. Rien à écrire à la main — le résultat suit les
sous-commandes au fur et à mesure qu'elles évoluent, sans dérive
possible.

## Ce qui change

- **Nouvelle sous-commande `codev completions <shell>`** — imprime
  sur stdout le script de complétion pour le shell donné.
  `<shell>` accepte les cinq valeurs standard de `clap_complete`
  (`bash`, `zsh`, `fish`, `powershell`, `elvish`).
- **Documentation d'installation** — le `--help` de la sous-commande
  cite la procédure recommandée par shell (redirection vers le bon
  fichier, `source`, etc.).
- **Nouvelle dépendance `clap_complete = "4"`** dans
  `crates/codev-cli/Cargo.toml` — aligné sur la version majeure de
  `clap` déjà utilisée.
- **Pas de scaffolding automatique** — la commande ne touche à aucun
  fichier système ; c'est à l'utilisateur de rediriger la sortie où
  il veut. Cohérent avec la philosophie codev (« la skill/commande
  guide, l'utilisateur agit »).

## Capacités

### Nouvelles capacités

- `shell-completions` — décrit le contrat de la sous-commande
  `codev completions` : shells supportés, format de sortie, absence
  d'effet de bord.

### Capacités modifiées

Aucune.

### Capacités retirées

Aucune.

## Impact

- **Code** :
  - Nouvelle variante `Command::Completions { shell: Shell }` dans
    `crates/codev-cli/src/main.rs`, où `Shell` est
    `clap_complete::Shell`.
  - Branche du `match` qui appelle
    `clap_complete::generate(shell, &mut Cli::command(), "codev",
    &mut io::stdout())`.
  - Un test qui vérifie que la génération pour chacun des cinq
    shells produit une sortie non vide et contient le nom `codev`.
- **Contrat JSON** : rien. La sortie est du script shell, pas du
  JSON. La commande n'a pas de flag `--json` (aucun sens).
- **Fichier écrit** : rien — sortie sur stdout uniquement.
- **Migration** : aucune. Feature purement additive.
- **Hors périmètre** :
  - **Complétion dynamique sur les noms de changes actifs** — nécessite
    un runtime lookup, `clap_complete` génère du statique. Reportable.
  - **Auto-installation dans `codev init`** — trop magique, dépend
    de la config shell de l'utilisateur.
  - **Support de shells exotiques** (nushell, xonsh) — hors des cinq
    de `clap_complete`. Reportable si besoin réel.
