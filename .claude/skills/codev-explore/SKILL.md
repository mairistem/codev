---
name: codev-explore
description: "Défricher une idée, enquêter sur un problème ou clarifier un besoin avant de créer un change codev. À utiliser quand la demande est floue, qu'il faut comparer plusieurs approches, ou qu'on ne sait pas encore quoi construire. N'écrit aucun fichier."
allowed-tools: "Bash(codev:*), Read, Glob, Grep"
license: MIT
metadata:
  generator: codev
  version: "0.1.0"
---

Défricher une idée, enquêter sur un problème, clarifier un besoin — sans rien
engager.

**Aucune écriture.** Ce workflow ne crée ni change, ni artefact, ni code. Il ne
modifie aucun fichier. C'est un partenaire de réflexion, et son seul produit est
une compréhension partagée.

---

## Entrée

Un sujet, une question, ou rien du tout. Si l'utilisateur n'a rien précisé,
demande :

> Qu'est-ce que tu veux explorer ?

## Comment mener l'exploration

**Commence par ce que le dépôt sait déjà.** Ne demande pas à l'utilisateur ce
que le code peut répondre. Dans l'ordre :

```bash
codev list --specs          # les comportements déjà spécifiés
codev list                  # les changes en cours, pour ne pas doubler un travail
```

Puis lis les specs pertinentes, les décisions en vigueur dans
`_codev/decisions/`, et l'implémentation concernée.

**Pose les questions qui font vraiment avancer.** Une bonne question porte sur
une dépendance, une contrainte ou un arbitrage que le code ne peut pas trancher
— pas sur un fait que tu pouvais aller chercher. Quand tu poses une question,
recommande une réponse par défaut et dis pourquoi.

**Compare les options franchement.** Deux ou trois approches, chacune avec ce
qu'elle coûte et ce qu'elle ferme. Une recommandation, pas un catalogue. Si une
option est meilleure, dis-le.

**Confronte aux décisions existantes.** Si une piste s'écarte d'une décision
acceptée dans `_codev/decisions/`, signale-le tôt : soit la piste change, soit
c'est la décision qu'il faut proposer de remplacer. Ne re-débats pas en silence
un choix déjà tranché.

**Dessine quand c'est plus clair qu'un paragraphe.** Un schéma en art ASCII pour
un flux de données, une machine à états ou une topologie.

## Sortie

L'exploration se termine de l'une de ces trois façons, et tu dis laquelle :

1. **Ça se cristallise.** Résume en trois points : le problème, l'approche
   retenue, le périmètre. Puis propose : « Je peux créer le change — demande-moi
   `/codev-propose`. »
2. **Il manque une information.** Nomme précisément ce qui manque et qui ou quoi
   peut le fournir.
3. **Ça ne valait pas le détour.** Dis-le. Une exploration qui conclut « ce n'est
   pas un problème » ou « le code le fait déjà » est une exploration réussie.

## Garde-fous

- N'écris aucun fichier. Ne crée aucun change. Ne modifie aucun code.
- Ne présente pas une hypothèse comme un fait observé. Dis « je suppose » quand
  tu supposes.
- Si l'utilisateur te demande d'implémenter pendant l'exploration, ne le fais
  pas : dis-lui que tu passes par `/codev-propose` d'abord, et pourquoi.