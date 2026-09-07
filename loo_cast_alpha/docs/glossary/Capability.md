---
canonical_name: Capability
status: WIP-draft
aliases:
   - API
---

The Capability is the core [[Vapor Ecosystem]]-level runtime/contract graph primitive for describing usable abilities,
authority surfaces, API exposure, composition structure, and orchestration seams across ecosystem, [[Engine]], [[Game]],
mod, and sub-mod layers.
Capabilities are organized by the [[Capability Framework]].
Capabilities are defined by Vapor and used by Vapor itself, engines, games, mods, and sub-mods.
A capability intentionally spans runtime graph node, contract surface, API surface, authority surface, metadata unit,
composition unit, and orchestration seam.
This breadth is deliberate, not accidental terminology overload.
Capabilities are the foundational Vapor substrate: engines, games, mods, packagepacks, SDK operations, launcher
operations, auth, publishing, installing, validation, fingerprinting, and launch surfaces may all be represented through
capability-shaped metadata and runtime/API surfaces.

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
- Capabilities are stable path-addressable objects by default.
- Visibility should roughly follow Rust-like visibility semantics where useful, including private, `pub(super)`,
  `pub(crate)`, and public-style scopes.
- Internal/private nodes are real full-graph nodes, not merely nodes hidden from user-facing projection.
- Visibility restrictions apply to all graph users: another full-graph node cannot touch an internal/private node unless
  the visibility policy permits it.
- Large/umbrella capabilities may contain private subgraphs, but leaf-like capabilities should usually not hide
  subgraphs.

Type/trait/source relationship:
Use [[Capability Type]] for a declared capability kind/category.
Use [[Capability Trait]] for a Rust-trait-like dynamic contract that Capability Types can implement.
Use [[Capability Extension Slot]] for an explicit graph extension/dependency point whose validity is expressed through
trait bounds and slot policy.
Use [[Capability Instance]] when the text means one concrete validated/materialized graph object in a specific graph
environment.
Use plain Capability for the broad Vapor concept/model, contract surface, type/trait/instance family, or when
instance-level precision is not needed.
Authored/pre-materialization forms should be called [[Capability Declaration]] or [[Capability Node]] where that
distinction matters.
`Capability Type Template` is deprecated active vocabulary.
The old profile/type-template pressure is now split across Capability Type, Capability Trait,
[[Capability Type Signature]], [[Capability Trait Signature]], [[Capability Instance Signature]], and Capability
Extension Slot.
Self-typing, self-dependency, and dependency cycles are invalid bootstrap shapes.
Non-circular type/dependency/dependent relationships are part of what forms the capability graph.

Graph shape:
Capability graph data structures are reused across multiple environments, not one monolithic process graph.
The SDK may have one graph per tool instance, the launcher has its own graph, the launcher can construct a proto-graph
root/seed for a selected composition, and the launched composition has its resolved runtime graph.
Because many validation, projection, diagnostic, authoring, and runtime surfaces route through these graph structures,
implementation should be concurrency- and multithread-friendly by design.
The exact structure is not locked, but likely pressure includes read-heavy snapshots, staged mutation, explicit
handoff/lock boundaries, and avoiding one global mutable bottleneck.
Packagepack, enginepack, gamepack, modpack, engine, game, mod, script, and user-facing views are projections or metadata
views over the relevant graph environment, not separate authoring truths.
The raw capability metadata may be simpler than the runtime graph, for example a registry scanned before graph
construction.
Metadata registries, lockfile/fingerprint material, and projections are support views over graph construction and
diagnostics; they are allowed to differ from the active runtime graph and should not be mistaken for the same object.
Launcher/SDK graphs are currently static-only/read-only/hardcoded environments.
Launchable Engine/Game compositions can expose mutable substrate layered onto the immutable startup-generated graph
core; products/packs that do not expose such APIs simply do not support that kind of runtime extension.

Staged construction:

1. Discover the artifact graph.
2. Build the user/modpack-author projection.
3. Run shallow metadata pre-validation over direct dependency/conflict-style composition metadata.
4. Expand the deeper dependency/capability graph.
5. Run deep validation against the full graph.
6. Establish the [[Runtime Lock]].
7. Enter the locked runtime graph representation.

`Capability Declaration` is the raw declared form of a [[Capability Node]], not one mandatory single-file object.
During iterative/topological startup, validated capability-node material may be promoted into staged
[[Capability Instance]]s before final Runtime Lock.
For Phase 3, Runtime Lock applies to launchable Engine/Game runtime composition, not to treating the launcher/SDK as
dynamic Rhai-authored runtime compositions.
Launcher and SDK capability environments are separate graph instances that currently lean static-only, read-only, and
hardcoded except where explicit SDK/launcher capability surfaces are implemented.

Capability flow across Rust/Rhai is cyclic, not one-way:
This is phase-separated runtime: declaration phase and execution phase coexist in one runtime but remain distinct.

1. Rust registers [[Rust Host Contract]]s, [[Scriptable Rust Surface]]s, projected API graph surfaces, and known
   type/trait/callback signature families into the [[Rust Surface Graph]].
2. [[Vapor.toml]] classifies typed source files inside [[Capability Module]] / [[Capability Node]] trees.
3. [[Rhai Asset]]s contribute data assets such as config/localization or declaration material for [[Capability Type]]s,
   [[Capability Trait]]s, [[Capability Callback]]s, [[Capability Node]]s, and [[Capability Module]]s.
4. Declaration payload contains structured data and metadata; declaration-level logic exists only as declared callbacks
   shaped by explicit type, trait, callback, extension-slot, and host contracts.
