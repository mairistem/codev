# Evaluation: plain-words-in-skills

For pull request mairistem/compagnon#65 (tasks 4.1 and 4.2).

## 4.1 — The suite against the baseline

<!-- EVALS -->

## 4.2 — Two replies in French

Scratch project: `compagnon init --yes --language fr` from this branch's
build, a shell script `salut.sh`, and two changes with `skip_specs: true`:

- `ajouter-salut`: proposal, design and tasks written, both tasks checked,
  the script committed on a branch, the plan review (`plan.challenge`)
  recorded;
- `ajouter-au-revoir`: only the proposal written.

Each was asked in French with `claude -p` (Claude Code 2.1.332, the
project's skills as `compagnon init` installs them).

### Tasks done — « Réalise la modification ajouter-salut. »

> La modification `ajouter-salut` est terminée et prête à être archivée.
>
> - **Tâches :** les 2 tâches étaient déjà cochées à mon arrivée, implémentées dans le commit `708ed66`. Je n'ai donc rien écrit dans le code. J'ai relancé les deux vérifications prévues et elles passent : `./salut.sh Ana` affiche « Salut, Ana ! » et `./salut.sh` affiche « Salut, monde ! ».
> - **Fichier touché :** `salut.sh` seulement, 2 lignes ajoutées.
> - **Tests :** le projet n'a pas de suite de tests automatisés. Seules les deux commandes ci-dessus ont été exécutées. `compagnon validate` ne signale aucun problème.
> - **Corrections pendant la relecture :** aucune. Aucun domaine n'est activé, donc aucun expert n'a relu.
> - **Points de relecture :** rien à signaler. Le relecteur « nouvel arrivant » trouve le script clair.
> - **À décider :** rien.
>
> L'archivage était bloqué parce que le registre de relecture n'avait pas de section pour la relecture du code. Je l'ai ajoutée dans `_compagnon/changes/ajouter-salut/review.md`, et l'archivage est maintenant possible. Ce fichier est modifié mais pas encore commité.
>
> Une fois que vous aurez relu la modification, dites :
>
> « archive la modification ajouter-salut » (raccourci : `/compagnon-archive`, ou `compagnon archive` dans un terminal)

### Plan incomplete — « Réalise la modification ajouter-au-revoir. »

> Je n'ai rien implémenté : le plan de la modification `ajouter-au-revoir` n'est pas terminé (`isPlanningComplete: false`).
>
> État des documents de planification :
> - **proposal.md** : rédigée
> - **specs** : ignorées (pas de spécification pour cette modification)
> - **design.md** : pas encore rédigée, c'est ce qui bloque
> - **tasks.md** : bloquée tant que la conception n'existe pas. Le fichier n'existe pas encore, donc il n'y a aucune tâche à réaliser.
>
> Il faut d'abord terminer le plan, en rédigeant la conception puis les tâches. Vous pouvez aussi rédiger ces fichiers à la main dans `_compagnon/changes/ajouter-au-revoir/`.
>
> « termine le plan de ajouter-au-revoir » (raccourci : `/compagnon-propose`)

### Checked

| | Tasks done | Plan incomplete |
|---|---|---|
| Reply in French | yes | yes |
| Lexicon: « modification », never « change » | yes | yes |
| One request alone on the last line | yes | yes |
| Shortcut after the request | `/compagnon-archive` | `/compagnon-propose` |

Two remarks:

- The archive line adds a second shortcut, `compagnon archive` in a
  terminal. It is still one request; the rule does not forbid it.
- A first attempt, before the scratch change had its plan review
  recorded, ended on « Fais la revue du plan de ajouter-salut »
  (raccourci : `/compagnon-update`). The request was right for that state,
  but the plan review belongs to the planning step, whose shortcut is
  `/compagnon-propose`. The scratch project was missing a record any real
  change has, so this is noted, not counted.
