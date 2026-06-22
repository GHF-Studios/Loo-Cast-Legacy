---
canonical_name: Capability Module
status: WIP-draft
aliases: []
---

A Capability Module is a typed source-layout unit for authoring capability graph material.
It is usually a folder with a [[Vapor.toml]], generated [[Vapor.lock]], and typed source subfolders.

Current owner direction:
A Capability Module may contain multiple [[Capability Node]]s.
It may contain dedicated source files for [[Capability Type]]s, [[Capability Trait]]s, and [[Capability Callback]]s.
[[Vapor.toml]] is the source of truth for classifying those files; folder and filename conventions make the layout
readable but should not be the only source of truth.

Reserved typed subfolders are expected to include at least:

- `types`
- `traits`
- `callbacks`
- `capabilities`

The `capabilities` folder is the recursive capability-node/module part of the source tree.
Other typed organization folders may exist later, but arbitrary folder names should not silently create capability
semantics.

Boundary:
Capability Module folder structure is graph-relevant as typed source organization and containment metadata.
It does not automatically mean inheritance, execution order, causality, or arbitrary logic.
Those relationships must be declared through types, traits, callbacks, extension slots, dependencies, or explicit
manifest metadata.

See also:

- [[Capability Node]]
- [[Capability Type]]
- [[Capability Trait]]
- [[Capability Callback]]
- [[Vapor.toml]]
- [[Capability Instance Signature]]

#glossary
