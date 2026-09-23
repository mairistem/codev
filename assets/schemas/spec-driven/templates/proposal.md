# Proposal : <titre du change>

## Pourquoi

<!-- Le problème ou l'opportunité, en une ou deux phrases. Pourquoi maintenant ? -->

## Ce qui change

<!-- Liste à puces, précise sur les capacités ajoutées, modifiées ou retirées.
     Marque toute rupture de compatibilité par **RUPTURE**. -->

## Capacités

### Nouvelles capacités

<!-- Une ligne par capacité, au format `chemin/de-la-capacite`. Chacune donnera
     un fichier `specs/<chemin>/spec.md`. Laisse vide si aucune. -->

### Capacités modifiées

<!-- Une ligne par capacité dont les EXIGENCES changent, avec son chemin exact
     sous `_codev/specs/`. Laisse vide si aucune. -->

### Capacités retirées

<!-- Une ligne par capacité que ce change retire entièrement. Chaque entrée
     est le chemin exact sous `_codev/specs/`. Requiert `retire_capabilities:
     true` dans `change.yaml` et un `## REMOVED Requirements` qui vide la spec.
     Sans le marqueur, la sync/archive refusera plutôt que d'agir sur un
     geste irréversible. Laisse vide si aucune. -->

## Impact

<!-- Code, API, dépendances et systèmes affectés. -->
