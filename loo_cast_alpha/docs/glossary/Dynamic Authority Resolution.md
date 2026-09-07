---
canonical_name: Dynamic Authority Resolution
status: WIP-draft
aliases: []
---

Dynamic Authority Resolution means effective authority is resolved at runtime by lifecycle context and operation,
rather than fixed as one global static fact.
Definition authority, runtime authority, and output/application authority stay distinct, and active authority is
interpreted per phase.

Runtime policy can narrow or re-open access per context, but cannot expand authority beyond
the [[Capability Graph Scope Envelope]].
Globally scoped rudimentary capability surfaces may be active across many contexts, but this does not imply global
domain-state authority.

Concrete examples:

- declaration-time authority can validate a [[Capability Declaration]] without owning runtime state mutation
- callback-time authority can expose a narrower or re-opened `ctx` projection without widening beyond the graph envelope
- reconcile/commit/apply authority can own canonical state progression even when a capability emitted the original
  request or intent
- persistence authority can reject or abort unsafe writes even when runtime simulation authority produced new state

See also:

- [[Capability Role]]
- [[Capability Graph Scope Envelope]]
- [[Capability Projection API]]
- [[Global Capability Surface]]
- [[USF Runtime Evolution Lifecycle]]

#glossary
