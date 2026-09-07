---
canonical_name: USF Runtime
status: WIP-draft
aliases: []
---

The USF Runtime is the Spacetime Engine runtime implementation of the [[USF Contract]] family.
It realizes the active USF module/framework behavior inside the Spacetime Engine stack and composes with Vapor-level
[[Capability Runtime]] semantics and [[Modding Runtime]] machinery where the selected engine architecture requires it.
Its Rust-side staged lifecycle orchestration can be implemented through the [[Workflow Framework]].
It invokes declaration-surface entrypoints with type/trait/callback-tailored `ctx` capability-object subgraphs during
activation, then executes the resulting [[Capability Instance]]s.
Those Capability Instances are derived from declarations and Rust host definitions established during staged startup and
final lock.
When invoking Rhai-declared callbacks, runtime uses resolved effective callback `ctx` path masks from allow/deny
policy resolution.
These subgraphs come from hierarchical API graph composition (atomic + composite nodes) with include/exclude
path declarations and runtime open/close behavior.
It executes contract-defined USF behavior rather than defining the contract itself.
Canonical capability lifecycle semantics, Rust/Rhai cyclic loop semantics, callback-path semantics, and multiplicity
classes are defined in [[Capability]].

Boundary note:
`USF Runtime` remains a valid term for the runtime side of the public/API-facing USF subsystem even though USF is not a
standalone product.

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)
- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)

#glossary
