# Approuver un plan sur GitHub

Cette page s'adresse à vous si votre équipe travaille sur GitHub et
qu'on vous demande d'approuver un plan, sans Claude Code et sans
[terminal](../glossary.md#terminal). Tout se passe dans votre navigateur,
sur le site que votre équipe utilise déjà.

Lisez d'abord [Relire un plan](review-a-plan.md) : il dit ce qu'il faut
vérifier dans chaque partie d'un plan. [Un exemple de plan
commenté](example-plan.md) en montre un complet.

<figure class="diagram">
<div class="scroll">
<svg viewBox="0 0 760 230" role="img" aria-label="Approuver sur GitHub, la vie de la demande en cinq étapes : 1, le plan est envoyé dans une demande de fusion ; 2, vous commentez ses lignes, et Claude met le plan à jour jusqu'à ce qu'il vous convienne ; 3, vous approuvez la demande ; 4, Claude construit exactement ce que dit le plan ; 5, l'équipe relit le code.">
<defs><marker id="dg-approve-a-plan-arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" style="fill:var(--dg-line)"/></marker></defs>
<rect x="4" y="6" width="752" height="218" rx="14" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="1.5"/>
<text x="740" y="30" text-anchor="end" font-size="13" style="fill:var(--dg-muted)">Demande de fusion sur GitHub</text>
<path d="M226,102 C222,58 80,58 77,98" style="fill:none;stroke:var(--dg-you)" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#dg-approve-a-plan-arrow)"/>
<text x="152" y="58" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">Claude met le plan à jour</text>
<line x1="96" y1="120" x2="206" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="248" y1="120" x2="358" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="400" y1="120" x2="510" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<line x1="552" y1="120" x2="662" y2="120" style="stroke:var(--dg-line)" stroke-width="2" marker-end="url(#dg-approve-a-plan-arrow)"/>
<circle cx="76" cy="120" r="16" style="fill:var(--dg-claude)"/>
<text x="76" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-box)">1</text>
<text x="76" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Plan envoyé</text>
<text x="76" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">le développeur envoie</text>
<text x="76" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">le plan de Claude</text>
<circle cx="228" cy="120" r="16" style="fill:var(--dg-you)"/>
<text x="228" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-on-you)">2</text>
<text x="228" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Commentaires</text>
<text x="228" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">vous commentez</text>
<text x="228" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">les lignes du plan</text>
<circle cx="380" cy="120" r="16" style="fill:var(--dg-you)"/>
<text x="380" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-on-you)">3</text>
<text x="380" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Accord</text>
<text x="380" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">vous approuvez</text>
<text x="380" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">la demande</text>
<circle cx="532" cy="120" r="16" style="fill:var(--dg-claude)"/>
<text x="532" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-box)">4</text>
<text x="532" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Claude construit</text>
<text x="532" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">exactement ce que</text>
<text x="532" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">dit le plan</text>
<circle cx="684" cy="120" r="16" style="fill:var(--dg-box);stroke:var(--dg-line)" stroke-width="2"/>
<text x="684" y="125" text-anchor="middle" font-size="14" font-weight="600" style="fill:var(--dg-ink)">5</text>
<text x="684" y="162" text-anchor="middle" font-size="15" font-weight="600" style="fill:var(--dg-ink)">Code relu</text>
<text x="684" y="184" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">l'équipe relit le code,</text>
<text x="684" y="201" text-anchor="middle" font-size="12" style="fill:var(--dg-muted)">puis le fusionne</text>
</svg>
</div>
<figcaption>Approuver sur GitHub : le plan arrive dans une demande, vous commentez jusqu'à ce qu'il vous convienne, vous approuvez, et alors seulement Claude construit le code que l'équipe relit.</figcaption>
</figure>

## Où arrive le plan

Quand un développeur a fait écrire un plan par Claude, il l'envoie à
l'équipe sous forme de demande d'ajout au projet : une
[demande de fusion](../glossary.md#demande-de-fusion) (pull request, sur
GitHub). Vous recevez en général une
notification ou un courriel avec son lien.

Le plan se trouve parmi les fichiers modifiés de la
demande, dans un dossier : `_compagnon/changes/<nom>/`, où `<nom>` est le
nom court de la modification, par exemple `add-dark-mode`. Le dossier
contient les quatre parties du plan :

- `proposal.md` : la proposition, le pourquoi ;
- `specs/` : les spécifications, ce qui sera vrai ensuite ;
- `design.md` : la conception, le comment ;
- `tasks.md` : les tâches, les étapes.

La description de la demande peut aussi nommer ce
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

Si vous ne trouvez pas un bouton, cherchez l'action plutôt que le mot :
GitHub renomme ses boutons de temps en temps.

## Après vos commentaires

Vous ne modifiez pas le plan vous-même. Le développeur ouvre Claude Code
et demande à Claude de tenir compte de vos commentaires, par exemple :

> Tiens compte dans le plan des commentaires de la demande de fusion.

Claude met le plan à jour, et le développeur envoie la nouvelle version
dans la même demande. Relisez les parties modifiées, répondez à vos
commentaires, et approuvez quand le plan dit ce que vous voulez. Alors
seulement, Claude construit.

## Pour aller plus loin

- [Relire un plan](review-a-plan.md)
- [Un exemple de plan commenté](example-plan.md)
- [Compte rendu de relecture](review-record.md)
