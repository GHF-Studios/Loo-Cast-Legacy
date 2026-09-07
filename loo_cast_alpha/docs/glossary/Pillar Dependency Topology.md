---
canonical_name: Pillar Dependency Topology
status: draft
aliases: []
---

The Pillar Dependency Topology maps role-level dependencies across the [[Vapor Ecosystem]], [[Spacetime Engine]], and
[[Loo Cast]].
The current owner-answer-informed stack is closer to:

```text
Vapor Ecosystem -> Spacetime Engine -> Loo Cast
```

The [[Vapor Ecosystem]] is the Steam-specific SDK/modding/distribution layer.
The [[Spacetime Engine]] is the first-party engine/framework product developed and distributed through that ecosystem.
The [[Loo Cast]] game is the first-party playable/content expression built on that stack.
[[USF]] is a pivotal module/framework part inside the Spacetime Engine, not a separate product pillar.

Replacement is architectural, not merely philosophical:
engines, games, and mods are intended to be first-party instances of broader ecosystem concepts.
Replacing the engine product may leave only Vapor-level ecosystem conventions in common with the first-party Loo Cast
stack, but USF itself is not currently framed as a directly replaceable product layer.

#glossary
