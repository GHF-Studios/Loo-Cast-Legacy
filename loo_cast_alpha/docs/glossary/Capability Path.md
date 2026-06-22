---
canonical_name: Capability Path
status: WIP-draft
aliases: []
---

A Capability Path identifies a nested route inside a capability/API graph or one projected capability/API facade.
Policy allow/deny decisions are evaluated against paths.
Path grammar remains intentionally high-level at this glossary layer and is still missing a finalized formal syntax.

Current pressure:
`Capability Location` is no longer a preferred active term.
Keep [[Capability Path]] as the active stable addressing/policy term.
The valid underlying concern should be split into clearer concepts: [[Capability Module]] / [[Capability Node]] source
placement, artifact placement, folder or pack nesting, storage integration metadata, in-memory graph identity, and
path-based projection policy.

Boundary:
A Capability Path is an address/policy concept.
It does not by itself imply dependency, execution flow, causality, or default propagation from folder nesting.

See also:

- [[Capability Projection API]]
- [[Global Capability API Graph]]
- [[Capability Graph Scope Envelope]]
- [[Capability Module]]
- [[Capability Node]]
- [[Capability Instance Signature]]

#glossary
