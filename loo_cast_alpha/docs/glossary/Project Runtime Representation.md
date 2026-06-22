---
canonical_name: Project Runtime Representation
status: WIP-draft
aliases: []
---

The Project Runtime Representation defines the active in-memory integrated form of the project at runtime.
It is the runtime-stage counterpart to [[Project Authoring Structure]] and [[Project Artifact Structure]].

At this stage, the runtime includes:

- active mod graph and resolved ownership mappings
- materialized capabilities and channels
- staged/runtime orchestration state
- active simulation/runtime state under the [[Runtime Substrate]]

Definition structure is fixed at [[Runtime Lock]].
Runtime evolution changes state and intent within that structure rather than mutating structure itself.
This is the project-level runtime shape of [[Closed Runtime and Open Design]].

At project scope, this representation includes [[Mod Runtime Representation]], the launched composition's resolved
capability graph/runtime machinery, [[USF Runtime]], and [[Modding Runtime]].

Implementation-facing note:
[Runtime Intent Reconcile Commit Apply Mapping Notes](Runtime%20Intent%20Reconcile%20Commit%20Apply%20Mapping%20Notes.md)

#glossary
