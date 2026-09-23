Implémenter les tâches d'un change codev — traiter chaque case `- [ ]` de
`tasks.md` dans l'ordre, cocher au fur et à mesure, s'arrêter au premier
blocage.

**Frontière d'implémentation.** Ce workflow **écrit du code du projet** :
c'est le seul de codev qui touche à autre chose que le dossier `_codev/`. En
contrepartie, il se cantonne strictement au change nommé : il ne modifie
aucun autre change, il n'archive pas, il ne sync pas. Ces derniers sont des
pas suivants explicites, à demander par l'utilisateur.

---

## Entrée

Un nom de change en argument, ou rien (auquel cas le change est résolu
implicitement s'il n'y en a qu'un seul actif).

## Étapes

### 1. Résoudre le change et vérifier que la planification est complète

Si l'utilisateur a nommé un change, prends celui-là. Sinon, appelle :

```bash
codev list
```

- Un seul change actif → c'est celui-là.
- Plusieurs changes actifs → demande à l'utilisateur lequel, en listant les
  noms. Ne devine pas.
- Aucun change actif → dis-le, propose `/codev-propose` pour en créer un.

Puis vérifie la planification :

```bash
codev status --change "<nom>" --json
```

Si `isPlanningComplete` est `false`, arrête : la planification n'est pas
prête. Indique quels artefacts manquent et propose `/codev-propose` ou
l'édition manuelle. N'implémente rien.

### 2. Lire les tâches

```
Read _codev/changes/<nom>/tasks.md
```

Le format attendu est strict :

- une tâche : `- [ ] X.Y Description, vérifiée par <test ou commande>`
- une tâche cochée : `- [x] X.Y …`
- des groupes sous des titres `## N. …`

Si tu trouves un format différent (`-[ ]` sans espace, `- [X]` majuscule,
`- [-]` autre marqueur), signale-le à l'utilisateur et propose de corriger
avant de continuer.

### 3. Traiter chaque tâche non cochée, dans l'ordre du fichier

Pour chaque `- [ ]` rencontrée, dans l'ordre où elle apparaît :

**a. Annonce.** « Tâche X.Y : <description résumée en un fragment>. »

**b. Étudie.** Lis les fichiers concernés par la tâche. Reste en lecture
seule le temps de comprendre, écris ensuite. Chaque tâche du `tasks.md`
énonce comment vérifier qu'elle est faite — cette vérification est le
critère d'acceptation, pas une suggestion.

**c. Implémente.** Écris le code, les tests, la configuration. Cible ce que
la tâche demande, rien de plus.

**d. Vérifie.** Lance la commande, le test, l'observation citée par la
tâche. Une compilation qui échoue, un test qui tombe rouge, un
comportement absent : la tâche n'est pas faite. Corrige, relance.

**e. Coche.** Modifie `tasks.md` avec `Edit` : la ligne `- [ ] X.Y …`
devient `- [x] X.Y …`. Reste précis — un `replace_all` remplacerait aussi
les tâches d'autres changes lues précédemment, à éviter. Utilise l'ancienne
ligne complète comme repère.

**f. Court retour à l'utilisateur.** « ✓ X.Y — <ce que ça a produit en une
ligne> ». Continue à la suivante.

### 4. Arrêt sur ambiguïté ou blocage

Une tâche n'est pas exécutable dans deux cas :

- **Ambiguïté matérielle** : sa formulation admet plusieurs interprétations
  qui changeraient matériellement le résultat (choix d'API, format de
  sortie, comportement sur cas limite). Dans ce cas :
  - **ne coche pas**,
  - décris les interprétations à l'utilisateur,
  - propose que la tâche soit scindée en `X.Y.a` / `X.Y.b` dans `tasks.md`
    — mais laisse l'utilisateur trancher ou reformuler.
- **Blocage technique** : dépendance manquante, test qui ne peut pas
  s'exécuter, une commande qui exige une intervention manuelle. Dans ce
  cas :
  - **ne coche pas**,
  - décris le blocage,
  - propose la piste de résolution qui te semble la meilleure, sans
    l'appliquer.

Dans les deux cas, tu t'arrêtes après la tâche courante. Les tâches
suivantes ne sont pas traitées tant que l'obstacle n'est pas levé.

### 5. Fin — inviter à archive comme pas suivant explicite

Quand toutes les cases sont cochées, résume :

- nombre de tâches faites,
- fichiers principaux touchés (une ligne),
- « Le change est prêt à être archivé. Lance `/codev-archive` (à venir) ou
  `codev archive` en pas suivant. »

**N'archive pas toi-même.** L'utilisateur doit relire d'abord.

## Sortie

Un résumé final, comme décrit à l'étape 5.

## Garde-fous

- **Frontière du change** : ne modifie aucun fichier d'un autre change
  actif ou archivé. Si une tâche te force à toucher un autre change, c'est
  un signe que le change présent est mal cadré — arrête et demande.
- **Pas de sync ni d'archive automatique** : ces opérations sont des
  décisions à part, prises par l'utilisateur.
- **Pas de contournement d'erreur** : un test qui échoue n'est pas coché.
  `--no-verify`, `#[ignore]`, un `expect_err` complaisant : chacune est un
  drapeau rouge qui remplace le silence par un mensonge.
- **Vérifications avant de cocher** : cocher une case sans avoir vérifié la
  ligne de vérification citée dans la tâche = régression annoncée. Toujours
  vérifier.
- **Édition ciblée de tasks.md** : la ligne changée est identifiée par son
  texte complet, jamais par un `- [ ]` seul. Sans cela, une autre case
  serait cochée par accident.
