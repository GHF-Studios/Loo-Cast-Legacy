---
canonical_name: Capability Runtime
status: WIP-draft
aliases: []
---

The Capability Runtime is the runtime orchestration layer for capabilities.
It is a [[Vapor Ecosystem]]-level concept.
It is part of the broader [[Capability Framework]].
Concrete launched Engine/Game compositions embed or adapt capability-runtime machinery, and the first-party
[[Spacetime Engine]] uses and extends that machinery rather than owning the concept.
It handles dynamic discovery, registration, coordination, and execution routing for capability implementations.
The central runtime manager should live in or be coordinated by `vapor_core`, with concrete engines instantiating or
initializing it.
The current launch-runtime direction is one resolved capability graph inside the launched Engine/Game composition, with
[[Packagepack]], [[Enginepack]], [[Gamepack]], [[Modpack]], engine, game, mod, Rhai, and user-facing views expressed as
projections over that graph.
Within a launched `core_engine` process, that resolved runtime graph should be the main authority source; raw metadata
registries, lockfile/fingerprint material, and player/modpack-author/developer projections are supporting views rather
than separate runtime graph truths.
The graph should be built/validated layer by layer so dependencies are registered and initialized before dependants are
allowed to use them.
It should also coordinate the [[Rust Surface Graph]] of [[Scriptable Rust Surface]] entries and loaded
[[Kernel Artifact]] registrations.
Declaration scripts consume [[Rhai Capability]] objects through type/trait/callback-tailored `ctx` capability-object
subgraphs; runtime [[Capability Instance]]s invoke sanctioned callback logic against runtime capability
implementations.
`ctx` capability-object subgraphs are composed from hierarchical API graph nodes (atomic + composite) via
include/exclude path declarations and can dynamically narrow/re-open by runtime policy inside the
[[Capability Graph Scope Envelope]].
These projected subgraphs are concrete [[Capability Projection API]] instances rather than raw global-graph access.
Callback invocation enforces resolved effective callback `ctx` path masks (allow/deny policy outcome), not implicit
carry-over from declaration-entrypoint access.
USF-aware capability implementations may expose [[Scaled Capability Channel]] structures as scale-scoped execution paths
for that runtime execution.
The runtime realizes contracts defined by the [[Capability Contract]] and coordinates
with [[Observer-Relative Simulation]].
Canonical lifecycle, Rust/Rhai loop semantics, callback-path semantics, and multiplicity classes are defined in
[[Capability]].

Execution boundary:
The runtime may execute through capabilities, but canonical state mutation is decided by host-side
execution/reconcile/commit/apply paths rather than by capability objects executing themselves.
Capability Runtime owns capability-level registration and binding policy; concrete scheduling may be delegated to
[[Runtime Substrate]], [[Workflow Framework]], Bevy systems, or engine systems.

Staging boundary:
For Phase 3 planning, capability construction is staged as artifact discovery, user/modpack projection, shallow metadata
pre-validation, dependency/capability expansion, deep validation, [[Runtime Lock]] establishment, then locked runtime
graph execution.
This staging is about the launchable Engine/Game runtime composition.

Graph environment boundary:
The same capability graph data structures can be used by multiple environments, but these are different graph instances:
one per SDK tool instance, one for the launcher, one launcher-time-constructed proto-graph/root/seed for the selected
composition, and one resolved graph for the launched composition.
Launcher and SDK authoring environments currently lean static-only/read-only/hardcoded; they should not become
Rhai-authored dynamic runtime compositions by default.
Launchable compositions can still expose APIs that add mutable substrate onto the immutable startup-generated graph
core.
If a Vapor product or pack does not expose those APIs, it simply does not support that class of dynamic extension.

Runtime graph layering:
The locked module/package graph and loaded kernel-registration graph are static or lock-governed inputs.
True runtime dynamism lives in explicit mutable runtime substrate/registry/state graphs, not arbitrary post-lock
rewriting of source/module/kernel identity.

Invalid graph shapes:
Dependency cycles indicate a bootstrap paradox and should hard-fail.
The root node is the only special bootstrap case and should not be treated as a normal cycle.

Phase 3 lock-candidate anchor:
Capability Runtime Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W03 and the Runtime Flow.
Phase 3 must build, validate, lock, and hand off a launchable composition graph before the selected Engine/Game fixture
runs.

Implementation-facing notes:

- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [USF Contract Runtime Boundary Notes](USF%20Contract%20Runtime%20Boundary%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [[Rust Surface Graph]]
- [[Capability Kernel]]

#glossary
