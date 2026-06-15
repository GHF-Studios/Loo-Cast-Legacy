---
canonical_name: Script Safety
status: WIP-draft
aliases: []
source_of_truth: []
---

Script Safety is the invariant that raw access to the unrestricted global capability/API graph is script-unsafe and is
therefore disallowed.
Script execution must use projected contextual facades.
Policy default is whitelist with optional blacklist override.

Lifecycle boundary:
Scripts may influence behavior through declarations, parameters, policy logic, hooks, messages/events, and declared
callbacks.
They do not own the fundamental lifecycle scheduling structure of the runtime.
Scheduling may still expose sanctioned extension/configuration points when the host contract allows them.

See also:

- [[Global Capability API Graph]]
- [[Capability Projection API]]
- [[Script Profile]]
- [[Callback Profile]]

#glossary
