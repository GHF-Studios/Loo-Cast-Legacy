---
canonical_name: Capability
status: WIP-draft
aliases:
   - API
source_of_truth: [ ]
---

The Capability is the canonical post-lock artifact in the runtime capability stack.
This note is the source of truth for capability lifecycle semantics, Rust/Rhai loop semantics, and capability
multiplicity classes.

Current broader-ecosystem pressure:
Capabilities are also emerging as the cross-layer contract substrate of the [[Vapor Ecosystem]], where engines, games,
mods, modules, and smaller API surfaces are in-memory ability/authority structures rather than only artifact hierarchy
nodes.
The `modpack -> mod -> module -> member` structure describes authoring, packaging, launcher configuration, and
artifacts; it is not by itself the same thing as the active capability graph of the running program.
This broader framing is owner-answer-informed but not fully formalized here yet.

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

Multiplicity classes currently include USF-associated examples, but those examples should not be read as making USF the
owner of the whole capability concept:

1. Slot-singleton capability types (`Scale`, `Scale Realizer`):
   exactly one effective capability per occupied scale slot (71-slot stack model).
2. Scale-collection capability types (`Phenomenon`, `Metric`):
   keyed, scale-bound capabilities; many per scale are allowed, with at least one required per scale.

See also:

- [[Capability Declaration]]
- [[Capability Profile]]
- [[Rhai Capability]]
- [[Scale Realizer Cardinality]]
- [[USF Instance Graph]]
- [[Capability Projection API]]
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)

#glossary
