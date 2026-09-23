# Design : `codev completions <shell>`

## Contexte

Voir `proposal.md`. Change purement additif, quelques lignes de code
qui délèguent à `clap_complete`.

## Décisions

### Décision : dépendance `clap_complete`, pas de génération manuelle

`clap_complete` est le compagnon officiel de `clap` — maintenu par
les mêmes mainteneurs, versionné en parallèle. Générer les scripts à
la main serait long, ferait dériver la doc, et il faudrait tout
refaire à chaque ajout de commande.

**Alternative écartée** : un script au bootstrap qui parse `--help` et
génère la complétion. Fragile — le format `--help` peut changer, la
grammaire des shells varie, et on écrit du code qui fait déjà partie
de l'écosystème.

### Décision : cinq shells, pas un choix limité

`clap_complete::Shell` liste les cinq shells (`bash`, `zsh`, `fish`,
`powershell`, `elvish`). Les supporter tous ne coûte **rien** en
code — un enum clap, un match à un bras. En revanche, refuser
d'emblée `fish` ou `elvish` créerait une réclamation légitime dans
quelques mois.

### Décision : sous-commande dédiée, pas de flag global

Deux options :

| Option | Pro | Contre |
|---|---|---|
| **A. `codev --completions <shell>`** flag global | Court à taper | Casse le pattern « verb-noun » du reste de la CLI |
| **B. `codev completions <shell>`** sous-commande | Cohérent avec `codev list`, `codev decision …` | Deux caractères de plus |

**Choisi : B.** Cohérence l'emporte.

### Décision : pas d'installation automatique

`codev init` ne va **pas** essayer de deviner le shell de
l'utilisateur, écrire dans `~/.zshrc` ou similaire. Le pattern serait
trop magique : chaque shell a son emplacement, ses conventions,
parfois un dossier de complétions à part (`~/.zfunc/`), parfois un
`autoload`. Un ratage laisserait une entrée orpheline dans le fichier
de config de l'utilisateur — impossible à annuler proprement.

Le message d'aide de la commande (`codev completions --help`) MUST
citer la procédure recommandée par shell — courte, copier-collable,
sans ambiguïté :

- **bash** : `codev completions bash > ~/.local/share/bash-completion/completions/codev`
- **zsh** : `codev completions zsh > "${fpath[1]}/_codev"` puis
  `compinit`
- **fish** : `codev completions fish > ~/.config/fish/completions/codev.fish`
- **powershell** : `codev completions powershell | Out-String |
  Invoke-Expression` (ou redirection vers `$PROFILE`)
- **elvish** : la doc officielle Elvish s'occupe du reste

### Décision : la commande n'a pas de flag `--json`

La sortie est un script shell — pas de JSON structuré possible ni
utile. La commande sort du contrat JSON global de codev :
`fail(json, shape, err)` n'est pas appelé sur cette branche, et il
n'y a pas de `Vec<StatusEntry>` à retourner. Le contrat public reste
respecté — aucun ajout, aucun retrait dans `codev-cli::contract::v1`.

## Risques et compromis

- **`clap_complete` peut évoluer et casser** — mainteneurs communs
  avec `clap`, semver respecté. Version 4.x tant qu'on est en 4.x sur
  clap.
- **Complétion statique — pas de noms de changes actifs** — pour
  compléter `codev status --change <TAB>` avec les vrais noms, il
  faudrait générer dynamiquement à chaque appel. `clap_complete`
  supporte ça via `ValueEnum` custom + `PossibleValue` — reportable,
  pas dans ce lot.

## Plan de migration

Aucune. Feature additive, opt-in.
