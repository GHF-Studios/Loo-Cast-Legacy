---
canonical_name: USF Runtime Evolution Lifecycle
status: WIP-draft
aliases: [ ]
---

The USF Runtime Evolution Lifecycle governs runtime progression after the definition lock transition.
Runtime interactions are driven by active [[Capability Instance]]s materialized from capabilities established at the
definition lock transition; these instances carry declaration data and sanctioned callback closures defined by
declaration scripts.
Execution flows through type/trait/callback-tailored `ctx` capability-object subgraphs that bound exposed capability
objects via hierarchical graph composition (atomic + composite nodes) and include/exclude path declarations.
When Rhai-declared callbacks are invoked in this phase, runtime enforces resolved effective callback `ctx` path masks
from allow/deny policy resolution.
Dynamic policy can narrow or re-open these callback paths per context, but only inside
the [[Capability Graph Scope Envelope]].
This runtime flow emits intents that runtime authorities reconcile, commit, and apply as state transitions.
This is the operational form of the [[Execution-Reconciliation Dual Core]].
ECS is substrate and execution medium in this flow, not capability type/trait/callback authority.
This lifecycle is entered after bootstrap fixed-point convergence and lock, not during unresolved bootstrap layering.
At this layer, lifecycle semantics are intentionally/unavoidably high-level and still underexplored in deeper
operational detail.

Implementation-facing notes:
[USF Runtime Evolution Lifecycle Notes](USF%20Runtime%20Evolution%20Lifecycle%20Notes.md)
and
[Runtime Intent Reconcile Commit Apply Mapping Notes](Runtime%20Intent%20Reconcile%20Commit%20Apply%20Mapping%20Notes.md)
Dependency-layer and callback-mask policy framing are defined in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

#glossary
