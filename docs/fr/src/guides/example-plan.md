# Un exemple de plan commenté

Cette page montre un plan complet, tel que Claude l'écrit avec Compagnon :
le mode sombre du [démarrage rapide](../quickstart.md), la modification
nommée `add-dark-mode`. L'application existe déjà et affiche toujours un
thème clair ; le plan lui fait suivre le réglage de l'ordinateur, et laisse
l'utilisateur choisir un thème à la main. Le plan se trouve dans
`_compagnon/changes/add-dark-mode/`, en quatre fichiers. Chaque partie
ci-dessous montre un fichier tel qu'il est écrit pour un projet en
français, puis ce que vous y vérifiez. [Relire un plan](review-a-plan.md)
explique chaque partie en général ; cette page fait de même sur un cas
réel.

## La proposition

La proposition, `proposal.md`, dit pourquoi le travail est nécessaire et ce
qu'il change.

```markdown
# Proposition : Ajouter un mode sombre

## Pourquoi

Les utilisateurs qui travaillent le soir trouvent le thème clair fatigant,
et plusieurs ont demandé un thème sombre. Leur ordinateur a déjà un réglage
clair ou sombre : l'application l'ignore.

## Changements

- L'application affiche le thème clair ou le thème sombre, selon le réglage
  de l'ordinateur, au lieu de toujours le thème clair.
- Un utilisateur peut choisir un thème à la main dans la page des réglages ;
  ce choix est gardé sur son appareil, même quand le réglage de
  l'ordinateur change.

## Capacités

### Nouvelles capacités

### Capacités modifiées

- appearance

### Capacités supprimées

## Impact

- Les couleurs des pages, la page des réglages.
- Rien ne change pour les comptes ni pour le serveur.

## Domaines

- [ ] security
- [ ] personal-data
- [x] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Note de version

- Ajouté : L'application suit le réglage clair ou sombre de votre ordinateur, et vous pouvez choisir un thème à la main dans les réglages.
```

### Ce qu'on vérifie ici

- **Pourquoi** : est-ce bien le problème des utilisateurs ? Ici, c'est la
  plainte de ceux qui travaillent le soir, avec leurs mots.
- **Changements** : tout ce que vous attendiez y est, rien de plus. Notez
  ce qui est laissé de côté : le choix est gardé sur l'appareil, pas dans
  le compte de l'utilisateur ; il ne le suit donc pas sur un autre
  ordinateur.
- **Domaines** : les cases cochées disent quels experts relisent aussi le
  plan ; ici, seulement celui de l'interface. Leurs noms restent en
  anglais.
- **Note de version** : la phrase que les utilisateurs liront à la sortie
  de la version. Lisez-la comme eux.

## Les spécifications

Les spécifications, ici `specs/appearance/spec.md` (le dossier « appearance », c'est-à-dire l'apparence), disent ce qui sera vrai
ensuite : une règle change, une règle s'ajoute.

```markdown
## Exigences MODIFIÉES

### Exigence : Thème de couleurs

L'application DOIT afficher ses pages dans le thème que demande le réglage de l'ordinateur, clair ou sombre, tant que l'utilisateur n'a pas choisi de thème à la main.

#### Scénario : Ordinateur en mode sombre

- **QUAND** un utilisateur qui n'a jamais choisi de thème ouvre l'application sur un ordinateur en mode sombre
- **ALORS** les pages s'affichent dans le thème sombre

#### Scénario : Ordinateur en mode clair

- **QUAND** un utilisateur qui n'a jamais choisi de thème ouvre l'application sur un ordinateur en mode clair
- **ALORS** les pages s'affichent dans le thème clair

## Exigences AJOUTÉES

### Exigence : Un thème choisi à la main est gardé

L'application DOIT garder le thème qu'un utilisateur choisit dans la page des réglages, clair ou sombre, même quand le réglage de l'ordinateur change.

#### Scénario : L'ordinateur passe en mode sombre

- **QUAND** un utilisateur qui a choisi le thème clair ouvre l'application sur un ordinateur en mode sombre
- **ALORS** les pages s'affichent dans le thème clair
- **ET** la page des réglages montre le thème clair comme choisi
```

### Ce qu'on vérifie ici

- **La règle modifiée** : « Thème de couleurs » existe déjà. Aujourd'hui,
  elle dit « L'application DOIT afficher ses pages dans le thème clair. »
  Le plan montre son nouveau texte en entier, qui remplace l'ancien :
  lisez les deux et vérifiez que la différence est exactement ce que vous
  avez demandé.
