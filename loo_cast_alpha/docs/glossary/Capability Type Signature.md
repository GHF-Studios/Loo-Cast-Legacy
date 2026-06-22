---
canonical_name: Capability Type Signature
status: WIP-draft
aliases: []
---

A Capability Type Signature is the metadata/fingerprint-like description of a [[Capability Type]].
It describes the type surface without being the concrete type implementation or a runtime [[Capability Instance]].

Likely contents:

- stable type identity
- version or fingerprint material
- implemented [[Capability Trait]]s
- declared, implemented, used, or exposed [[Capability Callback]]s
- required projection/context surfaces
- compatibility metadata needed by [[Capability Extension Slot]] bounds

Boundary:
Capability Type Signature replaces the old "Capability Type Template" pressure together with
[[Capability Trait]], [[Capability Trait Signature]], and [[Capability Extension Slot]].
It is metadata used for validation, diagnostics, dependency compatibility, and lockfile/fingerprint generation.

See also:

- [[Capability Type]]
- [[Capability Trait]]
- [[Capability Trait Signature]]
- [[Capability Instance Signature]]
- [[Capability Extension Slot]]

#glossary
