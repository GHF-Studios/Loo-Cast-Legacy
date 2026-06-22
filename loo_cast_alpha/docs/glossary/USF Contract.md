---
canonical_name: USF Contract Family
status: WIP-draft
aliases:
  - USF Contract
---

The USF Contract Family defines foundational simulation principles and public/API-facing contract-level structure inside
the [[Spacetime Engine]].
It defines the USF scale system and related simulation structures.
In that system, [[Scale]] is the canonical semantic coordinate, [[Scale Definition]] declares what is meaningful at
that coordinate, [[Scale Support]] declares positive scale support for scale-aware semantic surfaces, and
[[Scale Realizer]] defines per-slice semantic realization behavior.
Scale declaration and compatibility rules are defined by the [[Scale Contract]].
The runtime counterpart is [[USF Runtime]].

Current owner-answer-informed boundary:
USF is not a Vapor-level product contract and should not be modeled as directly replaceable just because it has internal
contract structure.
It may interact with capability/runtime mechanisms, but it is not itself the same thing as the broader
[[Capability Contract]].
The USF Contract remains a valid term for the public/API-facing contract structure of this Spacetime subsystem.

Implementation-facing notes: [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)

#glossary
