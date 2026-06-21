---
canonical_name: Callback Type
status: WIP-draft
aliases:
  - callback type
---

A Callback Type is the current working term for a typed callback function shape.
It is roughly the Vapor/Rhai-side equivalent of a custom function-pointer or closure signature, but should remain
framed through Vapor callback semantics rather than raw language mechanics.

Current owner-answer-informed framing:

- A Callback Type defines what kind of callback logic entrypoint is being declared.
- It can be single-fire, multi-fire, procedural-fire, or another firing category when the host surface defines that.
- It has a dedicated [[Callback Context Type]].
- It is not the same thing as a [[Capability Slot Type]].

Boundary:
Callback Type is one of the active callback-side terms.
The exact host representation is not locked.

See also:

- [[Callback Context Type]]
- [[Callback Signature]]
- [[Callback Scope Envelope]]
- [[Rhai Asset]]

#glossary
