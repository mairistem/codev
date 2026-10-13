# ledger

The ledger service keeps accounts and orders.

## Running

    cargo run --release

The service reads `DATABASE_URL` from the environment.

## Rotating your API token

API tokens do not expire. Rotate a token when a person who knew it leaves
the team, or when it may have been exposed:

1. Create a new token from **Settings → API tokens**. It is shown once:
   copy it into your integration's secret store, never into source code.
2. Deploy the integration with the new token.
3. Revoke the old token from the same page. Requests that still carry it
   are refused with `401 Unauthorized`.
