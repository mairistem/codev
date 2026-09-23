# Design : livrer `/codev-apply`

## Contexte

Voir `proposal.md` pour la motivation. Deux workflows existent déjà —
`propose` et `explore` — chacun est un couple {entrée dans le `CATALOG`,
fichier markdown sous `assets/workflows/`}. Ce design reprend la même
mécanique sans rien y ajouter côté code ; l'essentiel est le contenu du
markdown.

## Objectifs / Hors objectifs

Ce design cadre :

- l'ajout d'une entrée dans le `CATALOG` — trivial, mais listé ici pour
  clore explicitement la boucle avec les tests d'invariant existants ;
- la structure et les invariants du corps de la skill.

Il ne cadre **pas** les skills `sync`/`archive`/`update` (proposal hors
périmètre), ni l'ajout de `apply` à `DEFAULT_WORKFLOWS` — décision reportée
au retour d'expérience.

## Décisions

### Décision : `allowed-tools` inclut `Bash` en plus de `Bash(codev:*)`

`apply` doit pouvoir lancer les commandes de vérification écrites dans
`tasks.md` (`cargo test …`, `cargo build …`, etc.). Restreindre à
`Bash(codev:*)` comme `propose` et `explore` empêcherait la vérification
qu'exige justement chaque tâche.

Décidé de lister explicitement : `Bash(codev:*), Read, Write, Edit, Glob,
Grep, Bash`. L'ordre a une signification pour l'utilisateur qui lit le
frontmatter — le préfixe `Bash(codev:*)` en premier documente l'usage
principal ; `Bash` seul en dernier documente l'ouverture nécessaire.

**Alternative écartée** : restreindre à `Bash(cargo:*), Bash(codev:*)`. Trop
étroit — un test peut exiger `git`, `npm`, `python`. Le contexte projet
décide ; codev n'a pas à préjuger de la boîte à outils.

### Décision : la skill cocher les cases via `Edit`, pas via une commande CLI

Il n'existe pas de commande `codev task check <n>` — et il n'y en aura pas
tant que le format `tasks.md` reste stable, cf. la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) : le
cœur ne touche pas au disque, et le format markdown est la source de vérité
lue par `codev status`. La skill utilise donc `Edit` pour transformer
`- [ ] X.Y` en `- [x] X.Y` sur la ligne de la tâche.

**Rationale** : introduire une commande dédiée dupliquerait la logique du
parser de tasks (B3, non encore livré) sans gain — l'agent voit déjà les
lignes du fichier via `Read`, sait faire un `Edit` ciblé, et le round-trip
est trivial. Le jour où B3 arrivera, la skill pourra migrer sans casser son
contrat public.

### Décision : la skill s'arrête au dernier `[x]` et invite à archive

Un `apply` qui déclencherait automatiquement `archive` violerait la règle
« une skill fait une chose » — et surtout, l'utilisateur veut relire le
résultat avant d'archiver. La skill dit explicitement quel change est prêt
et laisse le pas suivant à l'utilisateur.

## Risques et compromis

- **Une tâche mal formulée bloque tout**. Une case qui décrit deux choses
  différentes force la skill à s'arrêter et demander. → **Compromis
  assumé** : c'est le comportement voulu. L'alternative — deviner — mène à
  la dette silencieuse. Le message d'arrêt cite la tâche et propose de la
  scinder en deux `X.Y.a` / `X.Y.b`, sans imposer.
- **Le format `- [ ]` est fragile aux espacements**. Un `-[ ]` sans espace,
  un `- [X]` majuscule, une case avec `- [-]` — chacun casse la
  reconnaissance. → **Atténuation** : la skill décrit exactement le format
  attendu (`- [ ]` avec espaces) et invite à corriger si autre chose est
  trouvé. Plus tard, B3 pourra tolérer les variantes.
- **Long tasks.md → session interminable**. La skill peut passer une heure
  sur une trentaine de tâches. → **Compromis assumé** : c'est l'objet même
  d'`apply`. Un futur `apply --batch <N>` limitera si le besoin apparaît.

## Plan de migration

Sans objet — c'est une nouvelle skill. Un projet existant qui a `workflows:
[propose, explore]` dans son `config.yaml` doit y ajouter `apply` et
relancer `codev update` pour l'installer. Le message de `codev update`
indique déjà la nouvelle skill quand elle apparaît.
