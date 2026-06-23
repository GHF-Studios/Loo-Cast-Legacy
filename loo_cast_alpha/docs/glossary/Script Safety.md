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
Scripts may influence behavior through data assets, declarations, parameters, policy data/rules, hooks,
messages/events, and declared callbacks.
They do not own the fundamental lifecycle scheduling structure of the runtime.
Scheduling may still expose sanctioned extension/configuration points when the host contract allows them, but the exact
meaning of "extension point" remains unresolved.
The default posture for hooks/events/messages should be non-consuming and non-cancellable unless a specific host
contract explicitly allows consumption or cancellation.

Asset boundary:
[[Rhai Asset]] declaration files are the canonical authored Rhai source format for typed capability material.
Rhai data assets include config, localization, constants, tuning values, authored tables, and similar structured
payloads.
Rhai declaration assets may contain data and metadata, but declaration-level executable behavior must be represented as
sanctioned callbacks.
Generated textures, models, sounds, and other media payloads are runtime outputs, caches, or delivery artifacts rather
than traditional authored source assets.

Capability boundary:
Rhai should not define fully independent capabilities without Rust host support except for trivial script-local
computation.
Low-level access, native integration, simulation kernels, IO, and authoritative mutation must stay mediated by
[[Rust Host Contract]]s, [[Scriptable Rust Surface]]s, [[Capability Kernel]]s, or other host capabilities.

See also:

- [[Global Capability API Graph]]
- [[Capability Projection API]]
- [[Capability Extension Slot]]
- [[Callback Type]]
- [[Callback Context Type]]
- [[Rust Host Contract]]
- [[Capability Kernel]]

#glossary
