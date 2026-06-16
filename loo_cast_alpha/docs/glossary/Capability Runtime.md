---
canonical_name: Capability Runtime
status: WIP-draft
aliases: []
---

The Capability Runtime is the runtime orchestration layer for capabilities.
In the first-party stack, the concrete runtime lives inside the [[Spacetime Engine]], but the concept is rooted in the
[[Vapor Ecosystem]] capability model.
It handles dynamic discovery, registration, coordination, and execution routing for capability implementations.
The current direction is one global runtime capability graph inside the running `core_engine` process, with package,
engine, game, mod, Rhai, and user-facing views expressed as projections over that graph.
The graph should be built/validated layer by layer so dependencies are registered and initialized before dependants are
allowed to use them.
Declaration scripts consume [[Rhai Capability]] objects through profile-tailored `ctx` capability-object subgraphs;
runtime materialized capability instances execute closure logic against runtime capability implementations.
`ctx` capability-object subgraphs are composed from hierarchical API graph nodes (atomic + composite) via
include/exclude path declarations and can dynamically narrow/re-open by runtime policy inside the
[[Capability Graph Scope Envelope]].
These projected subgraphs are concrete [[Capability Projection API]] instances rather than raw global-graph access.
Callback invocation enforces resolved effective callback `ctx` path masks (allow/deny policy outcome), not implicit
inheritance from declaration-entrypoint access.
Capability implementations expose [[Scaled Capability Channel]] structures as per-scale execution paths for that
runtime execution.
The runtime realizes contracts defined by the [[Capability Contract]] and coordinates
with [[Observer-Relative Simulation]].
Canonical lifecycle, Rust/Rhai loop semantics, callback-path semantics, and multiplicity classes are defined in
[[Capability]].

Execution boundary:
The runtime may execute through capabilities, but canonical state mutation is decided by host-side
execution/reconcile/commit/apply paths rather than by capability objects executing themselves.

Invalid graph shapes:
Dependency cycles indicate a bootstrap paradox and should hard-fail.
The root node is the only special bootstrap case and should not be treated as a normal cycle.

Implementation-facing notes:

- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)

#glossary
