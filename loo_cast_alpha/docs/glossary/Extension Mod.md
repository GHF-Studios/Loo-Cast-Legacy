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

- An Extension Mod attaches to another mod-like artifact as its target role.
- That relationship should be explicit [[Vapor.toml]] composition/placement metadata, not implicit code logic.
- Its capabilities may use only the public capability surfaces it explicitly depends on and can see through normal
  visibility rules.
- No implicit defaults should be assumed from folder nesting or attachment target.
- Normal dependencies, conflicts, target role constraints, and folder placement metadata are all declared explicitly.

Boundary:
`Extension Mod` is still a working term, but it is clearer than using plain `Mod` for both root-targeted and
mod-targeted contributions.

See also:

- [[Engine Mod]]
- [[Game Mod]]
- [[Modpack]]
- [[Capability]]

#glossary
