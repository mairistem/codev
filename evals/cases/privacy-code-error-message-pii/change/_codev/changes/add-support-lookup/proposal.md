# Proposal: Customer lookup and chat for support

## Why

Agents ask customers to repeat who they are, and the chat widget greets everyone as "Hi there".

## What Changes

- Agents look a customer up by e-mail address.
- The Helply chat widget greets the signed-in customer by name, so that the agent knows who they talk to.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None.

### Removed Capabilities

None.

## Impact

- `src/support.ts` and `src/routes.ts` (new).

## Domains

- [ ] security
- [x] personal-data — customers' records are read and the chat vendor receives who is chatting
- [ ] user-interface
- [ ] performance
- [ ] operations
- [ ] migration

## Release note

- Added: Support agents find customers by e-mail, and the chat greets customers by name.
