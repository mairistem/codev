## Objet

Ajoute la double authentification et retire l'ancien « Se souvenir de moi ».

## Exigences AJOUTÉES

### Exigence : Double authentification

Le portail DOIT proposer une double authentification par code TOTP.

#### Scénario : Activation

- **GIVEN** un agent sans double authentification
- **WHEN** il l'active dans ses réglages
- **THEN** un QR code s'affiche

## Exigences MODIFIÉES

### Exigence : Expiration de session

Les sessions DOIVENT expirer après 15 minutes d'inactivité.

#### Scénario : Inactivité

- **GIVEN** une session ouverte
- **WHEN** 15 minutes passent sans activité
- **THEN** la session est fermée

## Exigences SUPPRIMÉES

### Exigence : Se souvenir de moi

**Raison** : remplacé par la double authentification
**Migration** : les agents se reconnectent à chaque session

## Exigences RENOMMÉES

- DE : Expiration de session
- VERS : Durée de session
