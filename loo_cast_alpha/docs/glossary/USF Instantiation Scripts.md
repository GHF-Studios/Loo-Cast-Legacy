---
canonical_name: USF Instantiation Scripts
status: WIP-draft
aliases: []
---

The USF Instantiation Scripts are the declaration-centric authoring surface under [[Loo Cast]] for defining
USF-specific [[Capability Declaration]] / [[Capability Node]] material from the [[USF Contract]].
These declarations target Rust-registered host validation/materialization authorities.
Older notes call those authorities [[Capability Slot Type]]s; active platform wording now splits that pressure across
[[Capability Type]], [[Capability Trait]], [[Capability Extension Slot]], and signature metadata.
Executing these declaration entrypoints with type/trait/callback-tailored `ctx` capability-object subgraphs emits full
capability-node declaration material that includes structured data plus sanctioned callback closures.
Declaration entrypoint access and callback invocation access are distinct scopes (`decl_ctx` vs callback-scoped
`cb_ctx`) and resolve to different effective masks through allow/deny path-gating policy.
The `ctx` graph is hierarchical (atomic capability nodes + composite/category nodes) and filtered through
include/exclude path declarations for each declaration/projection policy.
Runtime activation then materializes active USF [[Capability Instance]]s into the [[USF Instance Graph]].
Capabilities are exposed to scripts as declaration-level [[Rhai Capability]] API objects through `ctx`; runtime behavior
is executed by the resulting Capability Instances.
Lifecycle, Rust/Rhai cyclic loop semantics, callback-path semantics, and capability multiplicity classes are
canonicalized in [[Capability]].
First-order declaration profiles are root-level and cannot depend on other capabilities.
In practice this makes scripts object descriptors first, with executable declaration behavior limited to callbacks.
They are effectively the closest thing to game-asset authoring in this project's architecture.
This surface is governed by the [[USF Definition Lifecycle]].

Implementation-facing notes:

- [USF Instantiation Capability Slot Notes](USF%20Instantiation%20Capability%20Slot%20Notes.md)
- [Capability Dependency Layer Notes](Capability%20Dependency%20Layer%20Notes.md)
- [USF Math Raw Model Foundation Notes](USF%20Math%20Raw%20Model%20Foundation%20Notes.md)
- [Rhai Generic Dispatch Policy Notes](Rhai%20Generic%20Dispatch%20Policy%20Notes.md)
- [Rhai Value Semantics and AccessCell Notes](Rhai%20Value%20Semantics%20and%20AccessCell%20Notes.md)

#glossary
