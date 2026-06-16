---
canonical_name: Capability
status: WIP-draft
aliases:
   - API
---

The Capability is the core [[Vapor Ecosystem]]-level runtime/contract graph primitive for describing usable abilities,
authority surfaces, API exposure, composition structure, and orchestration seams across ecosystem, [[Engine]], [[Game]],
mod, and sub-mod layers.
Capabilities are defined by Vapor and used by Vapor itself, engines, games, mods, and sub-mods.

Current broader-ecosystem pressure:
Capabilities are the cross-layer contract substrate of the [[Vapor Ecosystem]], where engines, games, mods, modules,
sub-mods, and smaller API surfaces are in-memory ability/authority structures rather than only artifact hierarchy nodes.
The `modpack -> mod -> module -> member` structure describes authoring, packaging, launcher configuration, and
artifacts; it is not by itself the same thing as the active capability graph of the running program.
The active capability graph models runtime authority, API exposure, and artifact/composition structure.
In this sense, capabilities are ABI-like for Vapor: they are the common protocol surface used to discover, validate,
mount, project, and orchestrate heterogeneous runtime pieces.

`Capability Declaration` is the pre-lock artifact.
At the definition lock transition, validated capability declarations are promoted into capabilities.

Capability flow across Rust/Rhai is cyclic, not one-way:
This is phase-separated runtime: declaration phase and execution phase coexist in one runtime but remain distinct.

1. Rust registers host templates and projected API graph surfaces.
2. Rhai declaration entrypoints run with profile-scoped `ctx` and emit one capability declaration.
3. Declaration payload includes structured data plus declared behavior callbacks/closures shaped by contract/profile.
4. Rust validates and lock-transitions that declaration into a capability.
5. Runtime materializes and executes capability instances, invoking Rhai callbacks through projected `ctx` handles.
6. Callback outcomes feed back into Rust-side reconcile/commit/apply paths.

Callback invocation paths are what restore script control flow freedom, but only through typed, scoped,
lifetime-bounded interfaces.
Declaration entrypoint context and callback invocation context are distinct policy surfaces and can expose different
effective capability-path masks after allow/deny resolution.
These runtime masks can only narrow/re-open inside the [[Capability Graph Scope Envelope]]; they cannot widen
profile scope.
Any attempted access outside the resolved effective `ctx` path mask is invalid and should hard-fail.

Access is asymmetric inside that cycle:
Rhai consumes projected handles and declaration surfaces, while Rust owns orchestration, state authority, and policy
gating.
Dependency-layer and seam-layer separation rules are canonicalized in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

Execution boundary:
Capabilities relay requests, expose structured authority, and describe what is possible.
They do not run themselves.
Canonical mutation authority belongs outside capability objects in the host-side execution/reconcile/commit/apply
pipeline.
This boundary is what allows Rhai callbacks to orchestrate through capabilities while Rust remains the normal
executor/kernel.

USF boundary:
[[USF]] is a user of the Vapor capability model, not the foundation of that model.
USF concepts such as [[Scale]], [[Scale Realizer]], [[Phenomenon]], and [[Metric]] are capabilities, but their concrete
capability shapes may be higher-order/layer-dependent rather than single flat nodes.

Open pressure:
The boundary between capability graph nodes, declarations, public API surfaces, runtime authority, and artifact/package
structure still needs a dedicated pass.
The exact relation between capability graph edges, slots, authority claims, registries, and integration apertures is
still under active pressure.

See also:

- [[Capability Declaration]]
- [[Capability Profile]]
- [[Rhai Capability]]
- [[Scale Realizer Cardinality]]
- [[USF Instance Graph]]
- [[Capability Projection API]]
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)

#glossary
