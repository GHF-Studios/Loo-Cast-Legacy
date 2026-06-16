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
A capability intentionally spans runtime graph node, contract surface, API surface, authority surface, and composition
unit.

Current broader-ecosystem pressure:
Capabilities are the cross-layer contract substrate of the [[Vapor Ecosystem]], where engines, games, mods, modules,
sub-mods, and smaller API surfaces are in-memory ability/authority structures rather than only artifact hierarchy nodes.
The `modpack -> mod -> module -> member` structure describes authoring, packaging, launcher configuration, and
artifacts; it is not by itself the same thing as the active capability graph of the running program.
The active capability graph models runtime authority, API exposure, and artifact/composition structure.
In this sense, capabilities are ABI-like for Vapor: they are the common protocol surface used to discover, validate,
mount, project, and orchestrate heterogeneous runtime pieces.
Capabilities should carry Vapor-readable metadata in a shared format so the runtime can reason about what a node is,
what it can do, what it depends on, and what may depend on it.

Identity and visibility:

- Capabilities should not be anonymous.
- Private/internal capabilities are allowed, but they still need identity and metadata.
- Visibility should roughly follow Rust-like visibility semantics where useful, including private, `pub(super)`,
  `pub(crate)`, and public-style scopes.
- Large/umbrella capabilities may contain private subgraphs, but leaf-like capabilities should usually not hide
  subgraphs.

Type relationship:
A capability can itself be a type/category used by other capabilities, but a capability cannot be its own type.
Self-typing, self-dependency, and dependency cycles are invalid bootstrap shapes.
Non-circular type/dependency/dependent relationships are part of what forms the capability graph.

Graph shape:
The running `core_engine` process should have one global runtime capability graph.
Package, engine, game, mod, script, and user-facing views are projections or metadata views over that graph, not
separate authoritative runtime graphs.
The raw capability metadata may be simpler than the runtime graph, for example a registry scanned before graph
construction.

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

Rust/Rhai boundary:

- A capability can exist entirely in Rust with no Rhai declaration surface.
- [[Rhai Capability]] support is itself a capability.
- A capability should not exist purely as a Rhai declaration with no Rust host support beyond trivial script-local
  computation.
- Rhai may do simple local work, but low-level data access, heavy kernels, and runtime orchestration should remain
  Rust-backed.

Execution boundary:
Capabilities emit intents, relay requests, expose structured authority, and describe what is possible.
Canonical mutation authority belongs outside capability objects in the host-side execution/reconcile/commit/apply
pipeline.
This boundary is what allows Rhai callbacks to orchestrate through capabilities while Rust remains the normal
executor/kernel.
Leaf capabilities may directly bind Rust functions/types, including read-only or mutating operations, but canonical
state progression still runs through host-side reconciliation.

Composite capabilities:
Composite capabilities are first-class capability nodes.
They are not merely named views over primitive nodes.
A composite capability may own policy that its children do not directly know about, and may be implemented through
mechanisms rather than one native Rust function.

Examples:
Non-USF-specific capabilities may include logging/console output, configuration access, application startup, Rhai
support, event/message/hook surfaces, ECS integration, or standard-library-like script APIs.

USF boundary:
[[USF]] is a user of the Vapor capability model, not the foundation of that model.
USF concepts such as [[Scale]], [[Scale Realizer]], [[Phenomenon]], and [[Metric]] are capabilities, but their concrete
capability shapes may be higher-order/layer-dependent rather than single flat nodes.

Open pressure:
The exact edge taxonomy is still unresolved: dependency edges, slot edges, API exposure edges, and authority edges may
be distinct edge kinds or shared edges with policy metadata.
The exact relation between capability graph edges, slots, authority claims, registries, and integration apertures also
remains under active pressure.

See also:

- [[Capability Declaration]]
- [[Capability Profile]]
- [[Rhai Capability]]
- [[Scale Realizer Cardinality]]
- [[USF Instance Graph]]
- [[Capability Projection API]]
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)

#glossary
