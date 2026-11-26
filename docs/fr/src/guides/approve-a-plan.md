# Approuver un plan sur GitHub ou GitLab

Cette page s'adresse à vous si votre équipe travaille sur GitHub ou GitLab
et qu'on vous demande d'approuver un plan, sans Claude Code et sans
[terminal](../glossary.md#terminal). Tout se passe dans votre navigateur,
sur le site que votre équipe utilise déjà.

Lisez d'abord [Relire un plan](review-a-plan.md) : il dit ce qu'il faut
vérifier dans chaque partie d'un plan. [Un exemple de plan
commenté](example-plan.md) en montre un complet.

## Où arrive le plan

Quand un développeur a fait écrire un plan par Claude, il l'envoie à
l'équipe sous forme de demande d'ajout au projet : une
[demande de fusion](../glossary.md#demande-de-fusion) (pull request sur
GitHub, merge request sur GitLab). Vous recevez en général une
notification ou un courriel avec son lien.

Sur les deux sites, le plan se trouve parmi les fichiers modifiés de la
demande, dans un dossier : `_compagnon/changes/<nom>/`, où `<nom>` est le
nom court de la modification, par exemple `add-dark-mode`. Le dossier
contient les quatre parties du plan :

- `proposal.md` : la proposition, le pourquoi ;
- `specs/` : les spécifications, ce qui sera vrai ensuite ;
- `design.md` : la conception, le comment ;
- `tasks.md` : les tâches, les étapes.

Sur GitHub seulement, la description de la demande peut aussi nommer ce
dossier, dans sa section « compagnon », à la ligne `- compagnon change:`,
si le développeur l'a remplie.

## Où sont les points à discuter

Avant que vous voyiez le plan, les relecteurs de Claude l'ont déjà mis à
l'épreuve. Ce qu'ils ont trouvé, et la réponse donnée à chaque point, est
écrit dans `review.md`, dans le même dossier. Commencez par là : vous
voyez ce qui reste ouvert, et quelles parties du plan lire de plus près.

## Sur GitHub

GitHub affiche ses boutons en anglais.

1. **Ouvrir les fichiers modifiés.** Dans la demande de fusion, cliquez
   sur l'onglet des fichiers modifiés (onglet « Files changed »).
   Cherchez les fichiers sous `_compagnon/changes/<nom>/`.
2. **Commenter une ligne.** Passez la souris sur la ligne, puis cliquez
   sur le « + » bleu qui apparaît à côté. Écrivez votre commentaire, puis
   commencez une relecture pour regrouper plusieurs commentaires et les
   envoyer ensemble (bouton « Start a review »), ou envoyez ce seul
   commentaire tout de suite (bouton « Add single comment »). Sans souris,
   allez jusqu'à la ligne avec la touche Tab : le même bouton de
   commentaire apparaît, et un lecteur d'écran l'annonce comme un bouton
   pour commenter. Vous pouvez aussi écrire vos commentaires dans l'onglet
   de la discussion (onglet « Conversation »), en nommant le fichier et la
   ligne.
3. **Envoyer votre réponse.** Ouvrez la fenêtre de réponse (bouton
   « Review changes », en haut à droite des fichiers modifiés ; la
   nouvelle page de GitHub peut l'appeler « Submit review »). Écrivez un
   court résumé, puis choisissez :
   - pour demander des corrections : demander des modifications (choix
     « Request changes ») ;
   - pour valider : approuver (choix « Approve ») ;
   - pour commenter sans trancher : commenter (choix « Comment »).

   Puis envoyez (bouton « Submit review »).

## Sur GitLab

GitLab affiche ses boutons dans votre langue. Les libellés sont donnés
ici en anglais, puis en français.

1. **Ouvrir les fichiers modifiés.** Dans la demande de fusion, cliquez
   sur l'onglet des fichiers modifiés (onglet « Changes » ou
   « Modifications »). Cherchez les fichiers sous
   `_compagnon/changes/<nom>/`.
2. **Commenter une ligne.** Passez la souris sur le numéro de la ligne,
   puis cliquez sur la bulle qui apparaît (sans souris, allez jusqu'à la
   ligne avec la touche Tab : un lecteur d'écran annonce le même bouton
   de commentaire ; ou écrivez dans l'onglet de vue d'ensemble, onglet
   « Overview » ou « Vue d'ensemble », en nommant le fichier et la ligne). Écrivez votre commentaire,
   puis commencez une relecture pour regrouper plusieurs commentaires
   (bouton « Start a review » ou « Commencer une revue »), ou envoyez ce
   seul commentaire tout de suite (bouton « Add comment now » ou
   « Ajouter un commentaire maintenant »).
3. **Demander des corrections.** Si vous avez commencé une relecture,
   terminez-la (bouton « Finish review » ou « Submit review », en
   français « Terminer la revue » ou « Soumettre la revue »), puis
   demandez des modifications (choix « Request changes » ou « Demander
   des modifications ») si votre GitLab le propose. Sinon, vos
   commentaires suffisent : dites clairement que le plan doit changer
   avant votre accord.
4. **Approuver.** Dans l'onglet de présentation de la demande (onglet
   « Overview » ou « Vue d'ensemble »), approuvez (bouton « Approve » ou
   « Approuver »). Votre accord est consigné sur la demande.

Si vous ne trouvez pas un bouton, cherchez l'action plutôt que le mot :
les deux sites renomment leurs boutons de temps en temps.

## Après vos commentaires

Vous ne modifiez pas le plan vous-même. Le développeur ouvre Claude Code
et demande à Claude de tenir compte de vos commentaires, par exemple :

> Tiens compte dans le plan des commentaires de la demande de fusion.

Claude met le plan à jour, et le développeur envoie la nouvelle version
dans la même demande. Relisez les parties modifiées, répondez à vos
commentaires, et approuvez quand le plan dit ce que vous voulez. Alors
seulement, Claude construit.
