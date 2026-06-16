---
canonical_name: USF
status: WIP-draft
aliases: []
---

The USF is the flagship first-party public/API-facing simulation framework subsystem/module inside the
[[Spacetime Engine]], used by the [[Loo Cast]] Game.
At this layer, it is the high-level simulation premise that bridges scale-aware simulation semantics and
capability-oriented execution.
The contract-level definition is the [[USF Contract]], and the runtime implementation is the [[USF Runtime]] inside the
Spacetime Engine stack.

Current owner-answer-informed correction:
USF is not a standalone product pillar and is not directly/exclusively replaceable as a Vapor-level product or
[[Capability]].
It is best treated as a pivotal Spacetime Engine module/framework part.
Loo Cast uses USF through Spacetime Engine.
Replacing the engine product is a Vapor-level possibility; replacing USF by itself is not currently the same kind of
composition operation.
Replacing USF means forking/modifying the Spacetime Engine enough that the resulting thing is effectively another
[[Engine]].

Public subsystem boundary:
USF is closer to a public Spacetime Engine subsystem than a private implementation detail.
Its APIs and contracts may be public even though it is not a product.
The Loo Cast Game layer should depend on the Spacetime Engine/`core_mod` surface, which is heavily USF-backed.
In practice, USF is most of the Spacetime-level world/model API.

Module graph boundary:
USF is a first-party module in the Spacetime Engine module/capability graph.
The capability graph and module graph are expected to converge conceptually: capabilities can model small API surfaces,
modules, and whole module trees.

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)

#glossary
