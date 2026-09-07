---
canonical_name: Rhai Capability
status: WIP-draft
aliases: [ ]
---

The Rhai Capability is a declaration-level capability API object exposed to scripts through type/trait/callback-tailored
`ctx` capability-object subgraphs.
It is dynamic/object-based and identified in human-readable terms for script ergonomics and policy gating.
It is a projected/contextual facade, not raw access to the unrestricted global host graph.
[[Rhai]] support itself should be treated as a capability.

Rhai capabilities are declaration-surface semantics.
Runtime execution/orchestration semantics are carried by runtime-side Rust implementations under the
[[Capability Runtime]] and the [[Runtime Substrate]].
Rhai capability usage participates in the cyclic Rust/Rhai execution loop through callback invocation paths; it is not
an authoring-only surface.
Callback invocation access resolves to effective callback `ctx` path masks through allow/deny policy gating, rather
than implicit carry-over from declaration-entry access, and remains bounded by the [[Capability Graph Scope Envelope]].
Canonical loop/lifecycle/multiplicity semantics are defined in [[Capability]].

Current owner-answer-informed clarification:
Rhai is declaration-only in the sense that scripts define data, declarations, policies, parameters, and callbacks rather
than owning scheduler structure, native integration, or [[Capability Kernel]] execution.
This is intentionally not the same as saying scripts contain no behavior.
Declared callbacks are the sanctioned declaration-level logic surface where the contract calls for behavior.
Policy material should usually be data/rules consumed by host contracts; Rust-owned host/runtime systems remain
responsible for lifecycle scheduling, native execution, state authority, and safety boundaries.
Callbacks are data-like declaration outputs that host runtime code calls at sanctioned times with sanctioned parameters.
Rhai declarations are startup/load-time outputs; runtime behavior re-enters Rhai only through host-scheduled callback
invocation paths.

Authoring boundary:
Rhai declarations may use structured procedural construction patterns such as builders.
This does not make Rhai the owner of runtime scheduling; it makes Rhai the authored declaration/data surface for
constructing typed capability payloads, capability types, capability traits, callback types, callback context types,
callback signatures, config, localization, and asset definitions.
Config, localization, tuning values, and authored tables are data assets; capability declarations may include data, but
their logic boundary is still callbacks.

Projection boundary:
Declaration-entry `ctx` and callback `ctx` should be modeled as projections over the same global capability graph.
The runtime likely needs a generalized way to create selective projections of specific subgraphs.

Callback boundary:
[[Capability Callback]]s are scheduled/mediated typed attachment points.
Whether a callback is considered part of its declaring capability or a linked callback capability remains unresolved.
Phase 3 should include a focused callback proof through startup/logging/output behavior, but the wording
`typed hook/callback capability slot` is not yet stable enough to lock.

Path/visibility policy:
Path-mask and allow/deny behavior should be driven by capability metadata.
A capability should know whether the Rust code it contains can be public, is intended to be public, and at what
visibility scope.

Open pressure:
The exact meaning of "Rhai closures as normal declaration content" still needs a dedicated pass.
The exact relationship between callback types, extension slots, traits, and capabilities also needs a dedicated pass.
The Rhai-side capability usage vs Rust-side [[Capability Kernel]] and [[Rust Host Contract]] usage split also needs
continued audit.

Phase 3 lock-candidate anchor:
Rhai Capability Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W04.
Phase 3 must prove declaration `ctx` projection, invalid declaration diagnostics, and a focused startup/logging/output
callback flow while keeping gameplay-style Rhai callbacks out of scope.

#glossary
