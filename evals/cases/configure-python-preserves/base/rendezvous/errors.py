"""Les erreurs métier, traduites en HTTP par `app.handle_domain_error`."""


class DomainError(Exception):
    status = 400


class SlotTaken(DomainError):
    status = 409


class UnknownSlot(DomainError):
    status = 404
