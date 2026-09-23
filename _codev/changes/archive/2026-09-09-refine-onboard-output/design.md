# Design : affiner la sortie de `/codev-onboard`

## Contexte

Voir `proposal.md`. Change trivial de rédaction du body markdown de la
skill ; aucune structure de code n'est touchée.

## Décisions

### Décision : la ligne « changes archivés » est **conditionnelle**

Sur un projet fraîchement initialisé, `_codev/changes/archive/` peut
être vide (ou contenir seulement `.gitkeep`). Afficher « 0 change
archivé » serait du bruit — on n'affiche la ligne **que si le compte
est strictement positif**. Cohérent avec le principe de sortie
compacte de la skill.

### Décision : la lecture est faite via un `ls` filtré, pas un CLI codev

`codev list --archived` n'existe pas (E6 dans la roadmap, non livré).
La skill compte donc les dossiers directement via une commande shell
équivalente à `ls _codev/changes/archive/ | grep -v '^\.'`. C'est
robuste : filtre les fichiers cachés (`.gitkeep`), les autres dossiers
de la forme `<date>-<nom>/`. Reportable à la livraison de E6 —
substitution alors triviale.

### Décision : la recommandation cite `README.md` inconditionnellement

Le coût d'une suggestion « lis `README.md` » sur un projet sans README
est négligeable — l'utilisateur essaie, ne trouve pas, passe à autre
chose. Détecter la présence du fichier avant de suggérer ajouterait un
`Glob` supplémentaire pour une valeur nulle. **Alternative écartée** :
détection conditionnelle. Rejeté au titre de la simplicité.

## Plan de migration

Aucune. Le nouveau comportement s'applique dès le prochain `codev
update` + redémarrage de la session Claude Code.
