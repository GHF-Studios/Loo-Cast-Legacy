---
canonical_name: Script Safety
status: WIP-draft
aliases: []
---

Script Safety is the invariant that raw access to the unrestricted global capability/API graph is script-unsafe and is
therefore disallowed.
Script execution must use projected contextual facades.
Policy default is whitelist with optional blacklist override.

Lifecycle boundary:
Scripts may influence behavior through declarations, parameters, policy logic, hooks, messages/events, and declared
callbacks.
They do not own the fundamental lifecycle scheduling structure of the runtime.
Scheduling may still expose sanctioned extension/configuration points when the host contract allows them, but the exact
meaning of "extension point" remains unresolved.
The default posture for hooks/events/messages should be non-consuming and non-cancellable unless a specific host
contract explicitly allows consumption or cancellation.

Asset boundary:
[[Rhai Asset]] declaration files are the canonical authored asset format.
Generated textures, models, sounds, and other media payloads are runtime outputs, caches, or delivery artifacts rather
than traditional authored source assets.

Capability boundary:
Rhai should not define fully independent capabilities without Rust host support except for trivial script-local
computation.
Low-level access and heavy execution must stay mediated by host capabilities.

See also:

- [[Global Capability API Graph]]
- [[Capability Projection API]]
- [[Script Profile]]
- [[Callback Type]]
- [[Callback Context Type]]

#glossary
