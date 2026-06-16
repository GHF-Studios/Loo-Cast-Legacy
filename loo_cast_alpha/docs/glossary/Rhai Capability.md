---
canonical_name: Rhai Capability
status: WIP-draft
aliases: [ ]
---

The Rhai Capability is a declaration-level capability API object exposed to scripts through profile-tailored `ctx`
capability-object subgraphs.
It is dynamic/object-based and identified in human-readable terms for script ergonomics and policy gating.
It is a projected/contextual facade, not raw access to the unrestricted global host graph.

Rhai capabilities are declaration-surface semantics.
Runtime execution/orchestration semantics are carried by runtime-side Rust implementations under the
[[Capability Runtime]] and the [[Runtime Substrate]].
Rhai capability usage participates in the cyclic Rust/Rhai execution loop through callback invocation paths; it is not
an authoring-only surface.
Callback invocation access resolves to effective callback `ctx` path masks through allow/deny policy gating, rather
than implicit inheritance from declaration-entry access, and remains bounded by the [[Capability Graph Scope Envelope]].
Canonical loop/lifecycle/multiplicity semantics are defined in [[Capability]].

Current owner-answer-informed clarification:
Rhai is declaration-only in the sense that scripts define declarations, policies, parameters, and closures/callbacks
rather than owning scheduler structure or heavy runtime kernels.
This is intentionally not the same as saying scripts contain no behavior.
Declared callbacks and local policy logic are expected to be normal declaration material where the contract calls for it,
while Rust remains responsible for lifecycle scheduling, heavy execution kernels, state authority, and safety boundaries.
Callbacks are data-like declaration outputs that host runtime code calls at sanctioned times with sanctioned parameters.
Rhai declarations are startup/load-time outputs; runtime behavior re-enters Rhai only through host-scheduled callback
invocation paths.

Authoring boundary:
Rhai declarations may use structured procedural construction patterns such as builders.
This does not make Rhai the owner of runtime scheduling; it makes Rhai the authored declaration surface for constructing
typed capability payloads, callback profiles, and asset definitions.

Open pressure:
The exact meaning of "Rhai closures as normal declaration content" still needs a dedicated pass.

#glossary
