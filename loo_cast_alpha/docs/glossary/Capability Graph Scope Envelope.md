---
canonical_name: Capability Graph Scope Envelope
status: WIP-draft
aliases:
  - Scope Envelope
---

The Capability Graph Scope Envelope is the hard maximum projected API/context scope allowed for a profile-bound script
or callback execution context.
Runtime policy can dynamically narrow access and later re-open previously narrowed paths, but this movement is always
inside the same envelope.
Runtime narrowing/re-opening cannot create scope that was never part of the envelope.

This applies to both domain-specific capability surfaces and explicitly declared global rudimentary surfaces.
The envelope is an umbrella concept: declaration entrypoint and callback entrypoint projections each use concrete
envelope instances.

See also:

- [[Capability Path]]
- [[Capability Projection API]]
- [[Dynamic Authority Resolution]]
- [[Global Capability Surface]]
- [[Script Safety]]

#glossary
