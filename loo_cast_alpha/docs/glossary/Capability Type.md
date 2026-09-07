---
canonical_name: Capability Type
status: WIP-draft
aliases:
  - Capability Kind
---

A Capability Type is the declared kind/category of capability that can later produce or classify
[[Capability Instance]]s.
It is not a runtime object and it is not a template.

In the current owner direction, a Capability Type is defined by a dedicated typed source file inside a
[[Capability Module]] or [[Capability Node]] source tree.
The file is classified by [[Vapor.toml]], usually alongside sibling typed files for [[Capability Trait]]s and
[[Capability Callback]]s.

Core rules:

- A Capability Type may implement one or more [[Capability Trait]]s.
- A Capability Type may declare, implement, use, or expose [[Capability Callback]]s.
- A Capability Type is summarized by a [[Capability Type Signature]] for validation, compatibility, fingerprinting, and
  diagnostics.
- A Capability Type should not be confused with [[Capability Instance]], which is the staged/runtime materialized graph
  object.
- `Capability Type Template` is deprecated active vocabulary; use Capability Type, Capability Trait,
  [[Capability Type Signature]], and [[Capability Extension Slot]] instead.

Boundary:
Capability Types are capability-relevant abstractions.
General functions are not capability-relevant merely because code exists; internal/private utilities and public/global
utilities should not be promoted into capability model concepts unless they participate in capability typing,
trait bounds, callback scheduling, projection, or graph validation.

See also:

- [[Capability Trait]]
- [[Capability Type Signature]]
- [[Capability Callback]]
- [[Capability Module]]
- [[Capability Node]]
- [[Capability Instance]]

#glossary
