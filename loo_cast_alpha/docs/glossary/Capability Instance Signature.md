---
canonical_name: Capability Instance Signature
status: WIP-draft
aliases: []
---

A Capability Instance Signature is the metadata/fingerprint-like description of a concrete declared, staged, or runtime
capability graph node.
It describes what a [[Capability Instance]] is expected to be without being the instance itself.

Likely contents:

- graph identity, name, and [[Capability Path]]
- source/module path information where relevant
- implemented [[Capability Trait]]s
- [[Capability Type Signature]] references
- declared or implemented [[Capability Callback]]s
- child or nested capability-node relationships
- fingerprint and diagnostic material

The signature may mirror folder or organization structure one-to-one when the source tree does that, but it must not
require that every graph node has a literal separate filesystem folder.

Boundary:
Capability Instance Signature is the graph-node-facing signature family.
It complements [[Capability Type Signature]], [[Capability Trait Signature]], and [[Callback Signature]].

See also:

- [[Capability Instance]]
- [[Capability Node]]
- [[Capability Module]]
- [[Capability Type Signature]]
- [[Capability Trait Signature]]
- [[Callback Signature]]

#glossary
