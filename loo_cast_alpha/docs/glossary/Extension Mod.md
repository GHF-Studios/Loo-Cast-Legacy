---
canonical_name: Extension Mod
status: WIP-draft
aliases:
  - ExtensionMod
  - Mod Mod
  - mod of a mod
---

An Extension Mod is the current best generic name for a mod whose parent attachment target is another mod rather than
the root [[Engine]] or [[Game]] fixture.

Current owner-answer-informed framing:

- An Extension Mod attaches to exactly one parent in the mod parent/child hierarchy.
- Its capabilities may use surfaces available through its mod ancestry.
- It cannot use capability surfaces that its parent/ancestry has not made available through the inherited composition
  structure.
- This parent/ancestry model is intentionally inheritance-like at the mod-composition level, even though the underlying
  [[Capability]] graph may still contain dependency and interaction edges across other visible surfaces.

Boundary:
`Extension Mod` is still a working term, but it is clearer than using plain `Mod` for both root-targeted and
mod-targeted contributions.

See also:

- [[Engine Mod]]
- [[Game Mod]]
- [[Modpack]]
- [[Capability]]

#glossary
