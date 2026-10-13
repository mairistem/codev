# Proposal: Switch card payments to Paylane

## Why

Paylane's fees are 0.4 points lower than Stripe's.

## What Changes

- Card payments go through Paylane instead of Stripe.
- A nightly worker settles payments.

## Capabilities

### New Capabilities

- `payments`

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `services/checkout`, a new `settlement-worker`, Kubernetes manifests.

## Domains

- [ ] security
- [ ] personal-data
- [ ] user-interface
- [ ] performance
- [x] operations — a new provider, a new worker, a production switch
- [ ] migration

## Release note

- Changed: Card payments are processed by Paylane.
