# Spécification Authentification

## Objet

Authentification des agents du portail : connexion et expiration des sessions.

## Exigences

### Exigence : Authentification des agents

Le portail DOIT authentifier un agent par son identifiant et son mot de passe.

#### Scénario : Identifiants valides

- **GIVEN** un agent avec un compte actif
- **WHEN** il saisit des identifiants valides
- **THEN** il accède au portail

#### Scénario : Identifiants invalides

- **GIVEN** un agent avec un compte actif
- **WHEN** il saisit un mot de passe erroné
- **THEN** le portail refuse la connexion

### Exigence : Expiration de session

Les sessions DOIVENT expirer après 30 minutes d'inactivité.

#### Scénario : Inactivité

- **GIVEN** une session ouverte
- **WHEN** 30 minutes passent sans activité
- **THEN** la session est fermée
