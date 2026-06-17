---
canonical_name: Packagepack
status: WIP-draft
aliases:
  - packagepack
---

A Packagepack is the complete user/modpack-author-facing launch composition in the [[Vapor Ecosystem]].
It is the spiritual successor to the ambiguous unqualified `package`, but with a deliberately narrower meaning.

Current owner-answer-informed framing:

- A Packagepack selects exactly one [[Enginepack]].
- A Packagepack selects exactly one [[Gamepack]] compatible with that Enginepack.
- A Packagepack includes the compatible [[Modpack]]s, [[Engine Mod]]s, [[Game Mod]]s, and [[Extension Mod]]s selected for
  that launch composition.
- A Packagepack carries or produces resolved compatibility/fingerprint metadata for the selected stack.
- A Packagepack is the thing the launcher validates, summarizes, and launches as one configured Vapor instance.

Boundary:
A Packagepack is not a Steam package, build artifact, source package, runtime artifact, or arbitrary archive.
Those terms must stay qualified because they describe distribution/build/runtime containers rather than the selected
composition itself.

See also:

- [[Enginepack]]
- [[Gamepack]]
- [[Modpack]]
- [[Vapor Product Instance Stack]]

#glossary
