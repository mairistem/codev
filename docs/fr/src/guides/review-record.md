# Compte rendu de revue

Quand le contradicteur et les experts challengent un plan, ou que le nouvel
arrivant et les experts relisent le code, ce qu'ils trouvent ne reste pas
dans la conversation. Tout va dans le compte rendu de revue du change,
`_codev/changes/<name>/review.md` : chaque constat avec sa gravité, la
preuve sur laquelle il repose, et chaque réponse qu'il a reçue — qui l'a
corrigé, qui l'a contesté et pourquoi, qui en a accepté le risque. codev
vérifie ce qui peut l'être, et `codev archive` ne clôt pas un change dont
la revue n'est pas tranchée.

## Ce que contient un constat

````markdown
## code.review

### security

#### R-security-1 The session lookup concatenates the token

```yaml
severity: high
confidence: high
location: "src/auth/session.rs:42-44"
evidence: |
  let query = format!("SELECT * FROM sessions WHERE token = '{}'", token);
recommendation: Bind the token as a query parameter.
history:
  - status: open
    by: security
```
````

- **L'identifiant**, `R-security-1`, est stable pendant toute la vie du
  change : l'identifiant du rôle et le prochain numéro libre. Il n'est
  jamais renuméroté ni réutilisé, si bien qu'une discussion peut le citer.
