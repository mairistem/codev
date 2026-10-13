# Proposal: Inline checkout validation

## Why

Customers submit the checkout form, then scroll back up to find what was wrong.

## What Changes

- Fields are validated as the customer leaves them; invalid fields are shown in red.
- The error list at the top of the form is removed.

## Capabilities

### New Capabilities

- `checkout-validation`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `web/src/checkout/Form.tsx`, `web/src/checkout/rules.ts`.

## Domains

- [ ] security
- [ ] personal-data
- [x] user-interface — changes how form errors are shown
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Changed: The checkout form shows invalid fields as you type.
