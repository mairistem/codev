# Rôles

Une skill codev est un seul agent, mais il n'aborde pas chaque étape dans le
même état d'esprit. Chaque étape reçoit un rôle : une question dont il a la
charge, et des critères « Done when » que l'agent vérifie avant de passer à
la suite. Vous et votre équipe validez à des moments définis — l'agent ne
décide jamais à votre place.

Les rôles sont des critères, pas un jeu de rôle. L'agent ne se fait pas
passer pour un product owner : il vérifie la proposal à l'aide de la liste
du product owner. Un rôle dont les critères sont remplis est terminé, quel
que soit le ton de la prose. Certains rôles portent aussi des
**connaissances** — les signaux concrets, les pièges par langage et les
références dont ils ont besoin pour bien vérifier leurs critères — voir
[Experts et domaines](#experts-et-domaines).

Les rôles sont des données, pas de la prose enfouie dans les skills. codev
les sert : à chaque étape, les skills demandent `codev roles --at <step>` et
appliquent ce qui leur revient, et `codev instructions` transmet à l'agent
le rôle de chaque artefact. Votre projet peut désactiver un rôle, ajouter
ses propres critères et changer l'activation pour un seul change — voir
[Configurer les rôles](#configurer-les-rôles).

## Rôles intégrés

| Id | Nom | Nature | Étape | Sa question |
|---|---|---|---|---|
| `analyst` | Analyste | étape | `plan.understand` | Que faut-il construire ? |
| `investigator` | Enquêteur | étape | `plan.investigate` | Que fait déjà le projet ? |
| `product` | Product owner | artefact (`proposal`) | `plan.write` | Pourquoi ce change, et qu'est-ce qui change ? |
| `qa` | Analyste QA | artefact (`specs`) | `plan.write` | Quel comportement observable doit tenir ? |
| `architect` | Architecte | artefact (`design`) | `plan.write` | Comment le construire ? |
| `lead` | Tech lead | artefact (`tasks`) | `plan.write` | Dans quel ordre, et comment vérifier chaque étape ? |
| `tracer` | Contrôleur de traçabilité | étape | `plan.trace` | Les artefacts concordent-ils entre eux ? |
| `skeptic` | Contradicteur | étape | `plan.challenge` | Le plan résisterait-il à un relecteur qui ne l'a pas écrit ? |
| `security` | Sécurité | expert (domaine `security`) | `plan.challenge`, `code.review` | Un attaquant peut-il détourner ce que le change ajoute ? |
| `privacy` | Données personnelles | expert (domaine `personal-data`) | `plan.challenge`, `code.review` | Le change ne collecte-t-il, ne conserve-t-il et ne partage-t-il des données personnelles que dans la mesure nécessaire ? |
| `accessibility` | Accessibilité | expert (domaine `user-interface`) | `plan.challenge`, `code.review` | Tout le monde peut-il utiliser ce que le change ajoute, au clavier ou avec une technologie d'assistance ? |
| `performance` | Performance | expert (domaine `performance`) | `plan.challenge`, `code.review` | Le change reste-t-il rapide quand les données et la charge augmentent ? |
| `ops` | Exploitation | expert (domaine `operations`) | `plan.challenge`, `code.review` | L'équipe peut-elle exploiter, observer et annuler ce que le change ajoute ? |
| `migration` | Migration | expert (domaine `migration`) | `plan.challenge`, `code.review` | Les données, la configuration et les consommateurs existants peuvent-ils passer à ce change sans risque ? |
| `developer` | Développeur | étape | `code.implement` | Chaque tâche fonctionne-t-elle, vérification à l'appui ? |
| `newcomer` | Nouvel arrivant | étape | `code.review` | Un développeur qui n'a pas vu le plan peut-il suivre ce code ? |

Les rôles d'étape et les experts sont intégrés à codev. Les rôles
d'artefact viennent du schéma par défaut `spec-driven`, qui déclare un rôle
par artefact ; un [schéma personnalisé](guides/custom-schemas.md) déclare
les siens, ou aucun.

Pour lire les critères « Done when » d'un rôle, interrogez codev — c'est la
liste que reçoit l'agent :

```bash
codev roles --at plan.challenge
```

Sans `--at`, `codev roles` liste tous les rôles avec leur activation ;
ajoutez `--knowledge` pour lire aussi les connaissances d'un rôle. Voir
[`codev roles`](reference/cli.md#codev-roles).

## Étapes

| Étape | Skill | Ce qui s'y passe |
|---|---|---|
| `plan.understand` | `/codev-propose` | La demande est comprise avant que le change soit créé ; un ticket comme `PROJ-123` est lu via le MCP Jira lorsqu'il est configuré |
| `plan.investigate` | `/codev-propose` | Le code, les tests, la configuration et la documentation sont étudiés avant la rédaction de chaque artefact |
| `plan.write` | `/codev-propose` | Chaque artefact est rédigé, puis vérifié à l'aide de son rôle, que `codev instructions` renvoie dans `role` |
| `plan.trace` | `/codev-propose` | Les artefacts sont confrontés entre eux ; les manques sont corrigés dans les artefacts |
| `plan.challenge` | `/codev-propose` | Le plan est relu par le contradicteur et par les experts qu'activent les domaines du change ; chaque constat va dans le [compte rendu de revue](#gravité-confiance-et-compte-rendu-de-revue), l'auteur le corrige ou le conteste et le relecteur vérifie la réponse ; au plus cinq **points à challenger**, tous rôles confondus et chacun marqué de son rôle, puis ce qu'il vous revient de décider terminent le résumé |
| `code.implement` | `/codev-apply` | Chaque tâche est implémentée, vérifiée, puis cochée |
| `code.review` | `/codev-apply` | Une fois toutes les tâches cochées, le code du change est relu par le nouvel arrivant et par les experts actifs ; chaque constat va dans le compte rendu de revue ; ce qui ne fait pas de doute est simplifié, et au plus cinq **notes de relecture**, chacune marquée de son rôle, puis ce qu'il vous revient de décider terminent le résumé — un constat de sécurité ou de données personnelles de gravité haute n'est jamais écarté |

La procédure de chaque étape — ce qu'il faut écrire, les plafonds, les
garde-fous — reste dans la skill. Les critères et les connaissances
viennent du rôle.

## Experts et domaines

Six experts rejoignent les étapes de relecture lorsqu'un change touche leur
domaine. Chacun est actif à `plan.challenge` — le design traite-t-il le
domaine ? — et à `code.review` — l'implémentation le respecte-t-elle ?

| Domaine | Expert | Ce qu'il examine |
|---|---|---|
| `security` | `security` | Validation des entrées aux frontières de confiance, autorisation côté serveur, secrets, injections, désérialisation non sûre, nouvelles dépendances |
| `personal-data` | `privacy` | Finalité et minimisation, conservation et suppression, droits des personnes, données personnelles dans les journaux, sous-traitants et transferts |
| `user-interface` | `accessibility` | Accès au clavier, focus, alternatives textuelles, étiquettes et erreurs de formulaire, contraste, sémantique et ARIA |
| `performance` | `performance` | Requêtes dans des boucles, résultats non bornés, E/S bloquantes sur les chemins critiques, allocations dans les boucles, coût des nouvelles dépendances |
| `operations` | `ops` | Journaux structurés sans secret, santé et métriques, délais d'expiration et nouvelles tentatives, chemin de retour arrière, erreurs exploitables |
| `migration` | `migration` | Changements incompatibles annoncés, chemins de migration, ordre de déploiement, migrations de schéma réversibles, dépréciation |

**Domaines.** La proposal liste les domaines que touche le change dans sa
section `## Domains` — les six ci-dessus, à cocher, ou la seule ligne
`None` :

```markdown
## Domains

- [x] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — nouveau worker en arrière-plan
- [ ] migration
```

Le product owner vérifie que chaque domaine touché par le change est coché.
`codev validate` signale un domaine inconnu (`unknown_domain`) ou une
section mal formée (`domains_invalid`) comme une erreur, et une section
absente sur un change actif (`domains_missing`) comme un avertissement —
voir [Formats de fichiers](reference/file-formats.md#proposalmd).

**Activation `on_domain`.** Un expert n'est actif à ses étapes que lorsque
la proposal du change coche son domaine. Sans change — `codev roles` sans
`--change` — ou tant que la proposal n'a pas de section `## Domains`, il
est inactif. `codev roles --json` indique pourquoi chaque rôle actif l'est
dans `activatedBy` : `always`, `project`, `change` ou `domain:<nom>`. Vous
pouvez toujours forcer un expert, pour le projet ou pour un change — voir
[Configurer les rôles](#configurer-les-rôles).

**Connaissances.** Les connaissances d'un expert se répartissent en cinq
sections, toujours dans cet ordre :

- **Scope** — ce qu'il examine, et ce qu'il laisse de côté pour éviter le
  bruit ;
- **Signals** — des motifs concrets à repérer, chacun avec un exemple
  d'une ligne ;
- **By stack** — les pièges en Rust, TypeScript/JavaScript, Python, Go et
  Java ; codev sert la partie qui concerne les langages de votre projet
  (voir [Filtre par stack](#les-déclencheurs-et-les-domaines-suggérés)) ;
- **References** — les référentiels qui font autorité, comme l'OWASP
  Top 10, les WCAG 2.2, le RGAA 4 ou le RGPD ;
- **Reporting** — comment signaler un constat, avec sa gravité.

Le `newcomer` porte des signaux de complexité par langage, et le `skeptic`
des signaux de planification fragile ; les autres rôles d'étape n'en
portent pas. Les connaissances ne sont servies qu'aux rôles actifs : un
expert inactif ne coûte rien à l'agent. Lisez-les avec
`codev roles --at code.review --change <nom> --knowledge`.

## Les déclencheurs et les domaines suggérés

Cocher les domaines est un jugement porté pendant la planification, avant
que le code existe. Pour qu'un domaine que le change finit par toucher ne
soit pas oublié en silence, chaque expert porte des **déclencheurs** — des
indices qu'un change touche son domaine — et codev les confronte au diff du
change.

**Trois sortes de déclencheurs**, en groupes qui nomment chacun le domaine
qu'ils suggèrent :

- `paths` — des globs sur les fichiers modifiés, relatifs à la racine du
  projet, sans tenir compte de la casse : `**/auth/**`, `**/*.tsx`. `*`
  reste dans un dossier, `**` traverse les dossiers ;
- `manifests` — des noms de manifestes de dépendances, à toute
  profondeur : `Cargo.toml`, `package.json`, `go.mod` ;
- `patterns` — des expressions régulières sur les lignes que le diff
  **ajoute** : `(?i)\b(ALTER|DROP)\s+(TABLE|COLUMN)\b`.

| Expert | Ses déclencheurs, en résumé |
|---|---|
| `security` | dossiers `auth`, `security`, `crypto`, fichiers `.env`, clés ; chaque manifeste de dépendances ; noms d'identifiants secrets, `eval`, `innerHTML`, `pickle`, `shell=True`, `unsafe {` |
| `privacy` | modèles `user`, `customer`, `profile`, `account` ; e-mail, téléphone, date de naissance, adresse postale, code postal, numéro de sécurité sociale, IBAN, adresse IP, géolocalisation |
| `accessibility` | balisage, feuilles de style, composants, gabarits serveur ; `aria-*`, `role=`, `alt=`, éléments de formulaire et de média |
| `performance` | `*.sql`, dossiers de requêtes et de dépôts, benchmarks ; requêtes SQL, `Promise.all`, `tokio::spawn`, caches, pagination |
| `ops` | Dockerfiles, fichiers compose, workflows de CI, Kubernetes, Helm, Terraform, dossiers de déploiement, configuration des logs ; variables d'environnement, délais, nouvelles tentatives, sondes de santé, métriques |
| `migration` | migrations, schémas, `*.proto`, `*.graphql`, OpenAPI, `*.d.ts` ; `ALTER`/`DROP`/`RENAME`, dépréciations, renommages `serde` |

Les motifs intégrés privilégient la précision : `access_token` plutôt que
`token` (un analyseur lexical a des tokens), `street_address` plutôt
qu'`address` (une socket en a une). Ce qu'ils manquent pour votre projet,
ajoutez-le avec [`triggers:`](reference/configuration.md#triggers) dans
`_codev/config.yaml`.

**Le diff.** codev demande à git, localement, ce que la branche a changé
depuis qu'elle a quitté la branche par défaut — `origin/HEAD`, sinon
`main`, sinon `master` —, commité ou non, plus les fichiers non suivis,
dans le seul dossier du projet. Il ne contacte jamais de dépôt distant.
Hors de git, sans commit ou sans branche par défaut — ou dans un clone
superficiel de CI sans la base commune ; utilisez-y `fetch-depth: 0` —, il
n'y a pas de diff : codev le dit et ne suggère rien, et aucune commande
n'échoue pour autant.

Certains fichiers échappent à tout déclencheur : le dossier de
planification `_codev/` et les skills codev (`planning`), les dépendances
embarquées — `node_modules`, `vendor`, `third_party` (`vendored`) —, les
fichiers de verrouillage comme `Cargo.lock` ou `package-lock.json`
(`lockfile`), et les fichiers générés comme `*.min.js` ou `*.pb.go`
(`generated`). Les fichiers binaires, la prose (`.md`, `.txt`…) et les
très gros fichiers ne sont confrontés que par leur chemin (`binary`,
`prose`, `large`). L'analyse s'arrête à 2000 fichiers, 5000 lignes
ajoutées par fichier, 1 Mio par fichier non suivi et 2 Mio de lignes
ajoutées au total, et ignore les lignes de plus de 2000 caractères ; le
diff indique alors qu'il a été tronqué.

**Les suggestions.** `codev roles --change <nom>` présente le diff et
chaque domaine suggéré avec son état :

- `ticked` — la proposal le coche ;
- `justified` — la proposal le laisse décoché avec une raison sur sa ligne ;
- `unticked` — ni l'un ni l'autre : la proposal doit y répondre.

```markdown
## Domains

- [x] security
- [ ] personal-data — the fields matched are test fixtures
```

`codev validate` signale chaque suggestion `unticked` par l'avertissement
`domain_suggested`, à la ligne du domaine, en nommant le déclencheur et le
fichier concernés ; `codev validate --strict` échoue alors. `codev
archive` et `codev doctor` ne lisent pas le diff. Le diff est celui de
l'arbre de travail : deux changes actifs sur une même branche le
partagent. Un rôle réglé sur `never` n'apporte aucune suggestion.

`/codev-propose` lit les suggestions avant d'écrire `## Domains`, et le
skeptic consigne un constat pour chacune restée `unticked`. Avant que le
code existe, le diff est en général vide : c'est la revue qui tranche.
`/codev-apply` liste chaque suggestion encore `unticked` sous **To
decide** — cochez-la, et son expert relit le code, ou justifiez-la.

**Filtre par stack.** codev détecte les langages de votre projet à partir
de ses manifestes de dépendances — à la racine et jusqu'à trois dossiers
en dessous, si bien qu'un monorepo obtient toutes ses stacks — et la
partie `By stack` des connaissances qu'il sert ne garde que ces langages.
`codev roles --json` les indique dans `stacks` ; sans aucun manifeste,
tout est conservé. `codev roles --all-stacks` sert tous les langages.

## Gravité, confiance et compte rendu de revue

Ce que trouvent les rôles de relecture — le contradicteur et les experts à
`plan.challenge`, le nouvel arrivant et les experts à `code.review` — est
écrit dans le **compte rendu de revue** du change,
`_codev/changes/<name>/review.md`, avant d'être corrigé ou signalé. Chaque
constat cite sa preuve mot pour mot, à un emplacement que codev peut
vérifier, et porte une gravité et une confiance sur une échelle commune :

| Gravité | Signification |
|---|---|
| `high` | Le change casserait une exigence, perdrait ou exposerait des données, ou ouvrirait une vulnérabilité, et ne doit pas être archivé tant qu'il n'est pas corrigé ou qu'une personne n'en accepte pas le risque. |
| `medium` | Un défaut ou un risque réel, à l'impact limité ou contournable, à corriger dans ce change ou à reporter en connaissance de cause. |
| `low` | Une amélioration — lisibilité, cohérence, coût mineur — qui ne décide pas de la justesse du change. |

| Confiance | Signification |
|---|---|
| `high` | La preuve montre le problème directement. |
| `low` | Le problème dépend de quelque chose que le relecteur n'a pas pu voir ou vérifier. |

Un constat de confiance basse reste dans le compte rendu et reçoit une
réponse comme les autres, mais il ne vous est présenté que si l'on vous
demande de le trancher.

L'équipe travaille ensuite comme une équipe : l'auteur — le rôle qui
possède l'artefact, ou le développeur pour le code — corrige chaque constat
ou le conteste avec un argument ; le relecteur vérifie chaque réponse une
fois, en validant la correction, en acceptant la contestation (`rejected`)
ou en la maintenant ; ce qui reste contesté vous revient, avec les deux
arguments, sous **À décider**. Vous seul acceptez un risque ou reportez un
constat de gravité haute, et le compte rendu dit qui a décidé.
`codev archive` refuse un change dont le compte rendu manque pour une étape
où des rôles ont relu, ou qui a encore un constat de gravité haute ouvert ou
contesté. Le format complet, les contrôles et le verrou d'archivage sont
dans [Compte rendu de revue](guides/review-record.md).

## Configurer les rôles

Tous les rôles sont actifs par défaut. Dans `_codev/config.yaml`, `roles:`
associe un identifiant de rôle à son réglage :

```yaml
roles:
  newcomer: never            # forme courte : always | never
  skeptic:
    when: always             # facultatif, always par défaut
    done_when:               # ajoutés après les critères propres du rôle
      - Le plan de retour arrière est indiqué
```

- **Activation.** `always`, `never` ou `on_domain`. Une étape dont tous les
  rôles sont inactifs est sautée, et le résumé de la skill indique qu'elle
  est désactivée par la configuration. `on_domain` est la valeur par défaut
  des experts ; elle est refusée sur un rôle lié à aucun domaine. Indiquez
  `security: always` pour que l'expert sécurité relise chaque change, ou
  `accessibility: never` dans un projet sans interface utilisateur.
- **Critères supplémentaires.** Les entrées de `done_when` s'ajoutent après
  les critères propres du rôle, sans jamais les remplacer. Elles portent
  l'origine `project`, comme votre
  [`context` et vos `rules`](reference/configuration.md#context).
- **Par change.** Le `change.yaml` d'un change peut changer l'activation
  pour ce change seulement :

  ```yaml
  roles:
    newcomer: always
  ```

- **Ce qui ne peut pas être désactivé.** Les rôles d'artefact (`product`,
  `qa`, `architect`, `lead`), l'`analyst` — le garde-fou contre un plan
  bâti sur un malentendu — et le `developer` : leur artefact est toujours
  rédigé, la demande toujours comprise, leurs tâches toujours
  implémentées. Leurs critères peuvent être complétés.
- **Les sources héritées** n'apportent aucun rôle : seul le `roles:` de
  votre projet est lu.

`codev validate` signale un identifiant de rôle inconnu (`unknown_role`),
une activation autre que `always`, `never` ou `on_domain`, ou `on_domain`
sur un rôle sans domaine (`invalid_role_activation`), et `never` sur un
rôle qui ne peut pas être désactivé (`role_not_disableable`) ;
`codev status`, `codev instructions` et `codev roles` échouent avec le même
code. `codev doctor` liste les rôles actifs, les rôles actifs selon le
domaine avec leurs domaines, et les rôles désactivés.

## Implémenter un change (`/codev-apply`)

### Développeur (`/codev-apply`)

Implémente les tâches d'un change planifié, dans l'ordre du fichier. Done
when, pour chaque tâche :

- les fichiers concernés sont lus avant d'écrire ;
- le code, les tests, la configuration et la documentation que la tâche
  demande sont écrits — rien de plus ;
- les tests sont écrits à la manière de la suite existante du projet, puis
  selon les conventions de son langage et les `rules` du projet ;
- la vérification citée par la tâche est lancée et réussit — la commande de
  test vient de `context`, des `rules`, du manifeste ou de la configuration
  de CI, et la skill vous la demande si elle n'en trouve aucune ;
- alors seulement, la case est cochée.

Elle s'arrête sur une ambiguïté importante ou un blocage technique sans
cocher la tâche, et décrit les options. Une fois toutes les cases cochées,
et après la [relecture](#nouvel-arrivant-et-experts-codev-apply),
elle lance une fois la suite de tests complète du projet ; un échec est
signalé à la place du résumé. Elle ne contourne jamais un test en échec, ne
touche jamais un autre change, ne synchronise jamais et n'archive jamais.

### Nouvel arrivant et experts (`/codev-apply`)

Une fois toutes les tâches cochées, et avant la suite de tests complète,
relit uniquement le code que le change a ajouté ou modifié : le nouvel
arrivant, comme un développeur qui rejoint le projet sans avoir vu ni le
plan ni la conversation, et les experts qu'activent les domaines du change,
chacun à l'aide de ses critères et de ses connaissances. Chaque constat va
d'abord dans le [compte rendu de
revue](#gravité-confiance-et-compte-rendu-de-revue). Ce qui ne
fait pas de doute est simplifié et les tests concernés sont relancés ; le
reste devient au plus cinq **notes de relecture** dans le résumé final —
chacune marquée de son rôle et de l'identifiant du constat : quoi, où,
pourquoi — ou la mention qu'aucune n'a été relevée, suivies de ce qu'il vous
revient de décider. Un constat de sécurité ou de données personnelles de gravité haute
est toujours listé, même au-delà de cinq.

La relecture n'élargit jamais le périmètre du change et ne touche jamais au
code que le change n'a ni ajouté ni modifié. Le nouvel arrivant ne change
jamais le comportement. Un expert ne le change jamais en silence : une
correction qui change le comportement devient une note de relecture, sauf
s'il s'agit d'un défaut manifeste dans le périmètre du change, alors
corrigé avec un test et mentionné dans le résumé.

## Les autres skills

Les autres skills gardent une posture fixe, écrite dans la skill elle-même.
Ces postures ne sont pas des rôles : elles ne figurent pas au catalogue et
ne se configurent pas.

| Skill | Posture | Sa question | Ce qu'elle écrit |
|---|---|---|---|
| `/codev-explore` | [Partenaire de réflexion](#partenaire-de-réflexion-codev-explore) | Cela vaut-il la peine d'être construit, et comment ? | Rien |
| `/codev-sync`, `/codev-archive` | [Mainteneur des specs vivantes](#mainteneur-des-specs-vivantes-codev-sync-codev-archive) | Les specs principales décrivent-elles ce qui a été planifié et livré ? | Uniquement via la CLI |
| `/codev-update` | [Éditeur](#éditeur-codev-update) | L'artefact révisé reste-t-il cohérent avec les autres ? | `_codev/changes/<name>/` uniquement |
| `/codev-onboard` | [Guide](#guide-codev-onboard) | Où en est ce projet, et que faire ensuite ? | Rien |
| `/codev-configure` | [Analyste de configuration](#analyste-de-configuration-codev-configure) | Que doit savoir un agent sur ce projet avant d'écrire ? | `_codev/config.yaml`, après confirmation |

### Partenaire de réflexion (`/codev-explore`)

Explore une idée, un problème ou un besoin, sans engager quoi que ce soit.
Done when :

- ce que le dépôt sait déjà passe en premier — specs, changes en cours,
  décisions en vigueur, code concerné — de sorte qu'on ne vous demande
  jamais ce que le code peut dire ;
- les questions posées portent sur une dépendance, une contrainte ou un
  compromis, chacune avec une réponse recommandée par défaut et sa raison ;
- deux ou trois options sont comparées, chacune avec ce qu'elle coûte et ce
  qu'elle exclut, et une recommandation est faite ;
- une direction qui s'écarte d'une décision acceptée est signalée tôt ;
- les hypothèses sont présentées comme des hypothèses ;
- l'exploration se termine de l'une de trois façons, nommée : elle se
  cristallise (problème, approche, périmètre, et la proposition de lancer
  `/codev-propose`), une information manque (laquelle, et qui peut la
  fournir), ou la piste ne valait pas la peine.

Elle n'écrit aucun fichier, ne crée aucun change et n'implémente pas, même
si on le lui demande.

### Mainteneur des specs vivantes (`/codev-sync`, `/codev-archive`)

Veille à ce que `_codev/specs/` décrive le système. `/codev-sync` fusionne
les deltas d'un change sans le déplacer ; `/codev-archive` les fusionne et
déplace le change dans `_codev/changes/archive/<date>-<name>/`. Done when :

- la planification est complète — sinon rien n'est lancé ;
- la fusion passe par `codev sync` ou `codev archive`, jamais par un fichier
  que la skill écrirait elle-même ;
- les specs créées, mises à jour et inchangées sont indiquées (et, pour
  l'archivage, l'endroit où le change a été déplacé).

Sur une erreur de validation, les deux refusent : elles répondent que le
change comporte des erreurs et renvoient vers `codev validate <name>`, sans
réessayer ni corriger le change elles-mêmes. Toute autre erreur est relayée
telle que la CLI l'a écrite.

### Éditeur (`/codev-update`)

Révise un artefact de planification qui existe déjà dans un change actif.
Done when :

- l'artefact et les autres qu'il peut affecter sont lus sur le disque avant
  d'écrire ;
- la révision respecte le contrat de l'artefact — sections attendues,
  format des scénarios, cases à cocher ;
- les répercussions sont annoncées : un fichier de spec orphelin ou manquant
  après une modification de la proposal, une tâche qui cite une exigence
  supprimée ou un scénario renommé, une décision du design qui ne
  correspond plus, une tâche qui contredit une exigence — avec une action
  suivante suggérée, appliquée seulement après votre confirmation ;
- `codev validate` a été lancé et son rapport est relayé.

Elle refuse de créer un artefact manquant, de toucher à un change archivé
ou de modifier le code.

### Guide (`/codev-onboard`)

Présente codev à qui le découvre. Done when elle a affiché trois blocs : ce
qu'est codev, l'état du projet (nombre de specs, de décisions et de changes,
et jusqu'à cinq éléments d'installation manquants signalés par
`codev doctor`, avec leur correction), et exactement une action recommandée,
citée sous la forme de la commande à taper. Elle est strictement en lecture
seule : elle ne lance aucune correction, pas même `codev init`.

### Analyste de configuration (`/codev-configure`)

Propose le `context:` et les `rules:` par artefact de `_codev/config.yaml` à
partir du projet lui-même. Done when :

- la lecture reste dans son budget — README et CONTRIBUTING en entier, au
  plus 5 fichiers de documentation, 8 fichiers source et 3 fichiers de
  test ;
- `context:` compte 2 à 5 lignes factuelles qui complètent la pile
  détectée ;
- `rules:` compte 1 à 2 règles pour `specs`, `design` et `tasks`, chacune
  formulée positivement, vérifiable en relecture et fondée sur ce que fait
  le projet, plus des règles de test tirées des tests existants du projet ;
- le diff est affiché et votre accord est demandé avant toute écriture.

Elle n'écrit qu'après votre oui, ne modifie que `context:` et `rules:`,
préserve tous les autres champs et commentaires, et refuse d'agir si
`_codev/config.yaml` n'existe pas.

## Là où vous décidez

L'agent s'arrête et vous passe la main à ces moments :

- **Questions de clarification.** `/codev-propose` vous interroge avant de
  créer un change lorsqu'une ambiguïté changerait sensiblement le résultat,
  et de nouveau lorsqu'un artefact demande une décision ou qu'une Open
  Question change ce qu'il faut construire. `/codev-explore` pose les
  questions auxquelles le code ne peut pas répondre. `/codev-apply` s'arrête
  sur une ambiguïté ou un blocage et vous laisse décider.
- **Valider le plan.** `/codev-propose` s'arrête une fois les artefacts
  rédigés. Commencez votre relecture par les points à challenger, ce qui
  reste à décider et les Open Questions du design ; quand le plan vous convient, demandez
  `/codev-apply`. Pour modifier d'abord un artefact, utilisez
  `/codev-update`.
- **Accepter les décisions.** Un design suit les décisions en vigueur ou
  propose explicitement d'en remplacer une. Une décision ne prend effet que
  lorsque vous lancez [`codev decision accept`](concepts.md#décision).
- **Confirmer la configuration.** `/codev-configure` n'écrit qu'après votre
  oui à son diff.
- **Appliquer une répercussion.** `/codev-update` signale ce que la révision
  affecte par ailleurs ; vous acceptez, refusez ou reportez chaque suite.
- **Mettre à jour codev.** Lorsqu'un codev plus récent est disponible ou que
  le projet exige une autre version, la skill demande **Update now** ou
  **Later** avant d'agir (voir
  [Annonce des mises à jour](skills.md#annonce-des-mises-à-jour)).
- **Configurer les rôles.** Les rôles de relecture qui s'exécutent, et les
  critères qu'ils ajoutent, relèvent du choix de votre projet dans
  `roles:` — voir [Configurer les rôles](#configurer-les-rôles).
- **Trancher la revue.** Une contestation que le relecteur a maintenue, ou
  un constat de gravité haute que personne n'a pu corriger, vous revient
  sous **À décider** : le corriger, en accepter le risque avec une
  justification, ou le reporter vers un autre change ou un ticket. La skill
  consigne votre décision avec votre nom ; elle n'accepte jamais un risque à
  votre place. `codev review` montre ce qui reste.
- **Archiver.** `/codev-apply` n'archive jamais : une fois l'implémentation
  relue, en commençant par ses notes de relecture, lancez `/codev-archive` ou `codev archive`.
