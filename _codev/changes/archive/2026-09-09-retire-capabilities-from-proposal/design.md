# Design : retirer une capacité entière

## Contexte

Voir `proposal.md`. La plomberie existe déjà à moitié : le champ
`retire_capabilities` est réservé dans `ChangeMetadata`, l'erreur de
merge documente son arrivée future. F5 le rend fonctionnel.

## Objectifs / Hors objectifs

Ce design cadre : le champ `deletions` du plan, la nouvelle méthode
`FileSystem::remove_file`, la signature étendue de `merge_into_existing`,
la propagation dans `sync/archive`, le contrat JSON, la nouvelle section
du template. Il ne cadre pas : le flag CLI `--retire-capabilities`, ni
la suppression automatique des décisions liées.

## Décisions

### Décision : `deletions` comme opération de premier ordre du `Plan`

Trois options :

| Option | Pro | Contre |
|---|---|---|
| **A. Signal par un `FileWrite` avec `contents = ""`** | Zéro changement de structure | Ambigu : un fichier légitimement vide devient indistinguable d'une suppression ; masque le geste destructeur |
| **B. Champ `deletions: Vec<PathBuf>` sur `Plan`** | Clarté sémantique — le plan dit ce qu'il fait ; l'exécuteur voit le geste destructeur explicite | Un peu de code en plus, un peu plus dans le contrat |
| **C. Retour spécial `MergeOutcome::DeleteFile` du merger** | Signal typé fort côté cœur | La coquille (sync) doit re-router — plus de plomberie que B pour le même bénéfice |

**Choisi : B.** Un `Plan` qui expose ses trois catégories (writes,
deletions, moves) rend l'exécution transactionnelle explicite : la
coquille peut, un jour, refuser une deletion sans confirmation (mode
`--dry-run`, prompt interactif), sans avoir à reparser des `contents=""`.
Alignement avec la décision
[0001](../../decisions/0001-coeur-fonctionnel-coquille-imperative.md) :
le cœur décrit tous les effets ; la coquille les applique.

### Décision : ordre d'exécution — dirs → writes → **deletions** → moves

Justification :

- **dirs avant writes** : les writes exigent leur dossier parent.
- **writes avant deletions** : une modification d'une spec `A` et la
  suppression de `B` sont indépendantes, mais mettre les deletions
  après réduit la fenêtre d'incohérence si une write plantait après une
  deletion.
- **deletions avant moves** : `archive` déplace le dossier du change à
  la fin — un fichier supprimé pendant le sync ne doit pas ressusciter
  parce qu'il vivait dans le dossier du change (il n'y vit pas — les
  deletions ciblent `_codev/specs/`, `move` cible `_codev/changes/`).

**Alternative écartée** : deletions avant writes. Aucune spec deletion
ne dépend d'une write, mais on préfère « on écrit ce qui est neuf, on
retire ce qui n'a plus lieu d'être » comme lecture humaine du plan.

### Décision : `merge_into_existing` prend `retire_capabilities: bool`

Signature étendue plutôt qu'une nouvelle fonction. Le merger sait déjà
détecter le vidage total (`removed_count >= spec.requirements.len()`) —
il en fait déjà une erreur. Le flag transforme cette erreur en signal
`should_delete_spec: true`.

**Alternative écartée** : une fonction séparée `merge_and_maybe_delete`.
Duplique la logique — deux endroits à maintenir en cohérence.

### Décision : `MergePlan` expose `should_delete_spec`, la coquille route

Le cœur reste ignorant du chemin de la spec principale (il ne connaît
que le texte du delta). C'est la coquille (`sync::plan_sync`) qui, voyant
`should_delete_spec: true`, ajoute la deletion au `Plan` avec le chemin
absolu qu'elle a déjà calculé pour la lecture initiale.

Cohérence avec l'existant : la fonction pure ne construit pas de
`PathBuf` absolus ; c'est le `Layout` (coquille) qui le fait.

### Décision : la section `### Capacités retirées` du template est documentaire

Le parseur ne l'utilise pas — la source de vérité reste `retire_capabilities`
dans `change.yaml`. Deux raisons :

1. **Parseur de proposal.md n'existe pas côté champ « capacités »** — le
   proposal est du texte libre côté outil, sa structure sert au relecteur
   humain et à l'agent.
2. **Découplage geste/déclaration** — un delta `REMOVED` sur une capacité
   dit ce qui change ; le flag `retire_capabilities` dit « et je suis
   prêt à en accepter la conséquence irréversible ». La liste dans le
   proposal aide à documenter, pas à imposer.

### Décision : `FileSystem::remove_file` obligatoire sur le port

Ajout au trait, avec implémentation par défaut refusée (chaque
implémentation doit s'y engager explicitement). `RealFileSystem` appelle
`std::fs::remove_file`. `MemoryFileSystem` retire l'entrée de sa
`HashMap`. Sans cette méthode, `Plan.deletions` serait un champ qu'aucun
port ne saurait honorer.

## Risques et compromis

- **Un utilisateur passe `retire_capabilities: true` par erreur, sur un
  change qui ne retire rien.** → **Compromis assumé** : sans REMOVED
  qui vide une spec, le flag est un no-op silencieux. Pas de warning
  dédié — le flag documente l'intention, pas plus.
- **Un `apply::execute` qui échoue à mi-plan laisse un état
  intermédiaire.** → **Compromis assumé** : atomicité stricte demanderait
  un système de journal (rollback). Trop cher pour l'usage actuel ; le
  `codev validate` + `git status` restent le filet.
- **Une capacité supprimée puis re-ajoutée dans le même change** — cas
  bizarre, mais possible : un REMOVED total suivi d'un ADDED sur la
  même capa dans un autre delta. → **Compromis assumé** : l'ordre des
  fichiers de delta est stable (`walk_files` trié), le premier delta
  détermine le sort de la spec. On documente ce cas au premier report,
  pas maintenant.

## Plan de migration

Aucune. Le champ `deletions` de `Plan` est initialement vide pour tous
les plans existants ; le comportement historique est bit-identique tant
que `retire_capabilities` n'est pas passé à `true`.
