---
canonical_name: Callback Type
status: WIP-draft
aliases:
  - callback type
  - Capability Callback Type
---

A Callback Type is the type of a [[Capability Callback]].
It describes a scheduled/mediated callback entrypoint, not an arbitrary capability function.

Current owner-answer-informed framing:

- A Callback Type defines what kind of callback logic entrypoint is being declared.
- It can be single-fire, multi-fire, procedural-fire, or another firing category when the host surface defines that.
- It has a dedicated [[Callback Context Type]].
- It has or implies a [[Callback Signature]].
- [[Capability Trait]]s may declare trait callbacks, and [[Capability Type]]s may implement them.
- It is not the same thing as a [[Capability Extension Slot]] or legacy [[Capability Slot Type]].

Boundary:
Callback Type is one of the active callback-side terms.
The exact host representation is not locked.
Do not use generic function wording as a synonym for callback; callback scheduling and policy mediation are the reason
this is capability-relevant.

See also:

- [[Capability Callback]]
- [[Callback Context Type]]
- [[Callback Signature]]
- [[Callback Scope Envelope]]
- [[Capability Trait]]
- [[Rhai Asset]]

#glossary
