---
canonical_name: USF
status: WIP-draft
aliases: []
source_of_truth: []
---

The USF is the flagship first-party public/API-facing simulation framework module inside the [[Spacetime Engine]], used
by the [[Loo Cast]] game.
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

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)

#glossary
