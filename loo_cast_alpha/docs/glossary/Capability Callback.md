---
canonical_name: Capability Callback
status: WIP-draft
aliases:
  - Capability Hook
---

A Capability Callback is a capability-relevant logic entrypoint that is scheduled, invoked, or mediated by its owning
callback policy rather than called freely like an ordinary function.

This term exists to avoid treating arbitrary functions as capability-surface concepts.
General functions may be private utilities, public/global utilities, Rust helpers, or Rhai helpers, but they are not
capability concepts merely because they are callable.

Core rules:

- A Capability Callback has a [[Callback Type]].
- A Capability Callback has or implies a [[Callback Signature]].
- A Capability Callback executes through a projected [[Callback Context Type]].
- [[Capability Trait]]s may declare trait callbacks, and [[Capability Type]]s may implement those callbacks.
- Runtime invocation is governed by scheduling/firing policy such as single-fire, multi-fire, procedural-fire, or later
  callback families.

Boundary:
Do not rename callbacks to generic capability functions in active doctrine.
If a future function-surface term exists, it must mean something clearly different from scheduled callbacks/hooks and
must not make capability modeling revolve around arbitrary function lists.

See also:

- [[Callback Type]]
- [[Callback Signature]]
- [[Callback Context Type]]
- [[Capability Trait]]
- [[Capability Type]]

#glossary
