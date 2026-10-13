# Design: Switch to Paylane

## Context

Card payments go through Stripe today, from `services/checkout`. Paylane
offers lower fees. Deployments are blue/green, about once a week.

## Decisions

### Decision: All traffic moves at deploy

The Stripe client is deleted and replaced by the Paylane client in the same
release. There is no switch between the two: keeping both would double the
code to maintain. The Stripe account is closed the day after the release.

**Alternatives considered**: a gradual rollout (more code).

### Decision: A worker settles payments nightly

A new worker, `settlement-worker`, runs at 02:00, fetches the day's Paylane
payments and matches them to orders. It runs as a Kubernetes CronJob and
writes its result to the database.

**Alternatives considered**: settling in the checkout request (too slow).
