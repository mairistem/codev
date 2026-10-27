# rendezvous

Service de prise de rendez-vous pour les cabinets médicaux : une API FastAPI
qui propose des créneaux et les réserve.

- `pytest` lance les tests de `tests/`, qui reprennent l'arborescence de
  `rendezvous/`.
- `ruff check .` vérifie le style.
- Les erreurs métier sont des exceptions de `rendezvous/errors.py`,
  traduites en réponses HTTP par un seul gestionnaire.