- **La règle ajoutée** : une règle nouvelle, pour l'utilisateur qui choisit
  un thème à la main.
- **Les scénarios** : chacun est un exemple que vous pouvez vous
  représenter. En manque-t-il un ? Ici, vous pourriez demander ce qui se
  passe quand l'utilisateur veut de nouveau suivre l'ordinateur : le plan
  ne le dit pas.
- **DOIT** veut dire que la règle est obligatoire ; **QUAND**, **ALORS**
  et **ET** se lisent comme en français courant. Chaque scénario devient
  un [test](../glossary.md#test).

## La conception

La conception, `design.md`, explique comment le travail sera fait.

```markdown
# Conception : Ajouter un mode sombre

## Contexte

Les pages prennent leurs couleurs dans une seule feuille de style, écrite
pour le thème clair. La page des réglages contient déjà la langue de
l'utilisateur.

## Objectifs / Hors objectifs

**Objectifs**

- Deux thèmes, clair et sombre, aussi lisibles l'un que l'autre.

**Hors objectifs**

- Garder le choix de l'utilisateur dans son compte, d'un ordinateur à
  l'autre.

## Décisions

### Décision : Des couleurs nommées, un jeu par thème

Chaque couleur de la feuille de style devient une valeur nommée (« fond »,
« texte », « lien »), avec un jeu pour le thème clair et un pour le thème
sombre. Le navigateur indique celui que demande le réglage de
l'ordinateur.

**Alternatives envisagées** : une seconde feuille de style, à part, pour le
thème sombre — deux fichiers à tenir à jour ensemble, et chaque nouvelle
page écrite deux fois.

### Décision : Le choix fait à la main est gardé dans le navigateur

Le thème choisi dans la page des réglages est gardé dans le navigateur de
l'appareil, et lu avant l'affichage de la page.

**Alternatives envisagées** : le garder dans le compte de l'utilisateur —
il le suivrait partout, mais il faudrait modifier le serveur ; remis à plus
tard.

## Risques / Compromis

- [La page s'affiche en clair un instant avant de passer en sombre] → le
  thème est appliqué avant que la page soit dessinée.
- [Certaines couleurs se lisent mal en thème sombre] → le contraste de
  chaque paire de couleurs de texte et de fond est vérifié.
```

### Ce qu'on vérifie ici

- Vous ne jugez pas les choix techniques. Lisez les **Risques** et les
  **Alternatives envisagées** : ils disent ce que les utilisateurs
  pourraient remarquer.
- Ici, la seconde décision veut dire que le choix ne suit pas un
  utilisateur d'un ordinateur à l'autre. Si cela compte pour vos
  utilisateurs, dites-le maintenant.
- Le premier risque est un éclair de couleur claire à l'ouverture de la
  page : vérifiez que la parade vous convient.

## Les tâches

Les tâches, `tasks.md`, listent les étapes du travail, chacune avec le test
qui prouve qu'elle est faite.

```markdown
# Tâches

## 1. Le thème suit l'ordinateur

- [ ] 1.1 Faire de chaque couleur de la feuille de style une valeur nommée, avec un jeu clair et un jeu sombre, vérifiée par les pages en clair restées identiques
- [ ] 1.2 Afficher le thème que demande le réglage de l'ordinateur, vérifiée par les scénarios « Ordinateur en mode sombre » et « Ordinateur en mode clair »
- [ ] 1.3 Vérifier le contraste de chaque paire de texte et de fond dans les deux thèmes, vérifiée par un contrôle de contraste sur chaque paire

## 2. Un thème choisi à la main

- [ ] 2.1 Ajouter un choix « Thème » à la page des réglages, vérifiée par le choix qui change aussitôt les pages
- [ ] 2.2 Garder le choix dans le navigateur et l'appliquer avant que la page soit dessinée, vérifiée par le scénario « L'ordinateur passe en mode sombre »
```

### Ce qu'on vérifie ici

- L'ordre est logique : d'abord les thèmes, puis le choix à la main.
- Chaque scénario des spécifications apparaît dans une tâche : chacun sera
  donc testé.
- Quelque chose que les utilisateurs voient reste-t-il sans test ? Ici,
  chaque étape visible nomme sa vérification.
