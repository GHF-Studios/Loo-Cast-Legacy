---
canonical_name: Global Capability Surface
status: WIP-draft
aliases: []
---

A Global Capability Surface is a deliberately global capability/API surface for rudimentary cross-domain utilities
such as math and logging.
It is globally scoped by contract, but it is not a blanket authority grant over domain state.
This is distinct from raw unrestricted host-global graph access.

Global availability does not bypass [[Capability Graph Scope Envelope]] constraints and does not expand
script/profile/template
authority beyond declared limits.
Runtime policy can narrow or deny access in context and can later re-open access, but only inside the same envelope.

See also:

- [[Capability Graph Scope Envelope]]
- [[Dynamic Authority Resolution]]
- [[Capability Resolution Semantics]]
- [[Global Capability API Graph]]

#glossary
