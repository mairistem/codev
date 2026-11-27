# Compte rendu de relecture

Quand le contradicteur et les experts mettent un plan à l'épreuve, ou que le nouvel
arrivant et les experts relisent le code, ce qu'ils trouvent ne reste pas
dans la conversation. Tout va dans le compte rendu de relecture de la modification,
`_compagnon/changes/<name>/review.md` : chaque constat avec sa gravité, la
preuve sur laquelle il repose, et chaque réponse qu'il a reçue — qui l'a
corrigé, qui l'a contesté et pourquoi, qui en a accepté le risque. Compagnon
vérifie ce qui peut l'être, et `compagnon archive` ne clôt pas une modification
dont la relecture n'est pas tranchée.

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

- **L'identifiant**, `R-security-1`, est stable pendant toute la vie de la
  modification : l'identifiant du rôle et le prochain numéro libre. Il n'est
  jamais renuméroté ni réutilisé, si bien qu'une discussion peut le citer.
- **La gravité** dit à quel point c'est grave si le relecteur a raison, et
  **la confiance** à quel point il en est sûr — voir les échelles dans
  [Rôles](../roles.md#gravité-confiance-et-compte-rendu-de-relecture). Un
  constat de confiance basse est conservé et reçoit une réponse, mais il ne
  vous est présenté que si vous devez le trancher.
- **L'emplacement** est `path:line` ou `path:start-end` depuis la racine du
  projet pour le code, ou `<file>#<heading>` depuis le dossier de la
  modification pour le plan (`"design.md#Decision: Sessions live in the database"`).
  Mettez-le entre guillemets : YAML lit `: ` comme une nouvelle clé.
- **La preuve** est le texte cité, copié mot pour mot — des lignes entières
  ou le fragment qui compte. Un relecteur prouve ce qu'il avance.

Le format complet est dans [Formats de fichiers](../reference/file-formats.md#reviewmd).
Les titres, les clés et les valeurs du bloc restent en anglais : Compagnon les
lit.

## Les relectures isolées

En [mode isolé](../roles.md#relecture-en-ligne-ou-isolée), le compte rendu dit
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
  ont été fusionnés indique `No finding.` ; `compagnon review` le montre
  crédité sur le constat partagé.
- **`Fallback:`** — le sous-agent du rôle a échoué deux fois et le rôle a
  relu en ligne, dans la conversation.

`compagnon validate` signale un `Mode:` autre que `inline` ou `isolated`, une
deuxième ligne `Mode:` ou `Fallback:`, et un `raised_by` qui n'est pas une
liste d'identifiants de rôles distincts commençant par le rôle du constat
(`review_invalid`). Un Compagnon antérieur à ces lignes les ignore, et signale
`raised_by` comme champ inconnu (`review_unknown_field`).

## La boucle de réponse

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 326" role="img" aria-label="La vie d'un constat de relecture : le relecteur le soulève, open ; il devient ensuite fixed, quand l'auteur le corrige et que le relecteur vérifie ; disputed, quand l'auteur le conteste ; rejected, quand le relecteur accepte la contestation ; accepted-risk, décidé seulement par une personne avec une justification ; ou deferred, vers une autre modification ou un ticket. Chaque constat passe ensuite le verrou d'archivage : compagnon archive refuse tant qu'un constat high reste open ou disputed.">
<defs><marker id="dg-review-record-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="8" y="112" width="150" height="84" rx="12" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="83" y="142" text-anchor="middle" class="mono" font-size="17" font-weight="600" style="fill:var(--dg-ink)">open</text>
<text x="83" y="164" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">le relecteur</text>
<text x="83" y="182" text-anchor="middle" font-size="13" style="fill:var(--dg-muted)">le soulève</text>
<rect x="214" y="14" width="262" height="46" rx="10" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="228" y="33" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">fixed</text>
<text x="228" y="51" font-size="12" style="fill:var(--dg-muted)">l'auteur corrige, le relecteur vérifie</text>
<line x1="160" y1="154" x2="210" y2="37" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<line x1="478" y1="37" x2="556" y2="154" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<rect x="214" y="70" width="262" height="46" rx="10" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="228" y="89" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">disputed</text>
<text x="228" y="107" font-size="12" style="fill:var(--dg-muted)">l'auteur conteste, arguments à l'appui</text>
<line x1="160" y1="154" x2="210" y2="93" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<line x1="478" y1="93" x2="556" y2="154" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<rect x="214" y="126" width="262" height="46" rx="10" style="fill:var(--dg-box);stroke:var(--dg-claude)" stroke-width="2"/>
<text x="228" y="145" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">rejected</text>
<text x="228" y="163" font-size="12" style="fill:var(--dg-muted)">le relecteur accepte la contestation</text>
<line x1="160" y1="154" x2="210" y2="149" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<line x1="478" y1="149" x2="556" y2="154" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<rect x="214" y="182" width="262" height="46" rx="10" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<text x="228" y="201" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">accepted-risk</text>
<text x="228" y="219" font-size="12" style="fill:var(--dg-muted)">vous seul, avec une justification</text>
<line x1="160" y1="154" x2="210" y2="205" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<line x1="478" y1="205" x2="556" y2="154" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<rect x="214" y="238" width="262" height="46" rx="10" style="fill:var(--dg-box);stroke:var(--dg-you)" stroke-width="2"/>
<text x="228" y="257" class="mono" font-size="14" font-weight="600" style="fill:var(--dg-ink)">deferred</text>
<text x="228" y="275" font-size="12" style="fill:var(--dg-muted)">vers une autre modification, un ticket</text>
<line x1="160" y1="154" x2="210" y2="261" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<line x1="478" y1="261" x2="556" y2="154" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-review-record-arrow)"/>
<rect x="560" y="88" width="192" height="132" rx="12" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2" stroke-dasharray="6 4"/>
<text x="576" y="116" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Verrou d'archivage</text>
<text x="576" y="138" class="mono" font-size="12" style="fill:var(--dg-muted)">compagnon archive</text>
<text x="576" y="162" font-size="13" style="fill:var(--dg-muted)">refuse tant qu'un constat</text>
<text x="576" y="180" font-size="13" style="fill:var(--dg-muted)">high reste open</text>
<text x="576" y="198" font-size="13" style="fill:var(--dg-muted)">ou disputed</text>
<circle cx="20" cy="307" r="6" style="fill:var(--dg-claude)"/>
<text x="32" y="312" font-size="13" style="fill:var(--dg-muted)">les rôles de Claude répondent</text>
<circle cx="290" cy="307" r="6" style="fill:var(--dg-you)"/>
<text x="302" y="312" font-size="13" style="fill:var(--dg-muted)">seule une personne décide</text>
</svg>
</div>
<figcaption>La vie d'un constat de relecture : les rôles de Claude répondent à la plupart ; vous seul acceptez un risque, et l'archivage attend qu'aucun constat <code>high</code> ne reste ouvert ou contesté.</figcaption>
</figure>

Un constat commence à l'état `open`. Ensuite :

1. **L'auteur répond.** Le rôle qui possède l'artefact cité à
   `plan.challenge` — responsable produit, analyste qualité, architecte, référent technique —
   ou le développeur à `code.review` le corrige (`fixed`, avec `ref` : 
   l'enregistrement ou le changement de fichier) ou le conteste (`disputed`, avec
   `note` : l'argument).
2. **Le relecteur vérifie la réponse, une fois.** Une correction est
   validée (`fixed` à nouveau, par le relecteur) ou rouverte (`open`, avec
   la raison). Une contestation est acceptée (`rejected`) ou maintenue
   (`disputed`, avec la raison).
3. **Ce qui reste vous revient.** Le résumé de la compétence liste, sous **À
   décider**, chaque contestation que le relecteur a maintenue et chaque
   constat de gravité haute encore ouvert, avec les deux arguments. Vous le
   corrigez, en acceptez le risque (`accepted-risk`, avec une
   justification) ou le reportez (`deferred`, avec `to` : la modification ou
   le ticket où il part). La compétence consigne votre décision sous la forme
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

## Ce que Compagnon vérifie

`compagnon validate` lit le compte rendu de chaque modification active. Il ne juge
jamais un constat — un constat `open` est l'état normal d'une relecture — mais
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
  actif à cette étape pour la modification a sa section (`review_role_missing`).
  Un rôle désactivé peut écrire `Disabled by configuration.`.

Aucun contrôle n'utilise git ni le réseau : ils lisent l'arbre de travail.

## compagnon review

```bash
compagnon review --change add-sessions
```

Affiche le compte rendu étape par étape — chaque rôle, chaque constat avec
sa gravité, son statut et son emplacement —, qui doit agir ensuite sur
chacun (`author`, `reviewer` ou `human`), les défauts du compte rendu, et
si la relecture permet d'archiver la modification. Les compétences lisent
cette sortie telle qu'elle s'affiche pour reprendre une relecture interrompue et vérifier le
compte rendu ; `--json` produit le document
[`ReviewV1`](../reference/json-output.md#review), pour les scripts. La commande se termine avec le code 1 seulement quand le
compte rendu contient une erreur.

## Archiver

`compagnon archive` vérifie la relecture après la validation. Il refuse, avec le
code `review_blocked`, quand :

- des rôles étaient actifs à `plan.challenge` ou `code.review` et que le
  compte rendu n'existe pas (`review_record_missing`), ou n'a pas de section
  pour cette étape (`review_step_missing`) ;
- un constat `high` est encore `open` ou `disputed` (`review_unresolved`) ;
- le compte rendu vient d'un Compagnon plus récent (`review_format_unsupported`).

Une étape sans rôle actif n'a pas besoin de section. `compagnon sync` ne
regarde pas le compte rendu.

## Les modifications commencées avant le compte rendu

Une modification planifiée avec un Compagnon antérieur n'a pas de compte rendu : son
archivage est donc refusé. Relancez les étapes de relecture —
`/compagnon-apply` relit le code et écrit le compte rendu — ou dites
explicitement que la modification n'a pas été relue en désactivant ses rôles de
relecture dans son `change.yaml` :

```yaml
roles:
  skeptic: never
  newcomer: never
```

Une équipe qui n'utilise pas du tout les compétences fait de même dans
`_compagnon/config.yaml`. Pour que tout le monde utilise un Compagnon qui applique
le verrou, relevez [`compagnon_version`](../reference/configuration.md) à la
version qui l'apporte : un Compagnon plus ancien ignore `review.md` et archive
quand même.

## Cas particuliers

- **Relancer une étape** complète le compte rendu ; un constat déjà
  consigné n'est pas relevé à nouveau, et une étape ou un rôle écrit deux
  fois est une erreur.
- **Une session interrompue** reprend à partir du compte rendu : `awaiting`
  dit qui doit répondre ensuite.
- **Une modification de documentation seule, ou encore sans code**, a tout de
  même ses sections de relecture — `No finding.` est une réponse valide.
- **Les fichiers générés, embarqués depuis un tiers (vendored) ou binaires** peuvent être cités ; les
  fichiers binaires et très volumineux ne sont pas comparés.
- **Windows** : `\` dans un emplacement se lit `/`, et les fins de ligne
  `\r\n` sont ignorées dans le compte rendu comme dans les fichiers cités.
- **Dépôts multiprojets (monorepos)** : les chemins sont relatifs à la racine du projet Compagnon ;
  un fichier hors de cette racine ne peut pas être cité.

## Pour aller plus loin

- [Rôles](../roles.md)
- [Formats de fichiers](../reference/file-formats.md)
- [Interface en ligne de commande](../reference/cli.md)

## Étape suivante

[Adopter Compagnon sur un projet existant](existing-project.md)
