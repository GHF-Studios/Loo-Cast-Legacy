---
canonical_name: Capability Kernel
status: WIP-draft
aliases:
  - Native Capability Kernel
---

A Capability Kernel is module-scoped native implementation backing for a [[Capability Module]].
It is the current WIP umbrella term for Rust-native code that performs domain work behind capability declarations,
callbacks, projections, and runtime behavior.

A kernel may expose many Rust functions, types, constructors, adapters, systems, callbacks, or registries.
It should not be interpreted as one tiny executable unit per capability.

Likely forms:

- built-in Rust code registered directly by `vapor_core`
- engine-provided or game-provided Rust modules
- Bevy systems or adapters
- platform-specific [[Kernel Artifact]] shared objects
- wrapper/adaptor code around std, Tokio, Steam, OS APIs, Bevy, or other external crates

Relationship to modules:
A Capability Module may import one or more kernels.
A kernel may back many types, traits, callbacks, and capability nodes inside that module.
Dynamic-library delivery is module-scoped by default, but the native contents inside the library can expose a richer
internal graph through registration metadata.

Boundary:
Private native kernel internals should not be re-exported as raw kernels.
Modules should re-export capability surfaces, traits, callbacks, data, or extension slots.
Host contracts then resolve those public surfaces to allowed native registry entries.

Open pressure:
The exact `Scriptable` trait family, registration metadata, kernel packaging shape, and runtime binding model remain
unsettled.

See also:

- [[Kernel Artifact]]
- [[Rust Host Contract]]
- [[Scriptable Rust Surface]]
- [[Rust Surface Graph]]
- [[Capability Module]]

#glossary