5. Rust validates and materializes declared node material into staged or runtime [[Capability Instance]]s.
6. Runtime binds Capability Instances to allowed [[Capability Kernel]] and host-runtime surfaces where native backing is
   needed.
7. Runtime executes Capability Instances, invoking Rhai callbacks through projected `ctx` handles.
8. Callback outcomes feed back into Rust-side reconcile/commit/apply paths.

Callback invocation paths are what restore script control flow freedom, but only through typed, scoped,
lifetime-bounded interfaces.
Declaration entrypoint context and callback invocation context are distinct policy surfaces and can expose different
effective capability-path masks after allow/deny resolution.
These runtime masks can only narrow/re-open inside the [[Capability Graph Scope Envelope]]; they cannot widen
profile scope.
Any attempted access outside the resolved effective `ctx` path mask is invalid and should hard-fail.

Access is asymmetric inside that cycle:
Rhai consumes projected handles and declaration surfaces, while Rust-owned runtime systems own host contracts,
orchestration, native kernel binding, state authority, and policy gating.
Dependency-layer and seam-layer separation rules are canonicalized in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

Rust/Rhai boundary:

- A capability can exist entirely in Rust with no Rhai declaration surface.
- [[Rhai Capability]] support is itself a capability.
- Native or hardcoded Rust capabilities must still be projectable into Rhai `ctx` surfaces when the relevant profile and
  visibility policy expose them.
- A capability should not exist purely as a Rhai declaration with no Rust host support beyond trivial script-local
  computation.
- Rhai assets are primarily data/declaration carriers.
- Rhai data assets include typed data, config, localization, constants, tuning values, authored tables, and similar
  structured inputs.
- Rhai declaration assets may carry data, metadata, graph structure, type/trait/callback declarations, and extension
  declarations.
- Rhai callbacks are first-class declaration content, but they are the sanctioned logic surface: callback behavior is
  host-invoked and policy mediated rather than arbitrary script-owned execution.
- Rhai may do simple local construction work, but low-level data access, native integration, simulation kernels, IO, and
  runtime orchestration should remain Rust-backed through [[Rust Host Contract]]s and [[Capability Kernel]]s.
- The distinction between Rhai-side capability usage and Rust-side kernel/host-contract usage is important enough for
  continued audit.

Execution boundary:
Capabilities emit intents, relay requests, expose structured authority, and describe what is possible.
Canonical mutation authority belongs outside capability objects in the host-side execution/reconcile/commit/apply
pipeline.
This boundary is what allows Rhai callbacks to orchestrate through capabilities while Rust remains the normal native
execution and authority layer.
Leaf capabilities may directly bind Rust functions/types, including read-only or mutating operations, but canonical
state progression still runs through host-side reconciliation.

Composite capabilities:
Composite capabilities are first-class capability nodes.
They are not merely named views over primitive nodes.
A composite capability may own policy that its children do not directly know about, and may be implemented through
mechanisms rather than one native Rust function.

Examples:
Engine/game-independent capabilities may include logging/console output, configuration access, application startup,
[[Rhai]] support, auth, publish, install, validate, fingerprint, launch, event/message/hook surfaces, or
standard-library-like script APIs.
SDK commands, launcher commands, and Steam integration surfaces should be capabilities, even when they are lightweight
Rust wrappers over Steam APIs or operating-system command execution.
CLI commands should generally map one-to-one to capabilities.
When a command appears broad, the corresponding capability may subdivide itself internally rather than making the
command/capability relation meaningless.

USF boundary:
[[USF]] is a user of the Vapor capability model, not the foundation of that model.
USF concepts such as [[Scale]], [[Scale Realizer]], [[Phenomenon]], and [[Metric]] are capabilities, but their concrete
capability shapes may be higher-order/layer-dependent rather than single flat nodes.

Open pressure:
Owner direction now favors a heterogeneous graph with heterogeneous edge kinds.
[[Capability Extension Slot]]s should be treated as graph edge types or edge-type-like declarations.
The exact edge taxonomy is still unresolved, but dependency edges, slot edges, API exposure edges, authority edges,
registry edges, and projection edges should not be prematurely collapsed into one generic edge model.
The exact relation between graph edge types, Capability Types, Capability Traits, extension slots, authority claims,
registries, and integration apertures remains under active pressure.

Phase 3 lock-candidate anchor:
Capability Phase 3 behavior is anchored by
[Phase 3 Vapor Execution Spec](../RFCS/phase_3_vapor_execution_spec.md), especially P3-W03.
Phase 3 must prove stable capability paths, visibility projections, staged graph construction, graph validation,
Runtime Lock for launched compositions, and diagnostics before any Engine/Game fixture is allowed to run.

See also:

- [[Capability Declaration]]
- [[Capability Framework]]
- [[Capability Node]]
- [[Capability Module]]
- [[Capability Type]]
- [[Capability Trait]]
- [[Capability Extension Slot]]
- [[Capability Instance]]
- [[Capability Type Signature]]
- [[Capability Trait Signature]]
- [[Capability Instance Signature]]
- [[Capability Callback]]
- [[Rust Host Contract]]
- [[Scriptable Rust Surface]]
- [[Rust Surface Graph]]
- [[Capability Kernel]]
- [[Capability Slot Type]]
- [[Callback Type]]
- [[Callback Context Type]]
- [[Callback Signature]]
- [[Rhai Capability]]
- [[Scale Realizer Cardinality]]
- [[USF Instance Graph]]
- [[Capability Projection API]]
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)

#glossary
