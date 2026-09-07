---
canonical_name: Runtime Substrate
status: WIP-draft
aliases: []
---

The Runtime Substrate is the execution substrate provided by the [[Spacetime Engine]] for running scale-layered
simulation and capability-driven runtime behavior.
It is the execution medium for [[Capability-Centric Semantics]], not the authority-definition layer itself.
It composes with Vapor-level [[Capability Runtime]] semantics and hosts engine-side [[Modding Runtime]] and
[[USF Runtime]] execution where the selected Engine exposes those systems.
Rust-side staged lifecycle orchestration is handled through the [[Workflow Framework]].
ECS is the underlying execution/data medium in this substrate, while semantic capability type/trait/callback authority
remains contract-defined.
[[Runtime Lock]] enforces deterministic-by-default activation and fixed startup composition boundaries, while explicit
runtime substrate policy owns any permitted post-lock state evolution, registries, external IO, and nondeterministic
runtime effects.
In the current capability-kernel framing, the runtime substrate is where the mutable inner-inner graph lives: runtime
state, registries, queues, resource maps, ECS worlds, IO handles, and other dynamic structures that are allowed after
the module/kernel graph is locked.

Implementation-facing notes:

- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Reflection Macro Surface Notes](Rhai%20Reflection%20Macro%20Surface%20Notes.md)
- [USF Position Stack and Overflow Policy Notes](USF%20Position%20Stack%20and%20Overflow%20Policy%20Notes.md)
- [Workflow Framework Premise Notes](Workflow%20Framework%20Premise%20Notes.md)
- [[Capability Kernel]]
- [[Rust Surface Graph]]

#glossary
