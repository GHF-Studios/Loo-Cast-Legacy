---
canonical_name: Mod Runtime Representation
status: WIP-draft
aliases: []
---

The Mod Runtime Representation defines the in-memory integrated form of a mod after loading and registration.
The runtime resolves capabilities established at Runtime Lock (for example [[Scale Definition]], metric, phenomenon, and
[[Scale Realizer]] declarations) into active capabilities and executable behavior through
the [[Modding Runtime]],
the [[Capability Runtime]],
and the [[USF Runtime]].
Reach and exposure are runtime-resolved from typed declaration context and API-surface gating rather than free
per-capability-instance knobs.
Scale binding remains explicit and required even when multiple scale-bound paths map to one shared internal
implementation.

#glossary
