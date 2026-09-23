# codev

Développement piloté par les specs, pour Claude Code.

`codev` ajoute à un dépôt une fine couche de specs pour que toi et ton agent
soyez d'accord sur **ce qui doit être construit** avant qu'une ligne de code ne
soit écrite — et pour que les décisions d'architecture déjà tranchées cessent
d'être re-débattues à chaque projet.

L'outil n'appelle jamais de LLM. Il gère des fichiers markdown, un graphe de
dépendances entre artefacts, la validation et la fusion des specs. C'est ton
agent qui rédige ; `codev` lui dit quoi écrire, où, et avec quelles
contraintes.

## Les deux moitiés

| Où | Quoi | Exemple |
|---|---|---|
| **Ton terminal** | le binaire `codev` — le moteur | `codev init`, `codev status`, `codev archive` |
| **Le chat de Claude Code** | des skills générées — le volant | `/codev-propose`, `/codev-apply` |

`codev init` installe les skills dans `.claude/skills/`. Ensuite tu vis dans le
chat, et les skills pilotent le CLI.

## Démarrage

```bash
# Installation (sans Rust) — macOS ou Linux
curl -sSL https://raw.githubusercontent.com/mairistem/codev/main/install.sh | sh

# ou, en tant que contributeur :
cargo install --path crates/codev-cli

cd mon-projet
codev init
```

Pour le détail des voies d'installation (téléchargement manuel,
version épinglée, PATH…), voir la section Installation de la
documentation : `codev docs`.

Puis, dans Claude Code :

```text
/codev-explore        # défricher une idée, sans rien engager
/codev-propose        # créer un change et rédiger ses artefacts de planification
```

## Le modèle

```
_codev/
├── specs/          # contrats de comportement — CE QUE fait le système
├── decisions/      # décisions d'architecture — POURQUOI c'est fait comme ça
├── changes/        # le travail en cours, qui modifie les deux
│   └── <nom>/      #   proposal.md, design.md, tasks.md, specs/**  (deltas)
├── schemas/        # workflows maison (optionnel)
└── config.yaml
```

Deux idées portent tout le reste :

- **Les changes sont des deltas.** Un change ne réécrit pas une spec entière, il
  déclare `ADDED` / `MODIFIED` / `REMOVED` / `RENAMED`. C'est ce qui rend
  l'outil utilisable sur du code existant, et non seulement en greenfield.
- **Les décisions sont immuables.** Une décision acceptée ne se modifie pas : on
  en écrit une nouvelle qui la remplace. La supersession *est* leur mécanisme de
  delta.

## Partage entre projets

Un projet peut hériter en lecture seule des conventions, specs et décisions
d'un autre dépôt — depuis le poste, ou depuis un dépôt git distant sans jamais
en garder de copie de travail :

```yaml
# _codev/config.yaml
inherits:
  - path: ~/codev/shared/back
  - git: git@github.com:mon-orga/codev-decisions.git
    ref: main
```

Le SHA résolu est épinglé dans `_codev/codev.lock`. Rien n'est lu « flottant »,
et rien d'exécutable n'est jamais hérité.

## État du projet

Chantier en cours. Voir [docs/inventory.md](docs/inventory.md) pour le
périmètre et son séquencement, et [docs/architecture.md](docs/architecture.md)
pour les partis pris.

## Origines

`codev` reprend le modèle de [OpenSpec](https://github.com/Fission-AI/OpenSpec)
(MIT), en Rust, ciblant Claude Code seul, et y ajoute les décisions
d'architecture comme objet de première classe.
