---
canonical_name: Capability Projection API
status: WIP-draft
aliases: []
---

A Capability Projection API is a contextual facade projected from a capability/API graph into one script-safe or
callback-safe execution context.
The projected object is handle-based and lightweight for Rhai usage, and is usually deeply nested/composite.
`decl_ctx` and callback `ctx` objects are concrete projection instances.
This page absorbs the old Scripting Projection Meta-Layer concept.
Scripting projection is capability projection; declaration and callback contexts are concrete projection cases rather
than a separate scripting-owned layer.

See also:

- [[Global Capability API Graph]]
- [[Capability Extension Slot]]
- [[Capability Path]]
- [[Script Safety]]

#glossary
