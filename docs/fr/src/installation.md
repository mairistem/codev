# Installation

Vous taperez deux commandes, une seule fois : l'une installe Compagnon,
l'autre prépare le dossier de votre projet. Ensuite, vous parlez seulement
à Claude. Si quelqu'un de votre équipe a l'habitude d'un
[terminal](glossary.md#terminal) (la fenêtre où l'on tape des commandes),
il peut faire cette partie pour vous.

## Partir de rien

Deux choses, avant toute commande :

- **Claude Code.** C'est l'assistant
  [Claude Code](glossary.md#claude-code) qui écrit le plan et construit
  ce que vous demandez. Procurez-le sur
  [claude.com/claude-code](https://claude.com/claude-code), qui explique
  comment l'installer. Il faut un compte Claude avec un abonnement qui
  comprend Claude Code ; cette page dit lequel.
- **Un dossier pour votre projet** sur votre ordinateur, vide pour
  commencer si vous partez d'une idée. Créez-le comme n'importe quel
  dossier, par exemple `inscription-evenement` dans vos Documents. Tout ce
  que Claude construit ira là.

## 1. Installer Compagnon

Ouvrez un terminal : sur macOS, l'application Terminal (appuyez sur
Cmd + Espace, tapez « Terminal », puis Entrée) ; sur Windows, PowerShell
(touche Windows, tapez « PowerShell », puis Entrée). Copiez la ligne qui
correspond à votre ordinateur, collez-la, puis appuyez sur Entrée.

Sur macOS ou Linux :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | sh
```

Sur Windows, dans PowerShell :

```powershell
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

Si l'installation affiche une ligne à ajouter, ou si la commande suivante
est introuvable (« not found »), fermez le terminal et ouvrez-en un
nouveau ; « Sous le capot : plateformes, versions et PATH », plus bas,
explique pourquoi, pour la personne qui vous aide.

<details>
<summary>Sous le capot : ce que fait l'installation</summary>

Sur macOS et Linux, le script télécharge la dernière version publiée pour
votre ordinateur depuis
[GitHub Releases](https://github.com/mairistem/compagnon/releases), vérifie
son empreinte SHA-256, et installe le programme dans
`~/.local/bin/compagnon`. Il refuse d'installer si l'empreinte ne
correspond pas. Si `~/.local/bin` n'est pas dans votre `PATH`, il affiche
la ligne à ajouter à `~/.zshrc` ou `~/.bashrc`. Il nécessite `curl`, `tar`,
ainsi que `sha256sum` ou `shasum`.

Sur Windows (PowerShell 5.1 ou ultérieur, PowerShell 7 compris), le script
vérifie le téléchargement de la même façon et copie `compagnon.exe` dans
`%LOCALAPPDATA%\Programs\compagnon\`. Il ne modifie **pas** votre
`PATH` : si le dossier en est absent, le script affiche la commande pour
l'y ajouter. Ouvrez ensuite une nouvelle fenêtre PowerShell. Sous Windows,
seule l'architecture x86_64 est précompilée ; sur ARM64, compilez depuis
les sources (plus bas).

Pour vérifier l'installation, `compagnon --version` affiche la version. Si
la commande reste introuvable dans un nouveau terminal, le dossier
d'installation n'est pas dans votre `PATH` : ajoutez-le comme
l'installateur l'a indiqué. `command -v compagnon` (macOS, Linux) ou
`Get-Command compagnon` (Windows) indique quel programme votre terminal
exécute.

</details>

## 2. Préparer le dossier de votre projet

Ouvrez un terminal dans le dossier de votre projet : sur macOS, dans le
Finder, faites un clic droit sur le dossier, puis Services et « Nouveau
terminal au dossier » ; sur Windows, faites un clic droit dans le dossier,
dans l'Explorateur de fichiers, puis « Ouvrir dans le Terminal ». Tapez
ensuite :

```bash
compagnon init
```

Il peut vous demander quelles [compétences](glossary.md#compétence)
donner à Claude : le choix proposé convient. Il demande ensuite quelques
mots sur votre projet : tapez toujours une phrase à son sujet, car une
réponse vide ouvre un éditeur de texte peu commode (si cela arrive,
appelez la personne qui vous aide). Il ajoute un dossier `_compagnon/`, où
vos plans seront rangés. Vous pouvez fermer le terminal : désormais, vous
parlez à Claude.

<details>
<summary>Sous le capot : ouvrir Claude Code depuis le terminal</summary>

Dans un terminal ouvert dans le dossier du projet, Claude Code démarre
avec :

```bash
claude
```

Démarrez-le après `compagnon init`, ou redémarrez-le, pour qu'il voie les
compétences. Ce que `compagnon init` écrit en plus de `_compagnon/` est
décrit dans [Déployer Compagnon pour une équipe](guides/team-rollout.md).

</details>

## Mise à jour

`compagnon self-update` installe la dernière version publiée, vérifiée
comme la première, puis rafraîchit les compétences du projet où vous êtes.
Voir [`compagnon self-update`](reference/cli.md#compagnon-self-update) et
[`compagnon update`](reference/cli.md#compagnon-update) pour les options.

## Autres façons d'installer

La plupart des gens n'en ont jamais besoin.

<details>
<summary>Sous le capot : plateformes, versions et PATH</summary>

Des copies toutes prêtes (binaires précompilés) sont publiées pour ces
plateformes :

| Plateforme | Cible |
|---|---|
| macOS, Apple Silicon | `aarch64-apple-darwin` |
| macOS, Intel | `x86_64-apple-darwin` |
| Linux, x86_64 (lié statiquement) | `x86_64-unknown-linux-musl` |
| Windows, x86_64 | `x86_64-pc-windows-msvc` |

Pour installer une version précise, définissez `COMPAGNON_VERSION` :

```bash
curl -sSL https://raw.githubusercontent.com/mairistem/compagnon/main/install.sh | COMPAGNON_VERSION=0.6.1 sh
```

```powershell
$env:COMPAGNON_VERSION = '0.6.1'
iwr -useb https://raw.githubusercontent.com/mairistem/compagnon/main/install.ps1 | iex
```

La ligne que `install.sh` vous demande d'ajouter, si `~/.local/bin` n'est
pas dans votre `PATH` :

```bash
export PATH="$HOME/.local/bin:$PATH"
```

La commande qu'affiche `install.ps1` sous Windows, qui ne modifie que le
`PATH` *utilisateur* :

```powershell
[Environment]::SetEnvironmentVariable('PATH', [Environment]::GetEnvironmentVariable('PATH', 'User') + ";$env:LOCALAPPDATA\Programs\compagnon", 'User')
```

Évitez `setx PATH "$env:PATH;..."` : cette commande tronque les valeurs de
plus de 1 024 caractères et recopie le `PATH` de la machine dans le `PATH`
utilisateur.

[git](glossary.md#git) n'est nécessaire que pour les
[sources héritées](guides/inherited-sources.md) hébergées dans un dépôt git.

</details>

<details>
<summary>Sous le capot : téléchargement manuel depuis GitHub Releases</summary>

Si vous préférez ne pas exécuter un script directement dans votre shell,
téléchargez vous-même les fichiers depuis la
[dernière version publiée](https://github.com/mairistem/compagnon/releases/latest).
Chaque version contient une archive par plateforme, nommée
`compagnon-<version>-<target>.tar.gz` (`.zip` sous Windows), ainsi qu'un fichier
`SHA256SUMS`.

**macOS et Linux.** Téléchargez votre archive et `SHA256SUMS` dans le même
dossier, puis :

```bash
# Vérifier l'empreinte — sur macOS : shasum -a 256 -c SHA256SUMS --ignore-missing
sha256sum -c SHA256SUMS --ignore-missing

tar -xzf compagnon-*.tar.gz
mkdir -p ~/.local/bin
cp compagnon-*/compagnon ~/.local/bin/
chmod 755 ~/.local/bin/compagnon
```

**Windows.** Téléchargez `compagnon-<version>-x86_64-pc-windows-msvc.zip` et
`SHA256SUMS`, puis dans PowerShell :

```powershell
# Vérifier l'empreinte
$zip = Get-Item compagnon-*-x86_64-pc-windows-msvc.zip
$expected = (Select-String -Path SHA256SUMS -Pattern $zip.Name).Line.Split(' ')[0]
$actual = (Get-FileHash $zip -Algorithm SHA256).Hash.ToLower()
if ($actual -ne $expected) { throw "SHA-256 mismatch" }

# Extraire et installer
Expand-Archive $zip -DestinationPath .
$dst = "$env:LOCALAPPDATA\Programs\compagnon"
New-Item -ItemType Directory -Path $dst -Force | Out-Null
Copy-Item compagnon-*\compagnon.exe $dst
```

Ajoutez ensuite le dossier au `PATH` de votre utilisateur comme indiqué
plus haut.

</details>

<details>
<summary>Sous le capot : compiler depuis les sources avec cargo</summary>

Avec une chaîne d'outils Rust (1.89 ou ultérieure), vous pouvez compiler
Compagnon à partir d'un clone du [dépôt](glossary.md#dépôt). C'est la voie
à suivre pour les contributeurs et pour les plateformes sans binaire
précompilé.

```bash
git clone https://github.com/mairistem/compagnon.git
cd compagnon
cargo install --path crates/compagnon-cli
```

Le binaire est installé dans `~/.cargo/bin/compagnon`.

</details>

<details id="complétion-du-shell">
<summary>Sous le capot : complétion du shell</summary>

`compagnon completions <SHELL>` écrit sur la sortie standard un script de
complétion pour `bash`, `zsh`, `fish`, `powershell` ou `elvish` :

```bash
# bash
compagnon completions bash > ~/.local/share/bash-completion/completions/compagnon

# zsh — puis lancez `compinit`
compagnon completions zsh > "${fpath[1]}/_compagnon"

# fish
compagnon completions fish > ~/.config/fish/completions/compagnon.fish
```

```powershell
# PowerShell, session en cours
compagnon completions powershell | Out-String | Invoke-Expression
```

</details>

<details>
<summary>Sous le capot : ce que remplace la mise à jour</summary>

`compagnon self-update` télécharge la version publiée la plus récente pour
votre plateforme, vérifie son SHA-256 avec le `SHA256SUMS` de cette version,
remplace le binaire sur place puis, dans un projet Compagnon, lance
`compagnon update` avec le nouveau binaire. Elle a besoin de `curl` et de
`tar`, fournis par macOS, la plupart des distributions Linux et Windows 10
ou ultérieur.

```bash
compagnon self-update                  # demande confirmation dans un terminal
compagnon self-update --check          # indique seulement la version installée et la plus récente
compagnon self-update --version 0.5.0  # une version précise, rétrogradation comprise
```

Elle ne remplace qu'un binaire installé par `install.sh`, `install.ps1` ou
à la main. Si Compagnon a été installé avec `cargo install`, compilé depuis
un clone ou installé par un gestionnaire de paquets, elle affiche la
commande qui le met à jour et ne modifie rien. Elle ne rétrograde jamais,
sauf si vous passez `--version`. Sous Windows, l'ancien binaire est
conservé sous le nom `compagnon.old.exe` à côté du nouveau, jusqu'à ce que
la commande Compagnon suivante le supprime.

`compagnon status` vous signale l'existence d'une version plus récente, au
plus une fois par jour ; voir
[Compagnon vérifie-t-il les mises à jour ?](faq.md#compagnon-vérifie-t-il-les-mises-à-jour-)
pour le désactiver.

Vous pouvez aussi relancer l'installateur. Ensuite, dans chaque projet,
lancez `compagnon update` : il régénère les compétences et rafraîchit, là
où `compagnon init` les a écrits, les sections gérées par Compagnon dans
`CLAUDE.md` et dans le modèle de
[demande de fusion](glossary.md#demande-de-fusion), et la version épinglée
dans `.github/workflows/compagnon.yml`. `compagnon doctor` vérifie le
résultat.

</details>

