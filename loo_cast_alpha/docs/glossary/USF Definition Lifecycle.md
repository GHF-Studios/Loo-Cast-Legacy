---
canonical_name: USF Definition Lifecycle
status: WIP-draft
aliases: []
---

The USF Definition Lifecycle establishes and validates singleton-like [[Capability Declaration]]s before runtime
activation.
This lifecycle includes iterative bootstrap/fixed-point passes rather than one linear single-pass initialization.
Declaration-side authoring/entrypoint semantics are carried by [[USF Instantiation Scripts]].
When declarations define callbacks, their callback access policy inputs must resolve to effective callback `ctx` path
masks before [[Runtime Lock]].
Iterative/topological startup promotes validated capability declarations into staged [[Capability Instance]]s before
final lock where needed.
Runtime activation executes active USF Capability Instances, for example Scale/Phenomenon instances.
Canonical capability lifecycle semantics and the Rust/Rhai loop model are defined in [[Capability]].
After the definition lock transition, runtime progression moves to the [[USF Runtime Evolution Lifecycle]]. Canonical
definition mutation is not part of the active runtime.
Definition-shape changes require explicit rebootstrap before a new activation and align with [[Runtime Lock]].

See also:

- [[Capability Bootstrap Fixed-Point Cycle]]
- [[Declaration Scope Envelope]]
- [[Callback Scope Envelope]]

#glossary
