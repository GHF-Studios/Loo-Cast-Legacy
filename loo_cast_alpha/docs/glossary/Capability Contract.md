---
canonical_name: Capability Contract Family
status: WIP-draft
aliases:
  - Capability Contract
  - Capability Contract Surface
---

The Capability Contract Family defines what Vapor-level capability contracts must declare and satisfy.
Older notes distinguished it from [[Capability Type Template]] host template authority; current active wording prefers
[[Capability Slot Type]] for that slot/context-shape concept.
Capabilities implement this family and expose [[Scaled Capability Channel]]s as scale-specific execution faces whose
required/allowed shape is derived from that family.
In declaration scripts, capabilities appear as [[Rhai Capability]] API objects surfaced through profile-tailored `ctx`
capability-object subgraphs.
The `ctx` graph is hierarchical (atomic capability nodes + composite/category nodes), with include/exclude path
declarations controlling exposed subgraphs.
Runtime executes behavior through materialized capabilities that bind these declared surfaces.

Dependency semantics are layered:

- provider dependencies resolve through mod/runtime ownership and key resolution
- declaration dependencies resolve through profile-scoped `ctx` path access requirements

This [[Contract Family]] defines compatibility and boundary rules for capability implementations within the
[[Contract]] through [[Capability Path]], [[Capability Resolution Semantics]], and slot-type/projection-governed APIs.
Runtime callback/API access can be dynamically narrowed/re-opened by policy, but remains bounded by the
[[Capability Graph Scope Envelope]].
Scale compatibility declarations are defined by the [[Scale Contract]] through [[Scale Support]] over [[Scale]]
coordinates.
Runtime orchestration and channel coordination are handled by [[Capability Runtime]].
Canonical dependency and seam rules are documented in
[Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md).

Boundary note:
USF concepts can be capabilities, but the [[USF]] is not the foundation of this contract family.
The capability model belongs to Vapor; USF is a major first-party user of it.

Metadata note:
Every capability needs enough Vapor-readable metadata for discovery, validation, visibility/projection, dependency
resolution, and diagnostics.
This metadata may be generated from Rust macros, supplied explicitly, or derived from declarations, but it must be
available to the capability runtime before lock.

#glossary
