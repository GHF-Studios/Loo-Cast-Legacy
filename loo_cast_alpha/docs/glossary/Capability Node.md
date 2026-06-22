---
canonical_name: Capability Node
status: WIP-draft
aliases:
  - Declared Capability Node
---

A Capability Node is the declared/source-side graph node that can later become a staged or runtime
[[Capability Instance]].
It is the current best replacement for thinking of a [[Capability Declaration]] as one large object or one Rhai file.

In source form, a Capability Node is usually represented by a folder or module-shaped source subtree with typed files
and manifest metadata.
It may define or reference [[Capability Type]]s, [[Capability Trait]]s, [[Capability Callback]]s, child capability nodes,
and extension-slot relationships.

Relationship to Capability Declaration:
[[Capability Declaration]] is the raw declared form of a Capability Node.
The declaration may be spread across a typed folder hierarchy rather than one file.
Validation/materialization turns that declared node material into one or more [[Capability Instance]]s, depending on the
final graph rules.

Boundary:
A Capability Node is source/declaration graph structure.
It is not automatically a runtime object until staged startup validation/materialization creates a
[[Capability Instance]].
Folder hierarchy can contribute source/declaration graph edges, but execution, inheritance, dependency, and authority
semantics must be explicit.

See also:

- [[Capability Module]]
- [[Capability Declaration]]
- [[Capability Instance]]
- [[Capability Instance Signature]]
- [[Capability Type]]
- [[Capability Trait]]
- [[Capability Callback]]

#glossary
