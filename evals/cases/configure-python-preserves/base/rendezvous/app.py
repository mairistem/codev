from fastapi import FastAPI, Request
from fastapi.responses import JSONResponse

from .errors import DomainError
from .slots import Agenda

app = FastAPI()
agenda = Agenda(free={"2026-11-02T09:00", "2026-11-02T09:30"})


@app.exception_handler(DomainError)
async def handle_domain_error(_: Request, exc: DomainError) -> JSONResponse:
    # Un seul endroit traduit les erreurs métier en HTTP.
    return JSONResponse({"error": type(exc).__name__}, status_code=exc.status)


@app.post("/slots/{slot}/book")
def book(slot: str, patient: str) -> dict[str, str]:
    agenda.book(slot, patient)
    return {"slot": slot, "patient": patient}
