---
canonical_name: Capability Contract Family
status: WIP-draft
aliases:
  - Capability Contract
  - Capability Contract Surface
---

The Capability Contract Family defines what Vapor-level capability contracts must declare and satisfy.
Older notes distinguished it from [[Capability Slot Type]] host template authority; current active wording splits that
pressure across [[Capability Type]], [[Capability Trait]], [[Capability Extension Slot]], and the signature family.
Capabilities and [[Capability Instance]]s implement this family at different layers: the broad model defines the
contract surface, while concrete instances satisfy it in a specific graph environment.
USF-aware capabilities may expose [[Scaled Capability Channel]]s as scale-specific execution faces whose
required/allowed shape is derived from the capability contract plus the USF scale contract.
In declaration scripts, capabilities appear as [[Rhai Capability]] API objects surfaced through
type/trait/callback-tailored `ctx` capability-object subgraphs.
The `ctx` graph is hierarchical (atomic capability nodes + composite/category nodes), with include/exclude path
declarations controlling exposed subgraphs.
Runtime executes behavior through [[Capability Instance]]s that bind these declared surfaces.

Dependency semantics are layered:

- provider dependencies resolve through mod/runtime ownership and key resolution
- declaration dependencies resolve through declaration-scoped `ctx` path access requirements

This [[Contract Family]] defines compatibility and boundary rules for capability implementations within the
[[Contract]] through [[Capability Path]], [[Capability Resolution Semantics]], trait/extension-slot rules, and
projection-governed APIs.
Runtime callback/API access can be dynamically narrowed/re-opened by policy, but remains bounded by the
[[Capability Graph Scope Envelope]].
USF scale compatibility declarations are defined by the [[Scale Contract]] through [[Scale Support]] over [[Scale]]
coordinates; this is a USF-specific compatibility layer, not the general shape of all capabilities.
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
Important signature families include [[Capability Type Signature]], [[Capability Trait Signature]],
[[Capability Instance Signature]], and [[Callback Signature]].

Split note:
This page is an overloaded umbrella and should split in a later structural rewrite.
Owner-approved split targets are metadata rules, declaration rules, projection rules, and runtime/instance rules.
Until that split happens, this page should be treated as a map of pressure areas, not one clean final contract object.

#glossary
