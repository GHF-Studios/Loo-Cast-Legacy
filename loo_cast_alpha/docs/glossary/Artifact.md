---
canonical_name: Artifact
status: WIP-draft
aliases: []
---

Artifact is a broad term for a concrete file/folder/archive/build/distribution object produced, consumed, installed, or
validated by the [[Vapor Ecosystem]].

Current pressure:

- [[Source Artifact]]s are authoring-side files and folders.
- [[Build Artifact]]s are built outputs that are not yet assembled/packaged for distribution.
- [[Distributable Artifact]]s are final packaged uploadable/distributable objects, including Workshop-uploadable forms.
- `Runtime artifact` should be avoided for in-memory representations because in-memory values are not artifacts by this
  definition.
- Workshop artifacts are Steam-distributed items or downloaded item contents and should usually be discussed as
  [[Distributable Artifact]]s or [[Steam Workshop]] items.
- Packagepacks, enginepacks, gamepacks, modpacks, and mods can have local grouped/aggregated artifact forms as well as
  published Workshop item forms that reference dependencies.

Boundary:
Artifact should not be used as a synonym for in-memory Capability.
When precision matters, prefer [[Source Artifact]], [[Build Artifact]], [[Distributable Artifact]], Workshop item,
[[Packagepack]],
[[Enginepack]], [[Gamepack]], or [[Modpack]].

#glossary
