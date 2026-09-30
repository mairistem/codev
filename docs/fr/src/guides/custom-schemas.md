# Schémas personnalisés

Un schéma définit le workflow de planification d'un change : les artefacts
qu'il comporte, leurs dépendances mutuelles, ainsi que l'instruction et le
template que suit l'agent pour chacun. codev embarque un schéma,
`spec-driven` — proposal, specs, design, tasks —, qui convient à la plupart des
projets. Lorsque ce n'est pas le cas, vous pouvez définir le vôtre, sous forme
de simples fichiers de données : ni code, ni recompilation.

## Emplacement des schémas

Un schéma de projet est un dossier situé sous `_codev/schemas/` :

```text
_codev/schemas/
└── lite/
    ├── schema.yaml
    └── templates/
        ├── brief.md
        ├── spec.md
        └── tasks.md
```

`codev schemas` liste les schémas disponibles :

```text
Available schemas:
  lite (project)
    brief → specs → tasks
  spec-driven (built-in)
    proposal → specs → design → tasks
```

Un schéma de projet nommé `spec-driven` remplace le schéma intégré, sans rien
changer d'autre.

## Écrire un schéma

```yaml
# _codev/schemas/lite/schema.yaml
name: lite
description: Small changes — a short brief, spec deltas and a task list
artifacts:
  - id: brief
    generates: brief.md
    description: What changes, and why
    template: brief.md
    instruction: |
      Explain in a few sentences what changes and why.
  - id: specs
    generates: "specs/**/*.md"
    description: Spec deltas
    template: spec.md
    requires: [brief]
  - id: tasks
    generates: tasks.md
    template: tasks.md
    requires: [specs]
apply:
  requires: [tasks]
  tracks: tasks.md
```

| Clé | Obligatoire | Signification |
|---|---|---|
| `name` | oui | Nom du schéma, en kebab-case |
| `version` | non | Version du format de schéma, `1` par défaut |
| `description` | non | Description en une ligne |
| `artifacts` | oui | Les artefacts d'un change, au moins un |
| `artifacts[].id` | oui | Identifiant de l'artefact, en kebab-case, unique |
| `artifacts[].generates` | oui | Chemin de sortie relatif au dossier du change ; peut être un motif glob comme `specs/**/*.md` |
| `artifacts[].description` | non | Courte description, présentée à l'agent |
| `artifacts[].template` | non | Nom du fichier de template, recherché dans le dossier `templates/` du schéma |
| `artifacts[].instruction` | non | Consignes pour l'agent qui rédige cet artefact |
| `artifacts[].requires` | non | Identifiants des artefacts qui doivent exister au préalable |
| `apply.requires` | oui | Artefacts qui doivent exister avant le début de l'implémentation |
| `apply.tracks` | oui | Fichier dont les cases à cocher suivent l'avancement de l'implémentation |
| `apply.instruction` | non | Consignes pour la phase d'implémentation |

Les clés inconnues sont rejetées : une faute de frappe ne peut donc pas
produire en silence un workflow qui se comporte autrement que ce que vous avez
écrit. codev rejette également un schéma dont les dépendances font référence à
un artefact inexistant ou forment un cycle, en nommant les artefacts
concernés.

Un artefact est **terminé** lorsque son fichier existe — ou, pour un motif
glob, lorsqu'au moins un fichier correspond. Il est **prêt** lorsque tous les
artefacts dont il dépend sont terminés.

## Utiliser un schéma

Choisissez un schéma pour un change donné :

```bash
codev new change quick-fix --schema lite
```

ou faites-en le schéma par défaut du projet dans `_codev/config.yaml` :

```yaml
schema: lite
```

Le schéma est enregistré dans le `change.yaml` de chaque change : modifier le
schéma par défaut n'affecte donc pas les changes déjà en cours.

Les skills fonctionnent avec n'importe quel schéma : `/codev-propose` rédige
les artefacts prêts, dans l'ordre des dépendances, au moyen de
`codev instructions`. Les `rules:` par artefact de la configuration sont
indexées par identifiant d'artefact : des règles écrites pour `brief`
s'appliquent donc à l'artefact `brief` du schéma `lite`.

## Comportements liés à des conventions

Quelques comportements reposent sur des conventions plutôt que sur la
configuration. Gardez-les à l'esprit lorsque vous concevez un schéma :

- **Les deltas de specs sont des fichiers situés sous `specs/`.** La
  validation, la synchronisation et l'archivage lisent les deltas dans le
  dossier `specs/` du change. Un artefact qui écrit ailleurs n'est pas
  fusionné dans les specs principales.
- **`skip_specs: true`** ignore tout artefact dont le `generates` commence par
  `specs/`, quel que soit son identifiant.
- **Les décisions sont injectées dans l'artefact dont l'identifiant est
  `design`.** Nommez votre artefact de design `design` si vous voulez que
  l'agent reçoive les décisions en vigueur au moment de le rédiger.
