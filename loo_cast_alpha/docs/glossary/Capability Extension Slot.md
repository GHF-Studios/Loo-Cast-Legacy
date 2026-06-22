---
canonical_name: Capability Extension Slot
status: WIP-draft
aliases:
  - Extension Slot
  - Capability Dependency Slot
---

A Capability Extension Slot is an explicit capability graph extension/dependency point.
It describes what kinds of capability contributions may attach to, depend on, extend, or fill a specific position in the
graph.

Current owner direction:
An extension slot should define trait bounds over [[Capability Type Signature]]s.
A candidate [[Capability Instance]] is valid for the slot when its [[Capability Type]] satisfies the required
[[Capability Trait]] bounds and any additional slot policy.
Multiple trait bounds are allowed, with exact rule composition still open.

This replaces the active role previously carried by [[Capability Slot Type]] and older capability-type-template/profile
language.

Boundary:
An extension slot is graph-significant.
It is not the same thing as a callback type, callback context, folder placement rule, raw dependency declaration, or
projection scope envelope.
Cardinality, ordering, optionality, replacement behavior, and conflict behavior should be modeled as explicit slot
policy rather than baked into the trait identity itself.

See also:

- [[Capability Trait]]
- [[Capability Type Signature]]
- [[Capability Instance]]
- [[Capability Slot Type]]
- [[Modding Contract]]

#glossary
