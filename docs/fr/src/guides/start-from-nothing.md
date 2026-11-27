# Partir de rien

Cette page s'adresse à vous si vous ne codez pas et voulez que Claude
construise quelque chose pour vous — un formulaire d'inscription, un petit
outil, une page — en partant seulement de l'idée. Vous taperez deux
commandes, une seule fois. Ensuite, vous parlez seulement à Claude.

## Ce qu'il faut d'abord

Deux choses, avant toute commande :

- **Claude Code.** C'est l'assistant
  [Claude Code](../glossary.md#claude-code) qui écrit le plan et construit
  ce que vous demandez. Procurez-le sur
  [claude.com/claude-code](https://claude.com/claude-code), qui explique
  comment l'installer. Il faut un compte Claude avec un abonnement qui
  comprend Claude Code ; cette page dit lequel.
- **Un dossier pour votre projet** sur votre ordinateur, vide pour
  commencer. Créez-le comme n'importe quel dossier, par exemple
  `inscription-evenement` dans vos Documents. Tout ce que Claude construit
  ira là.

## Deux commandes, tapées une fois

Ces deux commandes sont les seules que vous tapez. Si quelqu'un de votre
équipe a l'habitude d'un [terminal](../glossary.md#terminal) (la fenêtre
où l'on tape des commandes), il peut faire cette partie pour vous.

**1. Installer Compagnon.** Ouvrez un terminal : sur macOS, l'application
Terminal (appuyez sur Cmd + Espace, tapez « Terminal », puis Entrée) ; sur
Windows, PowerShell (touche Windows, tapez « PowerShell », puis Entrée).
Copiez la ligne qui correspond à votre ordinateur, collez-la, puis appuyez
sur Entrée.

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
nouveau. Si cela échoue encore, [Installation](../installation.md)
explique pourquoi, pour la personne qui vous aide.

**2. Préparer votre dossier.** Ouvrez un terminal dans le dossier de votre
projet : sur macOS, dans le Finder, faites un clic droit sur le dossier, puis
Services et « Nouveau terminal au dossier » ; sur Windows, faites un clic droit dans le dossier, dans
l'Explorateur de fichiers, puis « Ouvrir dans le Terminal ». Tapez
ensuite :

```bash
compagnon init
```

Il peut vous demander quelles [compétences](../glossary.md#compétence)
donner à Claude : le choix proposé convient. Il demande ensuite quelques
mots sur votre projet : tapez toujours une phrase sur votre idée, car une
réponse vide ouvre un éditeur de texte peu commode (si cela arrive,
appelez la personne qui vous aide). Il ajoute un dossier `_compagnon/`, où
vos plans seront rangés. Vous pouvez fermer le terminal.

## Votre première demande

Ouvrez Claude Code sur le dossier de votre projet, par exemple depuis
l'application Claude pour ordinateur, en choisissant ce dossier. Dites
ensuite, avec vos mots, ce que vous voulez, en demandant d'abord un plan :

> Prépare un plan pour un formulaire d'inscription à un événement.

Claude rédige un plan et s'arrête : il ne construit encore rien.

<details>
<summary>Sous le capot : ouvrir Claude Code depuis le terminal</summary>

Dans un terminal ouvert dans le dossier du projet, Claude Code démarre
avec :

```bash
claude
```

Démarrez-le après `compagnon init`, ou redémarrez-le, pour qu'il voie les
compétences.

</details>

## Lire le plan, puis laisser Claude construire

Le plan dit pourquoi, ce qui sera vrai ensuite, comment, et en quelles
étapes. [Relire un plan](review-a-plan.md) vous dit quoi vérifier dans
chaque partie ; [Un exemple de plan commenté](example-plan.md) en montre
un complet. Pour changer quelque chose, dites-le avec vos mots :

> Dans le plan, demande aussi le nombre d'invités.

Quand le plan dit ce que vous voulez :

> Le plan me va, réalise la modification.

Claude construit ce que dit le plan, étape par étape, et vous demande
quand quelque chose n'est pas clair. Pour voir le résultat, demandez-le :

> Comment je peux essayer le formulaire sur mon ordinateur ?

## Mettre en ligne

Compagnon aide Claude à planifier et à construire ; mettre le résultat en
ligne, pour que d'autres utilisent votre formulaire, sort du cadre de
Compagnon. Demandez à un développeur, ou demandez à Claude comment faire.

## Pour aller plus loin

- [Relire un plan](review-a-plan.md) : quoi vérifier avant que Claude
  construise.
- [Un exemple de plan commenté](example-plan.md) : un plan entier, expliqué.
- [Partir d'un ticket Jira](mcp.md) : si votre idée est déjà un ticket
  Jira.
- [Glossaire](../glossary.md) : chaque mot, en mots simples.
