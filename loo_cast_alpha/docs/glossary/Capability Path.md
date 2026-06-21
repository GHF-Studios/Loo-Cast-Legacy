---
canonical_name: Capability Path
status: WIP-draft
aliases:
  - Capability Location
---

A Capability Path identifies a nested route inside a capability/API graph or one projected capability/API facade.
Policy allow/deny decisions are evaluated against paths.
Path grammar remains intentionally high-level at this glossary layer and is still missing a finalized formal syntax.

Current pressure:
`Capability Location` may become the better name for an internal/resolved representation that combines capability path,
folder/project/pack placement, and storage integration metadata.
For now, keep [[Capability Path]] as the active stable addressing term and treat Capability Location as unresolved
implementation vocabulary.

Boundary:
A Capability Path or candidate Capability Location is an address/placement concept.
It does not by itself imply dependency, execution flow, causality, or default propagation from folder nesting.

See also:

- [[Capability Projection API]]
- [[Global Capability API Graph]]
- [[Capability Graph Scope Envelope]]

#glossary
