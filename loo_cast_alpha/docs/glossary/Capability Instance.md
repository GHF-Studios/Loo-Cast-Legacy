---
canonical_name: Capability Instance
status: WIP-draft
aliases:
  - Materialized Capability
  - Runtime Capability
---

A Capability Instance is a concrete validated/materialized occurrence of a [[Capability]] in a specific capability graph
environment.
Use this term when the text means the in-memory graph node or runtime/staged object, rather than the broad Capability
model, type/category, contract surface, or authored declaration.

Boundary:

- [[Capability Declaration]] is the authored pre-materialization payload.
- Capability Instance is the validated/materialized graph object produced from declarations, Rust host definitions, or
  other explicit capability sources.
- Plain [[Capability]] may still name the broad Vapor concept/model when instance-level precision is not needed.
- [[Capability Type]] names the declared kind/category behind an instance.
- [[Capability Trait]] names explicit contracts implemented by the instance's Capability Type.
- [[Capability Instance Signature]] describes graph-node identity and compatibility metadata for declared/staged/runtime
  instances.
- `Capability Type Template` is deprecated active vocabulary.

Startup timing:
Capability Instances can be created in staged/topological startup layers before the final [[Runtime Lock]].
Final Runtime Lock freezes the launchable composition's startup graph core; it is not the only moment where validated
declarations may become staged graph objects.

#glossary
