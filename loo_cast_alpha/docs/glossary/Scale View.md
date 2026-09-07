---
canonical_name: Scale View
status: WIP-draft
aliases: []
---

A Scale View is the observer-relative projection and traversal state over [[Scale]] coordinates.
It determines which scale context is actively observed and how traversal across scales is interpreted.
It is a semantic/view-state concept rather than a specific rendering, camera, or chunk-streaming implementation model.
Scale View constrains detail selection in [[Observer-Relative Simulation]] without redefining [[Scale Definition]] or
[[Scale Slice]].

Pre-alpha scope note:
Assume one primary observer/player/chunk-loader-equivalent scale view for now.
Multi-observer or multiplayer divergence between active scale views is out of scope until explicitly reopened.

Current owner-answer-informed boundary:
The active scale should be treated as the first-class change-authority scale, not simply the only simulated scale.
Higher scales remain simulated through slower/scaled-time semantics.
Lower-scale detail may be sampled, scoped-simulated, or temporarily inspected without becoming broad lower-scale active
simulation.

#glossary
