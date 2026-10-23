# Proposition : <titre du change>

## Pourquoi

<!-- Le problème ou l'opportunité, en une ou deux phrases. Pourquoi maintenant ? -->

## Changements

<!-- Liste à puces, précise sur les capacités ajoutées, modifiées ou supprimées.
     Marquez tout changement incompatible par **BREAKING**. -->

## Capacités

### Nouvelles capacités

<!-- Une ligne par capacité, sous la forme `chemin/de-la-capacite`. Chacune
     produit un fichier `specs/<chemin>/spec.md`. Laissez vide s'il n'y en a pas. -->

### Capacités modifiées

<!-- Une ligne par capacité dont les EXIGENCES changent, avec son chemin exact
     sous `_codev/specs/`. Laissez vide s'il n'y en a pas. -->

### Capacités supprimées

<!-- Une ligne par capacité que ce change supprime entièrement. Chaque entrée est
     le chemin exact sous `_codev/specs/`. Demande `retire_capabilities: true`
     dans `change.yaml` et une section `## Exigences SUPPRIMÉES` qui vide la
     spec. Sans ce marqueur, sync et archive refusent plutôt que d'effectuer une
     opération irréversible. Laissez vide s'il n'y en a pas. -->

## Impact

<!-- Code, API, dépendances et systèmes touchés. -->

## Domaines

<!-- Cochez chaque domaine que le change touche : son expert relit le plan et
     le code. Écrivez la seule ligne `Aucun` s'il n'en touche aucun. Les noms
     des domaines restent ceux de codev. -->

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Note de version

<!-- Ce que les utilisateurs liront dans le changelog. Une puce par élément :
     `- <Catégorie> : <une ou deux phrases>`, Catégorie parmi Ajouté, Modifié,
     Obsolète, Supprimé, Corrigé, Sécurité ; commencez le texte d'un élément
     incompatible par **BREAKING**. Un change purement interne écrit la seule
     ligne `Aucun`. -->