- **La gravité** dit à quel point c'est grave si le relecteur a raison, et
  **la confiance** à quel point il en est sûr — voir les échelles dans
  [Rôles](../roles.md#gravité-confiance-et-compte-rendu-de-revue). Un
  constat de confiance basse est conservé et reçoit une réponse, mais il ne
  vous est présenté que si vous devez le trancher.
- **L'emplacement** est `path:line` ou `path:start-end` depuis la racine du
  projet pour le code, ou `<file>#<heading>` depuis le dossier du change
  pour le plan (`"design.md#Decision: Sessions live in the database"`).
  Mettez-le entre guillemets : YAML lit `: ` comme une nouvelle clé.
- **La preuve** est le texte cité, copié mot pour mot — des lignes entières
  ou le fragment qui compte. Un relecteur prouve ce qu'il avance.

Le format complet est dans [Formats de fichiers](../reference/file-formats.md#reviewmd).
Les titres, les clés et les valeurs du bloc restent en anglais : codev les
lit.

## Les revues isolées

En [mode isolé](../roles.md#revue-en-ligne-ou-isolée), le compte rendu dit
aussi comment l'étape s'est déroulée, et qui a relevé quoi :

````markdown
## code.review

Mode: isolated.
Summarized: src/generated/schema.rs — over the material limit.

### security

#### R-security-1 The session lookup concatenates the token

```yaml
severity: high
confidence: high
location: "src/auth/session.rs:42-44"
evidence: |
  let query = format!("SELECT * FROM sessions WHERE token = '{}'", token);
recommendation: Bind the token as a query parameter.
raised_by: [security, newcomer]
history:
  - status: open
    by: security
```

### newcomer

Fallback: the sub-agent was stopped twice by its time limit.

No finding.
````

- **`Mode:`** — `inline` ou `isolated`, sous le titre de l'étape.
- **`Summarized:`** — une ligne par fichier que les relecteurs ont reçu
  sous forme de résumé plutôt qu'en entier, parce que la matière
  dépassait sa limite.
- **`raised_by`** — lorsque plusieurs relecteurs ont trouvé le même
  défaut, le constat est écrit une seule fois, sous le rôle qui l'a jugé
  le plus grave, et liste chaque rôle qui l'a relevé, le sien en premier.
  C'est ce rôle qui revérifie les réponses. Un rôle dont tous les constats
  ont été fusionnés indique `No finding.` ; `codev review` le montre
  crédité sur le constat partagé.
- **`Fallback:`** — le sous-agent du rôle a échoué deux fois et le rôle a
  relu en ligne, dans la conversation.

`codev validate` signale un `Mode:` autre que `inline` ou `isolated`, une
deuxième ligne `Mode:` ou `Fallback:`, et un `raised_by` qui n'est pas une
liste d'identifiants de rôles distincts commençant par le rôle du constat
(`review_invalid`). Un codev antérieur à ces lignes les ignore, et signale
`raised_by` comme champ inconnu (`review_unknown_field`).

## La boucle de réponse

Un constat commence `open`. Ensuite :

1. **L'auteur répond.** Le rôle qui possède l'artefact cité à
   `plan.challenge` — product owner, analyste QA, architecte, tech lead —
   ou le développeur à `code.review` le corrige (`fixed`, avec `ref` : le
   commit ou la modification de fichier) ou le conteste (`disputed`, avec
   `note` : l'argument).
2. **Le relecteur vérifie la réponse, une fois.** Une correction est
   validée (`fixed` à nouveau, par le relecteur) ou rouverte (`open`, avec
   la raison). Une contestation est acceptée (`rejected`) ou maintenue
   (`disputed`, avec la raison).
3. **Ce qui reste vous revient.** Le résumé de la skill liste, sous **À
   décider**, chaque contestation que le relecteur a maintenue et chaque
   constat de gravité haute encore ouvert, avec les deux arguments. Vous le
   corrigez, en acceptez le risque (`accepted-risk`, avec une
   justification) ou le reportez (`deferred`, avec `to` : le change ou le
   ticket où il part). La skill consigne votre décision sous la forme
   `by: human:<votre nom>` ; elle ne décide jamais à votre place.

| Statut | Posé par | Exige |
|---|---|---|
| `open` | le relecteur (première entrée, ou correction qui ne tient pas) | `note` quand il rouvre une correction |
| `fixed` | l'auteur, puis le relecteur pour la valider | `ref` |
| `disputed` | l'auteur, ou le relecteur qui la maintient | `note` |
| `rejected` | le relecteur, ou une personne, après une contestation | — |
| `accepted-risk` | une personne | `note` : la justification |
| `deferred` | l'auteur, ou une personne pour un constat `high` | `to` |

`accepted-risk`, `deferred` et `rejected` sont définitifs : pour revenir
sur l'un d'eux, ouvrez un nouveau constat.

## Ce que codev vérifie

`codev validate` lit le compte rendu de chaque change actif. Il ne juge
jamais un constat — un constat `open` est l'état normal d'une revue — mais
il vérifie tout ce qui ne demande pas de jugement :

- **Le format** : les titres, un bloc `yaml` par constat, les champs, les
  identifiants, les emplacements (`review_invalid`), et un champ inconnu
  (`review_unknown_field`, un avertissement).
- **L'historique** : la première entrée est `open` par le rôle du constat,
  chaque transition figure dans le tableau ci-dessus (`review_invalid`), et
  chaque entrée porte ce que son statut exige, décidée par qui doit la
  décider (`review_decision_incomplete`).
- **La preuve**, pour les constats `open`, `disputed`, `accepted-risk` et
  `deferred`, face aux fichiers tels qu'ils sont maintenant : le fichier, la
  ligne ou le titre existe (`review_location_missing`) ; chaque ligne citée
  se retrouve dans des lignes consécutives du fichier, espaces et fins de
  ligne ignorés, à 5 lignes au plus des lignes citées
  (`review_evidence_not_found`, ou `review_evidence_moved` quand elle est
  plus loin). Un constat `fixed` ou `rejected` n'est pas comparé : son code
  devait changer. Un fichier qui n'est pas du texte UTF-8, contient des
  octets NUL ou dépasse 4 Mio n'est pas comparé
  (`review_evidence_unchecked`).
- **Les sections** : quand le compte rendu contient une étape, chaque rôle
  actif à cette étape pour le change a sa section (`review_role_missing`).
  Un rôle désactivé peut écrire `Disabled by configuration.`.

Aucun contrôle n'utilise git ni le réseau : ils lisent l'arbre de travail.

## codev review

```bash
codev review --change add-sessions
```

Affiche le compte rendu étape par étape — chaque rôle, chaque constat avec
sa gravité, son statut et son emplacement —, qui doit agir ensuite sur
chacun (`author`, `reviewer` ou `human`), les défauts du compte rendu, et
si la revue permet d'archiver le change. `--json` produit le document
[`ReviewV1`](../reference/json-output.md#review) que les skills lisent pour
reprendre une revue interrompue. La commande sort avec 1 seulement quand le
compte rendu contient une erreur.

## Archiver

`codev archive` vérifie la revue après la validation. Il refuse, avec le
code `review_blocked`, quand :

- des rôles étaient actifs à `plan.challenge` ou `code.review` et que le
  compte rendu n'existe pas (`review_record_missing`), ou n'a pas de section
  pour cette étape (`review_step_missing`) ;
- un constat `high` est encore `open` ou `disputed` (`review_unresolved`) ;
- le compte rendu vient d'un codev plus récent (`review_format_unsupported`).

Une étape sans rôle actif n'a pas besoin de section. `codev sync` ne
regarde pas le compte rendu.

## Les changes commencés avant le compte rendu

Un change planifié avec un codev antérieur n'a pas de compte rendu : son
archivage est donc refusé. Relancez les étapes de relecture —
`/codev-apply` relit le code et écrit le compte rendu — ou dites
explicitement que le change n'a pas été relu en désactivant ses rôles de
relecture dans son `change.yaml` :

```yaml
roles:
  skeptic: never
  newcomer: never
```

Une équipe qui n'utilise pas du tout les skills fait de même dans
`_codev/config.yaml`. Pour que tout le monde utilise un codev qui applique
le verrou, relevez [`codev_version`](../reference/configuration.md) à la
version qui l'apporte : un codev plus ancien ignore `review.md` et archive
quand même.

## Cas particuliers

- **Relancer une étape** complète le compte rendu ; un constat déjà
  consigné n'est pas relevé à nouveau, et une étape ou un rôle écrit deux
  fois est une erreur.
- **Une session interrompue** reprend à partir du compte rendu : `awaiting`
  dit qui doit répondre ensuite.
- **Un change de documentation seule, ou encore sans code**, a tout de
  même ses sections de relecture — `No finding.` est une réponse valide.
- **Les fichiers générés, vendorisés ou binaires** peuvent être cités ; les
  fichiers binaires et très volumineux ne sont pas comparés.
- **Windows** : `\` dans un emplacement se lit `/`, et les fins de ligne
  `\r\n` sont ignorées dans le compte rendu comme dans les fichiers cités.
- **Monorepos** : les chemins sont relatifs à la racine du projet codev ;
  un fichier hors de cette racine ne peut pas être cité.
